pub use configger_derive::*;

pub struct ConfiggerField {
    pub name: &'static str,
    // pub ty: &'static str, // TODO Syn type?
    pub require_on_create: bool,
}

pub trait ConfiggerData {
    fn fields() -> Vec<ConfiggerField>;
}
