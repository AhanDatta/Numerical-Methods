use crate::differentiation;

//Defines the default maximum number of newton iterations
const MAX_NEWTON_ITER: usize = 100;

//Iterative implementation of Newton's method
pub fn newton_iter<F: Fn(f64) -> f64> (f: F, input: f64, iter_count: Option<usize>) -> f64 {
    //Sets the iteration count to default or optional arg
    let max_iter: usize = iter_count.unwrap_or(MAX_NEWTON_ITER);

    //main loop to update the position
    let mut x: f64 = input;
    for i in 0..max_iter {
        //finds derivative, checks for stationary points, and if so, offsets the derivative
        let mut derivative: f64 = differentiation::symmetric_differentiation(x, &f, None);
        if derivative == 0.0 {
            derivative = 0.1;
        }
        x -= f(x)/derivative;
    }
    return x;
}

pub fn recursive_newton_iter<F: Fn(f64) -> f64> (f: F, input: f64, iter_count: usize) -> f64 {
    //finds derivative, checks for stationary points, and if so, offsets the derivative
    let mut derivative: f64 = differentiation::symmetric_differentiation(input, &f, None);
    if derivative == 0.0 {
        derivative = 0.1;
    }

    //Does one iteration and sets the base case
    let x = input - f(input)/derivative;
    if iter_count == 0 {
        return x;
    }

    //Otherwise run one layer deeper
    return recursive_newton_iter(f, x, iter_count - 1);
}

mod root_finding_test {
    use crate::root_finding::*;

    #[test]
    //Finding both roots for f(x) = x^2 + x - 6, which are x = 2, -3
    //Also tests the case when f'(x) = 0, which is at f'(-0.5)
    fn quadratic_roots_newton() {
        let function = |x: f64| {
            return x.powi(2) + x - 6.0
        };

        assert_eq!(2.0, newton_iter(function, 1.0, None));
        assert_eq!(-3.0, newton_iter(function, -2.0, None));
        assert!(newton_iter(function, 0.5, None) != std::f64::NAN);
    }

    #[test]
    fn quadratic_roots_recursive_newton() {
        let function = |x: f64| {
            return x.powi(2) + x - 6.0
        };

        assert_eq!(2.0, recursive_newton_iter(function, 1.0, 10));
        assert_eq!(-3.0, recursive_newton_iter(function, -2.0, 10));
        assert!(recursive_newton_iter(function, 0.5, 10) != std::f64::NAN);
    }
}