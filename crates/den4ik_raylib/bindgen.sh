#!/usr/bin/env sh

set -xe

raylib_src="raylib-6.0/src"
ffi_src="src/ffi"

bindgen "${raylib_src}/raymath.h" >"${ffi_src}/raymath.rs"
