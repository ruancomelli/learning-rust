// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

const SECONDS_IN_EARTH_YEAR: u64 = 31557600;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! def_planet {
    ($planet:ident, $orbital_period:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                Earth::years_during(d) / $orbital_period
            }
        }
    };
}

pub struct Earth;

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_EARTH_YEAR as f64
    }
}

def_planet!(Mercury, 0.2408467);
def_planet!(Venus, 0.61519726);
def_planet!(Mars, 1.8808158);
def_planet!(Jupiter, 11.862615);
def_planet!(Saturn, 29.447498);
def_planet!(Uranus, 84.016846);
def_planet!(Neptune, 164.79132);
