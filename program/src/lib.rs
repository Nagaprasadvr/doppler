#![no_std]
#![cfg_attr(target_os = "solana", feature(asm_experimental_arch))]

// fastRQJt3nLdY3QA7n8eZ8ETEVefy56ryfUGVkfZokm
use doppler::{nostd_panic_handler, prelude::*};

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct PriceFeed {
//     pub price: u64,
// }

#[repr(C)]
#[derive(Clone, Copy)]
struct PriceFeedBig {
    price: u64,
    timestamp: u64,
    authority: [u8; 32],
    data: [u8; 1024],
}

nostd_panic_handler!();

#[no_mangle]
/// # Safety
///
/// This is a permissioned entrypoint only invokable by the
/// ADMIN keypair. It is as safe as you choose it to be.
pub unsafe extern "C" fn entrypoint(input: *mut u8) {
    Admin::check(input);
    Oracle::<PriceFeedBig>::check_and_update(input);
}
