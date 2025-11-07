fn main() {
    println!("=== Python vs Rust の比較 ===\n");

    // Rust の配列（固定長）
    println!("--- Rust の配列（固定長）---");
    let a = [1, 2, 3, 4, 5];
    println!("配列 a: {:?}", a);

    // スライス（参照）
    let slice = &a[1..3];
    println!("スライス &a[1..3]: {:?}", slice);
    println!("これは参照（ポインタ）です\n");

    // Rust の Vec（Pythonのlistに近い）
    println!("--- Rust の Vec（Pythonのlistに近い）---");
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Vec v: {:?}", v);

    v.push(6);  // 追加できる
    println!("v.push(6) した後: {:?}", v);

    v[0] = 999;  // 変更できる
    println!("v[0] = 999 した後: {:?}\n", v);

    // Vecのスライス
    println!("--- Vecのスライスも作れる ---");
    let vec_slice = &v[1..3];
    println!("&v[1..3]: {:?}", vec_slice);
    println!("これも参照です");

    // メモリの違い
    println!("\n=== メモリの違い ===");
    println!("Python:");
    println!("  a[1:3] → 新しいリストをコピー");
    println!("\nRust:");
    println!("  &a[1..3] → 元の配列を指すポインタ（コピーなし）");

    // ベクタの所有権（重要！）
    println!("\n=== ベクタはムーブする（コピーしない）===");
    let v_orig = vec![1, 2, 3];
    let v_moved = v_orig;  // ムーブ！

    // println!("{:?}", v_orig);  // ❌ エラー！
    println!("v_moved: {:?}", v_moved);
    println!("v_origはムーブされて使えない");

    // コピーしたい場合
    println!("\n--- 明示的にclone()すればコピーできる ---");
    let v1 = vec![10, 20, 30];
    let v2 = v1.clone();  // 明示的にコピー

    println!("v1: {:?}", v1);  // ✅ OK
    println!("v2: {:?}", v2);  // ✅ OK
}
