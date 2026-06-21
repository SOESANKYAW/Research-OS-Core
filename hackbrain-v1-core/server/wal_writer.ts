import { spawn } from 'child_process';
import * as crypto from 'crypto';

// Types mirroring the WAL Spec
export type ExecutionFingerprint = {
    provider: string;
    model: string;
    model_version: string;
    temperature: number;
    top_p?: number;
    reasoning_mode: "fast" | "balanced" | "deep";
    runtime_version: string;
    prompt_hash: string;
    environment_hash: string;
};

export type WALEventInput = {
    type: string;
    [key: string]: any;
};

export type WALRecord = {
    version: string;
    job_id: string;
    run_id: string;
    created_at: number;
    sequence: number;
    prev_hash: string;
    event_hash: string;
    fingerprint: ExecutionFingerprint;
    event: WALEventInput;
};

// Canonical JSON stringifier
function canonicalize(obj: any): string {
    if (obj === null || typeof obj !== 'object') {
        return JSON.stringify(obj);
    }
    if (Array.isArray(obj)) {
        return '[' + obj.map(canonicalize).join(',') + ']';
    }
    const keys = Object.keys(obj).sort();
    let result = '{';
    for (let i = 0; i < keys.length; i++) {
        if (i > 0) result += ',';
        result += JSON.stringify(keys[i]) + ':' + canonicalize(obj[keys[i]]);
    }
    result += '}';
    return result;
}

/**
 * The Epistemic Choke Point
 * No WAL mutation path exists outside this class.
 */
export class WALWriter {
    private rustBinPath: string;

    constructor(rustBinPath: string = '../kernel/target/release/hackbrain-wal') {
        this.rustBinPath = rustBinPath;
    }

    /**
     * Accepts events from the Orchestrator.
     * Validates fields, canonicalizes, and delegates to Rust for true mutation.
     */
    public async appendEvent(
        jobId: string, 
        runId: string, 
        fingerprint: ExecutionFingerprint, 
        event: WALEventInput
    ): Promise<WALRecord> {
        
        // 1. Validate required fields
        if (!jobId || !runId || !event || !event.type) {
            throw new Error("MALFORMED_EVENT: Missing required WAL fields");
        }
        if (!fingerprint || !fingerprint.environment_hash) {
            throw new Error("FINGERPRINT_MISSING: Cannot append without execution identity");
        }

        // 2. Prepare Canonical Payload
        const payload = {
            job_id: jobId,
            run_id: runId,
            fingerprint: fingerprint,
            event: JSON.parse(canonicalize(event)) // Ensure deep canonical structure
        };

        // 3. Delegate to Rust WAL Engine via Subprocess
        // The Rust engine enforces append-only rules, calculates hashes, and writes to disk.
        return this.executeRustAppend(payload);
    }

    private executeRustAppend(payload: any): Promise<WALRecord> {
        return new Promise((resolve, reject) => {
            const child = spawn(this.rustBinPath, ['append']);
            let output = '';
            let errorOut = '';

            child.stdout.on('data', (data) => {
                output += data.toString();
            });

            child.stderr.on('data', (data) => {
                errorOut += data.toString();
            });

            child.on('close', (code) => {
                if (code !== 0) {
                    reject(new Error(`RUST_WAL_ERROR: ${errorOut}`));
                } else {
                    try {
                        const record = JSON.parse(output) as WALRecord;
                        resolve(record);
                    } catch (e) {
                        reject(new Error("RUST_WAL_MALFORMED_RESPONSE: Failed to parse Rust output"));
                    }
                }
            });

            // Stream payload to Rust process stdin
            child.stdin.write(JSON.stringify(payload));
            child.stdin.end();
        });
    }

    /**
     * Triggers the deterministic replay engine.
     */
    public async replay(jobId: string, fromSeq?: number): Promise<any> {
        return new Promise((resolve, reject) => {
            const args = ['replay', '--job', jobId];
            if (fromSeq !== undefined) {
                args.push('--from', fromSeq.toString());
            }

            const child = spawn(this.rustBinPath, args);
            let output = '';

            child.stdout.on('data', (data) => {
                output += data.toString();
            });

            child.on('close', (code) => {
                if (code !== 0) reject(new Error("REPLAY_ENGINE_FAILED"));
                else resolve(JSON.parse(output));
            });
        });
    }
}
