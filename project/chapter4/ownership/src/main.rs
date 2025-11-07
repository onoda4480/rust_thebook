fn main() {
    let s = String::from("hello");  // sがスコープに入る
    println!("{}", s);
                                    // sの値を表示
    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない
    //println!("{}", s);          // エラー!                                
    let x = 5;                      // xがスコープに入る
    println!("{}", x);          // ここではxはまだ有効
    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫
    println!("{}", x);          // ここでもxはまだ有効

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  //

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。