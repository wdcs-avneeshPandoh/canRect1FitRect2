use std::io;

fn main() {
println!("first rectangle");
println!("enter length");
let mut a  = String::new();
io::stdin().read_line(&mut a).expect("failed");
let a:u32 = a.trim().parse().expect("please type a number");

println!("enter width ");
let mut b = String::new() ;
io::stdin().read_line(&mut b).expect("failed 2") ;
let b:u32 = b.trim().parse().expect("please enter a number");

let area1 = area(a,b);
println!("area of rect 1 ={:?}", area1);

 // draw rect 1
 for i in 0..a {
        for j in 0..b {
            print!("*");
        }
        println!(); 
    }

println!("second rectangle"); 
println!("enter length") ;
let mut c  = String::new();
io::stdin().read_line(&mut c).expect("failed 3");
let c:u32 = c.trim().parse().expect("please enter a number");

println!("enter width");
let mut d = String::new() ;
io::stdin().read_line(&mut d).expect("failed 4") ;
let d:u32 = d.trim().parse().expect("please enter a number");

let area2 = area(c,d);
println!("area of 2nd rectangle is {:?}", area2);
  // draw rect 2
for i in 0..c {
        for j in 0..d {
            print!("*");
        }
        println!(); 
    }

// can rect 1 fit rect 2 ?

if a > c && b > d && area1 > area2 {
    println!("rectangle 1 can easily fit rectangle 2 ");

}
else {
    println!("sorry it cannot fit rectangle 2");
}



}


fn area(a:u32, b:u32) -> u32{
    a*b
}
