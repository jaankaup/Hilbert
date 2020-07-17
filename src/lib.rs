pub mod hilbert {

    use modulo::Mod;
    use std::char;

    /// Calculate the binary reflected Gray Code for index i.
    pub fn gc(i: u32) -> u32 {
        i ^ (i >> 1)
    }
    
    /// Calculate the inverse Gray Code for gray code g.
    pub fn gc_inverse(g: u32) -> u32 {
    
        let m = (g as f64 + 1.0).log2().ceil() as u32;
    
        let (mut i,mut j) = (g, 1);
        while j < m {
            i = i ^ (g >> j);
            j = j + 1;
        }
        i
    }
    
    /// Get the i:th bit from given number.
    pub fn get_bit(n:u32, i: u32) -> u32 {
    
        assert_eq!(i < 32, true);
    
        (n & (1 << i)) >> i
    }
    
    pub fn g(i: u32) -> u32 {
        let temp = (gc(i) ^ gc(i+1)); 
        32 - temp.leading_zeros() - 1
    }
    
    /// Give the Gray Code for the entry i + 1
    pub fn next_entry(i: u32) -> u32 {
        i ^ (1 << g(i))
    }
    
    /// Calculate entry Gray Code for given index.
    pub fn entry(i: u32) -> u32 {
        if i == 0 { 0 }
        else {
            gc(2 * ((i-1) as f64 * 0.5).floor() as u32)
        }
    }
    
    /// Intra sub-hypercube direction.
    pub fn d(i: u32, n: u32) -> u32 {
        if i == 0 { println!("zero"); 0 }
        else if i & 1 == 0 { // is even
            println!("even");
            println!("g({}-1).modulo{} == {}", i, n, g(i-1) % n); 
            //g(i-1).modulo(n)
            g(i-1) % n
        }
        else if i & 1 == 1 { // is odd
            println!("odd");
            println!("g({}) % {} == {} % {} == {}", i, n, g(i), n, g(i) % n); 
            g(i) % n
        }
        else {
            panic!("A serious bug!"); // TODO: remove this.
        }
    }

    /// Convert number to binary representation. Bit range is the number of bits used to the
    /// reprsesentation.
    pub fn bit_to_string(number: u32, bit_range: u32) -> String {
        let mut binary_str = String::new();
        for i in 0..bit_range {
            binary_str.push(
                char::from_digit(get_bit(number, bit_range - 1 - i), 10).unwrap()
            );
        }
        binary_str
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn gray_codes() {
            assert_eq!(gc(0),  0);
            assert_eq!(gc(1),  1);
            assert_eq!(gc(2),  3);
            assert_eq!(gc(3),  2);
            assert_eq!(gc(4),  6);
            assert_eq!(gc(5),  7);
            assert_eq!(gc(6),  5);
            assert_eq!(gc(7),  4);
            assert_eq!(gc(8),  12);
            assert_eq!(gc(9),  13);
            assert_eq!(gc(10), 15);
            assert_eq!(gc(11), 14);
            assert_eq!(gc(12), 10);
            assert_eq!(gc(13), 11);
            assert_eq!(gc(14), 9);
            assert_eq!(gc(15), 8);
        }

        #[test]
        fn gray_inverse_codes() {
            assert_eq!(gc_inverse(0),   0);
            assert_eq!(gc_inverse(1),   1);
            assert_eq!(gc_inverse(2),   3);
            assert_eq!(gc_inverse(3),   2);
            assert_eq!(gc_inverse(4),   7);
            assert_eq!(gc_inverse(5),   6);
            assert_eq!(gc_inverse(6),   4);
            assert_eq!(gc_inverse(7),   5);
            assert_eq!(gc_inverse(8),  15);
            assert_eq!(gc_inverse(9),  14);
            assert_eq!(gc_inverse(10), 12);
            assert_eq!(gc_inverse(11), 13);
            assert_eq!(gc_inverse(12),  8);
            assert_eq!(gc_inverse(13),  9);
            assert_eq!(gc_inverse(14), 11);
            assert_eq!(gc_inverse(15), 10);
        }
    }
}

