# HackBrain Phase 0: Empirical Phase Diagram
**Subject:** State persistence under Linux write-path stress.
**System Characteristic:** The system is a discrete regime machine with sharp, discontinuous boundary transitions between IO semantics, not a smooth phase space.

## 1. Durability Field (WAL Semantics)
**Observation:** The Write-Ahead Log abstraction is not continuously deformable under stress. Extreme block IO does not cause "gradual corruption"—it causes a hard phase transition into non-log semantics.
*   **Regime D1 (Under-Capacity Writeback):** Both writers buffered cleanly. Deterministic merge into the page cache without triggering aggressive reclaim.
*   **Regime D2 (Eviction Arbitration Phase):** Dirty ratio threshold crossed. Kernel begins prioritizing reclaim. Tail truncation appears as writeback threads arbitrate between competing anonymous write pressures.
*   **Regime D3 (Crash Window Collapse):** `kill -9` intersects the dirty page flush cycle. Partial eviction + partial commit leads to non-reconstructible prefix loss.

## 2. Contention Field (Concurrency & Scheduling)
**Observation:** Line-level atomicity ($\le$ `PIPE_BUF`) is strictly preserved by the kernel. User-space timestamp drift is non-causal noise and does not represent absolute commit order.
*   **Regime C1 (Alternating Fairness):** Scheduler dominant. Classic `A B A B` structure.
*   **Regime C2 (Burst Batching):** Cache flush coalescing dominates. OS groups writes per process (`A x 100`, `B x 100`).
*   **Regime C3 (Mixed Interaction):** *[Observed]* Irregular clusters. Kernel scheduler and dirty page flush mechanisms actively interacting.

## 3. Reconstruction Field (State Recovery)
**Observation:** Reconstruction requires fsync discipline, explicit commit markers, and zero block-level overwrite contamination. When boundary conditions fail (e.g., transition to Regime D3), reconstruction space collapses discontinuously.
*   **Regime R1 (Full Recoverability):** All states mapped.
*   **Regime R2 (Partial/Prefix Recoverability):** Clean truncation of un-fsync'd tail.
*   **Regime R3 (Null Recoverability):** *[Observed]* 0 recoverable commits due to catastrophic causal chain severance.

---

### Phase Space Closure Status
*   **Signature Space Saturation (SSS):** ❌ **FAIL** (WAL failure and total overwrite are distinct classes; the boundary between IO pressure and direct overwrite is unmapped).
*   **Regime Stability (RS):** ⚠️ **PARTIAL** (Observed in contention field, but untested across varying `dirty_ratio` or IO depth parameters).
*   **Reconstruction Closure (RC):** ✔️ **PASS** (Finite, discrete collapse modes established).
