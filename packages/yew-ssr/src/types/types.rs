use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct SupportedResourcesRange {
    pub min_cpus: i32,
    pub min_gpus: i32,
    pub min_memory: i32,
    pub min_storage: i32,
    pub max_cpus: i32,
    pub max_gpus: i32,
    pub max_memory: i32,
    pub max_storage: i32,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Pin {
    pub uid: String,
    pub name: String,
    pub binding: i32,
    pub token_multiplicity: i32,
    pub data_multiplicity: i32,
    pub data_type_uid: String,
    pub data_type_name: String,
    pub data_structure_uid: Option<String>,
    pub data_structure_name: Option<String>,
    pub access_type_uid: Option<String>,
    pub access_type_name: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Release {
    pub version: String,
    pub date: String,
}

#[allow(non_snake_case)]
#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct AppType {
    pub shortDescription: Option<String>,
    pub icon: String,
    pub releases: Vec<Release>,
    pub inCockpit: Option<bool>,
    pub name: String,
    pub uid: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Unit {
    pub uid: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct ShelfType {
    pub uid: String,
    pub unit: Unit,
    // pub diagram_uid: String,
    // pub version: String,
    // pub status: i32,
    // pub date: String,
    // pub description: Option<String>,
    // pub open_source: bool,
    // pub usage_counter: i32,
}
