fn main() {
    //// day1
    println!("Hello, world!");

    //// day 2
    // creation
    let a: i32 = 5; // we use "let" keyword to create a Variables

    // Mutability
    let mut b: i32 = 5;
    b = 10; // this time we get an error that we can't assign twice to Immutable Variable, to do this.. we have to add the "mut" keyword.
 /* A key difference in Rust is that variables are immutable by default. 
 Once assigned a value, you can't directly change it. */

    // Shadowing
    let c: i32 = 10;
    let c: i32 = 20; // because these vars have the some name and some scope the second var will shadow the first var
    println!("C is: {c}");
 /*
 Imagine you have a box of balls. You label one box "red" and put a red ball inside. 
 That's like declaring a variable named c and giving it the value 10 (like the red ball).

Now, you open a new box and also label it "red" (shadowing!). 
This time, you put a blue ball inside (value 20). 
The first red box with the red ball is still there somewhere, but you can't see it right now because the new red box is blocking your view. 

In your code, you have two "boxes" named c. 
The first one has the value 10, but it's hidden because there's another "box" named c closer to you that has the value 20. 
This "closer" box is the one you use when you print C is: {c}.
*/

    // Scope
    let d: i32 = 30;
    {
        let d: i32 = 40; // this will shadow the outer scope one.
        println!("Inner D is: {d}"); // we can print inside the inner scope not outside.
    }
    println!("D is: {d}");
}
