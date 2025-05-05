fn main() {
    // Declaring an immutable variable:
    let _immutable_variable = 25;

    // It is not possible to change its value, as it is immutable.
    // _immutable_variable = 30;

    // Declaring a mutable variable:
    let mut mutable_variable = 30;
    mutable_variable = mutable_variable + 5;

    println!("Value of mutable_variable is {mutable_variable}!");

    // Constant values cannot be shadowed, and it is mandatory to use the type annotation. They can
    // also be declared on the global scope.
    const _HOUR_TO_SECOND: u16 = 60 * 60;

    // Shadowing is the practice of declaring again an already declared variable. Its type can be
    // changed too.
    let exploring_shadowing = 10;
    println!("The value of the exploring_shadowing variable is {exploring_shadowing}.");

    {
        let exploring_shadowing = "an string value!";
        println!("The value of the exploring_shadowing variable is {exploring_shadowing}.");
    }

    println!("The value of the exploring_shadowing variable is {exploring_shadowing}.");

    //
    // There's scalar and compound types. The first one represents a single value, and the latest
    // one, can represent multiple ones.
    // 
    
    // Scalar types

    // Integer ones range from 8-bit to 128-bit. They can be either signed or unsigned. The default
    // used are i32. The syntax
    // is "i" or "u" plus the size. There's also the "size" size, which refers to the OS architecture size. Some examples:
    let _small_integer: i8 = 10;
    let _bigger_integer: u128 = 9432423874;
    let _arch_integer: usize = 65535;

    // It is possible to write them in other ways too:
    let _decimal = 10_001;
    let _hex = 0xF8;
    let _octal = 0o22;
    let _binary = 0b11101011;
    let _byte = b'C';

    // Floating-point types are eotjer f32 or f64. They are all signed, and its default type are
    // f64.
    let _float_number = 10.001;

    // Number operations are all like the other programming languages.
    // +, -, *, /, %.

    // Boolean values are either "true" or "false".
    let _boolean_value = false;

    // Char values use single quotes. Its size is four bytes and are Unicode.
    let _char_value = 'A';
    let _emoji = '😻';

    // Compound types

    // Tuples: Groups many values of possible different types together.
    let _tuple_example_1: (u32, f64, char, bool) = (32, 64.0, 'A', false);

    // Destructuring and using pattern matching to retrieve a tuple values:
    let (_n1, _n2, _c1, _b1) = _tuple_example_1;

    // It is possible to retrieve the value based on its index:
    let _tuple_value_2 = _tuple_example_1.2;

    // An empty tuple '()' is called as an 'unit'. It represents an empty value or return type.
    // Expressions implicitly returns this value if no other value is returned.

    // Array type
    // An array is a type which groups values of the same type. Each one can be accessed by its
    // position on the array. Also, they can't grow or shrink. The type annotation describes the
    // array elements type and the fixed length of the array.
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the
    // stack.
    let _some_array: [i32; 3] = [-32, 0, 32];

    // Accessing array elements:
    let _array_element = _some_array[1];

    // As rust is memory-safe, it doesn't allow the access of items out of the bounds of the array.
    // No external memory is going to be accessed.
}
