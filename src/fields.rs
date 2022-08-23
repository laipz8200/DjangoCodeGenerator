mod bool;
mod char;
mod foreign;
mod int;

pub use self::bool::BooleanField;
pub use self::char::CharField;
pub use self::foreign::ForeignKey;
pub use self::int::IntegerField;

pub trait Field {
    fn model_field_code(&self) -> String;
}
