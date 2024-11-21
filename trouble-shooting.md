# Trouble shootings

#### Python extension usage

Grok's initially suggested code were suggesting to use `extension-module` feature for `pyo3`

Tried to solve with these specification and correcting missing env variables
Such as `PYTHON_HOME` and faced issues `ld` and `cc` compilation steps

```toml
[target.x86_64-apple-darwin]
rustflags = [
"-C", "link-arg=-undefined",
"-C", "link-arg=dynamic_lookup",
"-L", "$HOME/.pyenv/versions/3.13.0/lib",
"-lpython3.13"
]
```

Fix is

```diff
[dependencies.pyo3]
version = "0.23.1"
- features = ["extension-module"]
+ features = ["auto-initialize"]
```

#### Diesel database ORM and column name `star`

Yesterday(November 20, 2024) last more than 2 hours after adding a column name `star`. It is special for `diesel` to be used for `count` queries.

![image](/image/star-colum-naming-issue-reason.png)

```rust
// @generated automatically by Diesel CLI.

diesel::table! {
    species (id) {
        id -> Int4,
        planet -> Varchar,
        name -> Varchar,
        population -> Float8,
        traits -> Array<Nullable<Text>>,
        star -> Varchar, // Problematic
    }
}
```
