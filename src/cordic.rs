//computing trig functions on the interval [0,pi/2]
const K: f64 = 0.6072529350088812; //Length fixing constant
const MAX_CORDIC_ITER: i64 = 100;
const THRESHOLD: f64 = 1e-12; //How small the changes should get before begin considered negligible
const PI: f64 = std::f64::consts::PI;

pub fn trig(theta: f64) -> (f64, f64) {
    //Fixes theta to be in [0,pi/2] and adds appropriate sign
    let mut domain_fixed_theta: f64 = theta.rem_euclid(PI);
    if domain_fixed_theta > PI/2.0 {
        domain_fixed_theta = PI - domain_fixed_theta;
    }
    let quadrant_number: i64 = (theta/(PI/2.0)).floor() as i64;

    let mut x: f64 = K;
    let mut y: f64 = 0.0;
    let mut phi: f64 = domain_fixed_theta;
    let mut change: f64 = 1.0; //Start at anything above threshold
    let mut n_iter: i64 = 0;

    while n_iter < MAX_CORDIC_ITER && change > THRESHOLD {
        //Decision variable for which direction to go
        let mut d: f64 = 1.0;
        if phi != 0.0 {
            d = (phi/phi.abs()).round();
        }
            
        let curr_angle_change: f64 = (0.5_f64).powi(n_iter as i32);

        //Iteration steps found here: https://en.wikipedia.org/wiki/CORDIC
        let x_next: f64 = x - (d * y * curr_angle_change);
        let y_next: f64 = y + (d * x * curr_angle_change);
        let phi_next: f64 = phi - (d * curr_angle_change.atan());
        change = (x_next - x).abs() + (y_next - y).abs(); //Computed with L1 norm arbitrarily

        y = y_next;
        x = x_next;
        phi = phi_next;

        n_iter += 1;
    }

    //Fixing x and y to be in the right quadrant again
    dbg!(quadrant_number);
    (x,y) = match quadrant_number % 4 {
        0 => (x,y),
        1 => (-x,y),
        2 => (x,-y),
        3 => (-x,-y),
        -1 => (x, -y),
        -2 => (-x,-y),
        -3 => (-x,y),
        _ => (0.0,0.0),
    };

    return (x, y);
}

mod cordic_test {
    use super::*;
    use crate::arithmetic::{self, round_dp};

    #[test]
    fn cos_one () {
        assert_eq!(arithmetic::round_dp(trig(1.0).0, 4), round_dp((1.0_f64).cos(), 4))
    }

    #[test]
    fn sin_one () {
        assert_eq!(arithmetic::round_dp(trig(1.0).1, 4), round_dp((1.0_f64).sin(), 4))
    }

    #[test]
    fn cos_two () {
        assert_eq!(arithmetic::round_dp(trig(2.0).0, 4), round_dp((2.0_f64).cos(), 4))
    }

    #[test]
    fn sin_two () {
        assert_eq!(arithmetic::round_dp(trig(2.0).1, 4), round_dp((2.0_f64).sin(), 4))
    }

    #[test]
    fn cos_neg_one () {
        assert_eq!(arithmetic::round_dp(trig(-1.0).0, 4), round_dp((-1.0_f64).cos(), 4))
    }

    #[test]
    fn sin_neg_one () {
        assert_eq!(arithmetic::round_dp(trig(-1.0).1, 4), round_dp((-1.0_f64).sin(), 4))
    }

    #[test]
    fn cos_neg_two () {
        assert_eq!(arithmetic::round_dp(trig(-2.0).0, 4), round_dp((-2.0_f64).cos(), 4))
    }

    #[test]
    fn sin_neg_two () {
        assert_eq!(arithmetic::round_dp(trig(-2.0).1, 4), round_dp((-2.0_f64).sin(), 4))
    }
}