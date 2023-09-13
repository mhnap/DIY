#![feature(core_intrinsics)]

fn main() {
    {
        // Floating point division by zero will produce an infinity value.
        let a = 1 as f32;
        let b = 0 as f32;
        let c = a / b;
        dbg!(c);
    }

    {
        // Or will produce NAN if `a` is NAN.
        let a = f32::NAN;
        let b = 0 as f32;
        let c = a / b;
        dbg!(c);
    }

    {
        // Or will produce INFINITY if `a` is INFINITY.
        let a = f32::INFINITY;
        let b = 0 as f32;
        let c = a / b;
        dbg!(c);
    }

    {
        // Or will produce NEG_INFINITY if `a` is NEG_INFINITY.
        let a = f32::NEG_INFINITY;
        let b = 0 as f32;
        let c = a / b;
        dbg!(c);
    }

    //

    {
        // Compiler can detect integer literals division by zero.
        // let a = 1 / 0;
        // error: this operation will panic at runtime
        //   --> src/cpp_comparison/division_by_zero.rs:38:17
        //    |
        // 38 |         let a = 1 / 0;
        //    |                 ^^^^^ attempt to divide `1_i32` by zero
        //    |
        //    = note: `#[deny(unconditional_panic)]` on by default
        // dbg!(a);
    }

    {
        // Compiler can detect constexpr integer variables division by zero.
        // let a = 1;
        // let b = 0;
        // let c = a / b;
        // error: this operation will panic at runtime
        //   --> src/cpp_comparison/division_by_zero.rs:53:17
        //    |
        // 53 |         let c = a / b;
        //    |                 ^^^^^ attempt to divide `1_i32` by zero
        //    |
        //    = note: `#[deny(unconditional_panic)]` on by default
        // dbg!(c);
    }

    {
        // There is checked_div method that return Option<{number}>.
        fn get_zero() -> i32 { 0 }
        let a: i32 = 1;
        let b = get_zero();
        let c = a.checked_div(b);
        dbg!(c);
    }

    {
        // There is checks for runtime integer division by zero.
        fn get_zero() -> i32 { 0 }
        let a = 1;
        let b = get_zero();
        let c = a / b;
        dbg!(c);
        // thread 'main' panicked at 'attempt to divide by zero', src/cpp_comparison/division_by_zero.rs:69:17
    }

    {
        // There is also unsafe nightly-only API to divide unchecked.
        // https://stackoverflow.com/questions/42544491/can-i-disable-checking-for-zero-division-every-time-the-division-happens
        fn get_zero() -> i32 { 0 }
        let a = 1;
        let b = get_zero();
        let c = unsafe { std::intrinsics::unchecked_div(a, b) };
        dbg!(c);
        // The same result as for C/C++.
        // Process finished with exit code 136 (interrupted by signal 8: SIGFPE)
    }
}
