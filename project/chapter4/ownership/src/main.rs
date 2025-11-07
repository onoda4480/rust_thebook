fn main() {
    println!("=== 配列スライスの例 ===\n");

    let a = [1, 2, 3, 4, 5];

    println!("元の配列: {:?}", a);
    println!("インデックス: 0  1  2  3  4\n");

    // スライスを作る
    let slice = &a[1..3];

    println!("slice = &a[1..3]");
    println!("結果: {:?}", slice);
    println!("型: &[i32]");
    println!("内容: インデックス1と2の要素 [2, 3]\n");

    // 様々なスライス
    let first_three = &a[0..3];
    println!("first_three = &a[0..3]: {:?}", first_three);

    let last_two = &a[3..5];
    println!("last_two = &a[3..5]: {:?}", last_two);

    let middle = &a[1..4];
    println!("middle = &a[1..4]: {:?}", middle);

    // 省略形
    println!("\n--- 省略形 ---");
    let from_start = &a[..3];
    println!("&a[..3]: {:?}", from_start);

    let to_end = &a[2..];
    println!("&a[2..]: {:?}", to_end);

    let all = &a[..];
    println!("&a[..]: {:?}", all);

    // スライスの操作
    println!("\n--- スライスの操作 ---");
    println!("slice の長さ: {}", slice.len());
    println!("slice の最初の要素: {}", slice[0]);
    println!("slice の2番目の要素: {}", slice[1]);
}
