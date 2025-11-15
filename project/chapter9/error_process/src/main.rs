use std::io;
use std::io::Read;
use std::fs::File;
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
    //mainは返却値が()のためだめ
    //?演算子は戻り値にResultを持つ関数でしか使用できません。
}
