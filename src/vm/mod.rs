pub use self::machine::execute;
pub use self::frame::Frame;

pub mod ir;
pub mod frame;
pub mod machine;

pub type Value = i64;
