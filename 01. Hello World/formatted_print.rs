fn main() {
    // In general, the '{}' will be automatically replaced with any 
    // arguments. these will be stringified.
    println!("{} days", 31);

    // Positional arguments can be  used. Specifying an integer inside '{}'
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice","Bob");

    // as can named arguments.
    println!("{subject} {verb} {object}",
            object="the laxy dog",
            subject="the quick brown fox",
            verb="jumps over");
    
    // Different formatting can be invoked by specifying the format character 
    // after a ':'.
    println!("Base 10: {}", 69420); //69420
    println!("base 2 (binary): {:b}", 69420); //10000111100101100
    println!("base 8 (octal): {:o}", 69420); //207454
    println!("base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // you can right-justify text with a specified width. This will 
    // output '     1'. (four white spaces and a "1", for a total width of 5)
    println!("{number:>5}", number=1);

    // you can pad numbers with extra zeroes
    println!("{number:0>5}", number=1);
    // and left-adjust by flipping the sign. this will output "10000"
    println!("{number:0<5}", number=1);

    // you can use named arguments in the format specifier by appending a '$'
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // only types that implements fmt::Display can be formatted with '{}'
    // User - defined types do not implement fmt::Display by default
    #[allow(dead_code)] // disable 'dead_code' which warn against unused module
    struct Structure(i32);

    // this will not compile because 'Structure' does not implement
    // fmt::Display.
    //println!("This struct '{}' wont print...", Structure(3));
    // TODO: try uncommeingint this line ^
    
    
    // for rust 1.58 and above, you can directly capture the argument from 
    // a surrounding variable. just like the abovem this will output
    // '    1', 4 white spaces and a '1'
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // activities
    let pi = 3.141592;
    println!("Pi is roughly {:.3}",pi);
}