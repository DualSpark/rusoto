# System Setup
SHELL = bash

RUST_VERSION ?= stable

.PHONY: clean
clean:
	cargo +$$RUST_VERSION clean

.PHONY: fmt
fmt:
	cargo +$$RUST_VERSION fmt

.PHONY: generate
generate:
	(cd service_crategen && cargo +$$RUST_VERSION run -- generate -c ./services.json -o ../rusoto/services)

.PHONY: build
build: generate
	cargo +$$RUST_VERSION build --features all

.PHONY: docs
docs:
	cargo +$$RUST_VERSION doc --all --no-deps

.PHONY: unit_test
unit_test:
	cargo +$$RUST_VERSION test --all

.PHONY: skeptical
skeptical:
	(cd skeptical && cargo +$$RUST_VERSION test)

.PHONY: integration_test
integration_test:
	(cd integration_tests && cargo +$$RUST_VERSION test --features all)

.PHONY: check_integration_test
check_integration_test:
	(cd integration_tests && cargo +$$RUST_VERSION check --tests --features all)

.PHONY: rustls_unit_test
rustls_unit_test:
	(cd rusoto/core && cargo +$$RUST_VERSION test --no-default-features --features=rustls)
	(cd rusoto/services && ./rustls-unit-test.sh $$RUST_VERSION)

.PHONY: check_service_defintions
check_service_defintions:
	(cd service_crategen && cargo +$$RUST_VERSION run -- check -c ./services.json)

.PHONY: time_credentials
time_credentials:
	(cd rusoto/credential && cargo clean --package rusoto_credential && touch src/lib.rs && time cargo +$$RUST_VERSION build)

.PHONY: bench_s3
bench_s3:
	(cd rusoto/services/s3 && cargo +nightly bench)
