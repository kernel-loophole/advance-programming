extern crate regex;
use console::style;


use regex::Regex;
fn main() {
    // let vec1=vec![1,13,5,6,6];
    // let iter_test:Vec<_>=vec1.iter().map(|x| x+2).collect();
    // println!("{:?}",iter_test);
    // let mut str_test=String::from("test str");
    // let str_test_one:&str="test str";
    // println!("{:?}",str_test);
    // println!("{:?}",str_test_one);
    // let mut mut_x=&str_test;
    // println!("{:?}",mut_x);
    // println!("{:?}",str_test);
    // let mut test_final_str=String::from("helo str");
    // let mut final_str=this_function(&test_final_str);
    // final_str.retain(|c| c!='h');
    // println!("{:?}",final_str);
    // println!("{:?}",test_final_str);
  //   let test_str:String=String::from("[{}()]({}{})");
  // if   valid_par(&test_str){
  //     println!("true")
  // }
  //   else {
  //       println!("flase");
  //   }

    // let palin=String::from("A man, a plan, a canal: Panama");
    // if palindrome(&palin)
    // {
    //     println!("true")
    // }
    // else {
    //     println!("flase");
    // }

    // ==================haystack problem=============
    // let haytsack=String::from("sadsoiamsad");
    // let needle=String::from("sad");
    // let res=str_str(&haytsack,&needle);
    // println!("{:?}",res);
    // println!("This is {} neat", style("quite").blue());
    // println!("This is {} neat", style("testing color").red());
    let nums=vec![1,3,4,4,3,10];
    let target=13;
    let rs=two_sum(nums,target);
    println!("{:?}",rs);
}
fn this_function(test_str:&String)->String
{
    // test_str.push_str("hello");
    // test_str
    let mut final_str=test_str.clone();
    final_str.push_str("test added");
    //char_index
    for (i,j) in final_str.char_indices()
    {
        println!("index======>{},char===>{}",i,j);

    }
    final_str
}
//valid parr match find
//true if string is valid
//false if not
fn valid_par(test_str:&String)->bool
{
    let mut final_str=test_str.clone();
    let mut curl=String::new();
    let mut bar=String::new();
    let mut round=String::new();
    for i in final_str.chars()
    {
        println!("{}",i);
        if i=='['{ round.push(i); }
        else if i=='(' { bar.push(i); }
        else if i=='{'{curl.push(i);}
            //reverse match
        else if i==']'{
            if round.len()>0{
                round.pop();
            }
            else { return false }
        } else if i=='}' {
            if curl.len()>0
            {
                curl.pop();
            }
            else { return false }}
        else if i==')'
        {
            if bar.len()>0
            {
                bar.pop();
            }
            else { return false }}

    }
    if round.len()>0 || bar.len()>0 || curl.len()>0
    {
        return false
    }
    println!("{:?}",round);
    println!("{:?}",bar);
    println!("{:?}",curl);
    true
}
fn  palindrome(string_data:&str)->bool
{
    let final_str=string_data.clone() ;
    let re = Regex::new(r"[^a-zA-Z]").unwrap();
    let result=re.replace_all(final_str, "").to_lowercase();
    let final_str_reverse:String=result.chars().rev().collect();
    println!("given str======>{:?}",result);
    println!("reverse string===>{:?}",final_str_reverse);
    if result==final_str_reverse{
        return true
    }
    false
}
fn str_str(haystack:&String,needle:&String)->String
{
    if haystack.contains(needle)
    {
        let result = haystack.find(needle);
       let index_val=  match result {
        Some(index) => format!("{}", index),
        None => format!("'{}' not found", needle), };
        match index_val.parse::<i32>() {
            Ok(n) => println!("The number is: {}", n),
            Err(e) => println!("Failed to parse the string: {}", e),
        }
        return index_val;
    }
    else {
        let fromat_str=String::from("nothng found");
        return fromat_str;
    }

}
//Return the index whose sum is = target
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
let mut index_array=Vec::new();
    for (i,j) in nums.iter().enumerate(){
        println!("{}{}",i,j);
        // let sum= nums[i]+nums[i+1];
        if i+1<nums.len() {
            if let (Some(num_i), Some(num_j)) = (nums.get(i), nums.get((i+1))) {
                // println!("{} {}", num_i, num_j);
                let sum = num_i + num_j;
                if sum == target
                {
                    index_array.push(i);
                    index_array.push(i + 1);
                    return index_array;
                }
            }
        }
    }
    index_array
}