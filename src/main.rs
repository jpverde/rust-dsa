mod data_structures;

fn main() {
    data_structures::array::array();
    print_line_separator();
    towers_of_hanoi(3, 'A', 'C', 'B');

    println!("Hello, world!");
}

fn print_line_separator() {
    println!("--------------------------------------------");
}

fn towers_of_hanoi(n: i32, frompeg: char, topeg: char, auxpeg: char) {
    // Base case
    if n == 1 {
        println!("Move disk 1 from peg {} to peg {}", frompeg, topeg);
        print_line_separator();
        return;
    }

    // Ecursion, Move n-1 disks from `frompeg` to `auxpeg`
    towers_of_hanoi(n - 1, frompeg, auxpeg, topeg);

    // Move remaining disks from A to C
    println!("Move disk {} from peg {} to peg {}", n, frompeg, topeg);

    // Move n-1 disks from B to C using A as auxiliary
    towers_of_hanoi(n - 1, auxpeg, topeg, frompeg);
}
