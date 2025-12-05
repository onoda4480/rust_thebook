pub trait HelloMacro {
    fn hello_macro();
}

// 手続き的マクロを再エクスポート
pub use hello_macro_derive::HelloMacro;
