use std::char;
use pad::PadStr;
use modulo::Mod;

/// Calculate the binary reflected Gray Code for index i.
fn gc(i: u32) -> u32 {
    i ^ (i >> 1)
}

/// Calculate the inverse Gray Code for gray code g.
fn gc_inverse(g: u32) -> u32 {

    let m = (g as f64 + 1.0).log2().ceil() as u32;

    let (mut i,mut j) = (g, 1);
    while j < m {
        i = i ^ (g >> j);
        j = j + 1;
    }
    i
}

/// Get the i:th bit from given number.
fn get_bit(n:u32, i: u32) -> u32 {

    assert_eq!(i < 32, true);

    (n & (1 << i)) >> i
}

/// Convert number to binary representation. Bit range is the number of bits used to the
/// reprsesentation.
fn bit_to_string(number: u32, bit_range: u32) -> String {
    let mut binary_str = String::new();
    for i in 0..bit_range {
        binary_str.push(
            char::from_digit(get_bit(number, bit_range - 1 - i), 10).unwrap()
        );
    }
    binary_str
}

/// The bit of the Gray Code is going to change when we proceed to next index.
/// Dimension of change in the Gray Code.
fn tsb(i: u32) -> u32 {
    (gc(i) ^ gc(i+1)) - 1 
}

/// Give the Gray Code for the entry i + 1
fn next_entry(i: u32) -> u32 {
    i ^ (1 << tsb(i))
}

/// Calculate entry Gray Code for given index.
fn entry(i: u32) -> u32 {
    if i == 0 { 0 }
    else {
        gc((2 * ((i-1) as f64 * 0.5).floor() as u32))
    }
}

/// Intra sub-hypercube direction.
fn d(i: u32, n: u32) -> u32 {
    if i == 0 { 0 }
    else if i & 1 == 0 { // is event
        tsb(i-1).modulo(n)
    }
    else if i & 1 == 1 { // is odd
        tsb(i).modulo(n)
    }
    else {
        panic!("A serious bug!"); // TODO: remove this.
    }
}

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
