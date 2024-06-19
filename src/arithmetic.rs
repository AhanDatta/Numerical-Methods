//Basic arithmetic manipulation algorithms

//Rounding function to specified decimal with string manipulation
pub fn round_dp(num: f64, precision: usize) -> f64 {
    //Returns if precision is too high
    if num.to_string().len() < precision + 3 {
        return num;
    }

    //Plus two because we need to include the leading digit and the decimal point
    let pre_rounding: f64 = num
        .to_string()
        .chars()
        .take(precision + 2)
        .collect::<String>()
        .parse()
        .unwrap();
    let final_digit: u32 = num
        .to_string()
        .chars()
        .take(precision + 3)
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    //Checks for if rounding required
    if final_digit < 5 {
        return pre_rounding;
    } else {
        return pre_rounding + 1.0 * 10.0_f64.powi(-(precision as i32));
    }
}

//Returns a vector of all points in an interval up to a given resolution, inclusive to the start, exclusive to end
pub fn arrange (start_point: f64, end_point: f64, resolution: f64) -> Vec<f64> {
    let mut answer: Vec<f64> = vec![];
    let num_steps: usize = ((end_point - start_point).abs()/resolution).floor() as usize;
    answer.reserve(num_steps);

    let mut curr_point = start_point;
    while curr_point < end_point {
        answer.push(curr_point);
        curr_point += resolution; 
    }

    return answer;
}

//Assumes fixed inputs of a, b, a != 0 or b != 0
fn gcd(a: u64, b: u64) -> Result<u64, String> {
    //base case of a == 0 
    if a == 0 {
        if b == 0 {
            return Err("GCD of 0 with 0 is ill-defined.".to_string());
        }
        else{
            return Ok(b);
        }
    }

    //fixes direction of inequality
    if a > b {
        return gcd(b,a);
    }
    else {
        return gcd(b.rem_euclid(a), a); //recursive step of subtracting a from b
    }
}

mod arithmetic_test {
    use super::*;

    const test_num: f64 = 0.123456789101112;

    #[test]
    fn five_place_round() {
        assert_eq!(0.12346, round_dp(test_num, 5));
    }

    #[test]
    fn precision_greater_than_dec() {
        assert_eq!(0.5, round_dp(0.5, 5));
    }

    #[test]
    fn gcd_of_15_50() {
        assert_eq!(5, gcd(15,50).unwrap()); //have to hardcode the gcd bc rust doesnt have it in std??
    }

    #[test]
    fn gcd_of_50_15() {
        assert_eq!(gcd(50,15).unwrap(), gcd(15,50).unwrap()); //have to hardcode the gcd bc rust doesnt have it in std??
    }

    #[test]
    fn gcd_both_zero() {
        assert!(gcd(0, 0).is_err());
    }
}
