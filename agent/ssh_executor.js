const { Client } = require('ssh2');

class SSHExecutor {
  constructor(config) {
    this.config = config;
  }

  async executeCommand(command) {
    return new Promise((resolve, reject) => {
      const conn = new Client();
      conn.on('ready', () => {
        conn.exec(command, (err, stream) => {
          if (err) {
            conn.end();
            return reject(err);
          }
          let stdout = '';
          let stderr = '';
          
          stream.on('close', (code, signal) => {
            conn.end();
            resolve({ code, signal, stdout, stderr });
          }).on('data', (data) => {
            stdout += data;
          }).stderr.on('data', (data) => {
            stderr += data;
          });
        });
      }).on('error', (err) => {
        reject(err);
      }).connect(this.config);
    });
  }

  async dispatchJob(manifest) {
    // Ensure strict Docker invariants are enforced over SSH
    const dockerCmd = [
      'docker run',
      '--rm',
      '--network=none',
      '--privileged=false',
      `-v ${manifest.mount_spec}`,
      '-w /job/workdir',
      manifest.image_hash,
      ...manifest.command
    ].join(' ');

    console.log(`[SSHExecutor] Dispatching secured Docker payload for ${manifest.job_id}`);
    return this.executeCommand(dockerCmd);
  }
}

module.exports = SSHExecutor;
