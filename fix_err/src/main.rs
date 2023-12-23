fn main() {
    let n = 33;
    let k = 22;


    let res = if n >= k {
        n % k
    } else {
        k - (n % k)
    };
    

    println!("{}", res);
}
