use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("=== RefCell<Rc<T>> の例 ===\n");

    // 最初の状態
    let value_a = Rc::new(String::from("データA"));
    let value_b = Rc::new(String::from("データB"));

    // RefCell<Rc<String>> を作成
    // これは「どのRcを指すか」を後から変更できる
    let pointer = RefCell::new(Rc::clone(&value_a));

    println!("1. 最初の状態:");
    println!("   pointer は value_a を指している: {}", pointer.borrow());
    println!("   value_a の参照カウント: {}", Rc::strong_count(&value_a));
    println!("   value_b の参照カウント: {}", Rc::strong_count(&value_b));

    println!("\n2. pointer の参照先を value_b に変更:");
    // ポインタ自体を差し替える（参照先を変更）
    *pointer.borrow_mut() = Rc::clone(&value_b);

    println!("   pointer は value_b を指している: {}", pointer.borrow());
    println!("   value_a の参照カウント: {}", Rc::strong_count(&value_a));
    println!("   value_b の参照カウント: {}", Rc::strong_count(&value_b));

    println!("\n3. もう一度変更:");
    *pointer.borrow_mut() = Rc::clone(&value_a);
    println!("   pointer は value_a を指している: {}", pointer.borrow());

    println!("\n【重要】RefCell<Rc<T>> では:");
    println!("  - ポインタ（Rc）を別のポインタに差し替えられる");
    println!("  - データA、データB自体は変更されない（不変）");
}
