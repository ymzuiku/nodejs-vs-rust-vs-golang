use chrono;

fn fibonacci32(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibonacci32(n - 1) + fibonacci32(n - 2);
}

fn fibonacci64(n: f64) -> f64 {
    if n == 0f64 || n == 1f64 {
        return n;
    }
    return fibonacci64(n - 1f64) + fibonacci64(n - 2f64);
}

fn run32(){
    let t1 = chrono::offset::Local::now();
    fibonacci32(42);
    let t2 = chrono::offset::Local::now();
    let end = t2 - t1;
    println!("{}", end.num_milliseconds())
}

fn run64(){
    let t1 = chrono::offset::Local::now();
    fibonacci64(42f64);
    let t2 = chrono::offset::Local::now();
    let end = t2 - t1;
    println!("{}", end.num_milliseconds())
}



fn main() {
    run32();
    run64();
}
