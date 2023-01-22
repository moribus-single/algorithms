pub mod algorythms {
    pub fn find_common_denominator(mut m: i128, mut n: i128, flag: bool) -> i128 {
        // Euclid algorythm
        let mut r: i128;
    
        // E0: Ensure m >= n
        if m < n {
            let t: i128 = m;
            m = n;
            n = t;
        }
    
        loop {
            // E1: Find a remainder
            if flag { println!("m = {}\nn = {}", m, n); }
            r = m % n;
            
            // E2: Is it zero?
            if flag { println!("r = {}\n", r); }
            if r == 0 { break; }
    
            // E3: Interchange
            m = n;
            n = r;
        }
    
        n
    }

    #[test]
    fn find_common_denominator_tests() {
        assert_eq!(find_common_denominator(65, 15, false), 5);
        assert_eq!(find_common_denominator(119, 544, false), 17);
    }
}
