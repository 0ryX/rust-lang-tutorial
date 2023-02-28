fn main() {     // unsigned intergers are u32 etc... and are of positive value non neg
    const SECONDS_IN_MINTUES: u32 = 60;     //integer types to asigned constant
    println!("{}", SECONDS_IN_MINTUES);     // i8, i16, i32, i64, i128
    let mut x = 4;  //variable types immutable and constant!!
    println!("x is: {}", x);                // u8 0 - 2^8 -1 (0-255) bytes in binary
    x =5;                                   // i8 -2^7 - 2^7 - 1 (-128-127) negative to positve bytes
    println!("x is: {}", x);
    {
        let x = 2;                          //floats f32 f64 i.e let floating_point: f32 = 10.9
        println!("x is: {}", x);            //bool = true or false or 1 and 0 for true and false
    }
    let x = x +1;
    println!("x is: {}", x);
    let letter: char = 'a';  // any character you want as long as single quote ';'

    // compond types
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.1);
    //let tup: (i8, bool, char) = (1, true, 's') will not work becuase different byte type and variable isn't mutable

    let arr: [u32; 6] = [1, 2, 3, 4, 5, 6]; // array
    arr[4] 
}
