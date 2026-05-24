fn main(){
    #[allow(dead_code)]
    struct Structure(i32);
    let number: f64 = 2.0;
    let width: usize = 5;
    println!("{number:>width$}");
}