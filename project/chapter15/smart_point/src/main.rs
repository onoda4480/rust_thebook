enum List {
    Cons(i32, List),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
//再帰的な列挙子を含むListを定義しているのでエラーになる
//error[E0072]: recursive type `List` has infinite size