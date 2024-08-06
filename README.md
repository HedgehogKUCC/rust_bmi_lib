# Usage

## New Rust Project

```shell
$ cargo new hello_bmi
```

## Cargo.toml

Note: `bmi_calc_lib` 對應 `bmi_calc_lib/Cargo.toml > [package] > name`

```rust
[dependencies]
bmi_calc_lib = { git = "https://github.com/HedgehogKUCC/rust_bmi_lib.git", branch = "main" }
```

## `src/main.rs`

```rust
fn main() {
    let result = bmi_calc_lib::bmi(170, 65);

    match result {
        Ok(bmi) => {
            println!("BMI: {}", bmi);
        },
        Err(message) => {
            println!("{message}")
        }
    } 
}
```

```shell
$ cargo run
```
