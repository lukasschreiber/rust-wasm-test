# Start
Start and watch with `cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --debug"`
then run dev server with `cd web && npm ci && npm link ../pkg && npm run dev`