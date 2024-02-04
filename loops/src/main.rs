fn m() {
    let one=[1,23,4,5];
    let mut two=[23,23,23];
    for i in one.iter(){
        if i==&23{
            println!("found number");
        }
        println!("{}",i);
    }
    for j in two.iter(){
     
       println!("{}",j)
    }
    println!("{:?}",one);
    println!("{:?}",two);
}

extern crate regex; 
use std::{vec, collections::HashMap};

use regex::Regex;
fn strig() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
   dark square is a picture feverishly turned--in search of what?
   It is the same with books. What do we seek through millions of pages?";
    for line in quote.lines() {
    if line.contains(search_term) {
    println!("{}", line);
    }
    }
   } 
fn numbers(number_list:Vec<u8>)
{
    for i in number_list.into_iter(){
        println!("helo  there{}",i);
    }
} 
fn sum_number(number_list:Vec<u8>)->u8
{
    let mut sum=0;
    for i in number_list.into_iter()
    {
        sum=sum+i;

    }
    return sum; 
}
fn min_max(number_array:Vec<u8>)
{
    let mut sorted_array=number_array.clone();
    sorted_array.sort();
    let mut min=0;
    let mut max=0;
    for i in 0..sorted_array.len()-1
    {
        min=min+sorted_array[i];
        max=max+sorted_array[i+1];
     
         }
    println!("max {} and min {}",max,min);
}
fn test() {
    let re = Regex::new("picture").unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
   dark square is a picture feverishly turned--in search of what?
   It is the same with books. What do we seek through millions of pages?";
    for line in quote.lines() {
    match re.find(line) { 
    Some(_) => println!("{}", line),
    None => (),
    }
    };
    let  new_vec=vec![1,3,5,7,9];
    for i in new_vec.iter()
    {
        println!("{}",i);
    }

    // numbers(new_vec);
    // let total=    sum_number(new_vec);
    // println!("{}",total);
    // s
    let mut my_array=vec![3,5,90,17];
    my_array.sort();
    println!("{:?}",my_array);
    min_max(my_array);
//    min_max(new_vec);
}
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
   } 
impl  Suit {
    fn new()
    {

    }
    fn abc()
    {

        println!("");
    }
    
}



/* struct with the example */
// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }

//hasarea with triat impl ,trait for function sign

// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// trait HasArea {
//     fn area(&self) -> f64;
// }

// impl HasArea for Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }



trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn trait_example() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c);
    print_area(s);
}
fn urlfiy(tmp_string:String)->String{
    let copy_string=tmp_string.clone();
    copy_string.replace("", "20%");
    let mut tmp_string_one="".to_string();
    for i in copy_string.chars(){
        // println!("{}",i);
        if (i.to_string()!=" ")
        {
            tmp_string_one=tmp_string_one+&i.to_string();
        }
        if (i.to_string()==" "){
            println!("{}",i);
            tmp_string_one=tmp_string_one+"%20";
        }
    }
    return tmp_string_one

}
fn string_compression(string_tmp:String)->String
{
 let mut x=HashMap::new();
//  for i in string_tmp.chars()
//  {
//     println!("{}",i.next());
//  }
 
    return string_tmp;
}
fn main()
{
    test();
    trait_example();
    let x=urlfiy("Mr John Smith ".to_string());
    println!("{}",x);
}