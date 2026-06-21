# HackBrain Phase 5: WAL Crash Consistency Tasks

## 1. Prepare Payload
- `[x]` User creates `wal_physics.py` on the Ubuntu machine.

## 2. Execute & Crash
- `[x]` Fire the initial payload from the Mac.
- `[x]` Observe `D1_STABLE` chunk streams.
- `[x]` User executes `docker kill` on Ubuntu to crash the container.
- `[x]` Verify Mac Dashboard tracks the failure event correctly.

## 3. Recover
- `[x]` Fire the payload from the Mac again.
- `[x]` Verify script reads `state.wal` and successfully recovers monotonic state.
