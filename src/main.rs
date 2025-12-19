fn main(){

 fibonacci(10);
}

fn fibonacci(input: i32)-> i32{

    let mut a = 0;
    let mut b: i32 = 1;
    for _ in 0..=input {
        let sum = a + b;
        a = b;
        b = sum;
        println!("{}",b)
    }
    b
    
}