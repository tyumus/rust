// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(warnings)]

#![feature(ascii_ctype)]
#![feature(box_syntax)]
#![feature(core_float)]
#![feature(core_private_bignum)]
#![feature(core_private_diy_float)]
#![feature(dec2flt)]
#![feature(decode_utf8)]
#![feature(exact_size_is_empty)]
#![feature(fixed_size_array)]
#![feature(flt2dec)]
#![feature(fmt_internals)]
#![feature(hashmap_internals)]
#![feature(iterator_step_by)]
#![cfg_attr(stage0, feature(i128_type))]
#![cfg_attr(stage0, feature(inclusive_range_syntax))]
#![feature(iterator_try_fold)]
#![feature(iterator_flatten)]
#![cfg_attr(stage0, feature(conservative_impl_trait))]
#![feature(iter_rfind)]
#![feature(iter_rfold)]
#![feature(iterator_repeat_with)]
#![feature(nonzero)]
#![feature(pattern)]
#![feature(range_is_empty)]
#![feature(raw)]
#![feature(refcell_replace_swap)]
#![feature(slice_patterns)]
#![feature(slice_rotate)]
#![feature(sort_internals)]
#![feature(specialization)]
#![feature(step_trait)]
#![feature(test)]
#![feature(trusted_len)]
#![feature(try_from)]
#![feature(try_trait)]
#![feature(exact_chunks)]
#![feature(atomic_nand)]
#![feature(reverse_bits)]
#![feature(inclusive_range_fields)]

extern crate core;
extern crate test;
extern crate rand;

mod any;
mod array;
mod ascii;
mod atomic;
mod cell;
mod char;
mod clone;
mod cmp;
mod fmt;
mod hash;
mod intrinsics;
mod iter;
mod mem;
mod nonzero;
mod num;
mod ops;
mod option;
mod pattern;
mod ptr;
mod result;
mod slice;
mod str;
mod tuple;
