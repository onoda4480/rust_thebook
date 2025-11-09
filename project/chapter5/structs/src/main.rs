#![allow(unused)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
// ↑でいちいち引の変数名とフィールド名を呼ぶのは面倒だから
// ↓でフィールド初期化省略記法を使っている
fn build_user_syokika(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

}