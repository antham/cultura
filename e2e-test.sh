#!/usr/bin/env bash

# Run e2e tests on cultura

echo "Start the daemon"

cargo run -- -e true daemon start

cultura_process_count=$(pgrep -c -f cultura)

if [ "$cultura_process_count" -eq 1 ]; then
    echo "The daemon is started"
else
    echo "The daemon is not properly started"
    exit 1
fi

cargo run daemon start

cultura_process_count=$(pgrep -c -f cultura)

if [ "$cultura_process_count" -eq 1 ]; then
    echo "The daemon is started only once"
else
    echo "The daemon is not started only once"
    exit 1
fi

echo "Check the config folder exists"
stat ~/.config/cultura/

echo

echo "Check the database exists"
stat ~/.config/cultura/cultura.db

echo

echo "Check the config file exists"
stat ~/.config/cultura/config.toml

echo "Dump logs"
cat ~/.config/cultura/stdout.log
cat ~/.config/cultura/stderr.log

echo

echo "Dump config"
cat ~/.config/cultura/config.toml

sleep 10

echo "Check the database size"
stat ~/.config/cultura/cultura.db

echo

cargo run fact generate-random >generate-random.out

echo "Dump the content of the generation command"
cat generate-random.out
echo

echo "Check if the generation command works"
grep -E "Did you know that|Today I learned" generate-random.out
echo

echo "Kill the daemon"
cargo run daemon stop
if [[ $(pgrep -c -f cultura || true) -ne 0 ]]; then
    echo "The daemon stop command failed"
    exit 1
fi
