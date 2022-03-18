#!/bin/bash

cargo build --release
cp target/release/ip /usr/local/bin/
echo '🍺 ip installed successfully!'
