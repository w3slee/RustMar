
fn main(){
    // The {} is automatically replaced with any argument
    let pro = "pulse";
    let _rux: String = format!("some random {} days", 32);
    println!("value :  {} rux", pro);
    println!("{}", _rux);
    // without a suffix 31 become an i32. You can change what type 31 is
    // by providing a suffix. 31i64 for example has the type i64

    // optional positional args can also be used
    format!("{0} preps {1} collect {2} Purge {3} Store", 31, 33, 34, 35);

    // named Arguments\
    println!("value power voltage");
    println!("{value} {power} {voltage}", value="023428ceg", power="cycle", voltage="423v");

    // Rust even checks to make sure the correct number of arguments are used

    // create a structure named 'Structure' which contains an 'i32'
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct '{:?}' won't print ...", Structure(42));
}
