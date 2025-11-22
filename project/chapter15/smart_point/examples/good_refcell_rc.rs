use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    println!("=== RefCell<Rc<T>> の正しい使い方 ===\n");

    // 最初の構造: a → c
    let c = Rc::new(Nil);
    let a = Rc::new(Cons(5, RefCell::new(Rc::clone(&c))));

    println!("1. 初期状態:");
    println!("   a → c");
    println!("   a = {:?}", a);

    // bを作成: b → c
    let b = Rc::new(Cons(3, RefCell::new(Rc::clone(&c))));

    println!("\n2. bを作成:");
    println!("   a → c");
    println!("   b → c");

    // aの次を変更: a → b (循環参照にならない！)
    if let Cons(_, ref next) = *a {
        *next.borrow_mut() = Rc::clone(&b);
    }

    println!("\n3. aの次をbに変更:");
    println!("   a → b → c");
    println!("   a = {:?}", a);

    println!("\n✅ 循環参照なし！");
    println!("✅ RefCell<Rc<T>> は問題なく使えている");
    println!("✅ メモリは正常に解放される");

    // 参照カウントを確認
    println!("\n参照カウント:");
    println!("   a の参照カウント: {}", Rc::strong_count(&a));
    println!("   b の参照カウント: {}", Rc::strong_count(&b));
    println!("   c の参照カウント: {}", Rc::strong_count(&c));
}
