/* use std::sync::atomic::{ AtomicU64, Ordering };

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn main() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Counter: {}", COUNTER.load(Ordering::SeqCst));
}
 */

/* use std::{ collections::VecDeque, sync::Mutex };

static QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
fn main() {
    QUEUE.lock().unwrap().push_back("Hello".into());
    QUEUE.lock().unwrap().push_back("World".into());
    println!("Queue: {:?}", QUEUE.lock().unwrap().make_contiguous());
    let first = QUEUE.lock().unwrap().pop_front();
    println!("First: {:?}", first);
} */

/* use std::sync::LazyLock;

struct GlobalState {
    counter: u64,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState { counter: 0 }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn example(&self) {
        println!("Counter value: {}", self.counter);
    }
}

static STATE: LazyLock<std::sync::Mutex<GlobalState>> = LazyLock::new(||
    std::sync::Mutex::new(GlobalState::new())
);

fn main() {
    let mut state = STATE.lock().unwrap();
    state.increment();
    state.example();
} */

mod guess_number;
fn main() {
    // guess_number::guess_number();
    guess_number::learn_rand();
}
