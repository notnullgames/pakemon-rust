# this is just to share how to do things with others and keep notes
# actual build description us in Cargo.toml

.PHONY: help install pakemon trainer rattata release

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'



install: ## setup dev-tools used by other targets
	cargo install cross

pakemon: ## run frontend on current platform
	cargo run -p pakemon

trainer: ## run backend server (payload) on current platform
	cargo run -p trainer

rattata: ## run CLI frontend on current platform
	cargo run -p rattata

release: release-osx ## compile release packages for all supported platforms


release-osx:
	mkdir -p releases
	rm -f releases/pakemon-osx.zip
	cross build --target=x86_64-apple-darwin --release
	strip target/x86_64-apple-darwin/release/pakemon target/x86_64-apple-darwin/release/trainer target/x86_64-apple-darwin/release/rattata
	upx --best --lzma target/x86_64-apple-darwin/release/pakemon target/x86_64-apple-darwin/release/trainer target/x86_64-apple-darwin/release/rattata
	cd target/x86_64-apple-darwin/release && zip ../../../releases/rattata-osx.zip rattata
	cd target/x86_64-apple-darwin/release && zip ../../../releases/trainer-osx.zip trainer
	cd target/x86_64-apple-darwin/release && zip ../../../releases/pakemon-osx.zip pakemon
	zip releases/pakemon-osx.zip -r -x .gitkeep static