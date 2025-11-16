struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // ジェネリック型でそれぞれ違う型として定義できる
    // Point<T, U>のTとUにそれぞれ異なる型を指定できる
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let char_and_float = Point { x: 'a', y: 4.0 };
    println!("both_integer: ({}, {})", both_integer.x, both_integer.y);
    println!("both_float: ({}, {})", both_float.x, both_float.y);
    println!("integer_and_float: ({}, {})", integer_and_float.x, integer_and_float.y);
    println!("char_and_float: ({}, {})", char_and_float.x, char_and_float.y);
}