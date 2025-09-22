#[derive(Debug)]
struct WithDrop;

impl Drop for WithDrop {
    fn drop(&mut self) {
        println!("drop() for WithDrop");
    }
}

fn main() {
    {
        let with_drop = WithDrop;
        dbg!(&with_drop);
    }

    {
        // Note that because this function never returns, and that it terminates the process, no destructors on the current stack or any other thread's stack will be run.
        // If a clean shutdown is needed it is recommended to only call this function at a known point where there are no more destructors left to run; or, preferably, simply return a type implementing Termination (such as ExitCode or Result) from the main function and avoid this function altogether.
        let with_drop = WithDrop;
        dbg!(&with_drop);
        std::process::exit(0);
    }
}
