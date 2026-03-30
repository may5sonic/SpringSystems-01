// use std::{thread, time::Duration};

// struct NumberCache<T>
// where 
//     T: Fn() -> i32,
// {
//     calculation: T,
//     value: Option<i32>,
// }

// impl<T> NumberCache<T>
// where 
//     T: Fn() -> i32,
// {
//     fn new(calculation: T) -> Self {
//         NumberCache {
//             calculation,
//             value: None,
//         }
//     }

//     fn get_value(&mut self) -> i32 {
//         match self.value {
//             Some(v) => {
//                 println!("Retrieved from cache instantly!");
//                 v
//             }
//             None => {
//                 println!("Computing (this will take 2 seconds)...");
//                 thread::sleep(Duration::from_secs(2));
//                 let v = (self.calculation)();
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    calculation : T,
    value : Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        ComputeCache {
            calculation: computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let v = (self.calculation)();
                self.value = Some(v.clone());
                v
            }
        }
    }
}

fn main() {
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