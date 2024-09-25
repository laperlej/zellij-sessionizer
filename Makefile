open:
	@zellij action new-tab --layout ./plugin-dev-workspace.kdl

clean:
	@pkill watchexec

deploy:
	cargo build --target wasm32-wasi --release
	@cp $(shell pwd)/target/wasm32-wasi/release/zellij-sessionizer.wasm ~/.config/zellij/plugins
