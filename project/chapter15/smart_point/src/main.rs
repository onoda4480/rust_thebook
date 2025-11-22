fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    //yが、xの値を指す参照ではなく、 
    //xの値を指すボックスのインスタンスとして設定されている
}