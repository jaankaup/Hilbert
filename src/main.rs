use hilbert_project::hilbert::*;

fn main() {

    let amount: u32 = 16;

    // Some testing.
      
    println!("Print Gray Codes from {} to {}", 0, amount);
    for i in 0..amount {
        println!("{:width$} -> {:width$} :: {}", i, gc(i), bit_to_string(gc(i), 5), width=3);
    }
    println!("");

    //println!("Print The Inverse Gray Codes from {} to {}", 0, amount);
    //for i in 0..amount {
    //    println!("{:width$} -> {:width$} :: {}", i, gc_inverse(i), bit_to_string(gc(i), 5), width=3);
    //}
    //println!("");

    println!("The index number between the Gray Code index i and i+1 differs. In paper: g(i) == g(i)");
    println!("g({}) == {}", 0, g(0));
    println!("g({}) == {}", 1, g(1));
    println!("g({}) == {}", 2, g(2));
    println!("g({}) == {}", 3, g(3));
    println!("g({}) == {}", 4, g(4));
    println!("g({}) == {}", 5, g(5));
    println!("g({}) == {}", 6, g(6));
    println!("g({}) == {}", 7, g(7));
    println!("g({}) == {}", 8, g(8));
    println!("");

    println!("The entry Gray Code for given index.");
    println!("entry({}) == {}", 0, entry(0));
    println!("entry({}) == {}", 1, entry(1));
    println!("entry({}) == {}", 2, entry(2));
    println!("entry({}) == {}", 3, entry(3));
    println!("");

    println!("The intra sub-hypercube direction d(i).");
    println!("d({},{}) == {}", 0,2, d(0,2));
    println!("d({},{}) == {}", 1,2, d(1,2));
    println!("d({},{}) == {}", 2,2, d(2,2));
    println!("d({},{}) == {}", 3,2, d(3,2));
    println!("");

    //println!("rbr({}, {}, {}) == {}", 3,4,0, bit_to_string(rbr(3,4,0), 32));

    let n = 2;
    let m = 3;
    let p = [5,6];
    println!("hilbert_index({}, {}, [{},{}]) == {}", n,m,p[0], p[1], hilbert_index(n, m, p));
    //for i in 0..64 {
    //    let inverse = hilbert_index_reverse(n, m, i);
    //    println!("inverse {} :: [{}, {}]",i, inverse[0], inverse[1]); 
    //}

    // println!("lbr({}, {}, {}) == {}", 3,4,0, bit_to_string(lbr(3,4,0), 32));
    // println!("lbr({}, {}, {}) == {}", 3,4,1, bit_to_string(lbr(3,4,1), 32));
}
