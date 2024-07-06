use core::fmt;

mod data_structures;

fn main() {
    //data_structures::array::array();
    //print_line_separator();
    //towers_of_hanoi(3, 'A', 'C', 'B');
    data_structures::single_linked_list::linked_list();
    print_line_separator();
    //let point = Point::new(3.46, 5.67);
    //let point_str = Point::new("some", "text");
    //println!("Point: {} {}", point_str, point.get_x());
    //println!("Point: {} {}", new_val, new_val.get_x());
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

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }

    fn update(&mut self, x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
