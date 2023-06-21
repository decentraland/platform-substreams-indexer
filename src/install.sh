# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install essentials
sudo apt-get install build-essential

# Restart shell so rust becomes available
source "$HOME/.cargo/env"

# Install postgres client
sudo apt install postgresql-client -y 

# Download substreams binary
wget https://github.com/streamingfast/substreams/releases/download/v1.1.5/substreams_linux_x86_64.tar.gz
tar zxf substreams_linux_x86_64.tar.gz
rm substreams_linux_x86_64.tar.gz

# Download postgres sink binary
wget https://github.com/streamingfast/substreams-sink-postgres/releases/download/v2.3.2/substreams-sink-postgres_linux_x86_64.tar.gz
tar zxf substreams-sink-postgres_linux_x86_64.tar.gz
rm substreams-sink-postgres_linux_x86_64.tar.gz
