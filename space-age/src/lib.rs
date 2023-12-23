// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    fn new() -> Self;

    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury {
    orbital_period: f64,
}
pub struct Venus {
    orbital_period: f64,
}
pub struct Earth {
    orbital_period: f64,
}
pub struct Mars {
    orbital_period: f64,
}
pub struct Jupiter {
    orbital_period: f64,
}
pub struct Saturn {
    orbital_period: f64,
}
pub struct Uranus {
    orbital_period: f64,
}
pub struct Neptune {
    orbital_period: f64,
}

const EARTH_ORBITAL_PERIOD: f64 = 31557600.0;

impl Planet for Mercury {
    fn new() -> Self {
        Self {
            orbital_period: 0.2408467,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
impl Planet for Venus {
    fn new() -> Self {
        Self {
            orbital_period: 0.61519726,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
impl Planet for Earth {
    fn new() -> Self {
        Self {
            orbital_period: 1.0,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
impl Planet for Mars {
    fn new() -> Self {
        Self {
            orbital_period: 1.8808158,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
impl Planet for Jupiter {
    fn new() -> Self {
        Self {
            orbital_period: 11.862615,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
impl Planet for Saturn {
    fn new() -> Self {
        Self {
            orbital_period: 29.447498,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
impl Planet for Uranus {
    fn new() -> Self {
        Self {
            orbital_period: 84.016846,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
impl Planet for Neptune {
    fn new() -> Self {
        Self {
            orbital_period: 164.79132,
        }
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_ORBITAL_PERIOD / Self::new().orbital_period
    }
}
