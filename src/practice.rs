extern crate regex;

use std::collections::HashMap;
use std::env::var;
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
    // let nums=vec![1,3,4,4,3,10];
    // let target=13;
    // let rs=two_sum(nums,target);
    // println!("{:?}",rs);
    // let version1="1.20";
    // let version=String::from("1.30");
    // let version2=String::from("1.30");
    // let ver_call=compare_version(&version,&version2);
    // println!("{}",ver_call);
    // =======================word_dic code============
    // let mut word_dic=vec!["apple","pen","apple","dog"];
    // let word=String::from("applepen");
    // // word_dic.push("apple");
    // // word_dic.push("pen");
    // // word_dic.push("apple");
    // if word_break(&word,word_dic)
    // { println!("true") }
    // else { println!("false") }
    //==============Reverse a string like hello word======>word hello =======
    // let s=String::from("hello world");
    // let final_str=reverse_words(s);
    // println!("{:?}",final_str);
    // let tmp_str=String::from("aacccbb");
    // let res=string_compress(tmp_str);
    // println!("{:?}",res);
    let test_ana_gram=vec![String::from("ate")
                           , "eta".parse().unwrap(),
                           "str".parse().unwrap(),
                           "tsr".parse().unwrap()];
    let test_02=vec![String::from("eat")
                     ,String::from("tea"),
                     String::from("tan"),
                     String::from("ate"),
                     String::from("nat"),
                     String::from("bat")];
    group_anagram(&test_02);

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
fn compare_version(version1:&String,version2:&String)->i32
{
    let mut version_after_split:Vec<&str>=version1.split('.').collect();
    let mut version2_after_split:Vec<&str>=version2.split('.').collect();
    // println!("{:?}",version2_after_split);
    // println!("{:?}",version_after_split);
    if version_after_split[1]<version2_after_split[1]
    {
        return -1;
    }
    1
}
fn word_break(word:&String,word_dic:Vec<&str>)->bool
{
    for i in word_dic
    {
        if !word.contains(&i)
        {
            // println!("{}",i);
            return false }
    }
    true
}
fn reverse_words(s:String)->String{
    let reverse_string_list:Vec<&str>=s.split(" ").collect();
    let mut final_str=String::new();
    for i in reverse_string_list.iter().rev()
    {
        // println!("{:?}",reverse_string_list[reverse_string_list-i]);
        // println!("{:?}",i);
        final_str.push_str(i);
        final_str.push_str(" ")

    }

    // println!("{:?}",reverse_string_list);s
    final_str
}
fn string_compress(S: String) -> String {
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for c in S.chars() {
        let counter = char_count.entry(c).or_insert(0);
        // println!("{:?}",char_count);
        println!("{}===>{}",c,counter);
        *counter += 1;
    }

    let mut compressed = String::new();
    for (char, count) in char_count {
        compressed.push(char);
        compressed.push_str(&count.to_string());
    }

    compressed
}
fn group_anagram(array: &Vec<String>) ->&Vec<String>
{
    // let hash_vec=HashMap::new();
    let mut result_vec=Vec::new();
    for i in 0..array.len(){
        let mut tmp_vec=Vec::new();
        for j in i..array.len()
        {
            println!("{}{}",i,j);
            // if i==j{
            //
            // }

            let s1=array[i].clone();
            let s2=array[j].clone();
            let mut chars: Vec<char> = s1.chars().collect();
            chars.sort();
            let sorted_string_1: String = chars.into_iter().collect();
            let mut chars_02: Vec<char> = s2.chars().collect();
            chars_02.sort();
            let sorted_string: String = chars_02.into_iter().collect();
            println!("{} and two {}",sorted_string,sorted_string_1);
            if i!=j{
                if sorted_string_1==sorted_string
                {
                    tmp_vec.push(sorted_string);
                    tmp_vec.push(sorted_string_1);
                    // result_vec.push(tmp_vec);
                }
            }
            // println!("string one {:?} and string two {:?}",chars,chars_02);
            // println!(" tmp vec res {:?}",tmp_vec);
            let  tmp_copy=tmp_vec.clone();
            if tmp_copy.len()>0
            {
                if result_vec.contains(&tmp_copy){
                    println!("dup");
                }
                else {result_vec.push(tmp_copy)};
            }
                // if result_vec.len()>0
            // {
            //
            // }
        }
        // println!("{}",i);
    }
    println!("{:?}",result_vec);
 array
}
fn generate_parentheses(n: i32) -> Vec<String> {
    fn backtrack(result: &mut Vec<String>, current: String, open_count: i32, close_count: i32, max: i32) {
        if current.len() as i32 == max * 2 {
            result.push(current);
            return;
        }

        if open_count < max {
            backtrack(result, current.clone() + "(", open_count + 1, close_count, max);
        }

        if close_count < open_count {
            backtrack(result, current.clone() + ")", open_count, close_count + 1, max);
        }
    }

    let mut result = Vec::new();
    backtrack(&mut result, String::new(), 0, 0, n);
    result
}
fn generate_parentheses(n: i32) -> Vec<String> {
    fn backtrack(result: &mut Vec<String>, current: String, open_count: i32, close_count: i32, max: i32) {
        if current.len() as i32 == max * 2 {
            result.push(current);
            return;
        }

        if open_count < max {
            backtrack(result, current.clone() + "(", open_count + 1, close_count, max);
        }

        if close_count < open_count {
            backtrack(result, current.clone() + ")", open_count, close_count + 1, max);
        }
    }

    let mut result = Vec::new();
    backtrack(&mut result, String::new(), 0, 0, n);
    result
}
