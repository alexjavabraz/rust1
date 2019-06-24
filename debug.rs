//This structure cannot be printed either with 'fmt::Display' or
// with 'fmt::Debug'
// struct UnPrintable(i32);

// The 'derive' attribute automatically creates the implementation
// required to make this 'struct' printable with 'fmt::Debug'.
#[derive(Debug)]
struct Structure(i32);

// Put a 'Structure' inside of the structure 'Deep'. Make it printable
// also
#[derive(Debug)]
struct Deep(Structure);

fn main(){
    // Printint with '{:?}' is similar to width '{}'.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
                "Slater",
                "Christian",
                actor="actor's");

    // Structure is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with 'derive' is there is no control over how
    // the results look. What if i want this to just show a '7' ?
    println!("Now {:?} will print!", Deep(Structure(7)));
}