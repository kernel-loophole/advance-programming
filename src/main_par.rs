// // // // // use std::ffi::CString;
// // // // //
// // // // // // // use std::collections::HashMap;
// // // // // // // use std::borrow::Cow;
// // // // // // //
// // // // // // // fn abs_all(input: &mut Cow<'_, [i32]>) {
// // // // // // //     for i in 0..input.len() {
// // // // // // //         let v = input[i];
// // // // // // //         if v < 0 {
// // // // // // //             // Clones into a vector if not already owned.
// // // // // // //             input.to_mut()[i] = -v;
// // // // // // //         }
// // // // // // //     }
// // // // // // // }
// // // // // // // fn main()
// // // // // // // {
// // // // // // //     let slice = [0, 1, 2];
// // // // // // //     let mut input = Cow::from(&slice[..]);
// // // // // // //     abs_all(&mut input);
// // // // // // //
// // // // // // //     // Clone occurs because `input` needs to be mutated.
// // // // // // //     let slice = [-1, 0, 1];
// // // // // // //     let mut input = Cow::from(&slice[..]);
// // // // // // //     abs_all(&mut input);
// // // // // // //
// // // // // // //     // No clone occurs because `input` is already owned.
// // // // // // //     let mut input = Cow::from(vec![-1, 0, 1]);
// // // // // // //     abs_all(&mut input);
// // // // // // //    //  println!("Hello, world!");
// // // // // // //    //  let s1 = String::from("live");
// // // // // // //    //  let s2 =String::from("evil");
// // // // // // //    // if anagram(s1,s2){
// // // // // // //    //     // println!("{} and {}", s1,s2);
// // // // // // //    //     println!("yes");
// // // // // // //    // }
// // // // // // //    //  let test_str=String::from("hello world world");
// // // // // // //    //  reverse_world(test_str);
// // // // // // //    //  let tmp_string=String::from("abaab");
// // // // // // //    // //  longest_substring(tmp_string);
// // // // // // //    //  println!("{:?}",longestPalindrome(tmp_string));
// // // // // // //    //  let array = [0; 10];
// // // // // // //    //  let slice: &[u8] = &array;
// // // // // // //    //  println!("{:?}",array);
// // // // // // //    //  println!("{}",array.len());
// // // // // // //    //  let sub_array=&array[0..5];
// // // // // // //    //  for i in 0..array.len()
// // // // // // //    //  {
// // // // // // //    //      println!("{}",slice[i]);
// // // // // // //    //  }
// // // // // // //    //  println!("{:?}",sub_array);
// // // // // // //    //  let mut max_mu=HashMap::<String,i32>::new();
// // // // // // //    //  max_mu.insert("test".into(),1.into());
// // // // // // //    //  println!("{:?}", max_mu);
// // // // // // //    //  let get_key=max_mu.get("test");
// // // // // // //    //  println!("{:?}", get_key.unwrap());
// // // // // // //    //  let tmp_vec=vec![[12,3,3,3],[34,45,23]];
// // // // // // //    //
// // // // // // // //     let mut sarray:[[i32;3];3]=[
// // // // // // // // [1,2,3],
// // // // // // // //     [4,5,6],
// // // // // // // //     [7,8,9]
// // // // // // // // ];
// // // // // // // //     // let mut array: [[i32; 3]; 2] = [
// // // // // // // //     //     [1, 2, 3],
// // // // // // // //     //     [4, 5, 6],
// // // // // // // //     // ];
// // // // // // // //     for i in sarray.iter_mut()
// // // // // // // //     {
// // // // // // // //         for j in i.iter_mut()
// // // // // // // //         {
// // // // // // // //             println!("{:?}",i);
// // // // // // // //         }
// // // // // // // //     }
// // // // // // // //     // merge_k_sorted_list(tmpsvec);
// // // // // // // //
// // // // // // // // let debug=test_empty::new();
// // // // // // // //     let test_k_m=test_empty::
// // // // // // // //     let test_pr=test_empty{
// // // // // // // //         string_01: "".to_string(),
// // // // // // // //         name:String::from("hello"),
// // // // // // // //         itn_01:23
// // // // // // // //
// // // // // // // //
// // // // // // // //     };
// // // // // // // //     println!("{}", test_pr.string_01);
// // // // // // // //     println!("{}", test_pr.name);
// // // // // // // //     println!("{}", test_pr.itn_01);
// // // // // // // //     // let test_01=test_enum::name;
// // // // // // // //     // println!("{:?}",test_01);
// // // // // // // //     let dir=direction::East;
// // // // // // // //     print_dirction(dir);
// // // // // // // //     let heap_interger=Box::new(1);
// // // // // // // //     let box_heap=vec![0;100];
// // // // // // // //     let heap_string=String::from("hello world");
// // // // // // //
// // // // // // // }
// // // // // // // fn print_dirction(Direction:direction)
// // // // // // // {
// // // // // // //     match Direction {
// // // // // // //         direction::North => println!("North"),
// // // // // // //         direction::South => println!("South"),
// // // // // // //         direction::East => println!("East"),
// // // // // // //         direction::West => println!("West"),
// // // // // // //         _ => {}
// // // // // // //     }
// // // // // // //
// // // // // // // }
// // // // // // // #[derive(Debug)]
// // // // // // // enum  test_enum{
// // // // // // //     name,
// // // // // // //     itn_01,
// // // // // // //     string_01,
// // // // // // //     name_01,
// // // // // // // }
// // // // // // // #[derive(Debug)]
// // // // // // // enum direction
// // // // // // // {
// // // // // // //     north,
// // // // // // //     south,
// // // // // // //     east,
// // // // // // //     west,
// // // // // // //
// // // // // // //     North,
// // // // // // //     South,
// // // // // // //     East,
// // // // // // //     West,
// // // // // // // }
// // // // // // // // fn merge_k_sorted_list(array_list: Vec<i32>)->Vec<i32>
// // // // // // // // {
// // // // // // // //     let mut tmp_array=Vec::new();
// // // // // // // //     for i in array_list.iter()
// // // // // // // //     {
// // // // // // // //         for j in &tmp_array
// // // // // // // //         {
// // // // // // // //             println!("{}",j);
// // // // // // // //
// // // // // // // //         }
// // // // // // // //
// // // // // // // //
// // // // // // // //     }
// // // // // // // //     array_list
// // // // // // // // }
// // // // // // //
// // // // // // // fn anagram(s: String, t: String) -> bool {
// // // // // // //     if s.len() != t.len() {
// // // // // // //         return false;
// // // // // // //     }
// // // // // // //
// // // // // // //     let mut char_count = HashMap::new();
// // // // // // //
// // // // // // //      for ch in s.chars() {
// // // // // // //         *char_count.entry(ch).or_insert(0) += 1;
// // // // // // //     }
// // // // // // //
// // // // // // //      for ch in t.chars() {
// // // // // // //         if let Some(count) = char_count.get_mut(&ch) {
// // // // // // //             *count -= 1;
// // // // // // //             if *count == 0 {
// // // // // // //                 char_count.remove(&ch);
// // // // // // //             }
// // // // // // //         } else {
// // // // // // //             return false; // Character not found in the first string
// // // // // // //         }
// // // // // // //     }
// // // // // // //
// // // // // // //     char_count.is_empty() // True if all character counts matched
// // // // // // // }
// // // // // // // fn reverse_world(s:String)->String
// // // // // // // {
// // // // // // //     let test_vec:Vec<&str>=s.split_whitespace().collect();
// // // // // // //     println!("{:?}", test_vec);
// // // // // // //     let mut tmp_str=String::new();
// // // // // // //     // let mut reverse_str =test_vec.to_string();
// // // // // // //     for i in test_vec.iter().rev()
// // // // // // //     {
// // // // // // //         tmp_str.push_str(i);
// // // // // // //         tmp_str.push_str(" ");
// // // // // // //     }
// // // // // // //     // println!("{:?}", reverse_str);
// // // // // // //     // return reverse_str;
// // // // // // //     println!("{:?}", tmp_str);
// // // // // // //     s
// // // // // // // }
// // // // // // // fn longest_substring(s: String) -> i32{
// // // // // // //     // if s.len()
// // // // // // //     // {
// // // // // // //     //     return 1
// // // // // // //     // }
// // // // // // //     let mut tmp_vec=String::new();
// // // // // // //     let mut max=0;
// // // // // // //     for i in s.chars()
// // // // // // //     {
// // // // // // //         if tmp_vec.contains(i)
// // // // // // //         {
// // // // // // //             if tmp_vec.len() > max
// // // // // // //             {
// // // // // // //                 max = tmp_vec.len();
// // // // // // //             }
// // // // // // //             tmp_vec="".to_string();
// // // // // // //
// // // // // // //         }
// // // // // // //         else {
// // // // // // //             tmp_vec.push(i);
// // // // // // //         }
// // // // // // //
// // // // // // //     }
// // // // // // //     println!("{}",max);
// // // // // // //     max as i32
// // // // // // //
// // // // // // // }
// // // // // // // #[derive(Debug)]
// // // // // // // struct  test_empty {
// // // // // // //     string_01: String,
// // // // // // //     itn_01: i32,
// // // // // // //     name: String,
// // // // // // // }
// // // // // // //
// // // // // // // // impl test_empty {
// // // // // // // //     fn new() -> _ {
// // // // // // // //         todo!()
// // // // // // // //     }
// // // // // // // // }
// // // // // // //
// // // // // // // fn longestPalindrome(s: String) -> String{
// // // // // // //     let tmp_str=String::from("");
// // // // // // //     let mut my_vec:Vec<String>=Vec::new();
// // // // // // //     for i in 0..s.len()
// // // // // // //     {
// // // // // // //         let tmp_string=s[0..i].to_string();
// // // // // // //         // println!("{}",tmp_string);
// // // // // // //         let reverse_string:String=tmp_string.chars().rev().collect();
// // // // // // //         // print!("reversed string {}  and simple string {}",reverse_string,tmp_string);
// // // // // // //         if tmp_string==reverse_string
// // // // // // //         {
// // // // // // //             // println!("{}",tmp_string);
// // // // // // //             my_vec.push(tmp_string);
// // // // // // //
// // // // // // //         }
// // // // // // //
// // // // // // //         // println!("{}",reverse_string);
// // // // // // //         // println!("{}",tmp_str);
// // // // // // //     }
// // // // // // //     println!("{:?}", my_vec);
// // // // // // //     let max_eln=my_vec.iter().max_by_key( |&x| x.len()).unwrap();
// // // // // // // max_eln.to_string()
// // // // // // //
// // // // // // // }
// // // // // // // // fn reverse_integer(number:i32) -> i32
// // // // // // // // {
// // // // // // // //     let mut tmp_number=&number;
// // // // // // // //     while tmp_number> &9 {
// // // // // // // //         let reverse=tmp_number%10;
// // // // // // // //         tmp_number= &(tmp_number / 10);
// // // // // // // //         println!("{}", tmp_number);
// // // // // // // //         if tmp_number< &9
// // // // // // // //         {
// // // // // // // //             break
// // // // // // // //         }
// // // // // // // //
// // // // // // // //     }
// // // // // // // // number
// // // // // // // // }
// // // // // // fn inter_product(a:i32, b:i32) -> i32 {
// // // // // //     return a * b
// // // // // // }
// // // // // // fn main()
// // // // // // {
// // // // // //     // println!("product {}",inter_product(1,2));
// // // // // //     // let mut counter=0;
// // // // // //     // loop {
// // // // // //     //     if counter>10
// // // // // //     //     {
// // // // // //     //         break
// // // // // //     //     }
// // // // // //     //     println!("{}",counter);
// // // // // //     // counter+=1
// // // // // //     // }
// // // // // //     // counter+=1;
// // // // // //     //#####################shadowing in rust ######################
// // // // // //     let a = 10;
// // // // // //     println!("before: {a}");
// // // // // //     {
// // // // // //         let a = "hello";
// // // // // //         println!("inner scope: {a}");
// // // // // //         let a = true;
// // // // // //         println!("shadowed in inner scope: {a}");
// // // // // //     }
// // // // // //     println!("after: {a}");
// // // // // // }
// // // // // fn factorial(n: u32) -> u32 {
// // // // //     let mut product = 1;
// // // // //     for i in 1..=n {
// // // // //         product *= dbg!(i);
// // // // //     }
// // // // //     product
// // // // //
// // // // // }
// // // // // fn fizzbuzz(n: u32) -> u32 {
// // // // //     todo!()
// // // // // }
// // // // // fn main() {
// // // // //     // let n = 4;
// // // // //     // println!("{n}! = {}", factorial(n));
// // // // //     // let tuple=(32,32);
// // // // //     // triple_tuples(tuple);
// // // // //     // let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
// // // // //     // transpose(array);
// // // // //     // let input='p';
// // // // //     // match input {
// // // // //     //     'q'=> println!("fizzbuzz"),
// // // // //     //     'x' => println!("fizz"),
// // // // //     //     'z' => println!("buzz"),
// // // // //     //     _=> println!("invalid input"),
// // // // //     // }
// // // // //     let make_a_test=test_this::new("hello","test_suer");
// // // // //     make_a_test.print();
// // // // //     let test_dog=dog{name:String::from("Dog"),age:12};
// // // // //     test_dog.great();
// // // // //     println!("===========");
// // // // //     test_dog.wlak();
// // // // //
// // // // // }
// // // // // trait  animal
// // // // // {
// // // // //     fn count_lges(&self) -> u32;
// // // // // }
// // // // // trait pet:animal
// // // // // {
// // // // //     fn wlak(&self);
// // // // //     fn great(&self)->String;
// // // // // }
// // // // // struct dog
// // // // // {
// // // // //     name: String,
// // // // //     age: i32
// // // // // }
// // // // //
// // // // // impl animal for dog {
// // // // //     fn count_lges(&self) -> u32 {
// // // // //         todo!()
// // // // //     }
// // // // // }
// // // // //
// // // // // impl pet for dog
// // // // //     {
// // // // //     fn wlak(&self)
// // // // //     {
// // // // //     println!("walk{}",self.name);
// // // // //     }
// // // // //     fn great(&self)->String
// // // // //     {
// // // // //     format!("great{}",self.name)}
// // // // //     }
// // // // // struct test_this
// // // // // {
// // // // //     name_one: String,
// // // // //     name_two:String,
// // // // //
// // // // // }
// // // // // impl test_this
// // // // // {
// // // // //     fn new(name:&str,test_str:&str) -> Self
// // // // //     {
// // // // //         Self{ name_one: name.to_string(),name_two:test_str.to_string() }
// // // // //
// // // // //     }
// // // // //     fn print(&self)
// // // // //     {
// // // // //         println!("=======>  {}",self.name_one);
// // // // //         println!("=======>  {}",self.name_two);
// // // // //     }
// // // // //
// // // // // }
// // // // //
// // // // // fn triple_tuples(tuple:(i32,i32))
// // // // // {
// // // // //     let (x,y) = tuple;
// // // // //     let x=tuple.0;
// // // // //     let y=tuple.1;
// // // // //     println!("{},{}", x,y);
// // // // //
// // // // // }
// // // // // fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
// // // // //   let mut tmp_array:[[i32;3];3]=[[0;3];3];
// // // // //     println!("{:?}", tmp_array);
// // // // //     for i in 0..matrix.len() {
// // // // //     for j in 0..matrix[i].len() {
// // // // //         tmp_array[j][i] = matrix[i][j];
// // // // //     }
// // // // //   }
// // // // //     println!("{:?}", tmp_array);
// // // // //     matrix
// // // // // } fn ami
// // // // use std::fs::File;
// // // // use std::io::prelude::*;
// // // // use std::io::Read;
// // // // fn main()
// // // // {
// // // //     // println!("picking {}", pick(23, 2323, 23));
// // // //     // println!("{}", pick(23, "hello","test"));
// // // //     let file: Result<File, std::io::Error> = File::open("diary.txt");
// // // //     match file {
// // // //         Ok(mut file) => {
// // // //             let mut contents = String::new();
// // // //             if let Ok(bytes) = file.read_to_string(&mut contents) {
// // // //                 println!("Dear diary: {contents} ({bytes} bytes)");
// // // //             } else {
// // // //                 println!("Could not read file content");
// // // //             }
// // // //         }
// // // //         Err(err) => {
// // // //             println!("The diary could not be opened: {err}");
// // // //         }
// // // //     }
// // // // // }
// // // //
// // // // fn pick<T>(n:i32,even:T,odd:T)->T{
// // // //     if n%2==0
// // // //     {
// // // //         even
// // // //     }
// // // //     else {
// // // //         odd
// // // //     }
// // // //
// // // // }
// // // fn main()
// // // {
// // //     let add=|a,b| a+b;
// // //     let check_even=|number|if number%2==0{ true} else { false };
// // //     // println!("{}", add(1,2));
// // //     // println!("{}", check_even(10));
// // //     // let sum_array=|a| sum for i in a{ sum=sum+a }
// // //
// // //
// // // }
// // struct Point(i32, i32);
// // fn left_most<'a>(p1: & 'a Point, p2: &'a Point) -> & 'a Point {
// //     if p1.0 < p2.0 {
// //         p1
// //     } else {
// //         p2
// //     }
// // }
// // fn main() {
// // let p1: Point = Point(10, 10);
// // let p2: Point = Point(20, 20);
// // let p3 = left_most(&p1, &p2); // What is the lifetime of p3?
// // // println!("p3: {p3:?}");
// // //     println!("{:?}", p3);
// // }
//
// use std::vec;
//
// struct test
// {
//     name:String,
//     roll:i32
// }
// trait test_number
// {
//     fn test_prnitn()
//     {
//         todo!();
//
//     }
//
// }
// fn check_plaindrome(input_string:String)->bool
// {
//     // if input_string.len()==0 {return true}
//     let mut reverse_string=input_string.chars().rev().collect::<String>();
//
//
//     println!("{}",reverse_string);
//     if input_string==reverse_string
//     {
//         return true;
//     }
//     false
//
// }
// fn longest(input_string:String)->String
// {
//     let mut tmp=input_string.clone();
//     let mut tmp_vec=Vec::new();
//     for i in 0..tmp.len()
//     {
//         for j in i..tmp.len()
//         {
//             // println!("{}", tmp[j]);
//             let sub_array=&tmp[i..j];
//             let reverse_string=sub_array.chars().rev().collect::<String>();
//             if sub_array==reverse_string
//             {
//                 tmp_vec.push(reverse_string);
//             }
//             // println!("{}",sub_array);
//         }
//     }
//     println!("{:?}",tmp_vec);
//     let final_str=tmp_vec.iter().max_by_key(|s| s.len());
//     println!("{:?}",final_str.unwrap());
//     final_str.unwrap().parse::<String>().unwrap()
//
// }
// fn three_sum(input_vector:Vec<i32>)->Vec<Vec<i32>>
// {
//     let mut tmp_array=Vec::new();
//     // if input_vector.len()
//     // {
//     //     return input_vector
//     // }
//     for i in 0..input_vector.len()
//     {
//         for j in 0..input_vector.len()
//         {
//             for k in j..input_vector.len()
//             {
//                 if (input_vector[i] + input_vector[j] + input_vector[k] )== 0
//             {
//                 let mut tmp_vec=Vec::new();
//                 tmp_vec.push(input_vector[i]);
//                 tmp_vec.push(input_vector[j]);
//                 tmp_vec.push(input_vector[k]);
//                 tmp_array.push(tmp_vec);
//                 println!("")
//             }
//
//             }
//
//         }
//     }
//     tmp_array
// }
//
// fn main()
// {
//     // if check_plaindrome(String::from("madam")){
//     //     println!("yes ")
//     // }
//     // else { println!("no"); }
//     // println!("{:?}",longest("hello".to_string()));
//     println!("{:?}",three_sum(vec![-1,0,1,2,-1,-4]));
// }
use std::collections::HashMap;
use std::ptr::hash;

fn main()
{
    println!("Hello, world!");
    phone_call("23".to_string());

}
fn phone_call(input_string:String)->Vec<String>{
  let mut word_mapping=HashMap::new();
let mut final_vec=Vec::new();
    word_mapping.insert('2',"abc".to_string());
    word_mapping.insert('3',"def".to_string());
    word_mapping.insert('4',"ghi".to_string());
    word_mapping.insert('5',"jkl".to_string());
    word_mapping.insert('6',"mno".to_string());
    word_mapping.insert('7',"pqrs".to_string());
    word_mapping.insert('8',"tuv".to_string());
    word_mapping.insert('9',"wxyz".to_string());
    if input_string.len()==0
  {
      return vec![];
  }
    let mut match_vec=Vec::new();
    for i in input_string.chars()
    {
        // let get_val=word_mapping.get(&i)  ;
        // get_val as i32;
        match_vec.push(word_mapping.get(&i).unwrap().to_string());
    }

   // for i in 0..match_vec.len()
   // {
   //     let mut tmp_string=String::new();
   //     let number=i ;
   //
   //     for j in match_vec.iter(){
   //         //  println!("{}",j);
   //         // println!("{}",&match_vec[i]);
   //
   //         tmp_string.push(j[number]);
   //     }
   //     if tmp_string.len()>0
   //     {
   //         final_vec.push(tmp_string);
   //     }
   // }
    for i in 0..match_vec[0].len()*2 {
        let mut tmp_string = String::new();
        for j in match_vec.iter() {
            let chars: Vec<char> = j.chars().collect();
            if i < chars.len() {
                tmp_string.push(chars[i]);
            }
        }
        if !tmp_string.is_empty() {
            final_vec.push(tmp_string);
        }
    }
    // println!("{:?}", match_vec);
    println!("{:?}", final_vec);
    final_vec

}