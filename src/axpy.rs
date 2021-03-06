extern crate test;

use std::cmp;
use std::ops::{Add, Mul};


pub fn axpy<T>(a: T, x: &[T], y: &mut [T])
    where T: Copy + Clone + Add<T, Output=T> + Mul<T, Output=T>
{
    let len = cmp::min(x.len(), y.len());

    let xs = &x[..len];
    let ys = &mut y[..len];

    for i in 0..len {
        // NOTE Not using f32.mul_add because it seems to have a
        // consistent order of magnitude lower performance. Would
        // love to know why and if there's a way to fix that.
        ys[i] = ys[i] + a * xs[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    extern crate rblas;

    use self::rand::{Rng, StdRng, SeedableRng};

    // A constant seed that we'll use for initializing random vecs.
    const SEED: &'static [usize] = &[1, 2, 3, 4];

    #[test]
    fn saxpy() {
        use self::rblas::Axpy;

        use std::f32::EPSILON;
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();
        let y = rng.gen_iter::<f32>().take(3).collect::<Vec<f32>>();

        let mut fortran_result = y.clone();
        let mut rust_result = y.clone();

        Axpy::axpy(&PI, &x, &mut fortran_result);
        axpy(PI, &x[..], &mut rust_result[..]);

        assert_eq!(fortran_result.len(), rust_result.len());

        for (f, r) in fortran_result.iter().zip(rust_result) {
            assert!((f-r).abs() <= EPSILON);
        }
    }

    #[bench]
    fn saxpy_1000_rust(b: &mut test::Bencher) {
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let mut y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| axpy(PI, &x[..], &mut y[..]));
    }

    #[bench]
    fn saxpy_1000_fortran(b: &mut test::Bencher) {
        use self::rblas::Axpy;
        use std::f32::consts::PI;

        let mut rng: StdRng= SeedableRng::from_seed(SEED);

        let x = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();
        let mut y = rng.gen_iter::<f32>().take(1000).collect::<Vec<f32>>();

        b.iter(|| Axpy::axpy(&PI, &x[..], &mut y[..]));
    }
}
