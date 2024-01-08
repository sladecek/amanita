set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Build the web extension
build:
  cd amanita-chrome \
  && wasm-pack build --release --no-typescript --out-dir "extension/js/wasm" --out-name "wasm_mod" --target web \
  && rm -f extension/js/wasm/.gitignore extension/js/wasm/package.json
