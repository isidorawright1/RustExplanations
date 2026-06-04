#Variables and Mutability Explanation

##Variables
A variable is a named label that can hold different types of data and its value

By default, in Rust, variables are **immutable**, which means the value of a vaible cannot change throughout the code.

Here is an example of what will happen if you try to change the variable when it is immutable:
![Immutable Variable Change Error]

This error was received because the variable "var1", which is immutable, was assigned a number after it had already been assigned.

The reason Rust makes variable immutable by default is for safety. Code is prone to human error and attacks. By making a vairbale unchangeable by default, it ensures that a it can't be changed on accident by fat fingering a number key and an attacker can't change the value of a variable easily.

Even though safety of variables is important, mutable (changeable) variables are still useful and can make code easier to write.

This can be done by adding the key word "mut" in front of the variable name.

See the example below. Using the same code as before, adding the keyword *mut* in front of the variable name allows it to be changed from 5 to 6.

##Declaring Constants
Constants are variables that are always immutable and can never become mutable. As the name suggest, the value to that variable stays constant throughout the code. You assign it once in the beginning and that's it.

They are declared using the *const* key word with the variable name must be in all capital letters. They type of the variable must be explicitly written, otherwise, the Rust complier does not know how to define it.

Constants can be defined in any scope needed. Constants are mainly used for a value that mutliple parts of the code may need to know.

EX: const NAME_OF_VAR: i32 = 10;

##Shadowing
In Rust, you can declare a new variable with the same name as a previously declared one. This causes the first variable to become invalid or ***shadowed*** by the second. This means that Rust does not consider the first variable

This can be done by using the same vairbale name with the keyword ***let*** again. Since the keyword ***let*** is being used, it is like you are defining a new variable but with the same name. This means that the new variable can have a different type than the previous one if desired.

Here is an example that shows shadowing in both the same and different scopes as well as showing that the same variable name can have different types as it is shadowed.

Shadowing allows you to have different immutable values assigned to the same variable without them being mutable. This ensures that when you are done making changes, the variable is still immutable.

Here is an example of what error will occur if you try to shadow a mutable variable:
![image_here]
