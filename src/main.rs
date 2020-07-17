use hilbert_project::hilbert::*;

fn main() {

    let amount: u32 = 16;

    // Some testing.
      
    println!("Print Gray Codes from {} to {}", 0, amount);
    for i in 0..amount {
        println!("{:width$} -> {:width$} :: {}", i, gc(i), bit_to_string(gc(i), 5), width=3);
    }
    println!("");

    println!("Print The Inverse Gray Codes from {} to {}", 0, amount);
    for i in 0..amount {
        println!("{:width$} -> {:width$} :: {}", i, gc_inverse(i), bit_to_string(gc(i), 5), width=3);
    }
    println!("");

    println!("The index number between the Gray Code index i and i+1 differs. In paper: g(i) == tsb(i)");
    println!("tsb({}) == {}", 0, tsb(0));
    println!("tsb({}) == {}", 1, tsb(1));
    println!("tsb({}) == {}", 2, tsb(2));
    println!("tsb({}) == {}", 3, tsb(3));
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

    println!("The next entry for index i.");
    println!("entry({}) == {}", 0, entry(0));
    println!("entry({}) == {}", 1, entry(1));
    println!("entry({}) == {}", 2, entry(2));
    println!("entry({}) == {}", 3, entry(3));

    for i in 0..32 {
        println!("bit({},{}) == {}", 7, i, get_bit(7, i));
    }
}
