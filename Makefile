
windows: x86_64-windows

i686-windows:
	cargo +stable-i686-pc-windows-gnu build --target i686-pc-windows-gnu --release
	mkdir -p artifacts/i686-windows
	cp -f target/i686-pc-windows-gnu/release/valet.dll artifacts/i686-windows/index.node

x86_64-windows:
	cargo +stable-x86_64-pc-windows-gnu build --target x86_64-pc-windows-gnu --release
	mkdir -p artifacts/x86_64-windows
	cp -f target/x86_64-pc-windows-gnu/release/valet.dll artifacts/x86_64-windows/index.node

linux: x86_64-linux

x86_64-linux:
	cargo build --release
	mkdir -p artifacts/x86_64-linux
	cp -f target/release/libvalet.so artifacts/x86_64-linux/index.node

darwin: x86_64-darwin

x86_64-darwin:
	cargo build --release
	mkdir -p artifacts/x86_64-darwin
	cp -f target/release/libvalet.dylib artifacts/x86_64-darwin/index.node
