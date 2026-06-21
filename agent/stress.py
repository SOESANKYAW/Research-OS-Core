import os
import time

print("Starting Synthetic D2 Kernel Stress Test...")

# Create 50MB chunk of random bytes
chunk = os.urandom(1024 * 1024 * 50) 
file_path = "stress_test_artifact.dat"

print("Injecting asynchronous write pressure...")
try:
    with open(file_path, "wb") as f:
        # Write 5GB of data rapidly to flood the Linux page cache
        for i in range(100): 
            start = time.time()
            f.write(chunk)
            
            # We explicitly do NOT call f.flush()
            # This forces the OS to buffer dirty pages in memory.
            # When RAM fills up, the OS will panic and force a synchronous eviction (D2 Boundary).
            
            duration = time.time() - start
            
            # If a memory-buffer write suddenly takes > 0.05s, the kernel is blocking us!
            if duration > 0.05:
                print(f"[D2_BIFURCATION] Chunk {i}: OS blocked execution for {duration:.4f}s! (Eviction Pressure)")
            else:
                print(f"[D1_STABLE] Chunk {i}: Asynchronous write took {duration:.4f}s")
                
except Exception as e:
    print(f"Execution Error: {e}")

print("Stress test completed. Artifact generated.")
