# Rustを経験してみる

rust cargoのセットアップ
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
restart
rustc main.rs
./main
cargo new hello
hello/main.rs
cargo test

cargo new rvtest
rustup target add riscv32imac-unknown-none-elf
rustup install nightly
git clone --recursive https://github.com/riscv/riscv-gnu-toolchain //install riscv toolchain

## 2022/09/18
コンセプトから理解するRust　をやってみる

### 基本
cargo new start
cd start
実行は
cargo run
ビルドだけしたい場合は
cargo build
リリース時、コンパイルでの最適化をしながらするには
cargo build --release

rust-analyzer rust rls をvscodeに導入

## 既存のライブラリの導入
cargo.toml dependenciesに追記することで、コンパイルの時にダウンロードしてコンパイルしてくれる
プログラム本文に
use [library name]::[library method];
で使用できる

## 所有権理解
cargo new own