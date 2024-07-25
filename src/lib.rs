use serde::{Deserialize, Serialize};

/// Defines the type of input and its intial fields
#[derive(Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "input_type")]
pub enum InputType {
    Text,
    Expression,
    Number {
        min: Option<isize>,
        max: Option<isize>,
    },
    Dropdown {
        options: Vec<(String, String)>,
    },
}

#[derive(Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "type")]
pub enum InputFieldElement {
    Label {
        text: String,
    },
    Input {
        #[serde(flatten)]
        input: InputType,
        name: Option<String>,
        placeholder: Option<String>,
        default: Option<String>,
    },
}

#[derive(Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct InputField {
    #[serde(flatten)]
    r#type: InputFieldElement,
}
