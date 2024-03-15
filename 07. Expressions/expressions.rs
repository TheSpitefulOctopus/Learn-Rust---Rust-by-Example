fn main() {
    // statement
    // statement
    // statement

    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to 'y'
        x_cube + x_squared + x
    };

    let z = {
        // the semicolon suppresses this expression and '()' is asigned to 'z'
        2 * x;
    };

    println!{"x is {:?}",x};
    println!{"y is {:?}",y};
    println!{"z si {:?}",z};
}