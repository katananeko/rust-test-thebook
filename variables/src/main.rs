use std::thread;
use std::time::Duration;

fn main () {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    renerate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
