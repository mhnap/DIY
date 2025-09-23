use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    os::fd::RawFd,
    sync::{
        Arc, Mutex,
        atomic::{self, AtomicBool},
    },
};

use crate::future::*;

// The waker keeps everything connected.
#[derive(Clone)]
pub struct Waker(Arc<dyn Fn() + Send + Sync>);

impl Waker {
    pub fn wake(&self) {
        (self.0)()
    }
}

/// The scheduler keeps tracks of which tasks are runnable and polls them.
pub static SCHEDULER: Scheduler =
    Scheduler { runnable: Mutex::new(VecDeque::new()), shutdown: AtomicBool::new(false) };

pub type SharedTask = Arc<Mutex<dyn Future<Output = ()> + Send>>;

#[derive(Default)]
pub struct Scheduler {
    runnable: Mutex<VecDeque<SharedTask>>,
    shutdown: AtomicBool,
}

impl Scheduler {
    pub fn spawn(&self, task: impl Future<Output = ()> + Send + 'static) {
        self.runnable.lock().unwrap().push_back(Arc::new(Mutex::new(task)));
    }

    pub fn run(&self) {
        loop {
            if self.shutdown.load(atomic::Ordering::SeqCst) {
                println!("Shutdown scheduler");
                return;
            }

            loop {
                // Pop a runnable task off the queue.
                let Some(task) = self.runnable.lock().unwrap().pop_front() else { break };
                let t2 = task.clone();

                // Create a waker that pushes the task back on.
                let wake = Arc::new(move || {
                    SCHEDULER.runnable.lock().unwrap().push_back(t2.clone());
                });

                // Poll the task.
                task.lock().unwrap().poll(Waker(wake));
            }

            // If there are no runnable tasks, block on epoll until something becomes ready.
            REACTOR.with(|reactor| reactor.wait());
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, atomic::Ordering::SeqCst);
    }
}

thread_local! {
    /// The reactor marks tasks as runnable when epoll tells us something they are interested in becomes ready.
    pub static REACTOR: Reactor = Reactor::new();
}

pub struct Reactor {
    epoll: RawFd,
    tasks: RefCell<HashMap<RawFd, Waker>>,
}

impl Reactor {
    pub fn new() -> Reactor {
        Reactor { epoll: epoll::create(false).unwrap(), tasks: RefCell::new(HashMap::new()) }
    }

    // Add a file descriptor with read and write interest.
    //
    // `waker` will be called when the descriptor becomes ready.
    pub fn add(&self, fd: RawFd, waker: Waker) {
        let event = epoll::Event::new(epoll::Events::EPOLLIN | epoll::Events::EPOLLOUT, fd as u64);
        epoll::ctl(self.epoll, epoll::ControlOptions::EPOLL_CTL_ADD, fd, event).unwrap();
        self.tasks.borrow_mut().insert(fd, waker);
    }

    // Remove the given descriptor from epoll.
    //
    // It will no longer receive any notifications.
    pub fn remove(&self, fd: RawFd) {
        self.tasks.borrow_mut().remove(&fd);
    }

    // Drive tasks forward, blocking forever until an event arrives.
    pub fn wait(&self) {
        let mut events = [epoll::Event::new(epoll::Events::empty(), 0); 1024];
        let timeout = -1; // Forever.
        let num_events = match epoll::wait(self.epoll, timeout, &mut events) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("epoll::wait error: {e}");
                return;
            }
        };

        for event in &events[..num_events] {
            let fd = event.data as i32;

            // Wake the task.
            if let Some(waker) = self.tasks.borrow().get(&fd) {
                waker.wake();
            }
        }
    }
}
