pub mod hilbert {

    use std::char;

    /// Calculate the binary reflected Gray Code for index i.
    pub fn gc(i: u32) -> u32 {
        i ^ (i >> 1)
    }
    
    /// Calculate the inverse Gray Code for gray code g.
    pub fn gc_inverse(g: u32) -> u32 {
    
        //let m = 32 - g.leading_zeros() - 1;
        let m = (g as f64 + 1.0).log2().ceil() as u32; // A better way?
    
        let (mut i,mut j) = (g, 1);
        while j < m {
            i = i ^ (g >> j);
            j = j + 1;
        }
        i
    }
    
    /// Get the value fo the k:th bit.
    pub fn bit(a:u32, k: u32) -> u32 {
    
        assert_eq!(k < 32, true);
    
        (a & (1 << k)) >> k
    }

    /// Replace the i:th bit of number b with value v. Indexing of i starts at 0. 
    pub fn put_bit(b: u32, i: u32, v: u32) -> u32 {
        let mask = 1 << i;
        (b & !mask) | (v << i)
        
    }
    
    /// Trailing set bits function. Dimension of change in the Gray Code.
    pub fn g(i: u32) -> u32 {
        let temp = gc(i) ^ gc(i+1); 
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
            gc(2 * ((i-1) as f64 * 0.5).floor() as u32) // TODO: check for a better way to compute this.
        }
    }
    
    /// Intra sub-hypercube direction.
    pub fn d(i: u32, n: u32) -> u32 {
        if i == 0 { 0 }
        else if i & 1 == 0 { g(i-1) % n }
        else { g(i) % n }
    }

    /// The right bit rotation. Rotates number b to the right i times in bit range n.
    pub fn rbr(b: u32, n: u32, i: u32) -> u32 {
        // let upper_index = n - 1 + (i % n);
        // let lower_index = i % n;
        let mut temp = b;
        for _c in 0..i {
            let lsb = bit(temp,0); 
            let rotated = temp >> 1;
            temp = put_bit(rotated, n-1, lsb);
        }
        temp
    }

    /// The left bit rotation. Rotates number b to the left i times in bit range n.
    pub fn lbr(b: u32, n: u32, i: u32) -> u32 {
        // let upper_index = n - 1 + (i % n);
        // let lower_index = i % n;
        let mut temp = b;
        let mask = (1 << n) - 1;
        for _c in 0..i {
            let msb = bit(temp,n-1); 
            let rotated = temp << 1 & mask;
            temp = put_bit(rotated, 0, msb);
        }
        temp
    }

    /// Transform for the given entry e and intra direction d. b is the i:th 
    pub fn transform(e: u32, d: u32, b: u32) -> u32 {
        rbr(b ^ e, 2, d+1) 
    }

    pub fn inverse_transform(e: u32, d: u32, b: u32) -> u32 {
        transform(rbr(e, 2, d+1), 2-d-1, b)
    }
    
//    /// Create Hilbert index for 2d point. n is the dimension. m is the bit size for a single point
//    /// bit representation.
//    pub fn hilbert_index(n: u32, m: u32, p: [u32; 2]) -> u32 {
//        let (mut h, mut e, mut dd) = (0,0,1);
//        //println!("{:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$}","i","l","Ted(l)","w","e(w)","d(w)","e","d","h", width=7);
//        //println!("-----------------------------------------------------------------------------------------------------------------------");
//        for i in (0..m).rev() {
//            let l = 0 | (bit(p[1], i) << 1) | bit(p[0], i);  
//            let tl = transform(e, dd, l);
//            let w = gc_inverse(tl);
//            e = e ^ lbr(entry(w), 2, dd+1); 
//            dd = (dd + d(w,2) + 1) % n;
//            h = (h << n) | w;
//            //println!("{:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$}",i,l,tl,w,entry(w),d(w,2),e,dd,h, width=7);
//        }
//        h
//    }

    /// Create Hilbert index for 3d point. n is the dimension. m is the bit size for a single point
    /// bit representation.
    pub fn hilbert_index(n: u32, m: u32, p: [u32; 3]) -> u32 {
        let (mut h, mut e, mut dd) = (0,0,1);
        //println!("{:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$}","i","l","Ted(l)","w","e(w)","d(w)","e","d","h", width=7);
        //println!("-----------------------------------------------------------------------------------------------------------------------");
        for i in (0..m).rev() {
            let l = 0 | (bit(p[2], i) << 2) | (bit(p[1], i) << 1) | bit(p[0], i);  
            let tl = transform(e, dd, l);
            let w = gc_inverse(tl);
            e = e ^ lbr(entry(w), 3, dd+1); 
            dd = (dd + d(w,3) + 1) % n;
            h = (h << n) | w;
            //println!("{:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$} {:<width$}",i,l,tl,w,entry(w),d(w,2),e,dd,h, width=7);
        }
        h
    }

    /// Get the 2d-point from an Hilbert index. n is the dimension. m is the bit size for a single point
    /// bit representation. h is the hilbert index.
    pub fn hilbert_index_reverse(n: u32, m: u32, h: u32) -> [u32; 3] {
        let (mut e, mut dd) = (0,0);
        let mut p = [0,0,0];
        for i in (0..m).rev() {
            let w = 0 | (bit(h, i*n + n - 2) << 2) | (bit(h, i*n + n - 1) << 1) | bit(h, i*n);
            let mut l = gc(w);
            l = inverse_transform(e,dd,l);
            for j in 0..n {
                p[j as usize] = put_bit(p[j as usize], i, bit(l,j));    
            }
            e = e ^ lbr(entry(w), 3, dd+1);
            dd = (dd + d(w, 3) + 1) % n
        }
        p
    }

    /// Convert number to binary representation. Bit range is the number of bits used to the
    /// reprsesentation.
    pub fn bit_to_string(number: u32, bit_range: u32) -> String {
        let mut binary_str = String::new();
        for i in 0..bit_range {
            binary_str.push(
                char::from_digit(bit(number, bit_range - 1 - i), 10).unwrap()
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
