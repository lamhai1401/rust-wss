use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand_id() -> usize {
    let num = rand::thread_rng().gen_range(100000000000..100000000000000);

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let result = num + since_the_epoch.as_millis();

    return result as usize;
}

pub fn sum(a: i8, b: i8) -> i8 {
    return a + b;
}
