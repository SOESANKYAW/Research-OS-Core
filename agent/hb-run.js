#!/usr/bin/env node

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const portsConfig = require(path.join(__dirname, '../shared/config/ports.json'));

async function compileAndSubmit() {
  const args = process.argv.slice(2);
  
  if (args.length === 0) {
    console.error("Error: You must provide a command. Example: hb-run python analyze.py");
    process.exit(1);
  }

  const jobId = `job_${Date.now()}_${crypto.randomBytes(4).toString('hex')}`;
  const inputHash = crypto.createHash('sha256').update(args.join(' ')).digest('hex'); // Simplified for now
  
  const manifest = {
    job_id: jobId,
    status: "PENDING",
    input_hash: inputHash,
    image_hash: "python:3.9-slim", 
    command: args,
    mount_spec: `${process.cwd()}:/job/workdir`,
    artifact_manifest: []
  };

  console.log(`[Job Compiler] Generated Manifest for ${jobId}`);
  console.log(`[Job Compiler] Input Hash: ${inputHash}`);
  console.log(`[Job Compiler] Submitting to Node Control Agent...`);

  try {
    const response = await fetch(`http://127.0.0.1:${portsConfig.control_agent_port}/api/submit_job`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(manifest)
    });

    const data = await response.json();
    if (data.success) {
      console.log(`[Job Compiler] Job ${jobId} successfully SCHEDULED on Execution Plane.`);
    } else {
      console.error(`[Job Compiler] Dispatch failed:`, data.error);
    }
  } catch (error) {
    console.error(`[Job Compiler] FATAL: Could not reach Node Agent at 127.0.0.1:${portsConfig.control_agent_port}.\n${error.message}`);
  }
}

compileAndSubmit();
