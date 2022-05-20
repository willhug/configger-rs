use configger::{ConfiggerData, ConfiggerField};

// #[derive(ConfiggerData)]
// struct SchemaDef {
//     #[configger(require_on_create)]
//     name: String,
//     #[configger(require_on_create)]
//     desc: String,
//     database_type: String,

//     #[configger(list_def)]
//     models: Vec<ModelDef>,
// }

// #[derive(ConfiggerData)]
// struct ModelDef {
//     #[configger(require_on_create)]
//     name: String,
//     #[configger(require_on_create)]
//     desc: String,

//     #[configger(list_def)]
//     fields: Vec<FieldDef>,
// }

#[derive(ConfiggerData)]
struct FieldDef {
    #[configger(require_on_create)]
    name: String,
    #[configger(require_on_create)]
    desc: String,
    proto_number: u32,
}

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

// #[derive(ConfiggerData)]
// #[configger(extends(FieldDef), forced_extension)]
// struct NullableFieldDef {
//     nullable: bool,
// }

// Definitions:
// extends -> Extend the FieldDef struct with these fields
// forced_extension -> Disallow plugins that _dont_ depend on Nullable since we could get wrong functionality.
// require_on_create -> The constructor for this type should include this as a parameter

pub fn demo() {
    // TODO
    // let schema_bundle = configger::data_bundle!(SchemaDef);
    // let nullable_schema_bundle = configger::data_bundle!(NullableFieldDef, schema_bundle);

    // let plugin = configger::plugin!(DemoPlugin, nullable_schema_bundle)

    // builder = configger::builder!(nullable_schema_bundle, vec![plugin]);
}
