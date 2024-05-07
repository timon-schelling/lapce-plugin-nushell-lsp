alias c := clean
alias b := build
alias d := dist
alias p := publish

clean:
    cargo clean

build:
    cargo build --release --target wasm32-wasi

dist: build
    mkdir -p ./target/dist
    rm -rf ./target/dist/*
    cp ./target/wasm32-wasi/release/lapce-plugin-nushell-lsp.wasm ./target/dist/plugin.wasm
    cp ./icon.png ./target/dist/icon.png
    cp ./README.md ./target/dist/README.md
    cp ./volt.toml ./target/dist/volt.toml

dist-zip: dist
    rm -f ./target/dist.zip
    cd ./target/dist && zip -r ../dist.zip .

dist-tar: dist
    rm -f ./target/dist.tar
    touch ./target/dist.tar
    cd ./target/dist && tar -cvf ../dist.tar *

publish: dist
    cd ./target/dist && volts publish
