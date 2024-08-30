use std::io;

struct User {
  
  name:String,
  age:u32,
  
}

fn main() {
    println!("Hello, world!");

    println!("sum the sum is {}",add(55,66));

    variables();
    user_input();
    arrays();
    
}
fn variables(){


	let int:u8=45;

	println!("integer:{int}");

	let stock_price:f32=45.66;

	println!("stock price:{stock_price}");

   //integer literals
   let hex=0xfff;
   let octal=0o123;
   let binary=0b1010;

   println!("hex value:{hex}");
   println!("octal value:{octal}");
   println!("binary value:{binary}");
}

fn user_input(){

	let mut user_input=String::new();

	io::stdin()
	  .read_line(&mut user_input)
	  .expect("failed to read line");

	 println!("user input:{user_input}");
}
///the function to add two numbers
///it returns unsigned integer value
fn  add(x:u32,y:u32)->u32 {


	x+y
}

fn arrays(){

	let numbers=[10,20,30,40,50,60,70,80,90];

	for index in numbers {

		println!("value:{index}");
	}
}
