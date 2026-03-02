
/*
fn double(x:i32) -> i32 {
    //return x*2;
   // x*2
   //return x*2
   {
    let y = 10;
    x*2*y
   }
}
   */

// fn say_hi(x:i32){
//     println!("Hi John! My fav num {}", x);
// }
/*
fn main() {
    // More Functions
    println!("Double {} equals to {}",5,double(5));

    //Functions
    // say_hi(5);

    //Variables and Mutability
    // Part 2
    // let mut result : f32 = 0.0; // int
    // let y:i32 = 5; //float
    // result = result + y as f32; // no implicit conversion
    // println!("{}",result);

    // Part 1
    // let mut x = 10; // -> int8,int16,int32,int64,int128
    // x += 10;
    // println!("5*2 + 10 = {}",x);

    // //Shadowing
    // let x:i32 = 5;
    // //x = 1.012; // you can't
    // let x:f32 = x as f32 + 1.012;
    // println!("{}",x);

    // Shadowing
    // let x = 5;
    // let x = x + 1;  // Creates a new variable
    
    // // Mutation
    // let mut y = 5;
    // y = y + 1;  // Modifies the existing variable
    
    // println!("x: {}, y: {}", x, y);

    // let x = 5;

    // {
    //     let x = x + 10;
    //     println!("x: {}", x);
    // } // free will be called for you

    // println!("x: {}", x);


}
*/
/*
fn main() {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        //Vec<Vec<i32>>
        // 5
        // Vec<i32>
        // [[1], //0
        // [1,1], // 1
        // [1,2,1], // 2
        // [1,3,3,1], // 3 index
        // [1,4,6,4,1]]
        let n:usize = num_rows as usize;
        let mut res: Vec<Vec<i32>> =  vec![];
        for i in 0..n {
            // generate a row [i]
            let mut row: Vec<i32> = vec![1; i + 1];
            for j in 1..i { // my index i = 2
                // row[j]
                row[j] = res[i - 1][j] + res[i - 1][j - 1];
            }
            res.push(row);
        }
        // Filling res
        //res[0] ... ? arra [1]
        //res[3] .. ? array [1,3,3,1]
        res
    }
}
    */
    //fn main(){
    //    
    //}
#[derive(Debug)]
enum PaymentMethod {
    Cash,
    CreditCard,
    //DebitCard,
}

fn main() {
    let cash = PaymentMethod::Cash;
    let credit = PaymentMethod::CreditCard;

    println!("Payment method 1: {:?}", cash);
    println!("Payment method 2: {:?}", credit);
}