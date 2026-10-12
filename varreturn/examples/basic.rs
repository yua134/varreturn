use varreturn::{VarReturn, VarReturnable, var_push, var_return, var_return_call};

#[derive(VarReturnable)]
struct DVar<'a, T> {
    data: VarReturn<'a, T>,
}

fn main() {
    let n = 3;
    let _i32 = var_return_call!(
        dyn_callee(true) -> [u32; n];
        |arr| {
            println!("{:?}", &arr.data[0..n]);
            arr.data[0]
        }
    );

    let i = 3;
    let _i32 = var_return_call!(
        dyn_callee_push(i) -> [usize; i];
        |arr| {
            println!("{:?}", &arr.data[0..i]);
            arr.data[0]
        }
    );
}

#[varreturn::varreturn_callee]
fn dyn_callee<'a>(which: bool) -> DVar<'a, u32> {
    let data = if which {
        var_return!(
            [1, 2, 3]
        )
    } else {
        var_return!(
            [1, 2, 3, 4, 5, 6]
        )
    };

    DVar { data }
}

#[varreturn::varreturn_callee]
fn dyn_callee_push<'a>(count: usize) -> DVar<'a, usize> {
    for i in 0..count {
        var_push!(i);
    }

    DVar { data: var_return!() }
}