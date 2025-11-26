fn main() {
    let Some(x) = some_option_value;
    // コンパイルエラー: `x`はスコープ外です
    //パターンが論駁可能であることを意味する

    //論駁不可能なもの。例：let x = 5;のx
    //論駁可能なもの。例：let Some(x) = some_option_value;のx
}