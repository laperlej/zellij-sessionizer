# Development workflow
dev:
    zellij action new-tab --layout ./plugin-dev-workspace.kdl

# Clean up running processes
clean:
    pkill watchexec

# Build and deploy plugin
deploy:
    cargo build --target wasm32-wasip1 --release
    cp {{justfile_directory()}}/target/wasm32-wasip1/release/zellij-sessionizer.wasm ~/.config/zellij/plugins
