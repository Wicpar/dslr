use num_traits::real::Real;
use std::cmp::Ordering;

pub fn count<T: Real>(data: &[T]) -> T {
    T::from(data.len()).unwrap()
}

pub fn mean<T: Real> (data: &[T]) -> T {
    data.iter().copied().reduce(|a, b| a + b).unwrap_or(T::zero()) / count(data)
}

/// standard deviation
pub fn std<T: Real>(data: &[T]) -> T {
    let mean = mean(data);
    T::sqrt(data.iter().copied().map(|it|{
        let x = it - mean;
        x * x
    }).reduce(|a, b| a + b).unwrap_or(T::zero()) / count(data))
}

pub fn percentiles<T: Real, U: Real, const N: usize>(data: &[T], percentiles: &[U; N]) -> [T; N] {
    let mut data =  data.to_vec();
    data.sort_unstable_by(|a, b|a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let scale = U::from(data.len() - 1).unwrap();
    percentiles.map(|it| {
        let idx = (it * scale).ceil().to_usize().unwrap();
        data[idx]
    })
}