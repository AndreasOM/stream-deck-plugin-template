# list recipes
l:
	@just --list

# build debug for testing - you might want to adapt the target for your needs
b:
	cargo build --target aarch64-apple-darwin
	cp target/aarch64-apple-darwin/debug/stream-deck-plugin-template local.change-me.template.sdPlugin/stream-deck-plugin-template-apple
# package a build
p:
	cargo build --release --target aarch64-apple-darwin
	# :TODO: crate fat binary for x86 & aarch64
	cp target/aarch64-apple-darwin/release/stream-deck-plugin-template local.change-me.template.sdPlugin/stream-deck-plugin-template-apple
	# :TODO: add windows build

	# :TODO: ensure DistributionTool is installed, or mentioned in installation guide
	mkdir package
	DistributionTool -b -i local.change-me.template.sdPlugin -o package/
