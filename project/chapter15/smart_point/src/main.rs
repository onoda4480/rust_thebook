fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    //assert_eq!(5, y);だとコンパイルエラーになる
    //比較するには一旦yの参照を外せば、yが指している整数値にアクセスできる。
}