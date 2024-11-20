## Trouble shootings

Grok's initial suggestion were suggesting to use `extension-module` feature for `pyo3`

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
