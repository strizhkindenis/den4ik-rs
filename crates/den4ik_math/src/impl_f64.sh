#!/usr/bin/env sh

set -xe

cleanup() {
	[ -f tmp.rs ] && rm tmp.rs
}

trap cleanup EXIT

sed 's/f32/f64/g;s/F32/F64/g' f32.rs > f64.rs

sed '/f64/d' lib.rs > tmp.rs && mv tmp.rs lib.rs
sed '/pub mod f32;/a\
pub mod f64;' lib.rs > tmp.rs && mv tmp.rs lib.rs

cargo check
