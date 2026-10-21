// (e) n x n int array: how much memory is data and how much is overhead?
//
// Думаю загальна space complexity буде O(n^2 + n), тобто просто n^2.
// Плюс буде через те що алокатору потрібно зберігати додаткову інформацію
// про виділену пам'ять у хедерах (bookkeeping), і якщо аррей вкладений, то буде n алокацій.
// Можна зробити просто один арей і вручну ітеруватись як у мартиці, тоді буде плюс константа

use std::alloc::{GlobalAlloc, Layout, System};
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

// Passes everything to the system allocator, but counts allocations and bytes
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

// Returns (allocations, bytes) made while running f
fn count<T>(f: impl FnOnce() -> T) -> (T, usize, usize) {
    let (allocs, bytes) = (ALLOCS.load(Relaxed), BYTES.load(Relaxed));
    let result = f();
    (result, ALLOCS.load(Relaxed) - allocs, BYTES.load(Relaxed) - bytes)
}

fn main() {
    let n = 1000;
    let data = n * n * size_of::<i32>(); // n^2 * 4 = 4_000_000

    // nested: outer array of n rows + n row arrays
    let (nested, allocs, bytes) = count(|| vec![vec![0i32; n]; n]);
    let overhead = bytes - data; // n * 24 = 24_000 (ptr, cap, len per row)
    println!("(e) nested: data = {data} B, overhead = {overhead} B, allocations = {allocs}"); // n + 1 = 1001
    drop(nested);

    // flat: one array, a[i][j] -> a[i * n + j]
    let (flat, allocs, bytes) = count(|| vec![0i32; n * n]);
    let overhead = bytes - data; // 0
    println!("(e) flat:   data = {data} B, overhead = {overhead} B, allocations = {allocs}"); // 1
    drop(flat);
}
