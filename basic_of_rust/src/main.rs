fn main() {
    test();
    let x=10;
    let y=20;
    let c=add(x,y);
    println!("c is {} ",c);
    let list_of_elemnts=[12,4,3,2,6,7,8,8];
    for items in list_of_elemnts.iter()  {
        let item=*items;
        let result=match item{
            12 | 6 =>"hit",
            3 =>"hit 3",
            _=>"miss",
            
        };
        if result=="hit"{
            println!("hit {}",result);
        }
        else {
            println!("miss");
        }
        
    }
    let search_text="hello";
    let total_string="this 
    is the string.
    ihope you will find it helpful
    this is me hello";
    grep(total_string, search_text);
}
fn test()
{
    println!("hi there");
}
fn add(i:i32,j:i32)->i32 {
    //Types are required when defining functions Functions return the last expression’s result, meaning return is not required 
    i+j
}
//example of match keywords 
fn grep(text:&str,search_text:&str){
    for i in text.lines()
    {
        if i.contains(search_text) {
            println!("string {}",i)       }
        
    }
}