#[tokio::main]
async fn main() {
    // tokio::select! {}
    //     error: select! requires at least one branch.
    //     --> my/experiments/src/bin/tokio_select.rs:3:5
    //      |
    //    3 |     tokio::select! {}
    //      |     ^^^^^^^^^^^^^^^^^
    //      |

    //

    // Basic select with two branches.

    async fn do_stuff_async() {
        println!("do_stuff_async() expression")
    }

    async fn more_async_work() {
        println!("more_async_work() expression")
    }

    tokio::select! {
        _ = do_stuff_async() => {
            println!("do_stuff_async() handler")
        }
        _ = more_async_work() => {
            println!("more_async_work() handler")
        }
    };

    // By default, select! randomly picks a branch to check first.
    // This provides some level of fairness when calling select! in a loop with branches that are always ready.

    // Output can be:
    // do_stuff_async() expression
    // do_stuff_async() handler
    // or
    // more_async_work() expression
    // more_async_work() handler

    //

    // This behavior can be overridden by adding biased; to the beginning of the macro usage.
    // This will cause select to poll the futures in the order they appear from top to bottom.
    // There are a few reasons you may want this:
    //   The random number generation of tokio::select! has a non-zero CPU cost.
    //   Your futures may interact in a way where known polling order is significant.

    tokio::select! {
        biased;
        _ = do_stuff_async() => {
            println!("do_stuff_async() handler")
        }
        _ = more_async_work() => {
            println!("more_async_work() handler")
        }
    };

    // Output is:
    // do_stuff_async() expression
    // do_stuff_async() handler

    // But there is an important caveat to this mode.
    // It becomes your responsibility to ensure that the polling order of your futures is fair.
    // If for example you are selecting between a stream and a shutdown future, and the stream has a huge volume of messages and zero or nearly zero time between them, you should place the shutdown future earlier in the select! list to ensure that it is always polled, and will not be ignored due to the stream being constantly ready.

    //

    // The select! macro panics if all branches are disabled and there is no provided else branch.
    // A branch is disabled when the provided if precondition returns false or when the pattern does not match the result of <async expression>.

    // tokio::select! {
    //     None = async {Some(1)} => {}
    // };
    // thread 'main' panicked at my/experiments/src/bin/tokio_select.rs:69:5:
    // all branches are disabled and there is no else branch

    // tokio::select! {
    //     Some(v) = async {Some(1)}, if false => {
    //         println!("handler")
    //     }
    // };
    // thread 'main' panicked at my/experiments/src/bin/tokio_select.rs:75:5:
    // all branches are disabled and there is no else branch

    tokio::select! {
        Some(_v) = async {Some(1)}, if false => {
            println!("handler")
        }
        else => println!("else handler")
    };

    // Output is:
    // else handler

    //

    // Waits on multiple concurrent branches, returning when the first branch completes, cancelling the remaining branches.
    // The select! macro must be used inside of async functions, closures, and blocks.
    // The select! macro accepts one or more branches with the following pattern:
    // <pattern> = <async expression> (, if <precondition>)? => <handler>,
    //
    // Additionally, the select! macro may include a single, optional else branch, which evaluates if none of the other branches match their patterns:
    // else => <expression>
    //
    // The macro aggregates all <async expression> expressions and runs them concurrently on the current task.
    // Once the first expression completes with a value that matches its <pattern>, the select! macro returns the result of evaluating the completed branch’s <handler> expression.
    //
    // Additionally, each branch may include an optional if precondition.
    // If the precondition returns false, then the branch is disabled.
    // The provided <async expression> is still evaluated but the resulting future is never polled.
    // This capability is useful when using select! within a loop.

    async fn get_option_async(i: u8) -> Option<i32> {
        println!("get_option_async() expression {i}");
        Some(42)
    }

    fn check_i(i: u8) -> bool {
        println!("check_i() precondition {i}");
        i != 2
    }

    tokio::select! {
        Some(_v) = get_option_async(1), if check_i(1) => {
            println!("get_option_async() handler 1")
        }
        Some(_v) = get_option_async(2), if check_i(2) => {
            println!("get_option_async() handler 2")
        }
        None = get_option_async(3), if check_i(3) => {
            println!("get_option_async() handler 3")
        }
    };

    // Output can be:
    // check_i() precondition 1
    // check_i() precondition 2
    // check_i() precondition 3
    // get_option_async() expression 1
    // get_option_async() handler 1
    // or
    // check_i() precondition 1
    // check_i() precondition 2
    // check_i() precondition 3
    // get_option_async() expression 3
    // get_option_async() expression 1
    // get_option_async() handler 1

    // The complete lifecycle of a select! expression is as follows:
    // 1. Evaluate all provided <precondition> expressions.
    //    If the precondition returns false, disable the branch for the remainder of the current call to select!.
    //    Re-entering select! due to a loop clears the “disabled” state.
    // 2. Aggregate the <async expression>s from each branch, including the disabled ones.
    //    If the branch is disabled, <async expression> is still evaluated, but the resulting future is not polled.
    // 3. Concurrently await on the results for all remaining <async expression>s.
    // 4. Once an <async expression> returns a value, attempt to apply the value to the provided <pattern>, if the pattern matches, evaluate <handler> and return.
    //    If the pattern does not match, disable the current branch and for the remainder of the current call to select!.
    //    Continue from step 3.
    // 5. If all branches are disabled, evaluate the else expression.
    //    If no else branch is provided, panic.

    // By running all async expressions on the current task, the expressions are able to run concurrently but not in parallel.
    // This means all expressions are run on the same thread and if one branch blocks the thread, all other expressions will be unable to continue.
    // If parallelism is required, spawn each async expression using tokio::spawn and pass the join handle to select!.

    //

    // Collect the contents of two streams.
    // In this example, we rely on pattern matching and the fact that stream::iter is “fused”, i.e. once the stream is complete, all calls to next() return None.

    use futures::StreamExt;

    let mut stream1 = futures::stream::iter(vec![1, 2, 3]);
    let mut stream2 = futures::stream::iter(vec![4, 5, 6]);

    let mut values = vec![];

    loop {
        tokio::select! {
            Some(v) = stream1.next() => values.push(v),
            Some(v) = stream2.next() => values.push(v),
            else => break,
        }
    }

    dbg!(values);

    //

    // This version will print "operation completed" indefinitely.
    // loop {
    //     tokio::select! {
    //         _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
    //             println!("operation timed out");
    //             break;
    //         }
    //         _ = tokio::time::sleep(std::time::Duration::from_millis(300)) => {
    //             println!("operation completed");
    //         }
    //     }
    // }

    // This version will time out after sleep.
    let sleep = tokio::time::sleep(std::time::Duration::from_secs(1));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            _ = &mut sleep => {
                println!("operation timed out");
                break;
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(300)) => {
                println!("operation completed");
            }
        }
    }

    // This version will use `CancellationToken`.
    let mut i = 0;
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                println!("cancellation token is cancelled");
                break;
            }
            _ = async {
                i += 1;
                if i == 4 {
                    cancellation_token.cancel()
                };
                tokio::time::sleep(std::time::Duration::from_millis(300)).await
            } => {
                println!("operation completed");
            }
        }
    }
}
