start_server:
	python3 -m http.server
tui:
	cargo run --example tui --release -- --root / --depth 3
tui_curr:
	cargo run --example tui --release -- --root . --depth 3
