// https://fasterthanli.me/articles/pin-and-suffering
// https://rust-lang.github.io/async-book/part-reference/pinning.html
// https://blog.m-ou.se/super-let/#pin

use pin_project::pin_project;
use std::{
    future::Future,
    pin::{Pin, pin},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, ReadBuf},
    time::{Instant, Sleep},
};

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let mut buf = vec![0u8; 128 * 1024];
    let mut f = File::open("/dev/urandom").await?;

    let slr = SlowRead::new(&mut f);
    let mut sr = pin!(slr);

    // slr;
    // error[E0382]: use of moved value: `slr`
    //   --> my/async/src/bin/async_test.rs:22:5
    //    |
    // 19 |     let slr = SlowRead::new(&mut f);
    //    |         --- move occurs because `slr` has type `SlowRead<&mut tokio::fs::File>`, which does not implement the `Copy` trait
    // 20 |     let mut sr = pin!(slr);
    //    |                       --- value moved here
    // 21 |
    // 22 |     slr;
    //    |     ^^^ value used here after move

    let before = Instant::now();
    sr.read_exact(&mut buf).await?;
    println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

    let before = Instant::now();
    f.read_exact(&mut buf).await?;
    println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

    Ok(())
}

#[pin_project]
struct SlowRead<R> {
    #[pin]
    reader: R,
    #[pin]
    sleep: Sleep,
}

impl<R> SlowRead<R> {
    fn new(reader: R) -> Self {
        Self { reader, sleep: tokio::time::sleep(Default::default()) }
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut this = self.project();

        match this.sleep.as_mut().poll(cx) {
            Poll::Ready(_) => {
                this.sleep.reset(Instant::now() + Duration::from_millis(25));
                this.reader.poll_read(cx, buf)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
