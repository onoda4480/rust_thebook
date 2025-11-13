fn main() {
    let v = vec![1, 2, 3, 4, 5];

    //要素外参照のためのパニック！
    let does_not_exist = &v[100];
    println!("{:?}", does_not_exist);
    
    //Optionでの安全な参照
    //Noneが返る
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);
}