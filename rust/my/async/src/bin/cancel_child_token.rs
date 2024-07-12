use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    println!("Cancel parent token");
    let parent_token = CancellationToken::new();
    let child_token = parent_token.child_token();
    let grandchild_token = child_token.child_token();
    parent_token.cancel();
    dbg!(parent_token.is_cancelled());
    dbg!(child_token.is_cancelled());
    dbg!(grandchild_token.is_cancelled());

    println!("Cancel child token");
    let parent_token = CancellationToken::new();
    let child_token = parent_token.child_token();
    let grandchild_token = child_token.child_token();
    child_token.cancel();
    dbg!(parent_token.is_cancelled());
    dbg!(child_token.is_cancelled());
    dbg!(grandchild_token.is_cancelled());

    println!("Cancel grandchild token");
    let parent_token = CancellationToken::new();
    let child_token = parent_token.child_token();
    let grandchild_token = child_token.child_token();
    grandchild_token.cancel();
    dbg!(parent_token.is_cancelled());
    dbg!(child_token.is_cancelled());
    dbg!(grandchild_token.is_cancelled());
}
