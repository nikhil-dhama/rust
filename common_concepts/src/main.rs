fn main() {
    //let x=5;   x is immutable
    let mut x = 5; // x is mutable here
    println!("Value of x is {x}"); 
    // this will not work if x is immutable as in line 2
    x=6;
    println!("Value of X is now {x}");

    //Shadowing
    // redefine a var with same name, 
    // Adv, you do not need to make var mutuable,
    // You can change type of var while redefining
    let x = "Shadowed";
    println!("X after shadowing is {x}");

    // you can not mutate const
    // they must be type annotated
    // const can not be added to any run time calculated value
    // it should always be a constant
    // GOOD PARCTICE TO KEEP CONST NAMES IN ALL CAPS 
    const SUBS_COUNT: u32 = 123;   
    println!("const {SUBS_COUNT}");


    //Data type

    //Scaclar data-types:
    //  Integers, un/signed 8-128 bits, i8: 8-bit signed, u8: 8-bit unsigned
    //              or arch base, isize: signed, usize: unsigned
    let a = 98_222_1234; // Decimal  (=982221234)
    let b = 0xfff; //Hexadecimal
    let c = 0o777; //Octal
    let d = 0b101101; //Binary
    let e = b'A'; //Byte (u8 only)

    let f: u8 = 255;
    //let g: u8 = 266;   //overflow, it will be 0 in runtime env, and fail in dev
    println!("Integres: a={a}, b={b}, c={c}, d={d}, e={e}, f={f}");

    //Floating point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    // for any mathematical operation, both operand should be of same type
    // no self type casting
    // let sum = 10.2+12;  
    let sum = 10+12;
    let diff = 10.5-3.4;
    // let product = 12*12.2;  
    let product = 12.1*12.2; 
    let quotient = 435.2/213.9;
    let remainder = 33%9;

    println!("sum={sum}, diff={diff}, product={product}, quotient={quotient}, remainder={remainder}");


    // Boolean

    let t = true;
    let f:bool = false;
    println!("bool: t={t}, f={f}");


    // Character (any unicode character), should be in single quotes

    let z = 'Z';
    let h = 'Ȟ';
    let queen = '🨁';

    println!("Chars z={z}, h={h}, queen={queen}");


    /*******************************************************
    * Compound Types
    ****************************************************** */

    //Tuple
    let tup = ("Hello", 120210, 34);

    // destructing tuple
    let (hello, int1, int2) = tup;

    // or dot notation
    let test_int1 = tup.1;


    // Arrays
    // in rust arrays are of fixed length
    let codes = [200,404,134];

    let not_found = codes[1];

    let bytes=[0;8];

    let zero_bit = bytes[0];

    println!("{hello}, {int1}, {int2}, {test_int1}, {not_found}, {zero_bit}");
    //how to define functions
    let sum = new_function(10,12);
    println!("Sum of 10+12 is {sum}");

    //condtions statements
    control_flow_conditions();

    // loops
    control_flow_loops(20); 
    control_flow_loops(25);
}



// function def
fn new_function(x:i32, y:i32) -> i32{
    println!("Learn to right a function in Rust");
    let sum = x+y;
    println!("{x}+{y} = {sum}");
    // return sum;
    // sum     // this returns sum
    x+y        // this returns x+y
}

fn control_flow_conditions() {
    let number = 5;
    
    
    // conditions should be explicitly a boolean, any other type will fail
    // i.e. `if number` will not work
    if number < 10 {
        println!("first condtion was true as {number} < 10");
    } else if number < 22 {
        println!("Second condition was true as {number} < 22");
    } else {
        println!("All conditions was false");
    }

    let condition = true;
    let test = if condition {5} else {6};
    println!("test in control flow was {test}");
}


fn control_flow_loops(loop_break_point:i32) {
    // infinite loop, can also return a value from these find of loops
    let mut counter = 0;
    loop {
        println!("Infinite loop, next line is break, so Bye Bye");
        break;
    }

    let returned_value = loop {
                                counter += 1;
                                if counter == loop_break_point {
                                    break counter;   // will return the counter value
                                }
                            };

    println!("Loop should return {loop_break_point}, returned values is {returned_value}");



    //While loop
    let mut number = loop_break_point;
    while number >0 {
        print!("{number} ");
        number -= 1;
    }
    println!();
    println!("While loop was over after running {loop_break_point} times.");

    // For loop
    let a = [1,2,3,4,5,6,7,8,9,10];
    for ele in a.iter() {
        print!("{ele} ");
    }
    println!();

    // or in range `(start..end)` end is exclusive
    for number in (1..loop_break_point+1) { 
        print!("{number} ");
    }
    println!();
    println!("For loop ended for range 1 to {loop_break_point}");

    for number in 1..loop_break_point+1 { 
        print!("{number} ");
    }
    println!();
    println!("For loop ended without brackets for range 1 to {loop_break_point}");
}