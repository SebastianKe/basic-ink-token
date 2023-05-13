# Simple token contract using !ink

## prerequisites
1. Install rustup
>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
>sh source ~/.cargo/env
2. Install nightly version as needed for smart contract development
>rustup toolchain install nightly
<br />
rustup component add rust-src --toolchain nightly \n
<br />
rustup target add wasm32-unknown-unknown --toolchain nightly
3. Install ink!
> brew install binaryen
<br />
cargo install --force --locked cargo-contract
## run tests
> cargo test
## compile 
The resulting files will be placed in mytoken/target/ink/ folder. If the compilation is successful you will find there the following 3 files:
- mytoken.wasm is a binary WASM file with the compiled contract
- metadata.json containing our contracts ABI (Application Binary Interface)
- mytoken.contract which bundles the above two for more convenient interaction with the chain explorer
> cargo +nightly contract build --release