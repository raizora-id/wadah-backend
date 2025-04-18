use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// No-code schema builder models

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityDefinition {
    pub id: String,
    pub tenant_id: Uuid,
    pub product_id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub schema: EntitySchema,
    pub ui_schema: UiSchema,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntitySchema {
    pub fields: Vec<FieldDefinition>,
    pub indexes: Vec<IndexDefinition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub display_name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub unique: bool,
    pub default_value: Option<serde_json::Value>,
    pub validators: Vec<Validator>,
    pub ui_config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Date,
    DateTime,
    Email,
    Phone,
    Url,
    Select,
    MultiSelect,
    Reference,
    File,
    Image,
    Json,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexDefinition {
    pub name: String,
    pub fields: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Validator {
    pub name: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiSchema {
    pub layout: String,
    pub elements: Vec<UiElement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiElement {
    pub field: String,
    pub element_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiDefinition {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub entity_id: String,
    pub view_type: ViewType,
    pub definition: UiDefinitionSchema,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ViewType {
    List,
    Detail,
    Form,
    Dashboard,
    Custom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiDefinitionSchema {
    pub layout: Layout,
    pub components: Vec<ComponentDefinition>,
    pub actions: Vec<ActionDefinition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layout {
    pub type_: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentDefinition {
    pub id: String,
    pub component_type: String,
    pub field: Option<String>,
    pub config: serde_json::Value,
    pub children: Option<Vec<ComponentDefinition>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionDefinition {
    pub id: String,
    pub name: String,
    pub action_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkflowDefinition {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub entity_id: Option<String>,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub steps: Vec<WorkflowStep>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub step_type: String,
    pub config: serde_json::Value,
    pub next_steps: Vec<String>,
    pub condition: Option<String>,
}
