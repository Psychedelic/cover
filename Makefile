.PHONY: init candid build local test format lint clean

LOCAL_ADMIN_PRINCIPAL=$(shell dfx identity get-principal)
TEST_ADMIN_PRINCIPAL=$(shell cat test/admin-test-principal)

init:
	npm --prefix test i
	cargo check

candid:
	cargo run > cover.did
	didc bind -t ts cover.did > test/factory/idl.d.ts
	echo "// @ts-nocheck" > test/factory/idl.ts
	didc bind -t js cover.did >> test/factory/idl.ts

build: candid
	dfx ping local || dfx start --clean --background
	dfx canister create cover
	dfx build cover

build-ic-test:
	dfx build --network ic cover_test

build-ic-production:
	dfx build --network ic cover

local: build
	dfx deploy cover --argument '(opt record{admin=opt vec{principal"$(LOCAL_ADMIN_PRINCIPAL)"}})'

stop-replica:
	dfx stop

test: stop-replica build
	dfx canister install cover --argument '(opt record{admin=opt vec{principal"$(TEST_ADMIN_PRINCIPAL)"}})'
	npm --prefix test t
	dfx stop

deploy-ic-test:
	dfx canister --network ic install cover_test --upgrade

deploy-ic-production:
	dfx canister --network ic install cover --upgrade

format:
	npm --prefix test run prettier
	npm --prefix test run lint
	cargo fmt --all

lint:
	npm --prefix test run prebuild
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all

clean:
	cargo clean
	npm --prefix test run clean
