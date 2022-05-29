/*
- All types which want to use std::fmt formatting traits
require an implementation to be printable

- Automatic implementations are only provided for types such as in the std
library.

- The fmt::Debug trait makes manual implementation to be very straightforward.
- All types can derive (automatically create) the fmt::Debug implementation.

- This is not true for fmt::Display which must be manually implemented.
*/

// This struct cannot be printed either with fmt::Display or fmt::Debug
struct UnPrintable(i32);

// The 'derive' attribute automatically creates the implementations
// required to make this 'struct' printable with 'fmt::Debug'

#[derive(Debug)]
struct DebugPrintable(i32);

// All std library types are automatically printable with {:?} too:

// Derive the 'fmt::Debug' implementation for 'Structure'. 'structure'.
// is a structure which contains a single 'i32'
#[warn(dead_code)]
#[derive(Debug)]
struct Structure(i32);

// put a 'Structure' inside of structure 'Deep'. Make it printable also

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with '{:?}' is similar to with '{}'.
    println!("{:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    //  'Structure' is printable
    println!("Now {:?} will print!", Structure(420));

    // The problem with 'derive' is there is no control over how the results look
    // Rust also provides pretty printing with {:#?}
}
