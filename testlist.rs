// using ? on write! looks like this:

// Try 'write!' to see if it errors. If it errors, return
// the error. Otherwise continue.
write! {f, "{}", value}

// With ? available, implementing fmt::Display for Vec is straightforward
use std::fmt; // import the 'fmt' module

// Define a structure named 'List' containing a 'Vec'.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing and create a reference to 'vec'
        write!(f, "[")?;

        // iterate over 'v' in 'vec' while enumerating the iteration count in count
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors
            if count != 0 {
                write!(f, "{}", v)?;
            }
            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
