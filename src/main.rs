mod section_1_1;

use section_1_1::algorythms::find_common_denominator;

fn main() {
    let m: i128 = 119;
    let n: i128 = 544;

    println!("find_common_denominator(119, 544) - {}", find_common_denominator(m, n, true))
}