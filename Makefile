all:
	cargo test
	cargo fmt
	cargo clippy --all-targets
	cargo verify-project
	cargo audit

	# Run the "integration" test for CI: slow but makes sure it works
	cargo test -- --ignored
