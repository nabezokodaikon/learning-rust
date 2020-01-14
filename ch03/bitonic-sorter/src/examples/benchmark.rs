use num_cpus;

use bitonic_sorder::fourth::sort as par_sort;
use bitonic_sorder::third::sort as seq_sort;
use bitonic_sorder::utils::{is_sorted_ascending, new_u32_vec};
use bitonic_sorder::SortOrder;

use std::str::FromStr;
use std::time::Instant;
use std::{env, f64};

pub fn main() {
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");
        run_sorts(bits);
    } else {
        eprintln!(
            "Usage {} <number of elements in bits>",
            env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    }
}
