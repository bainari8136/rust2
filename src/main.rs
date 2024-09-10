extern crate rust_lib;

use std::io;
use std::collections::HashMap;
use rust_lib::submod;

struct Rectangle {
  width:u32,
  length:u32,
}

mod demo {

 pub	fn print_another_message(){

		println!("message from inline module");
	}
}

impl Rectangle {

 fn area(&self)->u32{

  self.width*self.length
 	
 }
}
fn main() {
    println!("Hello, world!");

    println!("sum the sum is {}",add(55,66));

    variables();
    user_input();
    arrays();
    references();
    using_structs();
    submod::print_message();
    demo::print_another_message();
    using_vectors();
    using_hashmaps();
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

fn references(){

let s1=String::from("string reference");

let len=calculate_length(&s1);

println!("string length is {len}");
	
}
fn calculate_length(str:&String)->usize {

   str.len()
	
}

fn using_structs(){

let rect1=Rectangle{
   width:12,
   length:24
};

 let area:u32=rect1.area();

 println!("the area is {area}");
}

fn using_vectors(){


   let mut v:Vec<i32> =Vec::new();

   v.push(45);
   v.push(12);
   v.push(67);

   for i in &v {

   	println!("{i}");
   }
	
}

fn using_hashmaps(){

	let mut scores=HashMap::new();

	scores.insert(String::from("red"),10);
	scores.insert(String::from("blue"),20);
	scores.insert(String::from("Green"),30);


	for(key,value) in &scores {

		println!("{key}:{value}");
	}
}
