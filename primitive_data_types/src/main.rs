fn main() {
    let x = 10; // this var is immutable by default
    let mut y = 20; // this one can be modified
    println!("x is {}", x);
    println!("y is {}", y);
    y = 30;
    println!("now y is {}", y);
    println!("---------decimal and float example-------------");
    let z = 10.123456789123456789;
    let a:f32 =  10.123456789123456789;
    println!("z is {}", z);
    println!("A is {}", a);
    // print macro formatting
    println!("z is {:3}", z);
    println!("z is {:8.3}", z);
    println!("z is {:08.3}", z);
    println!("z is {:3} and a is {}", z, a);
    println!("a is {1:3} and z is {0}", z, a);
    println!("z is {z:3} and a is {a}");
    // rust character type. these are unicode and bytes. 
    let uni_symbol = '\u{261D}';
    println!("{uni_symbol}");
}