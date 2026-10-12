# var-return

This crate provides a way to return variable-length arrays from functions without using heap allocation in Rust.

## Usage
```rust
use varreturn::{VarReturn, varreturn_callee, var_return, var_push, var_return_call};

fn caller() {
    let n = 5;
    var_return_call!(
        callee(n) -> [usize; n];
        |result| {
            assert_eq!(&*result, &[0, 1, 2, 3, 4]);
        }
    );
}

#[varreturn_callee]
fn callee<'a>(count: usize) -> VarReturn<'a, usize> {
    for i in 0..count {
        var_push!(i);
    }

    var_return!()
}
```
