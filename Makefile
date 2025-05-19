run-game-linux: debug-build-linux
	cd godot/ && godot-4

run-game-windows: debug-build-windows
	cd godot/ && godot4


export-linux-debug: debug-build-linux
	godot-4 --headless --path godot --export-debug "Linux" build/linux/Deeps-Creeps


debug-build-linux:
	cargo build --manifest-path rust/Cargo.toml --target x86_64-unknown-linux-gnu

debug-build-windows:
	cargo build --manifest-path rust/Cargo.toml --target x86_64-pc-windows-gnu

