fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);
    // エラー: 可変参照と不変参照の同時使用
    //解放されたメモリにアクセスしようとしているため、
    // コンパイルエラーになる。
    println!("The first element is: {}", first);
}