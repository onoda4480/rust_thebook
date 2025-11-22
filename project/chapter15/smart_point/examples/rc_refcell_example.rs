use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("=== Rc<RefCell<T>> の例 ===\n");

    // Rc<RefCell<i32>> を作成
    // これは「共有された値の中身」を変更できる
    let shared_value = Rc::new(RefCell::new(100));

    println!("1. 最初の状態:");
    println!("   共有値: {}", shared_value.borrow());

    // 同じ値を共有する別の所有者を作成
    let owner_a = Rc::clone(&shared_value);
    let owner_b = Rc::clone(&shared_value);
    let owner_c = Rc::clone(&shared_value);

    println!("   参照カウント: {}", Rc::strong_count(&shared_value));
    println!("   owner_a から見た値: {}", owner_a.borrow());
    println!("   owner_b から見た値: {}", owner_b.borrow());
    println!("   owner_c から見た値: {}", owner_c.borrow());

    println!("\n2. owner_a を使って共有値を変更:");
    *owner_a.borrow_mut() = 200;

    println!("   共有値: {}", shared_value.borrow());
    println!("   owner_a から見た値: {}", owner_a.borrow());
    println!("   owner_b から見た値: {}", owner_b.borrow());
    println!("   owner_c から見た値: {}", owner_c.borrow());
    println!("   → すべての所有者から変更が見える！");

    println!("\n3. owner_b を使ってさらに変更:");
    *owner_b.borrow_mut() = 300;

    println!("   共有値: {}", shared_value.borrow());
    println!("   owner_a から見た値: {}", owner_a.borrow());
    println!("   owner_c から見た値: {}", owner_c.borrow());

    println!("\n【重要】Rc<RefCell<T>> では:");
    println!("  - 共有されている値の中身を変更できる");
    println!("  - すべての所有者から変更が見える");
    println!("  - ポインタの差し替えはできない（常に同じRefCellを指す）");
}
