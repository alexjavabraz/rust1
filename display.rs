use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: 'f' . Returns 'fmt::Result' which indicates whether the
        // operation succeeded or failed. Note that 'write!' uses sintax which
        // is very similar to 'println!'. 
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main(){
    let minmax = MinMax(0, 14);
    
    println!("Compare structures: ");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range);

    let point = Point2D {x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

}