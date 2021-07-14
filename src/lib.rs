#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;


use typenum;
use blobby;
use generic_array;
use digest;
use byteorder;