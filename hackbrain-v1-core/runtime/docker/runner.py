import json
import sys
import time

def emit(event):
    sys.stdout.write(json.dumps(event) + "\n")
    sys.stdout.flush()

emit({
    "type": "JobStarted",
    "timestamp": time.time()
})

for line in sys.stdin:
    try:
        cmd = json.loads(line)
        if cmd.get("type") == "execute":
            emit({
                "type": "StepExecuted",
                "step": cmd.get("step", "unknown"),
                "timestamp": time.time()
            })
    except json.JSONDecodeError:
        pass

emit({
    "type": "JobFinished",
    "timestamp": time.time()
})
