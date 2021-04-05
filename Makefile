# this is just to share how to do things with others and keep notes
# actual build description us in Cargo.toml

.PHONY: help install pakemon rattata rattata-cli build-osx release-osx

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


install: ## setup dev-tools used by other targets
	cargo install cross

pakemon: ## run frontend on current platform
	cargo run --bin=pakemon

rattata: ## run backend server (payload) on current platform
	cargo run --bin=rattata_server

rattata-cli: ## run CLI frontend on current platform
	cargo run --bin=rattata_client


release-linux: releases/rattata_client-linux.zip releases/rattata_server-linux.zip ## create a release for linux (x86_64)
release-osx: releases/rattata_client-osx.zip releases/rattata_server-osx.zip       ## create a release for osx (x86_64)


# TODO: these need work. can't cross-build from OSX to linux or vice-versa

build-linux:
	cross build --target=x86_64-unknown-linux-gnu --release
	strip target/x86_64-unknown-linux-gnu/release/rattata_client
	strip target/x86_64-unknown-linux-gnu/release/rattata_server
	upx --best --lzma target/x86_64-unknown-linux-gnu/release/rattata_client
	upx --best --lzma target/x86_64-unknown-linux-gnu/release/rattata_server
	mkdir -p releases

releases/rattata_client-linux.zip: build-linux
	cd target/x86_64-unknown-linux-gnu/release && zip ../../../releases/rattata_client-linux.zip rattata_client

releases/rattata_server-linux.zip: build-linux
	cd target/x86_64-unknown-linux-gnu/release && zip ../../../releases/rattata_server-linux.zip rattata_server


build-osx:
	cross build --target=x86_64-apple-darwin --release
	strip target/x86_64-apple-darwin/release/rattata_client
	strip target/x86_64-apple-darwin/release/rattata_server
	upx --best --lzma target/x86_64-apple-darwin/release/rattata_client
	upx --best --lzma target/x86_64-apple-darwin/release/rattata_server
	mkdir -p releases

releases/rattata_client-osx.zip: build-osx
	cd target/x86_64-apple-darwin/release && zip ../../../releases/rattata_client-osx.zip rattata_client

releases/rattata_server-osx.zip: build-osx
	cd target/x86_64-apple-darwin/release && zip ../../../releases/rattata_server-osx.zip rattata_server