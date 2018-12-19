use std::ops::AddAssign;
use std::fmt::{Debug, Display};
use std::thread;
use std::time::Duration;

mod num_traits;
use crate::num_traits::zero::Zero;

fn sum<T>(list: &[T]) -> T
where
    T: AddAssign + Copy + Zero,
    {
    let mut total = T::zero();
    for value in list {
        total += *value;
    }
    total
}

fn min<T>(list: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if list.is_empty() {
        return None;
    }
    let mut min_value = list[0];
    for value in list {
        if *value < min_value {
            min_value = *value;
        }
    }
    Some(min_value)
}

fn check<T>(list: &[T])
where
    T: AddAssign + PartialOrd + Copy + Zero + Display + Debug,
{
    println!("list is {:?}", list);
    println!("sum is {}", sum(&list));
    let min_value = min(&list);
    match min_value {
        Some(value) => println!("min value is {}\n", value),
        None => println!("no min value found\n"),
    }
}

fn main() {
    check(&[2, 1, 3]);
    check(&[3.1, 2.1, 1.3]);
    check::<i32>(&[]);

    let list = vec![1, 2];
    let handle = thread::spawn(move || {
        println!("another thread spawned");
        thread::sleep(Duration::from_secs(2));
        sum(&list)
    });
    println!("main thread");
    match handle.join() {
        Ok(result) => println!("the result is {}", result),
        Err(e) => println!("error happened {:?}", e),
    }
}
