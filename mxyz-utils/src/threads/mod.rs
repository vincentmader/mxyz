use std::thread;

pub fn get_available_parallelism() -> usize {
    let count = thread::available_parallelism().unwrap().get();
    println!("{}", count); // -> on M1 Mac: 10
    count
}
