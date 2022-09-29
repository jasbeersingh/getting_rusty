fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mean: f64;
    let mut sum = 0.0;

    min = 0;
    max = 0;
    for x in numbers{
        if x as i32 > max{
            max = x;
        }
        if (x as i32) < min{
            min = x;
        }
        sum += x as f64;
    }

    mean = sum/numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("mean calculated correctly!!");
    
}