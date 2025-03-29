use std::io;
fn main() {
   let mut num_2 = String::new();
   let mut num_1 = String::new();

   println!("input first number");
   io::stdin()
       .read_line(&mut num_1)
       .expect("error in what you provided");
   let num_1:u32 = num_1.trim().parse().expect("error parsing"); 
   println!("input second number");
   io::stdin()
       .read_line(&mut num_2)
       .expect("error in what you provided");

   let num_2:u32= num_2.trim().parse().expect("error parsing");

   let sum_value = sum(num_1,num_2);
   println!("{sum_value}");
}
fn sum(a:u32,b:u32)->u32{
   a+b //no semicolon in expressions
}






