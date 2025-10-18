fn main() {
    let x = 5; // inferred variable type i32
    let y = "Hello"; // inferred variable type &str

    let a: i32 = 10; // explicit variable type i32 via type annotation

    // In Rust variables are immutable by default.
    let b = "immutable variable";
    println!("b = {}", b);
    // To make a variable mutable you need to use the mut keyword
    let mut c = "mutable variable";
    println!("c = {}", c);
    {
        let i = "I only exists in this scope";
    }
    /*
    Shadowing In Rust:
    - Feature that allows you to re-declare a variable in the same scope.
     */
    let d = 10;
    println!("d = {}", d);
    let d = d + 1;
    println!("d = {}", d);
    /*
    Constants in Rust:
    - const keyword
     */
    type U32 = u32;
    const MAX_POINTS: U32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
    // static variables are similar to constants but constants in contrast are more performant because they are inlined.
    static COUNTER: i32 = 30_000;
    // unused variables
    let (_, x) = (1, 2); // _ is unused
    println!("x = {}", x);
    // array variables
    let arr_test = [1, 2, 3];
    println!("arr_test = {:?}", arr_test);
    // vector variables
    let vec_test = vec![1, 2, 3];
    println!("vec_test = {:?}", vec_test);
}
