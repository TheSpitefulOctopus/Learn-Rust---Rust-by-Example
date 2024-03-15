fn main() {
    // this is an example of a line comment.
    // there are two slashes at the beginning of the line.
    // and nothing written after these will be read by the compiler

    // println!("Hello, world!");

    // run it. see? now try deleting the two slashes and run it again.

    /*
    * this is another type of comment, a block comment. In general
    * line comments are the recommended comment style. but block comments
    * are extremely useful for temporarily disabling chunks of code.
    * /* block comments can be /* nested */ */ so it takes only a few keystrokes
    * to comment out everything in this main() function.
    */

    /*
    Note: The previous cloumn of '*' was entirely for style. There's
    no actual need for it
    */

    // you can manipulate expressions more easily with block comments
    // then with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is 'x' 10 or 100? x = {}", x);
}