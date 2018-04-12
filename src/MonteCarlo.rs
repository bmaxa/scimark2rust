use super::*;
use Slice;
use random::*;
const SEED:i32 = 113;

pub fn montecarlo_numFlops(NumSamples:i32)->f64 {
    NumSamples as f64 * 4.0
}

pub fn montecarlo_integrate(NumSamples:i32)->f64{
    let mut R = Random::new_seed(SEED);

    let mut under_curve = 0;

    for _ in 0..NumSamples {
        let x = R.nextDouble();
        let y = R.nextDouble();

        if x*x + y*y <= 1.0 {
            under_curve += 1;
        }
    }
    under_curve as f64 / NumSamples as f64 * 4.0
}
