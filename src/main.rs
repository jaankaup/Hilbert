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

    println!("The intra sub-hypercube direction d(i). TODO: this gives wrong result at index 3.");
    println!("d({},{}) == {}", 0,2, d(0,2));
    println!("d({},{}) == {}", 1,2, d(1,2));
    println!("d({},{}) == {}", 2,2, d(2,2));
    println!("d({},{}) == {}", 3,2, d(3,2));
    println!("");

}
