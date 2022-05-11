
// A manual definition for what I want to generate

#[derive(Debug, Clone)]
enum PluginError {
    ValidationError(String),
}

trait Plugin {
    fn generate(&self, schemas: &[Schema]) -> Result<(), PluginError>;
}

struct DemoPlugin {
}

impl Plugin for DemoPlugin {
    fn generate(&self, schemas: &[Schema]) -> Result<(), PluginError> {
        schemas.iter().try_for_each(|schema| -> Result<(), PluginError> {
            println!("Schema: {}", schema.name);
            schema.models.iter().try_for_each(|model| -> Result<(), PluginError> {
                println!("Model: {}", model.name);
                model.fields.iter().try_for_each(|field| -> Result<(), PluginError> {
                    if field.name.starts_with('_') {
                        return Err(PluginError::ValidationError(format!("Field {} starts with an underscore", field.name)));
                    }
                    println!("Field: {} {}", field.name, field.ttype);
                    Ok(())
                })
            })
        })
    }
}

struct Builder {
    schemas: Vec<Schema>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Builder {
    fn new() -> Self {
        Self {
            schemas: Vec::new(),
            plugins: vec![Box::new(DemoPlugin {})],
        }
    }

    fn add_schema(&mut self, schema: Schema) {
        self.schemas.push(schema);
    }

    fn build(self) {
        for plugin in self.plugins {
            plugin.generate(&self.schemas).unwrap();
        }
    }
}

struct Schema {
    name: String,
    models: Vec<Model>,
}

impl Schema {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            models: Vec::new(),
        }
    }

    fn add_model(&mut self, model: Model)  {
        self.models.push(model);
    }
}

struct Model {
    name: String,
    fields: Vec<Field>,
}

impl Model {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }
}

struct Field {
    name: String,
    ttype: String,
}

impl Field {
    fn new_int(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ttype: "int64".to_string(),
        }
    }

    fn new_string(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ttype: "String".to_string(),
        }
    }

    fn new_datetime(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ttype: "DateTime".to_string(),
        }
    }
}

pub fn demo() {
    let mut builder = Builder::new();

    let mut schema = Schema::new("discordbot");

    let mut alarm = Model::new("scheduled_alarm");
    alarm.add_field(Field::new_int("id"));
    alarm.add_field(Field::new_int("requester_discord_id"));
    alarm.add_field(Field::new_int("channel_id"));
    alarm.add_field(Field::new_int("hour_of_day"));
    alarm.add_field(Field::new_int("day_of_week"));
    alarm.add_field(Field::new_datetime("created_at"));
    alarm.add_field(Field::new_string("message"));

    schema.add_model(alarm);

    builder.add_schema(schema);

    builder.build();
}