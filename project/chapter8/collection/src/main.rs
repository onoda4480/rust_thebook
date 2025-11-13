fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
    //*(参照外し)に関しては、
    // 第15章の「参照外し演算子」を参照してください。
}