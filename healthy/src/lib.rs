//!# Art
//!
//!一个用来建模艺术概念的代码库
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    use core::fmt;
    use std::fmt::Formatter;

    // RGB颜色模型
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    // RGB模型的调和色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 将两种等量的颜色混合
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}