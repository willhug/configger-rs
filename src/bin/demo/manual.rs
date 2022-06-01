// A manual definition for what I want to generate
use std::{cell::RefCell, rc::Rc};

use configger::PluginError;

use self::plugin_def::*;
use self::schema_bundle_def::*;

// Manually written "data" bundle
pub mod schema_bundle_def {
    // TODO how to not have to import ConfiggerField?
    use configger::{ConfiggerData, ConfiggerField};

    // #[derive(ConfiggerData)]
    pub struct BackendDef {
        // #[configger(list_def)]
        schemas: Vec<SchemaDef>,
    }

    impl ConfiggerData for BackendDef {
        fn fields() -> Vec<ConfiggerField> {
            vec![ConfiggerField::Node {
                name: "schemas",
                is_list: true,
                data: BackendDef::fields(),
            }]
        }
    }

    // #[derive(ConfiggerData)]
    pub struct SchemaDef {
        // #[configger(require_on_create)]
        name: String,

        // #[configger(list_def)]
        models: Vec<ModelDef>,
    }

    impl ConfiggerData for SchemaDef {
        fn fields() -> Vec<ConfiggerField> {
            vec![
                ConfiggerField::Leaf {
                    name: "name",
                    require_on_create: true,
                },
                ConfiggerField::Node {
                    name: "models",
                    is_list: true,
                    data: ModelDef::fields(),
                },
            ]
        }
    }

    // #[derive(ConfiggerData)]
    struct ModelDef {
        // #[configger(require_on_create)]
        name: String,

        // #[configger(list_def)]
        fields: Vec<FieldDef>,
    }

    // TODO autogen
    impl ConfiggerData for ModelDef {
        fn fields() -> Vec<ConfiggerField> {
            vec![
                ConfiggerField::Leaf {
                    name: "name",
                    require_on_create: true,
                },
                ConfiggerField::Node {
                    name: "fields",
                    is_list: true,
                    data: FieldDef::fields(),
                },
            ]
        }
    }

    // Manual Field Bundle Definition
    #[derive(ConfiggerData)]
    struct FieldDef {
        #[configger(require_on_create)]
        name: String,
        #[configger(require_on_create)]
        ttype: String,
        desc: String,
    }

    // Already generated!
    // impl ConfiggerData for FieldDef {
    //     fn fields() -> Vec<ConfiggerField> {
    //         vec![
    //             ConfiggerField {
    //                 name: "name",
    //                 // ty: "String", // TODO Syn type?
    //                 require_on_create: true,
    //             },
    //             ConfiggerField {
    //                 name: "desc",
    //                 // ty: "String", // TODO Syn type?
    //                 require_on_create: true,
    //             },
    //         ]
    //     }
    // }
}

// Plugin definition (mostly manual, some generation from field defs)
mod plugin_def {
    use configger::PluginError;

    // Generated from configger::data_bundle!("PrintPlugin", schema_bundle_def::SchemaDef)
    pub struct PrintPluginBackend {
        pub schemas: Vec<PrintPluginSchema>,
    }
    pub struct PrintPluginSchema {
        pub name: String,
        pub models: Vec<PrintPluginModel>,
    }
    pub struct PrintPluginModel {
        pub name: String,
        pub fields: Vec<PrintPluginField>,
    }
    pub struct PrintPluginField {
        pub name: String,
        pub ttype: String,
        pub desc: String,
    }

    // Generated Impl trait from data_bundle (this is what your plugin needs to implement)
    pub trait PrintPluginGenerator {
        fn generate(&self, backend: PrintPluginBackend) -> Result<(), PluginError>;
    }

    // Manual Def
    pub struct PrintPlugin {}

    // Manual plugin code (where you'd put your logic)
    impl PrintPluginGenerator for PrintPlugin {
        fn generate(&self, backend: PrintPluginBackend) -> Result<(), PluginError> {
            backend
                .schemas
                .iter()
                .try_for_each(|schema| -> Result<(), PluginError> {
                    println!("Schema: {}", schema.name);
                    schema
                        .models
                        .iter()
                        .try_for_each(|model| -> Result<(), PluginError> {
                            println!("Model: {}", model.name);
                            model
                                .fields
                                .iter()
                                .try_for_each(|field| -> Result<(), PluginError> {
                                    if field.name.starts_with('_') {
                                        return Err(PluginError::ValidationError(format!(
                                            "Field {} starts with an underscore",
                                            field.name
                                        )));
                                    }
                                    println!(
                                        "Field: {} {} {}",
                                        field.name, field.ttype, field.desc
                                    );
                                    Ok(())
                                })
                        })
                })
        }
    }
}

// The rest of the structs here should be generated by:
// configger::builder!(PrintPlugin)
// We might need to include the data bundle used
// configger::builder!(schema_bundle_def::SchemaDef, PrintPlugin)
// We should also be able to pass in multiple Plugins (and these might depend on different sets of data)
// configger::builder!(PrintPlugin, Print2Plugin, DemoPlugin)

// Generated trait for each plugin "wrapper" (which are also generated)
trait Plugin {
    fn generate(&self, backend: &Backend) -> Result<(), PluginError>;
}

struct PrintPluginWrapper {
    plugin: PrintPlugin,
}

impl From<Rc<Field>> for PrintPluginField {
    fn from(f: Rc<Field>) -> Self {
        let inner = f.inner.borrow();
        Self {
            name: f.name.clone(),
            ttype: f.ttype.clone(),
            desc: inner.desc.clone(),
        }
    }
}

impl From<Rc<Model>> for PrintPluginModel {
    fn from(m: Rc<Model>) -> Self {
        Self {
            name: m.name.clone(),
            fields: m.fields.borrow().iter().map(|f| f.clone().into()).collect(),
        }
    }
}

impl From<Rc<Schema>> for PrintPluginSchema {
    fn from(s: Rc<Schema>) -> Self {
        Self {
            name: s.name.clone(),
            models: s.models.borrow().iter().map(|m| m.clone().into()).collect(),
        }
    }
}

impl From<&Backend> for PrintPluginBackend {
    fn from(b: &Backend) -> Self {
        Self {
            schemas: b
                .schemas
                .borrow()
                .iter()
                .map(|m| m.clone().into())
                .collect(),
        }
    }
}

impl Plugin for PrintPluginWrapper {
    fn generate(&self, be: &Backend) -> Result<(), PluginError> {
        // Convert the types to what PrintPlugin expects (TODO make this all into?)
        self.plugin.generate(be.into())
    }
}

fn build(be: Backend) {
    let plugins = vec![Box::new(PrintPluginWrapper {
        plugin: PrintPlugin {},
    })];
    for plugin in plugins {
        plugin.generate(&be).unwrap();
    }
}

// The top-level type is special, and has it's own build command?  Not sure if I like that API, can revise later.
struct Backend {
    schemas: RefCell<Vec<Rc<Schema>>>,
}

impl Backend {
    fn new() -> Self {
        Self {
            schemas: RefCell::new(Vec::new()),
        }
    }

    fn new_schema(&mut self, name: &str) -> Rc<Schema> {
        let schema = Rc::new(Schema {
            name: name.to_string(),
            models: RefCell::new(Vec::new()),
        });
        self.schemas.borrow_mut().push(schema.clone());
        schema
    }
}

struct Schema {
    name: String,
    models: RefCell<Vec<Rc<Model>>>,
}

impl Schema {
    fn new_model(&self, name: &str) -> Rc<Model> {
        let model = Rc::new(Model {
            name: name.to_string(),
            fields: RefCell::new(Vec::new()),
        });
        self.models.borrow_mut().push(model.clone());
        model
    }
}

struct Model {
    name: String,
    fields: RefCell<Vec<Rc<Field>>>,
}

impl Model {
    fn new_int(&self, name: &str) -> Rc<Field> {
        let field = Rc::new(Field::new(name, "int64"));
        self.fields.borrow_mut().push(field.clone());
        field
    }

    fn new_string(&self, name: &str) -> Rc<Field> {
        let field = Rc::new(Field::new(name, "String"));
        self.fields.borrow_mut().push(field.clone());
        field
    }

    fn new_datetime(&self, name: &str) -> Rc<Field> {
        let field = Rc::new(Field::new(name, "DateTime"));
        self.fields.borrow_mut().push(field.clone());
        field
    }
}

struct Field {
    name: String,
    ttype: String,
    inner: RefCell<FieldMut>,
}

struct FieldMut {
    desc: String,
}

impl Field {
    fn new(name: &str, ttype: &str) -> Self {
        Self {
            name: name.to_string(),
            ttype: ttype.to_string(),
            inner: RefCell::new(FieldMut {
                desc: "".to_string(),
            }),
        }
    }

    fn desc(&self, desc: &str) -> &Field {
        self.inner.borrow_mut().desc = desc.to_string();
        self
    }
}

pub fn demo() {
    let mut backend = Backend::new();

    let schema = backend.new_schema("discordbot");

    let alarm = schema.new_model("scheduled_alarm");
    alarm.new_int("id").desc("desc");
    alarm.new_int("requester_discord_id").desc("desc");
    alarm.new_int("channel_id").desc("desc");
    alarm.new_int("hour_of_day").desc("desc");
    alarm.new_int("day_of_week").desc("desc");
    alarm.new_datetime("created_at").desc("desc");
    alarm.new_string("message").desc("desc");

    build(backend);
}
