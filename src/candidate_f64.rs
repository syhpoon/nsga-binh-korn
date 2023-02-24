use nsga::Solution;
use rand::{thread_rng, Rng};
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct CandidateF64 {
    pub val: f64,
    pub range: RangeInclusive<f64>,
}

impl Solution for CandidateF64 {
    // SBX Crossover
    // https://youtu.be/7-NPqSvutr0?t=718
    // https://github.com/baopng/NSGA-II/blob/master/nsga2/utils.py#L89
    fn crossover(&mut self, other: &mut Self) {
        let crossover_param: f64 = 2.;
        let u: f64 = thread_rng().gen_range(0.0..1.0);

        let beta = if u <= 0.5 {
            (2. * u).powf(1. / (crossover_param + 1.))
        } else {
            (2. * (1. - u)).powf(-1. / (crossover_param + 1.))
        };

        let x1 = (self.val + other.val) / 2.;
        let x2 = ((self.val - other.val) / 2.).abs();

        self.val = x1 + beta * x2;
        other.val = x1 - beta * x2;
    }

    // Polynomial mutation
    // https://youtu.be/7-NPqSvutr0?t=916
    // https://github.com/baopng/NSGA-II/blob/master/nsga2/utils.py#L108
    fn mutate(&mut self) {
        let mutation_param: f64 = 5.;
        let u: f64 = thread_rng().gen_range(0.0..1.0);
        let r0: f64 = *self.range.start();
        let r1: f64 = *self.range.end();

        let delta = if u < 0.5 {
            (2. * u).powf(1. / (mutation_param + 1.)) - 1.
        } else {
            1. - (2. * (1. - u)).powf(-1. / (mutation_param + 1.))
        };

        if u < 0.5 {
            self.val += delta * (self.val - r0)
        } else {
            self.val += delta * (r1 - self.val)
        }

        if self.val < r0 {
            self.val = r0
        } else if self.val > r1 {
            self.val = r1
        }
    }
}
