test:
	CARGO_INCREMENTAL=false cargo test --features asm-keccak -- tests::test_foo_fails --exact --show-output

test-native:
	CARGO_INCREMENTAL=false cargo test -- tests::test_foo_fails --exact --show-output