fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;
    //fn equal_to_x(z: i32) -> bool { z == x }
    //上記だとコンパイルエラーになる

    let y = 4;

    assert!(equal_to_x(y));
}