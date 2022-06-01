pub use configger_derive::*;

#[derive(Debug, Clone)]
pub enum PluginError {
    ValidationError(String),
}

pub enum ConfiggerField {
    Leaf {
        name: &'static str,
        // pub ty: &'static str, // TODO Syn type?
        require_on_create: bool,
    },
    Node {
        name: &'static str,
        is_list: bool,
        data: Vec<ConfiggerField>,
    },
}

pub trait ConfiggerData {
    fn fields() -> Vec<ConfiggerField>;
}
