#!/bin/sh
set -e

root_dir=$(dirname "$0")
root_dir=$(cd "$root_dir/.." && pwd)
if [ ! -f "$root_dir/LICENSE.txt" ]; then
	echo "Couldn't find the root dir"
	exit 1
fi

new_func() {
	echo
	echo "****************************************************************"
	echo "$1"
	echo "****************************************************************"
	echo
}

build_and_check() {
	new_func "Build and check"
	curr_dir=$(pwd)
	cd "$root_dir/src/rust/iced-x86-js"

	echo "==== FORMAT CHECK ===="
	cargo fmt -- --color always --check
	echo "==== CLIPPY instr_api decoder encoder block_encoder instr_create op_code_info instr_info gas intel masm nasm fast_fmt ===="
	cargo clippy --color always --target wasm32-unknown-unknown --no-default-features --features "instr_api decoder encoder block_encoder instr_create op_code_info instr_info gas intel masm nasm fast_fmt"
	echo "==== CLIPPY instr_api decoder encoder block_encoder instr_create op_code_info instr_info gas intel masm nasm fast_fmt bigint ===="
	cargo clippy --color always --target wasm32-unknown-unknown --no-default-features --features "instr_api decoder encoder block_encoder instr_create op_code_info instr_info gas intel masm nasm fast_fmt bigint"

	set -- \
		"decoder" \
		"instr_api decoder" \
		"encoder" \
		"instr_api encoder" \
		"encoder block_encoder" \
		"instr_create" \
		"encoder instr_create" \
		"encoder op_code_info" \
		"instr_api encoder op_code_info" \
		"instr_info" \
		"instr_api instr_info" \
		"gas" \
		"intel" \
		"masm" \
		"nasm" \
		"fast_fmt" \
		"instr_api nasm"
	for features in "$@"; do
		echo "==== $features ===="
		cargo check --color always --target wasm32-unknown-unknown --no-default-features --features "$features"
	done

	set -- \
		"no_vex" \
		"no_evex" \
		"no_xop" \
		"no_d3now" \
		"no_vex no_evex no_xop no_d3now"
	for features in "$@"; do
		echo "==== $features ===="
		cargo check --color always --target wasm32-unknown-unknown --features "$features"
	done

	cd "$curr_dir"
}

test_the_code() {
	new_func "Test the code"
	curr_dir=$(pwd)
	cd "$root_dir/src/rust/iced-x86-js"

	wasm-pack build --mode force --target nodejs
	cd src/tests
	npm install
	npm test
	cd ../..

	cd "$curr_dir"
}

if [ "$#" -gt 0 ]; then
	echo "No command line args are supported"
	exit 1
fi

echo
echo "=================================================="
echo "JavaScript build"
echo "=================================================="
echo

export RUSTFLAGS="-D warnings"

build_and_check
test_the_code