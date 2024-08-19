mod ch_1;
use std::sync::atomic::{ AtomicI32, Ordering };

use ch_1::ch_1;
mod ch_2;
use ch_2::ch_2;

fn main() {
    ch_2()
    // ch_1();
}
