use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Milliseconds(u32);

impl Milliseconds {
    pub fn new(ms: u32) -> Self {
        Milliseconds(ms)
    }

    pub fn from_duration(duration: Duration) -> Self {
        Milliseconds((duration.as_secs() * 1000) as u32 + duration.subsec_millis())
    }

    pub fn to_duration(self) -> Duration {
        Duration::from_millis(self.0 as u64)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl Add for Milliseconds {
    type Output = Milliseconds;

    #[inline]
    fn add(self, other: Milliseconds) -> Milliseconds {
        Milliseconds(self.0 + other.0)
    }
}

impl AddAssign for Milliseconds {
    #[inline]
    fn add_assign(&mut self, other: Milliseconds) {
        self.0 += other.0;
    }
}

impl Sub for Milliseconds {
    type Output = Milliseconds;

    #[inline]
    fn sub(self, other: Milliseconds) -> Milliseconds {
        Milliseconds(self.0 - other.0)
    }
}

impl SubAssign for Milliseconds {
    #[inline]
    fn sub_assign(&mut self, other: Milliseconds) {
        self.0 -= other.0;
    }
}
