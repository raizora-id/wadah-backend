use chrono::Utc;
use uuid::Uuid;

use crate::api::notification::{BatchNotificationDto, NotificationDto};
use crate::channels::{email::EmailChannel, sms::SmsChannel, push::PushChannel, inapp::InAppChannel};
use crate::config::Config;
use shared::{
    DatabaseConnection, RedisConnection,
    utils::error::AppError,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub recipient_id: Uuid,
    pub template_id: Option<Uuid>,
    pub channel: String,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub status: NotificationStatus,
    pub scheduled_at: Option<chrono::DateTime<Utc>>,
    pub sent_at: Option<chrono::DateTime<Utc>>,
    pub read_at: Option<chrono::DateTime<Utc>>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
    Read,
    Scheduled,
}

pub struct NotificationService {
    db_conn: DatabaseConnection,
    redis_conn: RedisConnection,
    config: Config,
}

impl NotificationService {
    pub fn new(db_conn: DatabaseConnection, redis_conn: RedisConnection, config: Config) -> Self {
        Self { db_conn, redis_conn, config }
    }
    
    pub async fn list_notifications(&self, tenant_id: Uuid, recipient_id: Uuid) -> Result<Vec<Notification>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock notifications for testing
        let notifications = vec![
            Notification {
                id: Uuid::new_v4(),
                tenant_id,
                recipient_id,
                template_id: None,
                channel: "email".to_string(),
                title: "Welcome to Klola".to_string(),
                body: "Thank you for joining Klola! We're excited to have you on board.".to_string(),
                data: None,
                status: NotificationStatus::Sent,
                scheduled_at: None,
                sent_at: Some(Utc::now()),
                read_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Notification {
                id: Uuid::new_v4(),
                tenant_id,
                recipient_id,
                template_id: None,
                channel: "inapp".to_string(),
                title: "New feature available".to_string(),
                body: "Check out our new dashboard feature!".to_string(),
                data: None,
                status: NotificationStatus::Read,
                scheduled_at: None,
                sent_at: Some(Utc::now()),
                read_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        Ok(notifications)
    }
    
    pub async fn send_notification(&self, tenant_id: Uuid, req: &NotificationDto) -> Result<Notification, AppError> {
        // In a real implementation, this would insert into the database and send via the appropriate channel
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate inputs
        if req.title.is_empty() || req.body.is_empty() {
            return Err(AppError::ValidationError("Title and body are required".to_string()));
        }
        
        // Validate channel
        if !["email", "sms", "push", "inapp"].contains(&req.channel.as_str()) {
            return Err(AppError::ValidationError(format!("Invalid channel: {}", req.channel)));
        }
        
        // Create notification
        let notification = Notification {
            id: req.id.unwrap_or_else(Uuid::new_v4),
            tenant_id,
            recipient_id: req.recipient_id,
            template_id: req.template_id,
            channel: req.channel.clone(),
            title: req.title.clone(),
            body: req.body.clone(),
            data: req.data.clone(),
            status: if req.scheduled_at.is_some() {
                NotificationStatus::Scheduled
            } else {
                NotificationStatus::Pending
            },
            scheduled_at: req.scheduled_at,
            sent_at: None,
            read_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Send notification via appropriate channel if not scheduled
        if notification.scheduled_at.is_none() {
            match notification.channel.as_str() {
                "email" => {
                    let email_channel = EmailChannel::new(&self.config.email);
                    email_channel.send(&notification).await?;
                },
                "sms" => {
                    let sms_channel = SmsChannel::new(&self.config.sms);
                    sms_channel.send(&notification).await?;
                },
                "push" => {
                    let push_channel = PushChannel::new();
                    push_channel.send(&notification).await?;
                },
                "inapp" => {
                    let inapp_channel = InAppChannel::new(self.redis_conn.clone());
                    inapp_channel.send(&notification).await?;
                },
                _ => {}
            }
        } else {
            // Add to scheduled queue
            // In a real implementation, this would add to a scheduler
        }
        
        Ok(notification)
    }
    
    pub async fn get_notification(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Notification>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock notification for testing
        let notification = Notification {
            id,
            tenant_id,
            recipient_id: Uuid::new_v4(),
            template_id: None,
            channel: "email".to_string(),
            title: "Welcome to Klola".to_string(),
            body: "Thank you for joining Klola! We're excited to have you on board.".to_string(),
            data: None,
            status: NotificationStatus::Sent,
            scheduled_at: None,
            sent_at: Some(Utc::now()),
            read_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(Some(notification))
    }
    
    pub async fn mark_as_read(&self, tenant_id: Uuid, id: Uuid) -> Result<bool, AppError> {
        // In a real implementation, this would update the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if notification exists
        // In a real implementation, this would query the database
        
        // Mark notification as read
        
        Ok(true)
    }
    
    pub async fn send_batch_notifications(&self, tenant_id: Uuid, req: &BatchNotificationDto) -> Result<Uuid, AppError> {
        // In a real implementation, this would create a batch job and enqueue it
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate inputs
        if req.title.is_empty() || req.body.is_empty() {
            return Err(AppError::ValidationError("Title and body are required".to_string()));
        }
        
        if req.recipient_ids.is_empty() {
            return Err(AppError::ValidationError("Recipients are required".to_string()));
        }
        
        // Validate channel
        if !["email", "sms", "push", "inapp"].contains(&req.channel.as_str()) {
            return Err(AppError::ValidationError(format!("Invalid channel: {}", req.channel)));
        }
        
        // Create batch ID
        let batch_id = Uuid::new_v4();
        
        // In a real implementation, this would create individual notifications and queue them
        // for processing in the background
        
        Ok(batch_id)
    }
}