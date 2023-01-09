# toml 各式寫法

```toml
[dependencies]
serde_json = "*"
serde = {version = "*", features = ["derive"]}

[dependencies.uuid]
version = "1.2.1"
features = [
    "v4",                # Lets you generate random UUIDs
    "fast-rng",          # Use a faster (but still sufficiently random) RNG
    "macro-diagnostics", # Enable better diagnostics for compile-time UUIDs
]
```
