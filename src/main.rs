

fn formattedPrint() {
    let x = 5 + /*90 */ 5;
    
    println!("IS `x` 10 or 100? x = {}", x);

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subj} {verb} {obj}", subj = "hi!", obj = "brown fox", verb = "jumps over");

    println!("Base 10 : {}", 69420);
    println!("Base 2 : {:b}", 69420);
    println!("Base 8 : {:o}", 69420);
    println!("Base 16 : {:x}", 69420);

    println!("{number:>5}", number = 1); // padding
    println!("{number:0>5}", number = 1); // padding
    println!("{number:0<5}", number = 1); // padding

    // println!("{number:>width}", number = 1, width = 5); // This is not worked. width <<-^ named argument never used
    println!("{number:>width$}", number = 1, width = 5); // padding

    // #[allow(dead_code)] // dead code warning 을 무시해준다.
    struct Structure(i32);

    let number : f64 = 1.0;
    let width : usize = 2;
    println!("{number:>width$}");
}

fn main() {
    // this is comment
    /*
     * This is block comment. same as c lang
     */

    

    formattedPrint();
}