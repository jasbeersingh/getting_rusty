fn main(){
    
    str_slice_eg();
    arr_slice_eg();

}

fn str_slice_eg(){
    // string slicing example
    let message = String::from("let there be peace");
    println!("message is: {message}");

    let last_word = &message[13..];
    println!("{last_word}");
}

fn arr_slice_eg(){
    let planets = [1,2,3,4,5,6,7,8];
    // let in_pla: &[i32] = &planets[..4];
    let in_pla= &planets[..4];
    println!("{in_pla:?}");
}