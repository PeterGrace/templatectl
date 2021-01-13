use crate::file::read_file;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct TemplateList {
    pub(crate) templates: Vec<TemplateObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct TemplateObject {
    pub(crate) name: String,
    pub(crate) filename: String,
    #[serde(rename = "iconCode")]
    pub(crate) icon_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) landscape: Option<Value>,
    pub(crate) categories: Vec<String>,
}

impl TemplateList {
    pub(crate) fn new(filepath: &str) -> Result<TemplateList> {
        let obj: TemplateList = serde_json::from_str(read_file(filepath)?.as_str()).unwrap();
        Ok(obj)
    }
}
