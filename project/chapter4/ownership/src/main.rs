fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);  // r1の最後の使用

    let r2 = &mut s;  // r1はもう使われないのでOK
    println!("{}", r2);
}
