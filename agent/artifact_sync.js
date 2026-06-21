const { exec } = require('child_process');
const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const util = require('util');

const execAsync = util.promisify(exec);

class ArtifactSync {
  constructor(config) {
    this.config = config; // { host, username, privateKeyPath, remoteDir, localDir }
  }

  async syncArtifacts(jobId) {
    const { host, username, privateKeyPath, localDir } = this.config;
    const remoteJobDir = `/hackbrain/jobs/${jobId}`;
    const localJobDir = path.join(localDir, jobId);
    
    // Ensure local directory exists
    if (!fs.existsSync(localJobDir)) {
      fs.mkdirSync(localJobDir, { recursive: true });
    }

    const command = `scp -i ${privateKeyPath} -r ${username}@${host}:${remoteJobDir}/* ${localJobDir}/`;
    
    try {
      await execAsync(command);
      return this.catalogArtifacts(localJobDir, jobId);
    } catch (error) {
      console.error("Sync failed:", error);
      throw error;
    }
  }

  catalogArtifacts(localJobDir, jobId) {
    const artifacts = [];
    const files = fs.readdirSync(localJobDir);

    for (const file of files) {
      const filePath = path.join(localJobDir, file);
      const stats = fs.statSync(filePath);
      
      if (stats.isFile()) {
        const fileBuffer = fs.readFileSync(filePath);
        const hashSum = crypto.createHash('sha256');
        hashSum.update(fileBuffer);
        
        artifacts.push({
          id: crypto.randomUUID(),
          job_id: jobId,
          type: this.determineType(file),
          path: filePath,
          size_bytes: stats.size,
          artifact_hash: hashSum.digest('hex'),
          timestamp: Date.now()
        });
      }
    }
    return artifacts;
  }

  determineType(filename) {
    if (filename.includes('wal')) return 'wal_segment';
    if (filename.includes('dump') || filename.includes('crash')) return 'crash_dump';
    if (filename.includes('metrics')) return 'metrics_log';
    return 'file_snapshot';
  }
}

module.exports = ArtifactSync;
