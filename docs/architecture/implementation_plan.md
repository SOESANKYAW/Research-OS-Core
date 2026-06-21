# WAL Crash Consistency Experiment

We will conduct a physical `kill -9` simulation to observe how our distributed system handles catastrophic failure and automatic state recovery, relying strictly on Write-Ahead Log (WAL) mechanics.

## User Review Required

> [!IMPORTANT]
> This experiment requires you to have **two terminal windows open on your Ubuntu machine** so you can run the `kill` command while the server is running in the background.

## Open Questions

None at this time. The mechanics of Linux crash-consistency are deterministic.

## Proposed Changes

We will introduce a new executable payload on the Ubuntu machine.

### `wal_physics.py` (To be created on Ubuntu)

#### [NEW] `wal_physics.py`

I will provide a simple, robust Python script for you to create on your Ubuntu machine. It will:
1. **Startup Check:** Look for a `state.wal` file. If it exists, read the last entry and recover its physical state.
2. **Execution Loop:** Continuously compute a heavy "physics" workload (e.g., matrix hashes).
3. **Fsync:** Before updating its internal state, physically write and `fsync()` the result to the `state.wal` file to guarantee it hits the bare metal disk.
4. **Delay:** Pause for 2 seconds per cycle, giving you an opening to trigger the crash.

## Verification Plan

We will perform the following manual verification sequence:

### Manual Verification
1. **Launch:** You will fire the payload from your Mac terminal using `curl`.
2. **Observe:** We watch the Mac Dashboard Trace Viewer stream the WAL events live.
3. **Execution:** On your Ubuntu machine, you will run `docker kill $(docker ps -q)` to instantly destroy the running container mid-execution.
4. **Observability Check:** We verify that the Mac Dashboard accurately captures the truncation/failure of the event stream.
5. **Recovery:** You will fire the exact same `curl` payload from your Mac terminal again.
6. **Validation:** We watch the Ubuntu logs and Mac dashboard to verify that the script successfully reads `state.wal`, recovers exactly where it left off, and resumes execution seamlessly.
