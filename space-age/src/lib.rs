pub struct Duration{
    seconds : u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{
            seconds: s
        }
    }
}

pub trait Planet {
    fn orbital_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31_557_600.0 / Self::orbital_period()
    }
}

macro_rules! planet {
    ($name:ident, $period:expr) => {
        pub struct $name;

        impl Planet for $name {
            fn orbital_period() -> f64 {
                $period
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
