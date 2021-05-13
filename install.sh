workdir .
cargo build --release
sudo cp target/release/ip /usr/local/bin/
echo '🍺 ip installed successfully!'
