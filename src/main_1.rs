mod lib;
use std::fs::File;
use std::collections::HashMap;

use comm::network;
use log::{debug, error};
use network::connect;
mod test_one;
// fn main() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("There was a problem opening the file:{:?}", error)
//         },
//     };
//
//
//
//             lib::network::connect();
//     test_one::abc_test::hello();
//     test_one::abc_test::test_more();
//     {
//         let mut test_vec = vec![1, 23, 3, 3, 3];
//         println!("{:?}",test_vec);
//         test_vec.push(23);
//         test_vec.push(90);
//         test_vec.push(45);
//         println!("{:?}",test_vec);
//         //get the val
//         //2 methods .get can access the out of bound val and [] cause pinic
//         let val=test_vec.get(1).unwrap();
//         let val_one=&test_vec[2];
//         println!("{:?}\n{}",val,val_one);
//     };
//     {
//         let mut has_val=HashMap::new();
//         has_val.insert(String::from("hello2"),4);
//         has_val.insert(String::from("hello3"),44);
//         has_val.insert(String::from("hello"),45);
//         println!("{:?}",has_val);
//     };
//     // exp_test();
// }
// fn main() {
//     let numbers = vec![34, 50, 25, 100, 65];
//     let result = largest(&numbers);
//     println!("The largest number is {}", result);
//     let chars = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&chars);
//     println!("The largest char is {}", result);
// }
// error handling

// fn exp_test()
// {
//     panic!("burn and exit");
//
// }

// Define a function `largest` that takes a slice of type T and returns an optional reference to the largest element
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    }
}

fn main() {
    let arr = [3, 9, 2, 5, 7];
    if let Some(largest_int) = largest(&arr) {
        println!("Largest integer in array: {}", largest_int);
    } else {
        println!("Array is empty");
    }

    let vec = vec![3.5, 9.1, 2.8, 5.6, 7.9];

    // Call the `largest` function with the vector
    if let Some(largest_float) = largest(&vec) {
        println!("Largest float in vector: {}", largest_float);
    } else {
        println!("Vector is empty");
    }
}
