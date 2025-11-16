struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let char_point = Point { x: 'a', y: 'b' };
    println!("integer Point: ({}, {})", integer.x, integer.y);
    println!("float Point: ({}, {})", float.x, float.y);
    println!("char Point: ({}, {})", char_point.x, char_point.y);
    //両方同じ方でないといけない
    //以下はエラー
    //let wont_work = Point { x: 5, y: 4.0 };
    //intとfloatでそれぞれ型が違うのでエラー
}