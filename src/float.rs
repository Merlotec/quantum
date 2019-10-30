use nalgebra::RealField;
use rendy::hal::format::Format;
use std::mem;
pub enum Precision {
    Bits32,
    Bits64,
}
pub trait EngineFloat : RealField + Send + Sync {
    /// Returns the precision of the float (e.g. 32 bit or 64 bit)
    /// This is used to determine shader formats etc.
    fn precision() -> Precision;

    /// The rgba format for this float for the given whole number of elements.
    fn vector_format(n: u8) -> Format where Self: Sized;
}

impl EngineFloat for f32 {
    fn precision() -> Precision {
        Precision::Bits32
    }

    /// The vector f32 format for the given whole number of elements.
    fn vector_format(n: u8) -> Format {
        match n {
            1 => Format::R32Sfloat,
            2 => Format::Rg32Sfloat,
            3 => Format::Rgb32Sfloat,
            4 => Format::Rgba32Sfloat,
            _ => panic!("Format 'n' out of range for f32."),
        }
    }
}

impl EngineFloat for f64 {
    fn precision() -> Precision {
        Precision::Bits64
    }

    /// The vector f64 format for the given whole number of elements.
    fn vector_format(n: u8) -> Format {
        match n {
            1 => Format::R64Sfloat,
            2 => Format::Rg64Sfloat,
            3 => Format::Rgb64Sfloat,
            4 => Format::Rgba64Sfloat,
            _ => panic!("Format 'n' out of range for f32."),
        }
    }
}