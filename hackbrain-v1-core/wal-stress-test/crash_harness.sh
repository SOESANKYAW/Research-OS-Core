#!/bin/bash

BIN=./target/debug/wal-stress-test
LOG=./wal.log

rm -f $LOG

$BIN &
PID=$!

echo "Runner PID: $PID"

for i in {1..10}; do
    sleep $((RANDOM % 3 + 1))

    echo "[CRASH $i] killing process"
    kill -9 $PID

    sleep 1

    echo "[RESTART]"
    $BIN &
    PID=$!
done

kill -9 $PID
echo "Done."
