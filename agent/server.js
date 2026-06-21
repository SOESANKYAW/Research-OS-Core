const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const LocalExecutor = require('./local_executor');
const ArtifactSync = require('./artifact_sync');
const EventBus = require('./event_bus');
const path = require('path');
const portsConfig = require('../shared/config/ports.json');

const app = express();
app.use(cors());
app.use(bodyParser.json());

// Expose synced artifacts statically to the frontend
app.use('/artifacts', express.static(path.join(__dirname, 'sync_dir')));

const eventBus = new EventBus(path.join(__dirname, 'logs'));
let privateKey = '';
const defaultKeyPath = process.env.OSPC_KEY_PATH || path.join(process.env.HOME, '.ssh/id_rsa');
try {
  privateKey = require('fs').readFileSync(defaultKeyPath);
} catch (e) {
  console.warn(`[WARNING] SSH Private Key not found at ${defaultKeyPath}. Remote execution will fail if triggered.`);
}

const sshConfig = {
  host: process.env.OSPC_HOST || '127.0.0.1',
  port: 22,
  username: process.env.OSPC_USER || 'root',
  privateKey: privateKey
};

const executor = new LocalExecutor();
const syncer = new ArtifactSync({
  ...sshConfig,
  privateKeyPath: defaultKeyPath,
  remoteDir: '/var/log/wal',
  localDir: path.join(__dirname, 'sync_dir')
});

app.post('/api/submit_job', async (req, res) => {
  const manifest = req.body;
  
  if (!manifest || !manifest.job_id) {
    return res.status(400).json({ success: false, error: "Invalid JobManifest" });
  }

  // State: SCHEDULED
  manifest.status = "SCHEDULED";
  eventBus.dispatch("JOB_STATE_CHANGE", sshConfig.host, manifest);
  
  // Acknowledge receipt back to the hb-run compiler immediately
  res.json({ success: true, message: "Job accepted by Control Agent", job_id: manifest.job_id });

  // Async Execution Pipeline (Non-blocking)
  (async () => {
    try {
      // State: RUNNING
      manifest.status = "RUNNING";
      eventBus.dispatch("JOB_STATE_CHANGE", sshConfig.host, manifest);

      // Execute via strictly sandboxed Docker boundary
      const result = await executor.dispatchJob(manifest);

      // State: COMPLETED
      manifest.status = "COMPLETED";
      eventBus.dispatch("JOB_STATE_CHANGE", sshConfig.host, manifest);
      
      // Phase 3.4: Artifact Binding Layer
      console.log(`[Data Plane] Extracting and hashing artifacts for ${manifest.job_id}...`);
      const artifacts = await syncer.syncArtifacts(manifest.job_id);
      
      manifest.artifact_manifest = artifacts.map(a => a.artifact_hash);
      
      // State: SEALED
      manifest.status = "SEALED";
      eventBus.dispatch("JOB_STATE_CHANGE", sshConfig.host, manifest);
      console.log(`[Control Plane] Job ${manifest.job_id} successfully SEALED with ${artifacts.length} artifacts.`);
    } catch (error) {
      manifest.status = "FAILED";
      eventBus.dispatch("JOB_STATE_CHANGE", sshConfig.host, { ...manifest, error: error.message });
    }
  })();
});

app.post('/api/sync', async (req, res) => {
  const { job_id } = req.body;
  if (!job_id) return res.status(400).json({ success: false, error: "job_id required for sync" });

  const event = eventBus.dispatch('SYNC_ARTIFACTS', sshConfig.host, { job_id });
  try {
    const artifacts = await syncer.syncArtifacts(job_id);
    res.json({ success: true, event, artifacts });
  } catch (error) {
    res.status(500).json({ success: false, event, error: error.message });
  }
});

app.get('/api/events', (req, res) => {
  res.json({ events: eventBus.getHistory() });
});

app.get('/api/events/stream', (req, res) => {
  res.setHeader('Content-Type', 'text/event-stream');
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Connection', 'keep-alive');
  res.flushHeaders();

  // Push existing history immediately to sync state
  const history = eventBus.getHistory();
  history.forEach(event => res.write(`data: ${JSON.stringify(event)}\n\n`));

  // Subscribe to live events
  eventBus.subscribe(res);

  req.on('close', () => {
    eventBus.unsubscribe(res);
  });
});

app.get('/api/services', (req, res) => {
  res.json({
    status: 'online',
    registry: portsConfig
  });
});

const PORT = process.env.PORT || portsConfig.control_agent_port;
app.listen(PORT, '0.0.0.0', () => {
  console.log(`Node Control Agent running on port ${PORT} (Listening on all network interfaces)`);
});

// Force event loop to stay alive (Node 22 patch for local environment)
setInterval(() => {}, 1000 * 60 * 60);
