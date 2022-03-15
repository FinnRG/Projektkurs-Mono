curl https://sh.rustup.rs -sSf | sh -s -- -y
apt update
apt -y update
apt -y install pkg-config libssl-dev libpq-devapt ffmpeg
source $HOME/.cargo/env
cargo install diesel_cli --no-default-features --features postgres


cd frontend
npm install
npm start &
cd ../backend
