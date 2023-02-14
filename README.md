# Noobuster
Async dirbuster-like tool. Developed in few minutes, then few hours.

## Quick start
- [Install Rust](https://www.rust-lang.org/tools/install) 
- `git clone https://github.com/Nariod/noobuster.git`
- `cd noobuster`
- `cargo run -- -t https://google.com -w wordlist.txt -r 200,301`

## Binary to go
Need to drop the binary on a box? Here you go.

Linux static binary
- [Install Rust](https://www.rust-lang.org/tools/install) 
- `git clone https://github.com/Nariod/noobuster.git`
- `cd noobuster`
- `sudo apt-get install pkg-config musl-tools`
- `rustup target add x86_64-unknown-linux-musl`
- `cargo build --target x86_64-unknown-linux-musl --release`

Windows static binary
- [Install Rust](https://www.rust-lang.org/tools/install) 
- `git clone https://github.com/Nariod/noobuster.git`
- `cd noobuster`
- `sudo apt-get install pkg-config musl-tools`
- `sudo apt-get install mingw-w64`
- `rustup target add x86_64-pc-windows-gnu`
- `cargo build --target x86_64-pc-windows-gnu --release`

## Legal disclaimer
Usage of anything presented in this repo to attack targets without prior mutual consent is illegal. It's the end user's responsibility to obey all applicable local, state and federal laws. Developers assume no liability and are not responsible for any misuse or damage caused by this program. Only use for educational purposes.