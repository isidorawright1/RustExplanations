fn main()
{
    let var1 = 5;
    println!("The value of var1 is: {var1}");
    var1 = 6; // This will cause a compile-time error because var1 is immutable by default
    println!("The value of var1 is: {var1}");
}

/*
fn main()
{
    let mut var1 = 5;
    println!("The value of var1 is: {var1}");
    var1 = 6; // This will cause a compile-time error because var1 is immutable by default
    println!("The value of var1 is: {var1}");
}


fn main()
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
}
*/