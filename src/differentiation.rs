// base step size for differentiation defined by h = 2 * sqrt(MACHINE_EPSILON) for 64 bit float
const BASE_STEP_SIZE: f64 = 0.000000029802322387695313;

//Using the finite symmetic difference df/dx = (f(x+h) - f(x-h))/2h
pub fn symmetric_differentiation<F> (point: f64, function: F, resolution: Option<f64>) -> f64 where F: Fn(f64) -> f64 {
    //A approximation of the optimal step size, overridden by the optional arg
    let dx = resolution.unwrap_or(BASE_STEP_SIZE);

    //Uses the formula and returns the answer
    let answer: f64 = (function(point + dx) - function(point - dx))/(2.0 * dx);
    return answer;
}

//Using the 5-point derivate formula df/dx = (-f(x + 2h) + 8f(x + h) - 8f(x - h) + f(x - 2h))/(12h)
pub fn stencil_differentiation<F>(point: f64, function: F, resolution: Option<f64>) -> f64 where F: Fn(f64) -> f64 {
    //A approximation of the optimal step size, overridden by the optional arg
    let dx = resolution.unwrap_or(BASE_STEP_SIZE);

    let answer: f64 = (-function(point + 2.0 * dx) + 8.0 * function(point + dx) - 8.0 * function(point - dx) + function(point - 2.0 * dx))/(12.0 * dx);

    return answer;
}

fn main() {}

//Testing differentiation on various functions 
mod differentiation_test {
    use super::*;
    use crate::arithmetic;

    const PRECISION: usize = 10;

    //Testing the symmetric difference on different kinds of functions
    #[test]
    fn symmetric_linear (){
        let point_1: f64 = 0.0;
        let point_2: f64 = 1.0;
        let point_3: f64 = 1.5;
        let slope = 2.0;
        let linear = |x: f64| -> f64 {
            return slope * x;
        };

        assert_eq!(slope, arithmetic::round_dp(symmetric_differentiation(point_1, linear, None), PRECISION));
        assert_eq!(slope, arithmetic::round_dp(symmetric_differentiation(point_2, linear, None), PRECISION));
        assert_eq!(slope, arithmetic::round_dp(symmetric_differentiation(point_3, linear, None), PRECISION));
    }
    
    #[test]
    fn symmetric_quadratic (){
        let point_1 = 0.0;
        let answer_1 = 0.0;
        let point_2 = 1.0;
        let answer_2 = 2.0;
        let point_3 = 1.5;
        let answer_3 = 3.0;
        let quadratic = |x:f64| -> f64 {
            return x.powf(2.0);
        };

        assert_eq!(answer_1, arithmetic::round_dp(symmetric_differentiation(point_1, quadratic, None), PRECISION));
        assert_eq!(answer_2, arithmetic::round_dp(symmetric_differentiation(point_2, quadratic, None), PRECISION));
        assert_eq!(answer_3, arithmetic::round_dp(symmetric_differentiation(point_3, quadratic, None), PRECISION));
    }

    //Testing the five point method on different functions
    #[test]
    fn five_point_linear (){
        let point_1: f64 = 0.0;
        let point_2: f64 = 1.0;
        let point_3: f64 = 1.5;
        let slope = 2.0;
        let linear = |x: f64| -> f64 {
            return slope * x;
        };

        assert_eq!(slope, arithmetic::round_dp(stencil_differentiation(point_1, linear, None), PRECISION));
        assert_eq!(slope, arithmetic::round_dp(stencil_differentiation(point_2, linear, None), PRECISION));
        assert_eq!(slope, arithmetic::round_dp(stencil_differentiation(point_3, linear, None), PRECISION));
    }
}