all:
	cargo test --all-targets
	cargo fmt
	# disallow warnings so they fail CI
	cargo clippy --all-targets -- -D warnings
	# fail for rustdoc warnings
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
	cargo verify-project
	cargo audit

	# Run the "integration" test for CI: slow but makes sure it works
	cargo test --all-targets -- --ignored
