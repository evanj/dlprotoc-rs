all:
	cargo test
	cargo fmt
	cargo clippy \
		--all-targets \
		-- \
		-D warnings \
		-D clippy::nursery \
		-D clippy::pedantic \
		-D clippy::style \
		-D missing_docs \
		-A clippy::option-if-let-else \
		-A clippy::missing-panics-doc \
		-A clippy::missing-errors-doc
		# -A clippy::cast_precision_loss \
		# -A clippy::significant_drop_tightening
	cargo verify-project
	cargo audit

	# Run the "integration" test for CI: slow but makes sure it works
	cargo test -- --ignored
