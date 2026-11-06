.PHONY: all build test coverage coverage-summary coverage-lcov coverage-open clean

all: build

build:
	cargo build --release

test:
	cargo test --workspace

coverage:
	cargo llvm-cov --workspace --html
	@echo "Coverage HTML report generated at ./target/llvm-cov/html/index.html"

coverage-summary:
	cargo llvm-cov --workspace --summary-only

coverage-lcov:
	cargo llvm-cov --workspace --lcov --output-path lcov.info
	@echo "LCOV coverage file generated at ./lcov.info"

coverage-open:
	cargo llvm-cov --workspace --html --open

clean:
	cargo clean
	rm -rf target/coverage target/llvm-cov lcov.info
