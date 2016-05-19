// fn main() {
// 	println!("Hello, World");
// }

fn main() {
	// main is a special function -- defines the beginning of every rust program
    println!("Hello, world!");
    // above line is calling a Rust macro -- this is how metaprogramming works in Rust 
    // if it were calling a function it would omit the bang operator 
    // the argument is a string -- a statically allocated string 

}

// Before running a rust program it has to be compiled 

// You use the Rust compiler by running the "rustc" command followed by the name of the source file to be compiled

// After compiling, rust will output a binary executable 