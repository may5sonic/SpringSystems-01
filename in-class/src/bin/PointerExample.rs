fn main() {
    let var = 100;
    let box_default = Box::new(var); // heap allocation
    let ref_default = &var; // stack allocation
    println!("Value of var: {}", var);
    println!("Value of box_default: {}", box_default);
    println!("Value of ref_default: {}", ref_default);
}