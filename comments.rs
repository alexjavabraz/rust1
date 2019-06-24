// This is an eample of a line comment
// Notice how there are two slashes at the beginning of the line
// And that nogthint written inside these will be ready by the compiler

// println("Hello, world!");

/*
 * This is another type of comment, the block comment. In general, the line comment is the recommended comment style however
 * the block comment is extremely useful for temporarily disabling
 * a large chunk of code. /* Block comments can be /* nested, */ */
 * so it takes only a few keystrokes to comment out all the lines 
 * in this main() function. /* /* /* Try it yourself! */ */ */ 
 */

fn main(){
    let x = 5 + /* 90 + */ 5;
    println!("Is 'x' 10 or 1000 ? x = {}", x);
}