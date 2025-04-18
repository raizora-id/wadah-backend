use chrono::Utc;
use uuid::Uuid;

use shared::{
    DatabaseConnection,
    models::entity::{UiDefinition, UiDefinitionSchema, ViewType},
    utils::error::AppError,
};

use super::schema::SchemaService;

pub struct UiService {
    db_conn: DatabaseConnection,
}

impl UiService {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    
    pub async fn list_ui_definitions(&self, tenant_id: Uuid, entity_id: Option<&str>) -> Result<Vec<UiDefinition>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock UI definitions for testing
        let definitions = vec![
            UiDefinition {
                id: Uuid::new_v4(),
                tenant_id,
                entity_id: "customer".to_string(),
                view_type: ViewType::List,
                definition: serde_json::from_value(serde_json::json!({
                    "layout": {
                        "type_": "table",
                        "config": {
                            "pageSize": 10,
                            "columns": ["name", "email"]
                        }
                    },
                    "components": [
                        {
                            "id": "name_col",
                            "component_type": "text",
                            "field": "name",
                            "config": {
                                "label": "Name"
                            },
                            "children": null
                        },
                        {
                            "id": "email_col",
                            "component_type": "text",
                            "field": "email",
                            "config": {
                                "label": "Email"
                            },
                            "children": null
                        }
                    ],
                    "actions": [
                        {
                            "id": "create",
                            "name": "Create",
                            "action_type": "navigate",
                            "config": {
                                "url": "/customers/new"
                            }
                        }
                    ]
                })).unwrap(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            UiDefinition {
                id: Uuid::new_v4(),
                tenant_id,
                entity_id: "customer".to_string(),
                view_type: ViewType::Form,
                definition: serde_json::from_value(serde_json::json!({
                    "layout": {
                        "type_": "form",
                        "config": {
                            "columns": 1
                        }
                    },
                    "components": [
                        {
                            "id": "name_field",
                            "component_type": "text_input",
                            "field": "name",
                            "config": {
                                "label": "Name",
                                "placeholder": "Enter name"
                            },
                            "children": null
                        },
                        {
                            "id": "email_field",
                            "component_type": "email_input",
                            "field": "email",
                            "config": {
                                "label": "Email",
                                "placeholder": "Enter email"
                            },
                            "children": null
                        }
                    ],
                    "actions": [
                        {
                            "id": "save",
                            "name": "Save",
                            "action_type": "submit",
                            "config": {}
                        },
                        {
                            "id": "cancel",
                            "name": "Cancel",
                            "action_type": "navigate",
                            "config": {
                                "url": "/customers"
                            }
                        }
                    ]
                })).unwrap(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        // Filter by entity_id if provided
        let filtered_definitions = if let Some(eid) = entity_id {
            definitions.into_iter().filter(|d| d.entity_id == eid).collect()
        } else {
            definitions
        };
        
        Ok(filtered_definitions)
    }
    
    pub async fn create_ui_definition(&self, tenant_id: Uuid, req: &serde_json::Value) -> Result<UiDefinition, AppError> {
        // In a real implementation, this would insert into the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Parse request
        let entity_id = req.get("entity_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("Entity ID is required".to_string()))?;
            
        let view_type_str = req.get("view_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("View type is required".to_string()))?;
            
        let view_type = match view_type_str {
            "list" => ViewType::List,
            "detail" => ViewType::Detail,
            "form" => ViewType::Form,
            "dashboard" => ViewType::Dashboard,
            "custom" => ViewType::Custom,
            _ => return Err(AppError::ValidationError(format!("Invalid view type: {}", view_type_str))),
        };
        
        let definition = req.get("definition")
            .ok_or_else(|| AppError::ValidationError("Definition is required".to_string()))?;
            
        // Validate that the entity exists
        let schema_service = SchemaService::new(self.db_conn.clone());
        let entity = schema_service.get_entity(tenant_id, entity_id).await?;
        
        if entity.is_none() {
            return Err(AppError::ValidationError(format!("Entity not found: {}", entity_id)));
        }
        
        // Validate definition (in a real implementation, this would be more comprehensive)
        if !definition.is_object() {
            return Err(AppError::ValidationError("Definition must be an object".to_string()));
        }
        
        // Create UI definition
        let definition_obj: UiDefinitionSchema = serde_json::from_value(definition.clone())
            .map_err(|e| AppError::ValidationError(format!("Invalid definition: {}", e)))?;
            
        let ui_definition = UiDefinition {
            id: Uuid::new_v4(),
            tenant_id,
            entity_id: entity_id.to_string(),
            view_type,
            definition: definition_obj,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(ui_definition)
    }
    
    pub async fn get_ui_definition(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<UiDefinition>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock UI definition for testing
        let ui_definition = UiDefinition {
            id,
            tenant_id,
            entity_id: "customer".to_string(),
            view_type: ViewType::List,
            definition: serde_json::from_value(serde_json::json!({
                "layout": {
                    "type_": "table",
                    "config": {
                        "pageSize": 10,
                        "columns": ["name", "email"]
                    }
                },
                "components": [
                    {
                        "id": "name_col",
                        "component_type": "text",
                        "field": "name",
                        "config": {
                            "label": "Name"
                        },
                        "children": null
                    },
                    {
                        "id": "email_col",
                        "component_type": "text",
                        "field": "email",
                        "config": {
                            "label": "Email"
                        },
                        "children": null
                    }
                ],
                "actions": [
                    {
                        "id": "create",
                        "name": "Create",
                        "action_type": "navigate",
                        "config": {
                            "url": "/customers/new"
                        }
                    }
                ]
            })).unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(Some(ui_definition))
    }
    
    pub async fn update_ui_definition(&self, tenant_id: Uuid, id: Uuid, req: &serde_json::Value) -> Result<Option<UiDefinition>, AppError> {
        // In a real implementation, this would update the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if UI definition exists
        let existing_definition = self.get_ui_definition(tenant_id, id).await?;
        
        if existing_definition.is_none() {
            return Ok(None);
        }
        
        let mut ui_definition = existing_definition.unwrap();
        
        // Update fields
        if let Some(view_type_str) = req.get("view_type").and_then(|v| v.as_str()) {
            ui_definition.view_type = match view_type_str {
                "list" => ViewType::List,
                "detail" => ViewType::Detail,
                "form" => ViewType::Form,
                "dashboard" => ViewType::Dashboard,
                "custom" => ViewType::Custom,
                _ => return Err(AppError::ValidationError(format!("Invalid view type: {}", view_type_str))),
            };
        }
        
        if let Some(definition) = req.get("definition") {
            if !definition.is_object() {
                return Err(AppError::ValidationError("Definition must be an object".to_string()));
            }
            
            ui_definition.definition = serde_json::from_value(definition.clone())
                .map_err(|e| AppError::ValidationError(format!("Invalid definition: {}", e)))?;
        }
        
        ui_definition.updated_at = Utc::now();
        
        Ok(Some(ui_definition))
    }
    
    pub async fn delete_ui_definition(&self, tenant_id: Uuid, id: Uuid) -> Result<bool, AppError> {
        // In a real implementation, this would delete from the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if UI definition exists
        let existing_definition = self.get_ui_definition(tenant_id, id).await?;
        
        if existing_definition.is_none() {
            return Ok(false);
        }
        
        // Delete UI definition
        
        Ok(true)
    }
    
    pub async fn render_ui(&self, tenant_id: Uuid, entity_id: &str, view_type: ViewType) -> Result<Option<serde_json::Value>, AppError> {
        // In a real implementation, this would query the database for the UI definition
        // and render it with data
        let conn = &mut self.db_conn.get_connection()?;
        
        // Find UI definition
        let definitions = self.list_ui_definitions(tenant_id, Some(entity_id)).await?;
        
        let ui_definition = definitions.into_iter().find(|d| d.view_type == view_type);
        
        if ui_definition.is_none() {
            return Ok(None);
        }
        
        let ui_definition = ui_definition.unwrap();
        
        // In a real implementation, this would fetch data for the entity
        // and merge it with the UI definition for rendering
        
        // For now, just return the definition
        Ok(Some(serde_json::to_value(ui_definition.definition).unwrap()))
    }
}