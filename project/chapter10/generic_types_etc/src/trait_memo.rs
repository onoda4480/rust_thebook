//トレイト境界
pub fn notify<T: Summary>(item: &T) {
    // 速報！ {}
    println!("Breaking news! {}", item.summarize());
}
//簡単なケースではimpl Trait構文を使えばいいが、
//より複雑なケースではトレイト境界を使う方がいい場合がある

//例
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    //
    println!("Breaking news! {}", item1.summarize());
    //
    println!("Breaking news! {}", item2.summarize());
}
//上記の引数が冗長なので、トレイト境界を使うと以下のようにできる
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    //
    println!("Breaking news! {}", item1.summarize());
    //
    println!("Breaking news! {}", item2.summarize());
}

//複数のトレイト境界
pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
//上記は以下と同じ
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}