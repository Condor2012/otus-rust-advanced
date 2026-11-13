macro_rules! unit {
    ($name:ident, $suffix:literal) => {
        pub struct $name(f32);

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, concat!("{}", $suffix), self.0)
            }
        }

        impl ::std::ops::Add for $name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                $name(self.0 + rhs.0)
            }
        }

        impl ::std::ops::Sub for $name {
            type Output = $name;

            fn sub(self, rhs: Self) -> Self::Output {
                $name(self.0 - rhs.0)
            }
        }
    };
    ($($name:ident => $suffix:literal);* $(;)?) => {
        $(unit!($name, $suffix);)*
    };
}

unit!(Meter, "m");
unit!(Kilometer, "km");
unit!(Millimeter, "mm");

unit! {
    Second => "s";
    Minute => "min";
    Hour => "h";
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_ops() {
        let m1 = Meter(1.0);
        let m2 = Meter(2.0);
        let m3 = Meter(3.0);

        let result = m1 + m2 - m3;

        assert!(result.0.abs() <= f32::EPSILON);
    }

    #[test]
    fn meter_format() {
        let m1 = Meter(1.1);

        let result = format!("{m1}");

        assert_eq!(result, "1.1m");
    }

    #[test]
    fn kilometer_ops() {
        let m1 = Kilometer(1.0);
        let m2 = Kilometer(2.0);
        let m3 = Kilometer(3.0);

        let result = m1 + m2 - m3;

        assert!(result.0.abs() <= f32::EPSILON);
    }

    #[test]
    fn kilometer_format() {
        let m1 = Kilometer(1.1);

        let result = format!("{m1}");

        assert_eq!(result, "1.1km");
    }



    #[test]
    fn second_ops() {
        let m1 = Second(1.0);
        let m2 = Second(2.0);
        let m3 = Second(3.0);

        let result = m1 + m2 - m3;

        assert!(result.0.abs() <= f32::EPSILON);
    }

    #[test]
    fn second_format() {
        let m1 = Second(1.1);

        let result = format!("{m1}");

        assert_eq!(result, "1.1s");
    }
}
