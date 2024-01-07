#!/usr/bin/env bash

readonly BASEDIR=$(dirname $(readlink -f "$0"))
readonly HOST_CRATE="host"
readonly TEST_PLUGIN_CRATE="trivial-plugin"
readonly TEST_PLUGIN_FILE_NAME="libtrivial_plugin.so"

export RUST_LOG="trace"

cargo build --release -p ${TEST_PLUGIN_CRATE} && \
cargo run --release -p ${HOST_CRATE} -- ${BASEDIR}/target/release/${TEST_PLUGIN_FILE_NAME}
