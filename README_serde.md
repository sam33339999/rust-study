# SERDE
> 反序列化、序列化工具。

> Serde 框架的套件(crate)名稱為 `serde`，如果要使用它提供的程式巨集，就要使用 `serde_derive` 套件

## 閱讀
- [利用Serde框架在Rust程式語言上完成資料的序列化(Serialization)與反序列化(Deserialization)](https://magiclen.org/rust-serde/)

## 用法解釋
- 在某個類別上的 `#[derive(Debug, Deserialize, Serialize)]` 加上 `Deserialize` 參數，可以讓該類別自動實作 `serde::de::Deserialize` 特徵。

- 如果類型要序列化，則是 `Serialize` 這個特徵。


## 巨集進階使用

- 忽略欄位(`skip_serializing`)
- 有預設值(`default`)
- 選擇性忽略(`skip_serializing_if`)
    - `#[serde(skip_serializing_if = "Option::is_none", default)]`
    - `#[serde(skip_serializing_if = "Vec::is_empty", default)]`
- 忽略反序列化(`#[serde(skip_deserializing)]`) 就是反序列化時，值會被忽略
- 忽略序列及反序列化(`#[serde(skip)]`) 等效於 `#[serde(skip_serializing, skip_deserializing)]`
- 序列化重新命名(`#[serde(rename = "X")]`)
    - 序列化及反序列化個別設定:
        - `#[serde(rename(serialize = "X", deserialize = "a"))]`

```rust

```


## 解析動態 json 

```toml
serde_json = "*"
serde = {version = "*", features = ["derive"]}
```


```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MyJson {
    key1: String,
    key2: Vec<i32>,
    key3: InnerJson,
}

#[derive(Debug, Deserialize, Srialize)]
struct InnerJson {
    inner_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_str = r#"{
      "key1": "value1",
      "key2": [1, 2, 3],
      "key3": {"innerKey": "innerValue"}
    }"#;

    // 將 json 字串解析為 MyJson 物件
    let obj: MyJson = serde_json::from_str(json_str)?;
    println!("{:?}", obj);

    // 將 MyJson 物件序列化為 json 字串
    let serialized = serde_json::to_string(&obj)?;
    println!("{}", serialized);

    Ok(())
}
```

## 如果 json 某個 key 值，可能是陣列、字串、null，則使用 `serde_json::Value` 來表示
```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct MyJson {
    key1: String,
    key2: Vec<i32>,
    key3: Value,
}
```

也可以使用 `Option<T>` 類型來表示有時有值有時沒值
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MyJson {
    key1: String,
    key2: Vec<i32>,
    key3: Option<String>,
}
```

如果 key3 沒值，則在 解析 JSON 成為 MyJson 物件時，
`key3` 的值就會是 `None`，也可以使用 `Vec<T>` 類型來表示欄位有時是陣列，有時是單一值。

