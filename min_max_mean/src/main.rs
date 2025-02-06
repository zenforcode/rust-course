fn main() {
    let values = [2,80,5,6,7,8,10, 2];
    let mut sum = 0;
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    for v in values {
        sum+=v;
        if v < min {
            min = v;
        }
        if v > max  {
            max = v;
        }
    }
    println!("Minumum {}, Max {}, Average {}", min, max, sum / (values.len() as i64) );
}
