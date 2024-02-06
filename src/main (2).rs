use std::cmp::max;
use std::result;
use std::io;
//
// fn main() {
//     // println!("Hello, world!");
//     // let x:i32=90;
//     // print!("{}",x);
//     // let  y=abc();
//     // println!("{}",y);
//     // let  test_string=String::from("test_str");
//     // mut_ref(&test_string);
//     // let string_pr_one_string=String::from("abcabc");
//     // let test_one=String::from("abcabcbb");
//     // let test_two=String::from("bbbbb");
//     // let result_one=string_pr_one(&string_pr_one_string);
//     // let test_case_one=string_pr_one(&test_one);
//     // let test_case_two=string_pr_one(&test_two);
//     // println!("result one {}",result_one);
//     // println!("result two{}",test_case_one);
//     // println!("result three {}",test_case_two);
//     //string plaindrome
//     let test_plaindrome=String::from("babab");
//     let longest_palindromic_substring_reslut=longest_palindromic_substring(&test_plaindrome);
//     println!("{}",longest_palindromic_substring_reslut)
// }
fn abc()->i32{
    println!("hello");
    32
}
fn mut_ref(s:& str)
{
    println!("{}",s);
    let len=s.len();
    let sub_slice=&s[3..len];
    println!("{}",len);
    println!("{}",sub_slice);

}
fn string_pr_one(s:&str)->usize
{
    let mut tmp_str=String::from("");
    let mut max_val=0;
    for i in s.chars(){
        if tmp_str.contains(i)
        {
            if max_val<tmp_str.len() {
                max_val = tmp_str.len();
            }
       tmp_str.clear();
        }
        else {
            tmp_str.push(i);
        }
    }
max_val
}

fn longest_palindromic_substring(s: &str) -> String {
    let mut longest = String::new();

    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            let substring = &s[i..j];
            if is_palindrome(substring) && substring.len() > longest.len() {
                longest = String::from(substring);
            }
        }
    }

    longest
}

fn is_palindrome(s: &str) -> bool {
    let test_str=String::from("hello");
    let sub_part=&test_str[0..test_str.len()];
    s.chars().eq(s.chars().rev())
}
//chapter#05
//====================Struct=====================
#[derive(Debug)]
struct User {
     username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
     }
fn main()
{
    let mut username=String::new();
    let mut email=String::new();
    let mut sign=34;
    let mut active =true;
    println!("enter email");
    io::stdin().read_line(&mut username);
    let user_one=User{
        username:username,
        email:email,
        sign_in_count:sign,
        active:active,
    };
    println!("{}{}",user_one.sign_in_count,user_one.email);
    let user_test=test
    {
        user_hash:String::from("kajskas_papas"),
        user_time:32,
        user_io:0.1,
        user_ibx:21,
    };
    println!("{:?}",user_test);
    user_test.diplay_with_one_x();
    user_test.get_ibx_time();
}
#[derive(Debug)]
struct  test
{
    user_hash:String,
    user_ibx:i32,
    user_time:i32,
    user_io:f32,

}
impl  test{
    fn diplay_with_one_x(&self)
    {
    println!("{}",self.user_hash);
    }
    fn get_ibx_time(&self)
    {
        println!("{}{}",self.user_ibx,self.user_time);
    }
}