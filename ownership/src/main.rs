fn main()
{
    // let s = vec!["udon".to_string(), "ramen".to_string(),
    // "soba".to_string()];
    // println!("{}",s)
    // drop_out_string();
    // no_drop_out_ex();
    // moves_and_control();
    // another_example_of_moves();
    // moving_example_of_int();
    // shadowing();
    // mut_and_imut();
    freezing_val();
    }

fn drop_out_string()
{
    let s="joe".to_string();
    //value of s drop here 
    let s="john".to_string();
    println!("{}",s);
}
//drop value function
fn no_drop_out_ex()
{
    let mut s="s string".to_string();
    let t=s;//No drop
    // t take the ownership of s
    let s="after s change".to_string();
    println!("value of s ===>{}",s);
    println!("value of t===>{}",t);
    

}
fn moves_and_control()
{
    // Build a vector of the strings "101", "102", ... "105"
            // let mut v = Vec::new();
            // for i in 101 .. 106 {
            // v.push(i.to_string());
            // }
    /*  Pull out random elements from the vector.
        let third = v[2];  error: Cannot move out of index of Vec
        let fifth = v[4];  here too
        */
    let mut v=Vec::new();
    for i in 101 .. 105{
        v.push(i.to_string());
    }
    for i in v.iter()
    {
        println!("string { }",i);
    }


}
fn another_example_of_moves()
{
    let v=vec!["hello".to_string(),"string".to_string(),"world".to_string()];
    println!("before the ownership");
    // for i in v.iter()
    // {
        
    //     println!("{}",i);
    // }
    // //s will take ownership of v
    for mut s in v
    {
        s.push('1');
        println!("{}",s)
    }
}
//integers in Rust 
//
fn moving_example_of_int()
{
    let num1:i32=32;
    let num2=num1;
    println!(" value of num1 {}\n",num1);
    println!("value of num2 {}\n",num2);
}
//shadowing the value 
fn shadowing()
{
    let x=12;
    let x=12.6;
    println!("the value of x ===>{}",x);
    // mut the value 
    let mut y=20;
    let y=25;
   
    // defining the scope of var
    //the lifetime of y is inside the bracket only 
    {
        let y=30;
        println!("the value of y inside the block is {}",y);
    }
    println!("the value of y is {}",y);


}
fn mut_and_imut()
{
    let some_value=13;
    
    // this can not be done as varibles are imutable by default
            //   some_value+=1;
    println!("the value of vari{}",some_value);
    //update the value of some_value varibles 
    let some_value=20;
    println!("after assing new value{}",some_value);

}
fn freezing_val()
{
    let mut _mut_val=20;
    {
        let _mut_val=40;
        //cannnot assign value to imutable variable
        //_mut_val=10;
        println!("{}",_mut_val);
    }

}