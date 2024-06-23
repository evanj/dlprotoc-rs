all:
	cargo test
	cargo fmt
	# disallow warnings so they fail CI
	cargo clippy --all-targets -- -D warnings
	cargo verify-project
	cargo audit

	# Run the "integration" test for CI: slow but makes sure it works
	cargo test -- --ignored
