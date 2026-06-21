const { exec } = require('child_process');

class LocalExecutor {
  constructor() {}

  async executeCommand(command) {
    return new Promise((resolve, reject) => {
      exec(command, (error, stdout, stderr) => {
        if (stdout) console.log(stdout);
        if (stderr) console.error(stderr);
        
        if (error) {
          // Docker execution errors or crashes
          resolve({ code: error.code || 1, signal: error.signal, stdout, stderr });
        } else {
          resolve({ code: 0, signal: null, stdout, stderr });
        }
      });
    });
  }

  async dispatchJob(manifest) {
    // Ensure strict Docker invariants are enforced locally
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

    console.log(`[LocalExecutor] Dispatching secured Docker payload for ${manifest.job_id}`);
    return this.executeCommand(dockerCmd);
  }
}

module.exports = LocalExecutor;
