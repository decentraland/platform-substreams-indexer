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



###### for ownership server
curl -sL https://deb.nodesource.com/setup_18.x -o /tmp/nodesource_setup.sh
sudo bash /tmp/nodesource_setup.sh
sudo apt install -y nodejs

# sudo apt-get install gcc g++ make
# curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor | sudo tee /usr/share/keyrings/yarnkey.gpg >/dev/null
#      echo "deb [signed-by=/usr/share/keyrings/yarnkey.gpg] https://dl.yarnpkg.com/debian stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
#      sudo apt-get update && sudo apt-get install yarn