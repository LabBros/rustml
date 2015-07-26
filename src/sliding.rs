//! Sliding windows for arbitrary dimensions.

extern crate num;
use self::num::iter;

#[derive(Copy, Clone)]
pub struct DimensionParameters {
    pub area_width: usize,
    pub window_width: usize,
    pub delta: usize
}

impl DimensionParameters {
    pub fn new(area_width: usize, window_width: usize, delta: usize) -> DimensionParameters {
        DimensionParameters {
            area_width: area_width,
            window_width: window_width,
            delta: delta
        }
    }
}

pub struct SlidingWindowBuilder {
    dimensions: Vec<DimensionParameters>
}

impl SlidingWindowBuilder {

    pub fn add(&self, area_width: usize, window_width: usize, delta: usize) -> SlidingWindowBuilder {

        let mut v = self.dimensions.clone();
        v.push(DimensionParameters::new(area_width, window_width, delta));
        SlidingWindowBuilder {
            dimensions: v
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<usize>> {
        sliding_window(&self.dimensions)
    }

    pub fn to_2d(&self) -> Option<Vec<(usize, usize)>> {

        if self.dimensions.len() != 2 {
            None
        } else {
            Some(sliding_window_2d(&self.dimensions[0], &self.dimensions[1]))
        }
    }
}

pub fn builder() -> SlidingWindowBuilder {
    SlidingWindowBuilder {
        dimensions: vec![]
    }
}


pub fn sliding_window(dp: &[DimensionParameters]) -> Vec<Vec<usize>> {

    let mut r: Vec<Vec<usize>> = Vec::new();

    if dp.len() == 1 {
        for i in iter::range_step_inclusive(0, dp[0].area_width - dp[0].window_width, dp[0].delta) {
            r.push(vec![i]);
        }
    } else {
        let (x, y) = dp.split_at(1);
        let v = sliding_window(y);
        for ref values in v {
            for i in iter::range_step_inclusive(0, x[0].area_width - x[0].window_width, x[0].delta) {
                let mut k: Vec<usize> = Vec::new();
                k.push(i);
                for item in values {
                    k.push(*item);
                }
                r.push(k);
            }
        }
    }
    r
}

pub fn sliding_window_1d(dp: &DimensionParameters) -> Vec<usize> {

    sliding_window(&[*dp]).get(0).unwrap().clone()
}

pub fn sliding_window_2d(x: &DimensionParameters, y: &DimensionParameters) -> Vec<(usize, usize)> {

    sliding_window(&[*x, *y])
        .iter().map(|v| (v[0], v[1])).collect::<Vec<(usize, usize)>>()
}

// TODO tests
