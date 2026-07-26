/*fn main()
{
    let var1 = 5;
    println!("The value of var1 is: {var1}");
    var1 = 6; // This will cause a compile-time error because var1 is immutable by default
    println!("The value of var1 is: {var1}");
}*/

/*fn main()
{
    let mut var1 = 5;
    println!("The value of var1 is: {var1}");
    var1 = 6; // This will cause a compile-time error because var1 is immutable by default
    println!("The value of var1 is: {var1}");
}*/

/*fn main()
{
    let var1 = 5;
    let var1 = var1 + 1; // This is called shadowing, it allows us to reuse the same variable name
    println!("The value of var1 is: {var1}"); //now, var1 should be 6

    //the brackets represent a new scope, the variable var1 inside the brackets is a different variable than the one outside the brackets
    {
        // This is also shadowing, it creates a new variable var1 that shadows the previous one
        //This is shadowing inside of a new scope
        let var1 = var1 * 2;
        println!("The value of var1 in the inner scope is: {var1}");
        //shows that the variable can be a different type
        let var1 = var1.to_string(); // This is also shadowing, it creates a new variable var1 that shadows the previous one and converts it to a string
        println!("The value of var1 in the inner scope is: {var1}"); //now, var1 should be 12
    }

    //now, we are back to the outer scope, so var1 should be 6 again
    println!("The value of var1 in the outer scope is: {var1}");
}*/

/*fn main() {
    let x: (i32, f64, bool) = (-1, 6.4, true);
    let first_index = x.0;
    let first_index = first_index + 5;
    println!("First index after change is: {}", first_index);
    let float_val = x.1;
    println!("Floating point number is: {}", float_val);
    let boolean_val = x.2;
    if boolean_val
    {
            println!("The boolean value is true");
    }
    else
    {
            println!("The boolean value is false");
    }
}*/

//enter in range
//ask user input
/*print!("Enter first number in range: ");
let mut input1 = String::new();
//flush
io::stdout().flush().unwrap();
io::stdin().read_line(&mut input1).expect("Failed to read line for first number");

print!("Enter second number in range: ");
let mut input2 = String::new();
//flush
io::stdout().flush().unwrap();
io::stdin().read_line(&mut input2).expect("Failed to read line for second number");

let first_num = input1.trim().parse::<i32>().expect("Not a number");
let second_num = input2.trim().parse::<i32>().expect("Not a number");

let range = random_range(first_num..=second_num);
println!("{:.1}", range);*/














//gen a secret number with user guessing
//need a do-while loop where loop until condition is found. Loop until secret number is guessed by the user
/*
inside loop, prompt user for a number
if the number is the same as secret, then exit loop and tell user they got the number
else, tell user they are incorrect and go through the loop again

Steps:
1. create our random secret number in range between 0-10 inclusive
2. loop -- infinite loop
    Prompts the user for their guess
    error check to ensure the number entered is within the range specified, tell user and prompt again for a new number
    validate for integer numbers
    if guess == secret numer, give user output and exit loop
    else, tell user to try again

    press q to quit

use results and error handling. next chapter in rust book
use functions and write test cases for them

*/
mod parse_config;

use rand::random_range;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

fn secret_random_num() -> usize {
    //generate a number between 0 and 10 inclusive
    random_range(0..=10)
}

//function for handling both exceptions
fn handle_exceptions(result: Result<i32, ParseIntError>, number_guessed: &mut i32) -> bool {

    match result {
        Ok(num) => {
            //now that it is an integer, check the range as well
            if num < 0 || num > 10 {
                println!("Number is out of range! The number must be between 0 and 10 inclusive.");
                return false;
            }
            *number_guessed = num;
            true
        },
        Err(error) => {
            println!("The number entered was not an integer! Try again! \nError is: {}\n", error);
            false
        },
    }

}

/*
use llm to review code and optimize it for performance and stability and best practices
at least 2 models
write a summary of recommendation and agree or not
use cargo doc to generate documentation and review
*/

fn main() {

    //store the secret number in a variable
    let secret_num: usize = secret_random_num();

    let mut num: i32 = 0;

    loop {
        //prompt user for guess
        print!("Enter a number between 0 and 10 (press q to quit): ");
        let mut input1 = String::new();

        //flush
        io::stdout().flush().unwrap();

        //error output if read failed
        io::stdin().read_line(&mut input1).expect("Failed to read line for the user number guess");

        //check if the input is q, if so, quit
        if input1.trim() == "q" {
            break;
        }

        //store the Result in a variable to pass into the handle function
        let parsed_num = input1.trim().parse::<i32>();

        //if the number is not an int or is not in the correct range, error out and loop again
        if handle_exceptions(parsed_num, &mut num) {
            //now check to see if the number is the same as the secret
            if num == secret_num as i32 {
                println!("Congratulations! You guessed right!");
                break;
            }
            else
            {
                println!("Not correct :( Try again!")
            }
        }

    }
}
//test functions!
#[cfg(test)]
mod tests {
    use crate::{handle_exceptions, secret_random_num};

    #[test]
    fn test_correct_secret_random_num() {
        let value = secret_random_num();

        assert!((0..=10).contains(&value));
    }

    #[test]
    fn test_true_both_handle_exceptions() {
        //create a good string that is an integer between 0 an 10 inclusive
        let int_num_string = String::from("10");

        let parsed_num = int_num_string.trim().parse::<i32>();

        //call function
        let result = handle_exceptions(parsed_num, &mut 0);

        assert_eq!(result, true);
    }

    #[test]
    fn test_false_int_handle_exceptions() {
        //create a good string that is an integer between 0 an 10 inclusive
        let int_num_string = String::from("Hello!");

        let parsed_num = int_num_string.trim().parse::<i32>();

        //call function
        let result = handle_exceptions(parsed_num, &mut 0);

        assert_eq!(result, false);
    }

    #[test]
    fn test_false_range_handle_exceptions() {
        //create a good string that is an integer between 0 an 10 inclusive
        let int_num_string = String::from("100");

        let parsed_num = int_num_string.trim().parse::<i32>();

        //call function
        let result = handle_exceptions(parsed_num, &mut 0);

        assert_eq!(result, false);
    }
}

