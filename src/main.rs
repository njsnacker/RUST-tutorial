// 기본적인 파일들을 prelude 라고 하는데. 여기서 찾아볼 수 있다.
// https://doc.rust-lang.org/std/prelude/index.html



// 없는거는 아래처럼 직접 library (crate) 를 load 하여 사용한다.
use std::time::Duration;
use std::io; // import input/output library. io library comes from std library



fn formattedPrintExample() {
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


fn serialExample() {
    let ports = serialport::available_ports().expect("No ports found!");

    for p  in ports {
        println!("{}", p.port_name);
    }


    let mut serial = serialport::new("COM15", 9600)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    let output = "This is output".as_bytes();
    serial.write(output).expect("Writed Failed");


    let mut serial_buf : Vec<u8> = vec ! [0;32];
    serial.read(serial_buf.as_mut_slice()).expect("Found no data!");
}

fn basicIoExample() {
    println!("Guess number");
    println!("Input your num");

    let mut guess = String::new(); // String::new() 에서 ::new() 는 associated function 호출을 의미한다.


    
    io::stdin() // Call stdin() function in io library. std::io::stdin() 과 같이 쓸수도 있지만 namespace 를 줄여준다.
        .read_line(&mut guess)
        .expect("Failed to read line");
q
    println!("You guessed {}", guess);
}

fn main() {

    basicIoExample();
}