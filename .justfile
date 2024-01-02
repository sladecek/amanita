set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Build the web extension
build:
  cd amanita-chrome && wasm-pack build --release -t web
