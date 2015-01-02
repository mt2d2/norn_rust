pub use self::machine::execute;
pub use self::frame::Frame;

pub mod ir;
pub mod frame;
pub mod machine;

type Value = i64;
