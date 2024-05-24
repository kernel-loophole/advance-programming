use std::cmp::max;
// Define a Node struct to hold data and a reference to the next Node
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
//
// // Define a LinkedList struct which holds a reference to the head Node
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
//
impl<T> LinkedList<T> {
    // Create a new empty LinkedList
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node = Node {
            data: data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
//
//     // Remove and return the first element from the list
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
//
//     // Check if the list is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
//
//     // Iterate over the elements of the list
    fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}
//
// // Define an iterator struct for LinkedList
struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
//
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}
//
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("List elements:");
    for element in list.iter() {
        println!("{}", element);
    }

    println!("Popping element: {:?}", list.pop());
    println!("Popping element: {:?}", list.pop());
    println!("Popping element: {:?}", list.pop());
}

// fn main() {
//     let mut array_1=vec![1,12,2,2,34,5];
//     let mut array_2=vec![12,1,2,12,1];
//     // let result=medium_of_sorted_array(&array_1,&array_2);
//     // println!("{}",result);
//     // let number=239;
//     // let re=reverse_integers(number);
//     array_1.sort();
//     println!("{:?}",array_1);
//     let x=2;
//     search_range(&array_1,x);
// }
use std::collections::{HashMap, HashSet};
// fn permutation_of_array(array:&Vec<i32>)->Vec<i32>
// {
//     let array_copy=array.clone();
//     for i in 0..array_copy.len(){
//
//
//     }
//
//
//     array_copy
//
// }
fn medium_of_sorted_array(array: &Vec<i32>, array2: &Vec<i32>) -> f64 {
    let mut sorted_array = array.clone();
    let mut sorted_array_2 = array2.clone();
    let mut unique_val: HashSet<i32> = HashSet::new();
    let mut merged_sorted_array = Vec::new();
    sorted_array.sort();
    sorted_array_2.sort();
    unique_val.extend(sorted_array.iter());
    //unique val
    merged_sorted_array.extend(sorted_array.into_iter());
    merged_sorted_array.extend(sorted_array_2.into_iter());
    //sorted
    merged_sorted_array.sort();
    let mut merge_uniq:HashSet<i32>=HashSet::new();
    merged_sorted_array.retain(|&x|  merge_uniq.insert(x));
    println!("{:?}",merged_sorted_array);
    let len = merged_sorted_array.len();
    if len % 2 == 0 {
        let mid = len / 2;
        return (merged_sorted_array[mid - 1] + merged_sorted_array[mid]) as f64 / 2.0;
    } else {
        return merged_sorted_array[len / 2] as f64;
    }
}
// fn search_range<T>(array: &[i32], value: i32) -> (T, T)
//     where
//         T: Default + Copy,
// {
//     let mut tmp_vec = Vec::new();
//
//     let default_value: T = Default::default();
//     let default_tuple: (T, T) = (default_value, default_value);
//
//     if let Some(index) = array.iter().position(|&x| x == value) {
//         tmp_vec.push(index);
//         for i in index..array.len() {
//             println!("{}", array[i]);
//             if array.get(i + 1) != Some(&value) {
//                 tmp_vec.push(i);
//                 return (index.into(), i.into());
//             }
//         }
//     } else {
//         println!("Value {} not found in the array", value);
//         return default_tuple;
//     }
//
//     default_tuple
// }


fn reverse_integers(number: i32) -> String {
    let mut number_copy = number;
    let mut test_str = String::new();

    while number_copy != 0 {
        let number_tmp = number_copy % 10;
        let number_tmp_char = (number_tmp as u8 + b'0') as char;
        test_str.push(number_tmp_char);
        number_copy = number_copy / 10;
    }

    test_str
}
