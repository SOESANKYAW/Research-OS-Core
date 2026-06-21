# HackBrain WAL Execution Checklist

> [!CAUTION]
> **Hard Stop Rule:** If you feel the urge to model the OS, formalize environment behavior, build DSLs, or simulate adversarial kernels — you are outside the system boundary. Return to: *"What happens if I run `kill -9` during `fsync` on a real disk?"*

Use this 1-page boundary rule before adding ANY new idea to HackBrain Studio.

### 1. ❓ Is this required for the 3 WAL properties?
Only answer **YES** if it directly improves:
- **Crash consistency:** (`SIGKILL` $\rightarrow$ safe recovery)
- **Storage durability approximation:** (`fsync` + journaling correctness under pressure)
- **Recovery correctness:** (prefix truncation + hash-chain validity)

$\rightarrow$ **If NO: STOP immediately.**

### 2. 🧪 Can this be tested with real Linux tools?
Ask: *"Can I test this with `kill -9`, `fio`, `drop_caches`, or `fsync` behavior?"*

$\rightarrow$ **YES:** Valid.
$\rightarrow$ **NO:** Likely over-design. If it requires simulation, DSL, or modeling: **STOP.**

### 3. 🔁 Does it add a new system?
Check: new runtime? new abstraction layer? new compiler / DSL / framework?

$\rightarrow$ **If YES: STOP** (unless it strictly replaces a broken component).

### 4. ⚖️ Does it reduce or increase moving parts?
- Fewer components $\rightarrow$ **GOOD**
- More components $\rightarrow$ **DANGER**

$\rightarrow$ **If complexity increases without removing failure modes: STOP.**

### 5. 🧱 Is this observable in production logs?
If you cannot observe it via:
1. WAL log output
2. Crash recovery result
3. Filesystem state after restart

$\rightarrow$ **It is not part of the system. STOP.**

---

## 🧠 Core Philosophy
Real systems are not proven. They are stressed.
- **No abstraction layers** unless debugging requires it.
- **No formal models** unless production fails to explain behavior.
- **No frameworks** unless repetition becomes unmanageable.

> [!IMPORTANT]
> **The Mental Shortcut**
> Before any design decision, ask: *"Does this change what happens when I run `kill -9` + `fsync` + `recovery`?"*
> - YES $\rightarrow$ allowed
> - NO $\rightarrow$ over-design $\rightarrow$ **STOP**
