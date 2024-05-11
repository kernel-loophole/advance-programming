// // functional programing
// // =====>Closures: a function-like construct you can store in a variable.
// // =====> Iterators: a way of processing a series of elements.
//
// use std::thread;
// use std::time::Duration;
// //cal for a second
// fn simulated_expensive_calculation(intensity: i32) -> i32
// {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }
// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;
//     // generate_workout(simulated_user_specified_value, simulated_random_number);
//     // iter_test();
//     iter_with_collect();
// }
// fn generate_workout(intensity: i32, random_number: i32)
// {
//     let expensive_closure = |num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//     if random_number==9{
//         println!("today is your lucky day");
//     }
//     else if random_number>50
//     {
//         println!("needed to work hard");
//     }
//      if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_closure(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_closure(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay
// hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             )
//         }
//     }
// }
// fn iter_test()
// {
//     let vec_one=vec![1,2,2,3,3,3];
//     let iter_tool=vec_one.iter();
//     // for i in iter_tool.clone()
//     // {
//     //     println!("{}",i);
//     // }
//
//
// //==============other iter methods======
//     // let sum_val:i32=iter_tool.sum();
//     // println!("{}",sum_val);
//
// }
// fn iter_with_collect()
// {
//     let test_vec=vec![1,2,3,4,4,4,23,2,5];
//     let v2:Vec<_>=test_vec.iter().map(|x| {if x%2==0 {x;} }).collect();
//     println!("{:?}",v2);
// }
// use List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     println!("{:?}", list);
// }


// Rust rc SMART POINTER
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)
    ))));
    let b = Cons(3, a.clone());
    let c = Cons(4, a.clone());
    println!("{:?}",b);
}