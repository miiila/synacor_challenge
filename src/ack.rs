use std::thread;
use std::collections::HashMap;

const STACK_SIZE: usize = 4 * 1024 * 1024 * 1024;
fn main() {
       let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn run() {
    for i in 1..32768 {
        let mut memo = HashMap::new();
        if ack(4,1,i, &mut memo) == 6 {
            println!("FOUND: {}", i);
            break
        };
    }
}


fn ack(m: u64, n: u64, k: u64, memo: &mut HashMap<(u64,u64), u64>) -> u64 {
    //dbg!(m);
    if memo.contains_key(&(m,n)) {
        return *memo.get(&(m,n)).unwrap();
    }
	let result = match (m, n) {
		(0, n) => (n + 1) % 32768,
		(m, 0) => ack(m - 1, k, k, memo),
		(m, n) => ack(m - 1, ack(m, n - 1, k, memo), k, memo),
	};
    memo.insert((m,n), result);
    result
}


