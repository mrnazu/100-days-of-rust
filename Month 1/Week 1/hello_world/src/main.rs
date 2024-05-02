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

    //// day 3
    // Data Types

    // boolean
    let b1: bool = true;

    // unsigend integers,
    // they must be postive numbers
    // and they start with letter "u" folowed by the amount of bit
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    // sigend integers,
    // they must be postive or negetive numbers
    // plus they start with letter "i"
    let i6: i8 = 1;
    let i7: i16 = 1;
    let i8: i32 = 1;
    let i9: i64 = 1;
    let i10: i128 = 1;

    // floating point
    // they are decimal numbers
    // they start with letter "f"
    // and can either be 32 bit or 64 bit
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str, and String
    let c1: char = "c";
    let s1: &str = "Hey";
    let s2: String = String::from("Hey");

    // Arrays
    let a1: /*[i32; 5]*/ = [1,2,3,4,5];

    let i1 132 = a1[4];

    // tuples
    let t1: (i32, i32, i32, i32) = (1,2,3,4);
    println(t1);
    let t1: (i32, &str, String, char) = (1, "Hey", "Tuples", "R");
    println!("Tuples is {t1}");

    let s1: &str = t1.2;
    println!("Slicing is: {s1}");
    let (i1: i32, f1: f64, s1: &str) = t1;
    println!("Slicing is: {t1}");

    let uint: () = ();

    // Type aliasing
    type age = u8;

    let a1: age = 57;
}
