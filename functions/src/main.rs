fn main() {
    println!("Hello, world!");
    another_function();

    println!();

    sum(10, 35);

    //
    // Expressions and statements.
    // Statements are instructions that perform SOMETHING, and they don't return any value.
    // Expressions evaluate to a result value, and they do return some value.
    //

    // A function definition, a variable declaration and a loop control keyword are examples of
    // statements.
    // Expressions don't end with a semicolon on the end. If do so, then they turn into a
    // statement. A function call is an expression.
}

// Parameterless function.
fn another_function() {
    println!("Running the another_function function...");
}

// Expects two numbers, and returns its sum, an i32.
// As it is possible to see, the "n1 + n2" expression is returned because it is the last expression
// of the function body.
fn sum(n1: i32, n2: i32)-> i32 {
    let r = n1 + n2;

    println!("The sum is... {r}.");
    n1 + n2
}
