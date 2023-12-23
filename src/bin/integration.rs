//My implementation of integration algorithms

use rayon::prelude::*;
mod arithmetic;

//Defines useful constants for integration
const BASE_STEP_SIZE: f64 = 0.000001;

//Integrating using Simpson's Rule (Read more here: math24.net/simpsons-rule.html)
pub fn quad_simpson<F> (start_point: f64, end_point: f64, f: F, resolution: Option<f64>) -> f64 where F: Fn(f64) -> f64 {
    //If optional arg resolution is passed in, set the step width equal to it
    let dx = resolution.unwrap_or(BASE_STEP_SIZE);

    //Finds the numbers of steps to iterate over
    //Subtract one because we consider the start and end points as special cases
    let interval_width: f64 = end_point - start_point;
    let num_steps: u64 = (interval_width/dx).ceil() as u64 - 1;

    //applies formula over the middle of the interval, finding the unscaled result
    let scaling_const: f64 = dx/3.0; 
    let mut answer: f64 = f(start_point) + f(end_point); 
    let mut point: f64 = start_point;
    for i in 1..num_steps{
        //Find where the function is being evaluated and multiplies it by the coefficient in Simpson's Rule
        point += dx;
        let coefficient: f64 = 2.0 * (1 + i%2) as f64;
        answer += coefficient * f(point);
    }

    return scaling_const * answer;
}

//Integrating using the trapezoid rule
pub fn quad_trapezoid<F> (start_point: f64, end_point: f64, f: F, resolution: Option<f64>) -> f64 where F: Fn(f64) -> f64 {
    //Sets the step size according to the optional arg
    let dx = resolution.unwrap_or(BASE_STEP_SIZE);

    //Finds the numbers of steps to iterate over
    //Subtract one because we consider the start and end points as special cases
    let interval_width: f64 = end_point - start_point;
    let num_steps: u64 = (interval_width/dx).floor() as u64;

    //Iterates using trapezoids. int{f(x) dx} = h * (0.5 * f(x_0) + f(x_1) + f(x_2) +...+ f(x_{n-1}) + 0.5 * f(x_n))
    let mut answer: f64 = 0.5 * (f(start_point) + f(end_point));
    let mut point: f64 = start_point;
    for _i in 1..num_steps{
        point += dx;
        answer += f(point); 
    }
    return dx * answer;
}

//Trapezoidal integration run in parallel
pub fn par_quad_trapezoid<F: Fn(f64) -> f64 + Send + Sync> (start_point: f64, end_point: f64, f: F, resolution: Option<f64>) -> f64  {
    //Sets step size to optional arg or default
    let dx = resolution.unwrap_or(BASE_STEP_SIZE);

    //The vector over which we will iterate and perform the summation
    let interval: Vec<f64> = arithmetic::arrange(start_point, end_point, dx);

    //sums the adjacent functional outputs throughout the interval
    let sum: f64 = interval.par_iter().map(|val| f(*val) + f(*val + dx)).sum();
    return 0.5 * dx * sum;
}

fn main() {

}

//Checking accuracy 
mod integration_test {
    use super::*;

    //The decimal precision expected of answers
    const PRECISION: usize = 4;

    //Sets up the integral tests for each kind of function, including bounds and expected answers
    #[test]
    fn test_linear() {
        let linear_lower_bound: f64 = 0.0;
        let linear_upper_bound: f64 = 1.0;
        let linear_expected = arithmetic::round_dp(0.5, PRECISION);
        let linear = |x: f64| -> f64{
            return x;
        };
        
        assert_eq!(linear_expected, arithmetic::round_dp(quad_simpson(linear_lower_bound, linear_upper_bound, linear, None), PRECISION));
        assert_eq!(linear_expected, arithmetic::round_dp(quad_trapezoid(linear_lower_bound, linear_upper_bound, linear, None), PRECISION));
        assert_eq!(linear_expected, arithmetic::round_dp(par_quad_trapezoid(linear_lower_bound, linear_upper_bound, linear, None), PRECISION));
    }

    #[test]
    fn test_quadratic() {
        let quadratic_lower_bound: f64 = 0.0;
        let quadratic_upper_bound: f64 = 1.0;
        let quadratic_expected = arithmetic::round_dp(1.0/3.0, PRECISION);
        let quadratic = |x: f64| -> f64{
            return x.powi(2);
        };

        assert_eq!(quadratic_expected, arithmetic::round_dp(quad_simpson(quadratic_lower_bound, quadratic_upper_bound, quadratic, None), PRECISION));
        assert_eq!(quadratic_expected, arithmetic::round_dp(quad_trapezoid(quadratic_lower_bound, quadratic_upper_bound, quadratic, None), PRECISION));
        assert_eq!(quadratic_expected, arithmetic::round_dp(par_quad_trapezoid(quadratic_lower_bound, quadratic_upper_bound, quadratic, None), PRECISION));
    }

    #[test]
    fn test_cubic() {
        let cubic_lower_bound: f64 = 0.0;
        let cubic_upper_bound: f64 = 1.0;
        let cubic_expected: f64 = arithmetic::round_dp(0.25, PRECISION);
        let cubic = |x: f64| -> f64{
            return x.powi(3);
        };

        assert_eq!(cubic_expected, arithmetic::round_dp(quad_simpson(cubic_lower_bound, cubic_upper_bound, cubic, None), PRECISION));
        assert_eq!(cubic_expected, arithmetic::round_dp(quad_trapezoid(cubic_lower_bound, cubic_upper_bound, cubic, None), PRECISION));
        assert_eq!(cubic_expected, arithmetic::round_dp(par_quad_trapezoid(cubic_lower_bound, cubic_upper_bound, cubic, None), PRECISION));
    }

    #[test]
    fn test_sqrt() {
        let sqrt_lower_bound: f64 = 0.0;
        let sqrt_upper_bound: f64 = 1.0;
        let sqrt_expected: f64 = arithmetic::round_dp(2.0/3.0, PRECISION);
        let sqrt = |x: f64| -> f64{
            return x.powf(0.5);
        };

        assert_eq!(sqrt_expected, arithmetic::round_dp(quad_simpson(sqrt_lower_bound, sqrt_upper_bound, sqrt, None), PRECISION));
        assert_eq!(sqrt_expected, arithmetic::round_dp(quad_trapezoid(sqrt_lower_bound, sqrt_upper_bound, sqrt, None), PRECISION));
        assert_eq!(sqrt_expected, arithmetic::round_dp(par_quad_trapezoid(sqrt_lower_bound, sqrt_upper_bound, sqrt, None), PRECISION));
    }

    #[test]
    fn test_exponential() { 
        let exp_lower_bound: f64 = 0.0;
        let exp_upper_bound: f64 = 3.0;
        let exp_expected: f64 = arithmetic::round_dp(std::f64::consts::E.powf(3.0) - 1.0, PRECISION);
        let exponential = |x: f64| -> f64{
            return x.exp();
        };

        assert_eq!(exp_expected, arithmetic::round_dp(quad_simpson(exp_lower_bound, exp_upper_bound, exponential, None), PRECISION));
        assert_eq!(exp_expected, arithmetic::round_dp(quad_trapezoid(exp_lower_bound, exp_upper_bound, exponential, None), PRECISION));
        assert_eq!(exp_expected, arithmetic::round_dp(par_quad_trapezoid(exp_lower_bound, exp_upper_bound, exponential, None), PRECISION));
    }
}