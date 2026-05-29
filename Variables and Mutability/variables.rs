fn main()
{
    let var1 = 5;
    println!("The value of var1 is: {var1}");
    var1 = 6; // This will cause a compile-time error because var1 is immutable by default
    println!("The value of var1 is: {var1}");
}