# Aeroe Wallet
Aeroe Wallet is your go-to tool for managing and mining on Nockchain, offering a straightforward and effective way to engage with the Nockchain ecosystem.

## Development

```
# install
yarn

# dev
yarn tauri dev

# dev but runs fast
yarn tauri dev --release

# build
yarn tauri build
```

## Project Structure

- scripts: shell scripts that help with the dev and build process
- src: svelte frontend
- src-tauri: backend code, rust and tauri configuration files
- static: static files
- VENDOR_COMMIT: git commit hash of the vendored nockchain repo