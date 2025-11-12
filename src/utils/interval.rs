use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Interval>,
    {
        let mut it = iter.into_iter();
        if let Some(first) = it.next() {
            let mut acc = first;
            for iv in it {
                acc = acc.merge(&iv);
            }
            acc
        } else {
            Interval::empty()
        }
    }

    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }

    pub fn expand(&mut self, delta: f64) {
        let padding = delta / 2.0;
        self.min -= padding;
        self.max += padding;
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self::new(self.min.min(other.min), self.max.max(other.max))
    }
}

impl<'a> Add<&'a f64> for Interval {
    type Output = Self;

    fn add(self, other: &'a f64) -> Self {
        Self {
            min: self.min + other,
            max: self.max + other,
        }
    }
}

impl<'a> Add<&'a Interval> for f64 {
    type Output = Interval;

    fn add(self, other: &'a Interval) -> Interval {
        Interval::new(self + other.min, self + other.max)
    }
}
