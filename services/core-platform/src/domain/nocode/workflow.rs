use chrono::Utc;
use uuid::Uuid;

use shared::{
    DatabaseConnection,
    models::entity::{WorkflowDefinition, WorkflowStep},
    utils::error::AppError,
};

use super::schema::SchemaService;

pub struct WorkflowService {
    db_conn: DatabaseConnection,
}

impl WorkflowService {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    
    pub async fn list_workflow_definitions(&self, tenant_id: Uuid, entity_id: Option<&str>) -> Result<Vec<WorkflowDefinition>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock workflow definitions for testing
        let definitions = vec![
            WorkflowDefinition {
                id: Uuid::new_v4(),
                tenant_id,
                name: "Customer Onboarding".to_string(),
                description: Some("Workflow for new customer onboarding".to_string()),
                entity_id: Some("customer".to_string()),
                trigger_type: "entity.created".to_string(),
                trigger_config: serde_json::json!({
                    "entity": "customer"
                }),
                steps: vec![
                    WorkflowStep {
                        id: "send_welcome_email".to_string(),
                        name: "Send Welcome Email".to_string(),
                        step_type: "notification.email".to_string(),
                        config: serde_json::json!({
                            "template_id": "welcome_email",
                            "recipient_field": "email"
                        }),
                        next_steps: vec!["create_task".to_string()],
                        condition: None,
                    },
                    WorkflowStep {
                        id: "create_task".to_string(),
                        name: "Create Follow-up Task".to_string(),
                        step_type: "task.create".to_string(),
                        config: serde_json::json!({
                            "title": "Follow up with {{name}}",
                            "description": "New customer onboarding follow-up",
                            "due_date": "{{now|add_days:3}}"
                        }),
                        next_steps: vec![],
                        condition: None,
                    },
                ],
                status: "active".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            WorkflowDefinition {
                id: Uuid::new_v4(),
                tenant_id,
                name: "Order Processing".to_string(),
                description: Some("Workflow for processing new orders".to_string()),
                entity_id: Some("order".to_string()),
                trigger_type: "entity.created".to_string(),
                trigger_config: serde_json::json!({
                    "entity": "order"
                }),
                steps: vec![
                    WorkflowStep {
                        id: "check_inventory".to_string(),
                        name: "Check Inventory".to_string(),
                        step_type: "script.execute".to_string(),
                        config: serde_json::json!({
                            "script": "checkInventory"
                        }),
                        next_steps: vec!["send_confirmation".to_string(), "send_backorder".to_string()],
                        condition: Some("result.inStock == true ? 'send_confirmation' : 'send_backorder'".to_string()),
                    },
                    WorkflowStep {
                        id: "send_confirmation".to_string(),
                        name: "Send Order Confirmation".to_string(),
                        step_type: "notification.email".to_string(),
                        config: serde_json::json!({
                            "template_id": "order_confirmation",
                            "recipient_field": "customer.email"
                        }),
                        next_steps: vec![],
                        condition: None,
                    },
                    WorkflowStep {
                        id: "send_backorder".to_string(),
                        name: "Send Backorder Notification".to_string(),
                        step_type: "notification.email".to_string(),
                        config: serde_json::json!({
                            "template_id": "backorder_notification",
                            "recipient_field": "customer.email"
                        }),
                        next_steps: vec![],
                        condition: None,
                    },
                ],
                status: "active".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        // Filter by entity_id if provided
        let filtered_definitions = if let Some(eid) = entity_id {
            definitions.into_iter().filter(|d| d.entity_id.as_ref().map_or(false, |e| e == eid)).collect()
        } else {
            definitions
        };
        
        Ok(filtered_definitions)
    }
    
    pub async fn create_workflow_definition(&self, tenant_id: Uuid, req: &serde_json::Value) -> Result<WorkflowDefinition, AppError> {
        // In a real implementation, this would insert into the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Parse request
        let name = req.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("Name is required".to_string()))?;
            
        let description = req.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let entity_id = req.get("entity_id").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let trigger_type = req.get("trigger_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("Trigger type is required".to_string()))?;
            
        let trigger_config = req.get("trigger_config")
            .ok_or_else(|| AppError::ValidationError("Trigger config is required".to_string()))?;
            
        let steps = req.get("steps")
            .and_then(|v| v.as_array())
            .ok_or_else(|| AppError::ValidationError("Steps are required".to_string()))?;
            
        // Validate steps
        if steps.is_empty() {
            return Err(AppError::ValidationError("At least one step is required".to_string()));
        }
        
        // Validate entity if provided
        if let Some(eid) = &entity_id {
            let schema_service = SchemaService::new(self.db_conn.clone());
            let entity = schema_service.get_entity(tenant_id, eid).await?;
            
            if entity.is_none() {
                return Err(AppError::ValidationError(format!("Entity not found: {}", eid)));
            }
        }
        
        // Create workflow definition
        let steps_vec: Vec<WorkflowStep> = serde_json::from_value(serde_json::json!(steps))
            .map_err(|e| AppError::ValidationError(format!("Invalid steps: {}", e)))?;
            
        let workflow = WorkflowDefinition {
            id: Uuid::new_v4(),
            tenant_id,
            name: name.to_string(),
            description,
            entity_id,
            trigger_type: trigger_type.to_string(),
            trigger_config: trigger_config.clone(),
            steps: steps_vec,
            status: "active".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(workflow)
    }
    
    pub async fn get_workflow_definition(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<WorkflowDefinition>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock workflow definition for testing
        let workflow = WorkflowDefinition {
            id,
            tenant_id,
            name: "Customer Onboarding".to_string(),
            description: Some("Workflow for new customer onboarding".to_string()),
            entity_id: Some("customer".to_string()),
            trigger_type: "entity.created".to_string(),
            trigger_config: serde_json::json!({
                "entity": "customer"
            }),
            steps: vec![
                WorkflowStep {
                    id: "send_welcome_email".to_string(),
                    name: "Send Welcome Email".to_string(),
                    step_type: "notification.email".to_string(),
                    config: serde_json::json!({
                        "template_id": "welcome_email",
                        "recipient_field": "email"
                    }),
                    next_steps: vec!["create_task".to_string()],
                    condition: None,
                },
                WorkflowStep {
                    id: "create_task".to_string(),
                    name: "Create Follow-up Task".to_string(),
                    step_type: "task.create".to_string(),
                    config: serde_json::json!({
                        "title": "Follow up with {{name}}",
                        "description": "New customer onboarding follow-up",
                        "due_date": "{{now|add_days:3}}"
                    }),
                    next_steps: vec![],
                    condition: None,
                },
            ],
            status: "active".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(Some(workflow))
    }
    
    pub async fn update_workflow_definition(&self, tenant_id: Uuid, id: Uuid, req: &serde_json::Value) -> Result<Option<WorkflowDefinition>, AppError> {
        // In a real implementation, this would update the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if workflow definition exists
        let existing_workflow = self.get_workflow_definition(tenant_id, id).await?;
        
        if existing_workflow.is_none() {
            return Ok(None);
        }
        
        let mut workflow = existing_workflow.unwrap();
        
        // Update fields
        if let Some(name) = req.get("name").and_then(|v| v.as_str()) {
            workflow.name = name.to_string();
        }
        
        if req.get("description").is_some() {
            workflow.description = req.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        }
        
        if req.get("entity_id").is_some() {
            workflow.entity_id = req.get("entity_id").and_then(|v| v.as_str()).map(|s| s.to_string());
            
            // Validate entity if provided
            if let Some(eid) = &workflow.entity_id {
                let schema_service = SchemaService::new(self.db_conn.clone());
                let entity = schema_service.get_entity(tenant_id, eid).await?;
                
                if entity.is_none() {
                    return Err(AppError::ValidationError(format!("Entity not found: {}", eid)));
                }
            }
        }
        
        if let Some(trigger_type) = req.get("trigger_type").and_then(|v| v.as_str()) {
            workflow.trigger_type = trigger_type.to_string();
        }
        
        if let Some(trigger_config) = req.get("trigger_config") {
            workflow.trigger_config = trigger_config.clone();
        }
        
        if let Some(steps) = req.get("steps") {
            if let Some(steps_array) = steps.as_array() {
                if steps_array.is_empty() {
                    return Err(AppError::ValidationError("At least one step is required".to_string()));
                }
                
                workflow.steps = serde_json::from_value(serde_json::json!(steps_array))
                    .map_err(|e| AppError::ValidationError(format!("Invalid steps: {}", e)))?;
            } else {
                return Err(AppError::ValidationError("Steps must be an array".to_string()));
            }
        }
        
        if let Some(status) = req.get("status").and_then(|v| v.as_str()) {
            workflow.status = status.to_string();
        }
        
        workflow.updated_at = Utc::now();
        
        Ok(Some(workflow))
    }
    
    pub async fn delete_workflow_definition(&self, tenant_id: Uuid, id: Uuid) -> Result<bool, AppError> {
        // In a real implementation, this would delete from the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if workflow definition exists
        let existing_workflow = self.get_workflow_definition(tenant_id, id).await?;
        
        if existing_workflow.is_none() {
            return Ok(false);
        }
        
        // Delete workflow definition
        
        Ok(true)
    }
    
    pub async fn execute_workflow(&self, tenant_id: Uuid, id: Uuid, data: &serde_json::Value) -> Result<serde_json::Value, AppError> {
        // In a real implementation, this would execute the workflow
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Get workflow definition
        let workflow = self.get_workflow_definition(tenant_id, id).await?;
        
        if workflow.is_none() {
            return Err(AppError::ValidationError(format!("Workflow not found: {}", id)));
        }
        
        let workflow = workflow.unwrap();
        
        // Validate data
        if !data.is_object() {
            return Err(AppError::ValidationError("Data must be an object".to_string()));
        }
        
        // In a real implementation, this would execute the workflow steps
        // based on the workflow definition and the input data
        
        // Mock execution result
        let result = serde_json::json!({
            "workflow_id": workflow.id,
            "status": "completed",
            "steps": workflow.steps.iter().map(|step| {
                serde_json::json!({
                    "step_id": step.id,
                    "status": "completed",
                    "start_time": Utc::now(),
                    "end_time": Utc::now(),
                    "output": {}
                })
            }).collect::<Vec<_>>()
        });
        
        Ok(result)
    }
}