fn main() {
// array
    let array_1=[1,2,3];
    let two:[u8;3]=[1,2,3];
    let blank1=[0;3];
    let array_total=[array_1,two,blank1];
    for a in &array_total{
        for n in a.iter(){
            print!("\t{} + 10 = {}", n, n+10);
        }
    }
//vector defining
// 
println!("\n");
 let mut mutex:Vec<i32>=Vec::new();
 mutex.push(10);
 mutex.push(20);
 mutex.push(30);
 mutex.push(30);
 
    for i in mutex.iter()
        {
            println!("{}\n",i);
        }

}
