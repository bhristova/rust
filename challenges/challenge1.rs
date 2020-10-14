//fibonacci
fn main() {
    let n = 5;
    let x = fib(5);
    let y = fib_recurs(5);
    println!("no recursion: {:?}; recursion: {:?}", x, y);
}

pub fn fib_no_recurs(n: u32) -> u32 {
    let mut tup: (u32, u32, u32) = (0,1,1);
    let mut cnt = 0;
    
    while cnt <= n {
        let sum = tup.1 + tup.2;
        tup.0 = tup.1;
        tup.1 = tup.2;
        tup.2 = sum;
        cnt += 1;
    }

pub fn fib_recurs(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}