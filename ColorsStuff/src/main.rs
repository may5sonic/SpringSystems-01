// fn patter_match_simple () {
//     let num = 3;
//     let letter = match num {
//         1 => 'A',
//         2 => 'B',
//         3 => {
//             (64 + 1 + 2 as u8) as char
//         },
//         _ => '#', // rust will not guess
//     };
//     println!("{}", letter);
// }

fn get_rgb(c:char) -> (u8,u8,u8){

    match c {
        'R' => (255,0,0),
        'G' => (0,255,0),
        'B' => (0,0,255),
        _ => (0,0,0),
    }

    // if c == 'R' {
    //     return (255,0,0);
    // }
    // if c == 'G' {
    //     return (0,255,0);
    // }
    // if c == 'B' {
    //     return (0,0,255);
    // }
    // return(0,0,0);
}

fn main() {
// imagine you need to write a function covert to RGB
// you will provide letter R => (255,0,0),, 'G'=> (0,255,0), 'B' => (0,0,255)


// Array

let letters:[char;3] = ['R','G','B'];
 
 for idx in 0..letters.len(){
     let (r,g,b) = get_rgb(letters[idx]);
      println!("RED intensity {}, GREEN intensity {}, BLUE intensity {} ", r,g,b);
 }
    // Tuple is an immutable collection of elements
    //let tup:(u8,u8,u8) = (255,0,0);
//     let (r,g,b) = get_rgb('R');
//     println!("RED intensity {}, GREEN intensity {}, BLUE intensity {} ", r,g,b);
//     let (r,g,b) = get_rgb('G');
//     println!("RED intensity {}, GREEN intensity {}, BLUE intensity {} ", r,g,b);
//     let (r,g,b) = get_rgb('B');
//     println!("RED intensity {}, GREEN intensity {}, BLUE intensity {} ", r,g,b);
}
