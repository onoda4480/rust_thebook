fn foo(x: i32) {
    //関数シグニチャが引数にパターンを使用している例
    // コードがここに来る
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    // 現在の位置: ({}, {})
    //値&(3, 5)はパターン&(x, y)と合致する
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}