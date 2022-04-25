fn main() {
    //define a variable  
    let learn_language = "Rust";
    // if construct 
    if learn_language == "Rust" { 
       println!("You are learning Rust language!");
    }


    //define a variable if...else
    let learn_language = "Rust";
    // if else construct 
    if learn_language == "Russian" { 
        println!("You are learning Rust language!");
    }
    else {
      println!("You are learning some other language!");
    } 


      //define a variable 
      let learn_language="Rust";
      // if..elseif..else construct 
      if learn_language == "Rust" { 
         println!("You are learning Rust language!");
      }
      else if learn_language == "Java" { 
         println!("You are learning Java language!");
      }
      else {
         println!("You are learning some other language!");
      } 

      let learn_language1 = "Rust";
      let learn_language2 = "Java";
      // outer if statement
      if learn_language1 == "Rust" {  // inner if statement
          if learn_language2 == "Java"{
                println!("You are learning Rust and Java language!");
          }
      }
      else {
        println!("You are learning some other language!");
      } 


       //define a variable  
    let learn_language = "Rust";
    // short hand construct
    let res= if learn_language == "Rust" {"You are learning Rust language!"} else {"You are learning some other language!"};
    println!("{}", res);

    let x = "Rust";

    let y: bool = if x == "Rust" { true } else { false };

    // let z: bool = if x == "Rust" { true; } else { false; };

    println!("x:{}", x);
    println!("y:{}", y);

    // age challenge
        let age=23; 
        if age >=21{ 
           println!("Age is greater than 21");
        }
         else if age <21{
            println!("Age is less than 21");
         }
         println!("Value Printed");

         // tennis & baseball
        
         let age = 23; 
  let play = true; 
  let activity="Baseball" ;
  if age >= 21 && play==true || activity == "Tennis" { 
    println!("Age is greater than 21");
    println!("You are allowed to play");
    println!("The sport is {}",activity);
  }
  else if  age >= 21 && play == true && activity == "Tennis"{ 
    println!("Age is greater than 21");
    println!("You are allowed to play");
    println!("The sport is {}",activity);
  }
  else if age <21 && play == false && activity == "Tennis"{
    println!("Age is less than 21");
    println!("You are allowed to play");
    println!("The sport is {}",activity);
  }
  else{
    println!("Value Printed");
  }

  let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner","course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    // define a scrutinee expression    
    let course = ("Rust", "beginner","data");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner", c) = course {
        println!("Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    } 
    else {
        // do not execute this block
        println!("Value unmatched");
    }

 // define a scrutinee expression     
 let course = ("Rust", "beginner");
 // pattern does not match with the scrutinee expression
 if let ("Java", c) = course {
     println!("Course is {}", c);
 } else {
     // execute this block
     println!("Value unmatched");
 }


 // define a variable
 let course = "Rust";
 // return value of match expression in a variable
 let found_course = match course {
    "Rust" => "Rust",
    "Java" => "Java",
    "C++" => "C Plus Plus",
    "C#" => "C Sharp",
    _ => "Unknown Language"
 };
 println!("Course name : {}",found_course);


//challenge

/* fn test(_a:i32){
    if _a % 2 == 0{
      print!("Number {} is even", _a);
    }
    else{
      print!("Number {} is odd", _a);
    }
*/

let num: i32 = 23;
    
let result = if let 0=num%2{
    "Even"
}
else{
    "Odd"
};

println!("Number is: {}",result);


}


