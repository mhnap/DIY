use signal_hook::consts::signal::SIGINT;
use signal_hook::iterator::Signals;

use crate::future::*;

pub fn ctrl_c() -> impl Future<Output = ()> {
    spawn_blocking(|| {
        let mut signal = Signals::new(&[SIGINT]).unwrap();
        let _ctrl_c = signal.forever().next().unwrap();
    })
}
