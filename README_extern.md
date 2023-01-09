# 引用外部庫

```rust
extern crate serde 

#[macro_use]
extern crate serde_derive 
```

這兩句話
- 第一句 `extern crate serde `：
代表本程序引用了外部 Rust crate (包)。

- 第二句 `#[macro_use] extern crate serde_derive `：
引用了 serde_derive crate 中的宏。這些 宏 (macro) 可以讓你使用 `#[derive(Serialize, Deserialize)]` 等註解，來自動產生序列化及反序列化的程式碼。
