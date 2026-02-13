// rust doesnt take ownership and thus solves the c++ problem where u might 
// forget that c uses ownership and u might forget where u have owned or what 
// rust solves this by allowing ownership by passing references 

#[derive(Debug)]
struct User {
    name: String, 
    age: i32,
    height: i32,
}

impl User {
    // No &self here! We are creating the object.
    fn new(name: &str, age: i32, height: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
            height,
        }
    }

    fn change_name(&mut self, new_name : &str) -> Result<(),&str>{
        if new_name.is_empty() {
            Err("Name cannot be empty!") // Return error
        } else {
            self.name = new_name.to_string(); // Update value
            Ok(()) // Return success
        }
    }
}

fn main() {
    let s1 = String::from("hello world!");
    let result = calculate(&s1); // passing reference here 
    println!("the lenght of string {} is {}", s1, result);

    let ar = [123,32,3123,1321,213,3];

    let res_ar = run(&ar);

    println!("Total sum : {}", res_ar);


    let mut user1 = User::new("Alice", 30, 170);
    match user1.change_name("Bob") {
        Ok(_) => println!("Name updated!"),
        Err(e) => println!("Error: {}", e),
        }

    println!("{:?}",user1)
}

fn calculate(s : &String) -> usize{
    // &String is asking for reference 
    s.len()
}

fn run(ar : &[i32]) -> i32 {
    let mut sum=0;
    // run for loop for summing all elements 
    for num in ar{
        println!("summing {}",num);
        sum+=num;
    } 
    sum
}


#[test]
fn test1(){
    println!("test");
}
