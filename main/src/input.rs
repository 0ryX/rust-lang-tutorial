use std::io; //import crate library std input and ouput

fn main(){
    let mut inpput = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}