// A manual definition for what I want to generate

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
enum PluginError {
    ValidationError(String),
}

trait Plugin {
    fn generate(&self, schemas: &[Rc<Schema>]) -> Result<(), PluginError>;
}

struct DemoPlugin {}

impl Plugin for DemoPlugin {
    fn generate(&self, schemas: &[Rc<Schema>]) -> Result<(), PluginError> {
        schemas
            .iter()
            .try_for_each(|schema| -> Result<(), PluginError> {
                println!("Schema: {}", schema.name);
                schema
                    .models
                    .borrow()
                    .iter()
                    .try_for_each(|model| -> Result<(), PluginError> {
                        println!("Model: {}", model.name);
                        model.fields.borrow().iter().try_for_each(
                            |field| -> Result<(), PluginError> {
                                if field.name.starts_with('_') {
                                    return Err(PluginError::ValidationError(format!(
                                        "Field {} starts with an underscore",
                                        field.name
                                    )));
                                }
                                println!("Field: {} {}", field.name, field.ttype);
                                Ok(())
                            },
                        )
                    })
            })
    }
}

struct Builder {
    schemas: Vec<Rc<Schema>>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Builder {
    fn new() -> Self {
        Self {
            schemas: Vec::new(),
            plugins: vec![Box::new(DemoPlugin {})],
        }
    }

    fn new_schema(&mut self, name: &str) -> Rc<Schema> {
        let schema = Rc::new(Schema {
            name: name.to_string(),
            models: RefCell::new(Vec::new()),
        });
        self.schemas.push(schema.clone());
        schema
    }

    fn build(self) {
        // TODO Flatten the schemas struct (should all be generated anyway)
        for plugin in self.plugins {
            plugin.generate(&self.schemas).unwrap();
        }
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
    let mut builder = Builder::new();

    let schema = builder.new_schema("discordbot");

    let alarm = schema.new_model("scheduled_alarm");
    alarm.new_int("id").desc("desc");
    alarm.new_int("requester_discord_id").desc("desc");
    alarm.new_int("channel_id").desc("desc");
    alarm.new_int("hour_of_day").desc("desc");
    alarm.new_int("day_of_week").desc("desc");
    alarm.new_datetime("created_at").desc("desc");
    alarm.new_string("message").desc("desc");

    builder.build();
}
