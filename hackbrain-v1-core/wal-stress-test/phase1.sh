#!/bin/bash
cd /Users/soesan/.gemini/antigravity-ide/scratch/hackbrain-v2/wal-stress-test

rm -f wal.log stress.tmp

./wal_writer ./wal.log > /dev/null 2>&1 &
A=$!

dd if=/dev/zero of=./stress.tmp bs=1M > /dev/null 2>&1 &
B=$!

sleep 2

kill -9 $A $B > /dev/null 2>&1
sync

./verify_wal ./wal.log
