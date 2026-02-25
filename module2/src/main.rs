//fn fn2(2: i32) {
    //println("")
//}

    //fn concat_strings(s1: &String, s2:&String, s3:&String) -> String {
    //    return s1.clone() + s2 + s3;
   // }

//fn clone_and_modify(s: &String) -> String {
    // Your code here
//}

//#[allow(unused_variables, unused_mut)]
//fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
//    for i in low..=high {
//        *total += i;
//    }
//}

fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    //unimplemented();

    let mut i = low;

    while i <= high {
        *total += i;
        i += step;
    }
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}


//fn main() {

    //let name = String::from("Alice");
    
   // match name {
    //    ref n => println!("Name: {}", n),
    //}
    
   // println!("Name is still: {}", name);  // This works because `name` wasn't moved

    //let s = String::from("Hello, ");
    //let modified = clone_and_modify(&s);
    //println!("Original: {}", s); // Should print: "Original: Hello, "
    //println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
    //let s1 = String::from("Hello, ");
    //let s2 = String::from("World!");
    //let s3 = String::from("World3!");
    //let result = concat_strings(s1, s2, s3);
    //println("{}", result);
    //let mut word = "UT".to_string(); 
    //fn update(word: &mut String) {
    //    word.push_str("RGV");
    // }
    //update(&mut word);
    //println!("{word}")

    //let s1 = String::from("Hello");
    //let s2 = s1.clone();

    //println!("s1 is: {}, s2 is: {}", s1, s2);
    //let x = 5;
    //fn2(x);
    //let y = &x;
    //println!("{}", y);
    //println!("{:p} == {:p}",y,&x);
    //println!("Hello, world!");
    /*
    let x = 5;
    let y = x;
    println!("x is : {}, y is : {}", x, y);

    //let s1 = String::from("");

    //{
    let s = String::from("Hello"); // Allocates memory on the heap
    println!("message from heap: {}", s);
    //}

    let mut s = 1234.to_string(); // Note: rules regarding mutability still apply
    println!("message from heap: {}", s);

    // Strings are mutable
    s.push_str("4567");
    println!("My string number: {}", s);
    */

    
//}
