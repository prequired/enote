use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;

pub mod local_ai;
pub mod cloud_ai;
pub mod enhancer;

pub use enhancer::NoteEnhancer;
pub use local_ai::LocalAI;
pub use cloud_ai::CloudAI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementRequest {
    pub content: String,
    pub enhancement_types: Vec<EnhancementType>,
    pub style: WritingStyle,
    pub sensitivity: Sensitivity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnhancementType {
    Clarity,
    Structure,
    Tags,
    Summarization,
    Grammar,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WritingStyle {
    Formal,
    Casual,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Sensitivity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementResponse {
    pub original_content: String,
    pub enhanced_content: String,
    pub suggestions: Vec<Suggestion>,
    pub confidence: f32,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub suggestion_type: EnhancementType,
    pub description: String,
    pub before: String,
    pub after: String,
    pub confidence: f32,
    pub applied: bool,
}

pub trait AIProvider: Send + Sync {
    async fn enhance_text(&mut self, request: &EnhancementRequest) -> Result<EnhancementResponse>;
    fn is_available(&self) -> bool;
    fn get_name(&self) -> &str;
}

pub struct AIManager {
    local_ai: Option<LocalAI>,
    cloud_ai: Option<CloudAI>,
    current_provider: AIProviderType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AIProviderType {
    Local,
    Cloud,
}

impl AIManager {
    pub async fn new() -> Result<Self> {
        let local_ai = LocalAI::new().await.ok();
        let cloud_ai = CloudAI::new().await.ok();
        
        let current_provider = if local_ai.is_some() {
            AIProviderType::Local
        } else if cloud_ai.is_some() {
            AIProviderType::Cloud
        } else {
            AIProviderType::Local // Default fallback
        };
        
        Ok(Self {
            local_ai,
            cloud_ai,
            current_provider,
        })
    }
    
    pub fn set_provider(&mut self, provider: AIProviderType) {
        self.current_provider = provider;
    }
    
    pub fn get_current_provider(&self) -> AIProviderType {
        self.current_provider.clone()
    }
    
    pub fn is_provider_available(&self, provider: AIProviderType) -> bool {
        match provider {
            AIProviderType::Local => self.local_ai.as_ref().map_or(false, |ai| ai.is_available()),
            AIProviderType::Cloud => self.cloud_ai.as_ref().map_or(false, |ai| ai.is_available()),
        }
    }
    
    pub async fn enhance_text(&self, request: &EnhancementRequest) -> Result<EnhancementResponse> {
        match self.current_provider {
            AIProviderType::Local => {
                if let Some(local_ai) = &self.local_ai {
                    local_ai.enhance_text(request).await
                } else {
                    self.fallback_enhancement(request).await
                }
            }
            AIProviderType::Cloud => {
                if let Some(cloud_ai) = &self.cloud_ai {
                    cloud_ai.enhance_text(request).await
                } else if let Some(local_ai) = &self.local_ai {
                    local_ai.enhance_text(request).await
                } else {
                    self.fallback_enhancement(request).await
                }
            }
        }
    }
    
    async fn fallback_enhancement(&self, request: &EnhancementRequest) -> Result<EnhancementResponse> {
        // Simple rule-based enhancement when no AI is available
        let start_time = std::time::Instant::now();
        let mut enhanced_content = request.content.clone();
        let mut suggestions = Vec::new();
        
        // Apply basic enhancements
        if request.enhancement_types.contains(&EnhancementType::Structure) {
            enhanced_content = self.add_basic_structure(&enhanced_content);
            suggestions.push(Suggestion {
                suggestion_type: EnhancementType::Structure,
                description: "Added basic markdown structure".to_string(),
                before: request.content.clone(),
                after: enhanced_content.clone(),
                confidence: 0.7,
                applied: true,
            });
        }
        
        if request.enhancement_types.contains(&EnhancementType::Grammar) {
            enhanced_content = self.basic_grammar_fixes(&enhanced_content);
            suggestions.push(Suggestion {
                suggestion_type: EnhancementType::Grammar,
                description: "Applied basic grammar fixes".to_string(),
                before: request.content.clone(),
                after: enhanced_content.clone(),
                confidence: 0.6,
                applied: true,
            });
        }
        
        if request.enhancement_types.contains(&EnhancementType::Tags) {
            let tags = self.suggest_basic_tags(&enhanced_content);
            if !tags.is_empty() {
                enhanced_content.push_str(&format!("\n\nSuggested tags: {}", tags.join(", ")));
                suggestions.push(Suggestion {
                    suggestion_type: EnhancementType::Tags,
                    description: "Suggested basic tags".to_string(),
                    before: request.content.clone(),
                    after: enhanced_content.clone(),
                    confidence: 0.5,
                    applied: true,
                });
            }
        }
        
        Ok(EnhancementResponse {
            original_content: request.content.clone(),
            enhanced_content,
            suggestions,
            confidence: 0.6,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }
    
    fn add_basic_structure(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return content.to_string();
        }
        
        let mut result = String::new();
        
        // Add a title if the first line doesn't look like one
        if !lines[0].starts_with('#') && !lines[0].is_empty() {
            result.push_str(&format!("# {}\n\n", lines[0]));
            if lines.len() > 1 {
                result.push_str(&lines[1..].join("\n"));
            }
        } else {
            result = content.to_string();
        }
        
        // Add bullet points for list-like content
        let mut enhanced_lines = Vec::new();
        for line in result.lines() {
            if line.trim().starts_with("- ") || line.trim().starts_with("* ") || line.starts_with('#') {
                enhanced_lines.push(line.to_string());
            } else if line.trim().is_empty() {
                enhanced_lines.push(line.to_string());
            } else if line.contains(',') && !line.contains('.') {
                // Might be a list
                enhanced_lines.push(format!("- {}", line.trim()));
            } else {
                enhanced_lines.push(line.to_string());
            }
        }
        
        enhanced_lines.join("\n")
    }
    
    fn basic_grammar_fixes(&self, content: &str) -> String {
        let mut result = content.to_string();
        
        // Basic fixes
        result = result.replace(" i ", " I ");
        result = result.replace(" i'", " I'");
        result = result.replace("  ", " "); // Remove double spaces
        result = result.replace(" .", ".");
        result = result.replace(" ,", ",");
        result = result.replace(" !", "!");
        result = result.replace(" ?", "?");
        
        // Capitalize first letter of sentences
        let sentences: Vec<&str> = result.split(". ").collect();
        let mut capitalized = Vec::new();
        
        for sentence in sentences {
            if let Some(first_char) = sentence.chars().next() {
                let rest: String = sentence.chars().skip(1).collect();
                capitalized.push(format!("{}{}", first_char.to_uppercase(), rest));
            } else {
                capitalized.push(sentence.to_string());
            }
        }
        
        capitalized.join(". ")
    }
    
    fn suggest_basic_tags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let content_lower = content.to_lowercase();
        
        // Basic keyword detection
        let keywords = [
            ("meeting", "meeting"),
            ("project", "project"),
            ("task", "task"),
            ("idea", "idea"),
            ("note", "note"),
            ("todo", "todo"),
            ("plan", "planning"),
            ("research", "research"),
            ("work", "work"),
            ("personal", "personal"),
        ];
        
        for (keyword, tag) in &keywords {
            if content_lower.contains(keyword) {
                tags.push(format!("#{}", tag));
            }
        }
        
        tags.dedup();
        tags.truncate(5); // Limit to 5 tags
        tags
    }
}