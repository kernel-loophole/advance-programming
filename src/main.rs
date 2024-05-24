use std::{borrow::Borrow, fmt::format};

fn sum_number(i:i32,j:i32)->i32{
    return i+j
}
fn ret_nothing(name:&str){
//    let x=name.to_owned();
    print!("hello{}",name)
}
fn modify_string(input: String) -> String {
    // Perform some modification on the input string
    let modified_string = format!("Modified: {}", input);

    // Return the modified string
    modified_string
}
fn return_str(name:String)-> String{
    let test_str:String=format!("test string :{}",name);
    return  test_str;
}

fn main() {
    // let x:i32=sum_number(32,45);
    // println!("{}",x);
    // ret_nothing("ali");
    // let fn_re=return_str(String::from("test user"));
    // println!("{}",fn_re);
    //Note
    //str and String different
    //str->imutable
    //String->heap allocate and can grow on runtime
    //############# some string fn##################
    let mut string_test:String=String::from("test str");
    let push_str=string_test+"hello";
    let mut x=push_str.len();
    let mut y=x;
    println!("Y={}",y);
    println!("X={}",x);
    y+=10;
    println!("{}",y);
    println!("{}",x);
    print!("{}",push_str.len());
    print!("{}",push_str);
}
fn make_it()
{
    println!("test fn");
}