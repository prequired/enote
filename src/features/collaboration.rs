use anyhow::{Result, anyhow};
use operational_transform::{OperationSeq, Operation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeEdit {
    pub id: String,
    pub note_id: String,
    pub user_id: String,
    pub operation: TextOperation,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub revision: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOperation {
    pub ops: Vec<OpComponent>,
    pub base_length: usize,
    pub target_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpComponent {
    Retain(usize),
    Insert(String),
    Delete(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub note_id: String,
    pub participants: Vec<User>,
    pub current_revision: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub color: String, // Hex color for cursor/selection display
    pub cursor_position: Option<usize>,
    pub selection_range: Option<(usize, usize)>,
}

pub struct CollaborationManager {
    sessions: Arc<Mutex<HashMap<String, CollaborationSession>>>,
    pending_operations: Arc<Mutex<HashMap<String, Vec<CollaborativeEdit>>>>,
    websocket_url: String,
    user_id: String,
    user_name: String,
}

impl CollaborationManager {
    pub fn new(websocket_url: String, user_id: String, user_name: String) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            pending_operations: Arc::new(Mutex::new(HashMap::new())),
            websocket_url,
            user_id,
            user_name,
        }
    }
    
    pub async fn join_session(&mut self, note_id: &str) -> Result<mpsc::Receiver<CollaborativeEdit>> {
        let url = format!("{}/notes/{}/collaborate", self.websocket_url, note_id);
        let (ws_stream, _) = connect_async(&url).await?;
        
        let (tx, rx) = mpsc::channel(100);
        
        // Create user for this session
        let user = User {
            id: self.user_id.clone(),
            name: self.user_name.clone(),
            color: self.generate_user_color(),
            cursor_position: None,
            selection_range: None,
        };
        
        // Initialize session
        let session = CollaborationSession {
            note_id: note_id.to_string(),
            participants: vec![user],
            current_revision: 0,
            is_active: true,
        };
        
        self.sessions.lock().unwrap().insert(note_id.to_string(), session);
        
        // Spawn WebSocket handler
        let sessions_clone = self.sessions.clone();
        let pending_ops_clone = self.pending_operations.clone();
        let note_id_clone = note_id.to_string();
        
        tokio::spawn(async move {
            Self::handle_websocket_messages(ws_stream, tx, sessions_clone, pending_ops_clone, note_id_clone).await;
        });
        
        Ok(rx)
    }
    
    pub fn apply_local_operation(&mut self, note_id: &str, operation: TextOperation, content: &str) -> Result<String> {
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions.get_mut(note_id)
            .ok_or_else(|| anyhow!("No active collaboration session for note"))?;
        
        // Create collaborative edit
        let edit = CollaborativeEdit {
            id: Uuid::new_v4().to_string(),
            note_id: note_id.to_string(),
            user_id: self.user_id.clone(),
            operation: operation.clone(),
            timestamp: chrono::Utc::now(),
            revision: session.current_revision + 1,
        };
        
        // Apply operation locally
        let new_content = self.apply_operation(&operation, content)?;
        
        // Store pending operation for synchronization
        self.pending_operations.lock().unwrap()
            .entry(note_id.to_string())
            .or_insert_with(Vec::new)
            .push(edit);
        
        session.current_revision += 1;
        
        Ok(new_content)
    }
    
    pub fn apply_remote_operation(&self, edit: &CollaborativeEdit, content: &str) -> Result<String> {
        // Transform operation against pending local operations
        let pending_ops = self.pending_operations.lock().unwrap();
        let local_ops = pending_ops.get(&edit.note_id).unwrap_or(&Vec::new());
        
        let mut transformed_op = edit.operation.clone();
        
        // Apply operational transformation
        for local_edit in local_ops {
            if local_edit.revision > edit.revision {
                transformed_op = self.transform_operation(&transformed_op, &local_edit.operation)?;
            }
        }
        
        self.apply_operation(&transformed_op, content)
    }
    
    pub fn update_cursor_position(&mut self, note_id: &str, position: usize) -> Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions.get_mut(note_id)
            .ok_or_else(|| anyhow!("No active collaboration session"))?;
        
        if let Some(user) = session.participants.iter_mut()
            .find(|u| u.id == self.user_id) {
            user.cursor_position = Some(position);
        }
        
        Ok(())
    }
    
    pub fn update_selection(&mut self, note_id: &str, start: usize, end: usize) -> Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions.get_mut(note_id)
            .ok_or_else(|| anyhow!("No active collaboration session"))?;
        
        if let Some(user) = session.participants.iter_mut()
            .find(|u| u.id == self.user_id) {
            user.selection_range = Some((start, end));
        }
        
        Ok(())
    }
    
    pub fn get_session_info(&self, note_id: &str) -> Option<CollaborationSession> {
        self.sessions.lock().unwrap().get(note_id).cloned()
    }
    
    pub fn leave_session(&mut self, note_id: &str) -> Result<()> {
        self.sessions.lock().unwrap().remove(note_id);
        self.pending_operations.lock().unwrap().remove(note_id);
        Ok(())
    }
    
    fn apply_operation(&self, operation: &TextOperation, content: &str) -> Result<String> {
        let mut result = String::new();
        let mut content_chars: Vec<char> = content.chars().collect();
        let mut position = 0;
        
        for op in &operation.ops {
            match op {
                OpComponent::Retain(count) => {
                    let end = (position + count).min(content_chars.len());
                    result.extend(content_chars[position..end].iter());
                    position = end;
                }
                OpComponent::Insert(text) => {
                    result.push_str(text);
                }
                OpComponent::Delete(count) => {
                    position = (position + count).min(content_chars.len());
                }
            }
        }
        
        // Append remaining content
        if position < content_chars.len() {
            result.extend(content_chars[position..].iter());
        }
        
        Ok(result)
    }
    
    fn transform_operation(&self, op1: &TextOperation, op2: &TextOperation) -> Result<TextOperation> {
        // Simplified operational transformation
        // In production, use a more sophisticated OT algorithm
        let mut transformed_ops = Vec::new();
        let mut i1 = 0;
        let mut i2 = 0;
        let mut offset = 0i32;
        
        while i1 < op1.ops.len() && i2 < op2.ops.len() {
            match (&op1.ops[i1], &op2.ops[i2]) {
                (OpComponent::Retain(r1), OpComponent::Retain(r2)) => {
                    let min_retain = (*r1).min(*r2);
                    transformed_ops.push(OpComponent::Retain(min_retain));
                    
                    if r1 == r2 {
                        i1 += 1;
                        i2 += 1;
                    } else if r1 < r2 {
                        i1 += 1;
                        // Modify the second operation
                    } else {
                        i2 += 1;
                        // Modify the first operation
                    }
                }
                (OpComponent::Insert(text), _) => {
                    transformed_ops.push(OpComponent::Insert(text.clone()));
                    i1 += 1;
                    offset += text.len() as i32;
                }
                (_, OpComponent::Insert(text)) => {
                    transformed_ops.push(OpComponent::Retain(text.len()));
                    i2 += 1;
                    offset -= text.len() as i32;
                }
                (OpComponent::Delete(d1), OpComponent::Delete(d2)) => {
                    let min_delete = (*d1).min(*d2);
                    
                    if d1 == d2 {
                        i1 += 1;
                        i2 += 1;
                    } else if d1 < d2 {
                        i1 += 1;
                    } else {
                        i2 += 1;
                    }
                }
                _ => {
                    // Handle other combinations
                    i1 += 1;
                    i2 += 1;
                }
            }
        }
        
        // Add remaining operations
        while i1 < op1.ops.len() {
            transformed_ops.push(op1.ops[i1].clone());
            i1 += 1;
        }
        
        Ok(TextOperation {
            ops: transformed_ops,
            base_length: op1.base_length,
            target_length: (op1.target_length as i32 + offset) as usize,
        })
    }
    
    async fn handle_websocket_messages(
        ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tx: mpsc::Sender<CollaborativeEdit>,
        sessions: Arc<Mutex<HashMap<String, CollaborationSession>>>,
        pending_ops: Arc<Mutex<HashMap<String, Vec<CollaborativeEdit>>>>,
        note_id: String,
    ) {
        use futures_util::{SinkExt, StreamExt};
        
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(edit) = serde_json::from_str::<CollaborativeEdit>(&text) {
                        if edit.note_id == note_id {
                            // Update session with remote edit
                            if let Ok(mut sessions_guard) = sessions.lock() {
                                if let Some(session) = sessions_guard.get_mut(&note_id) {
                                    session.current_revision = session.current_revision.max(edit.revision);
                                }
                            }
                            
                            // Send to local handler
                            if tx.send(edit).await.is_err() {
                                break;
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(_) => break,
                _ => {}
            }
        }
    }
    
    fn generate_user_color(&self) -> String {
        // Generate a consistent color based on user ID
        let hash = md5::compute(&self.user_id);
        let hash_bytes = hash.0;
        
        // Use hash to generate HSL color with good saturation and lightness
        let hue = ((hash_bytes[0] as u16) * 360) / 255;
        let saturation = 70 + ((hash_bytes[1] as u16) * 30) / 255; // 70-100%
        let lightness = 40 + ((hash_bytes[2] as u16) * 20) / 255;  // 40-60%
        
        format!("hsl({}, {}%, {}%)", hue, saturation, lightness)
    }
}

// Helper functions for creating operations
impl TextOperation {
    pub fn new(base_length: usize) -> Self {
        Self {
            ops: Vec::new(),
            base_length,
            target_length: base_length,
        }
    }
    
    pub fn retain(&mut self, count: usize) -> &mut Self {
        if count > 0 {
            self.ops.push(OpComponent::Retain(count));
        }
        self
    }
    
    pub fn insert(&mut self, text: &str) -> &mut Self {
        if !text.is_empty() {
            self.ops.push(OpComponent::Insert(text.to_string()));
            self.target_length += text.len();
        }
        self
    }
    
    pub fn delete(&mut self, count: usize) -> &mut Self {
        if count > 0 {
            self.ops.push(OpComponent::Delete(count));
            self.target_length = self.target_length.saturating_sub(count);
        }
        self
    }
    
    pub fn from_text_change(old_text: &str, new_text: &str, cursor_pos: usize) -> Result<Self> {
        // Simple diff algorithm to generate operations
        let mut operation = TextOperation::new(old_text.len());
        
        // Find common prefix
        let common_prefix = old_text.chars()
            .zip(new_text.chars())
            .take_while(|(a, b)| a == b)
            .count();
        
        // Find common suffix
        let old_suffix = &old_text[common_prefix..];
        let new_suffix = &new_text[common_prefix..];
        
        let common_suffix = old_suffix.chars().rev()
            .zip(new_suffix.chars().rev())
            .take_while(|(a, b)| a == b)
            .count();
        
        // Calculate changes
        let old_middle_end = old_text.len() - common_suffix;
        let new_middle_end = new_text.len() - common_suffix;
        
        // Build operation
        if common_prefix > 0 {
            operation.retain(common_prefix);
        }
        
        let deleted_count = old_middle_end - common_prefix;
        let inserted_text = &new_text[common_prefix..new_middle_end];
        
        if deleted_count > 0 {
            operation.delete(deleted_count);
        }
        
        if !inserted_text.is_empty() {
            operation.insert(inserted_text);
        }
        
        if common_suffix > 0 {
            operation.retain(common_suffix);
        }
        
        Ok(operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_operation_creation() {
        let old_text = "Hello World";
        let new_text = "Hello Beautiful World";
        
        let operation = TextOperation::from_text_change(old_text, new_text, 6).unwrap();
        
        // Should retain "Hello ", insert "Beautiful ", retain "World"
        assert_eq!(operation.ops.len(), 3);
        assert_eq!(operation.base_length, 11);
        assert_eq!(operation.target_length, 21);
    }
    
    #[tokio::test]
    async fn test_collaboration_manager() {
        let manager = CollaborationManager::new(
            "ws://localhost:8080".to_string(),
            "user1".to_string(),
            "Test User".to_string(),
        );
        
        // Test would require a running WebSocket server
        assert_eq!(manager.user_id, "user1");
        assert_eq!(manager.user_name, "Test User");
    }
}