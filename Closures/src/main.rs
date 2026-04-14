use std::{thread, time::Duration};

fn main() {
    println!(" ");
    task1();
    println!(" ");
    task2();
    println!(" ");
    task3();
    println!(" ");
    task5();
}

fn task1() {
    let operation = |a: i32, b: i32| a * b;
    println!("Task 1 Result: {}", operation(10, 5));
}

fn task2() {
    let mut tracker = 0;

    let mut update = || {
        tracker += 1;
        println!("Tracker: {}", tracker);
    };

    update();
    update();
}

fn task3() {
    let numbers = vec![1, 2, 3];

    let doubled: Vec<i32> = numbers.clone().into_iter().map(|x| x * 2).collect();

    let replaced: Vec<i32> = numbers.into_iter()
        .map(|x| if x > 2 { 0 } else { x })
        .collect();

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}

fn task5() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());

    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}



struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let result = (self.computation)();
                self.value = Some(result.clone());
                result
            }
        }
    }
}