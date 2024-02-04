fn main() {
    // *****************OWNERSHIP rules*********************
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the woner goes out of scope, the value will be droped.

    { // s is not valid here, it is not yet declared
        // allocate on stack
        let s = "hello";   // s is valid from this point forward
        
        // allocate from heap, allocation and deallocation is managed by Rust.
        let t = String::from("hello");
        // do computer with s here
    } // scope of s,t is over now, and s,t are no longer valid

    // simple types on stack are copies, rest are moved

    let x =5;
    let y = x; // copy x to y

    let s1 = String::from("hello");
    let s2 = s1;     // Move s1 to s2 (not shallow copy)

    // println!("s1={s1}"); // will give compile error, as s1 is moved in previous line

    // if we can to copy a value we should clone it..
    let s3 = s2.clone();
    println!("s2={s2}, s3={s3}");    // will run as s2 was cloned


    let s = String::from("hello");

    // passing s to fn will move it to the parameter and will not copy
    takes_ownership(s);

    // println!("{s}"); // will fail as s is moved


    let x = 45;
    let t = true;
    let f = 2.5;
    let c = 'a';
    makes_copy(x,t,f,c);
    println!("x={x}, t={t}, f={f}, c={c}"); // this will work as integers are copied // primitive types.

    let other_str = gives_ownership();
    println!("{other_str}");

    let send = String::from("send");
    let recieve = takes_and_gives_ownership(send);

    // println!("send = {send}") // will fail as ownership is given to fn
    println!("Got ownership back from method\n\trecieve = {recieve}");


    let str = String::from("find my lentgh");
    let length = calc_length(&str);
    println!("lenght is {length}, and I have ownership (not given to function) of str as it is \"{str}\"");

    let mut str_append = String::from("can you modify me?");
    println!("before changing: \n\t{str_append}");
    // passing a mutable reference
    // we can only have 1 mutable refrence to a particaular piece of data in a particular scope
    // i.e. we can only provie one mut references for str_apppend
    // we can not have a mutable refernece if we have a mutable references
    // WE can have multiple immutable reference
    mutable_reference_change_str(&mut str_append);
    println!("after passing mutable reference:\n\t{str_append}");


    let mut s = String::from("hello");

    // multiple immutable refs
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {r1}, r2 = {r2}");

    // Scope of a ref starts when it was first introduced
    // and ends when it was last used
    // we can create a mutable ref to s, as last ref was in print statement above
    let mut r3 = &mut s;
    r3.push_str("\n\tIt was a mutable ref so I did this!!!");
    println!("`s` is now\n\t{s}");




    /*************************
     * String slicing
     ************************/

     let s = String::from("Hello World");

     let hello = &s[0..5];  // omit start and end, if it is start and end of the string
     let world = &s[6..11];

     println!("Slices of {s} are: \n\t1. {hello}\n\t2. {world}");
     // slices are bineded to the original string,
     // if we change original string 
     //     then slice will also change

     let word = first_word(&s);
     println!("first word is : {word}");
     
     // string literals are string slices, stroed in binary form
     let s2 = "Slices of bread";
     let word2 = first_word(s2);
     println!("word2 = {word2}");


     // we can crate slices of arraya as follows
     let a = [1,2,3,4,5];
     let slice = &a[1..3];

} 


fn takes_ownership(some_string: String) {
    println!("takes_ownership: \n\t{some_string}");
}

fn makes_copy(some_int: i32, some_bool:bool, some_float: f64, some_char: char) {
    println!("makes_copy: \n\tint:{some_int},\n\tbool: {some_bool},\n\tfloat: {some_float},\n\tchar: {some_char}")
}


// ownership of returned var will be moved to caller
fn gives_ownership() -> String {
    let some_str =  String::from("get ownership from fn");

    some_str
}

fn takes_and_gives_ownership(temp_string: String) -> String {
    temp_string
}

/********************************************************************
 * Rules of References
 * 1. At any given time, you can have either one mutable ref
 *  or any number of immutable refs
 * 
 * 2. Refs must always be valid, (can not be dangling)
 *******************************************************************/


// method taking a reference, which does not take ownership
// called borrowwing
// references are immutable by default
fn calc_length(s: &String) -> usize {
    s.len()
}

fn mutable_reference_change_str(s: &mut String) {
    // inplace append to string
    s.push_str("\n\t\tYes, it was mutable, so I appended this text");
}

//Rust does not allow this as `s` will go out of scope after fn finishes
// and &s will be a dangling reference, it will throw error
// fn dangle() -> &String {
//     let s = String::from("dangles this str");

//     &s   
// }


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
 &s[..]
}