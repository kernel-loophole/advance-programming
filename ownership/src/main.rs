fn main()
{
    // let s = vec!["udon".to_string(), "ramen".to_string(),
    // "soba".to_string()];
    // println!("{}",s)
    drop_out_string();
    no_drop_out_ex();
    moves_and_control();
    another_example_of_moves();
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