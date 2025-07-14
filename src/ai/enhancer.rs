use super::{AIManager, EnhancementRequest, EnhancementResponse, EnhancementType, WritingStyle, Sensitivity, AIProviderType};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NoteEnhancer {
    ai_manager: Arc<Mutex<AIManager>>,
    settings: EnhancerSettings,
}

#[derive(Debug, Clone)]
pub struct EnhancerSettings {
    pub enabled_types: Vec<EnhancementType>,
    pub default_style: WritingStyle,
    pub default_sensitivity: Sensitivity,
    pub preferred_provider: AIProviderType,
    pub auto_apply_suggestions: bool,
    pub confidence_threshold: f32,
}

impl Default for EnhancerSettings {
    fn default() -> Self {
        Self {
            enabled_types: vec![
                EnhancementType::Clarity,
                EnhancementType::Structure,
            ],
            default_style: WritingStyle::Casual,
            default_sensitivity: Sensitivity::Medium,
            preferred_provider: AIProviderType::Local,
            auto_apply_suggestions: false,
            confidence_threshold: 0.7,
        }
    }
}

impl NoteEnhancer {
    pub async fn new() -> Result<Self> {
        let ai_manager = Arc::new(Mutex::new(AIManager::new().await?));
        let settings = EnhancerSettings::default();
        
        Ok(Self {
            ai_manager,
            settings,
        })
    }
    
    pub async fn enhance_note(&self, content: &str) -> Result<EnhancementResponse> {
        let request = EnhancementRequest {
            content: content.to_string(),
            enhancement_types: self.settings.enabled_types.clone(),
            style: self.settings.default_style.clone(),
            sensitivity: self.settings.default_sensitivity.clone(),
        };
        
        self.enhance_with_request(&request).await
    }
    
    pub async fn enhance_with_request(&self, request: &EnhancementRequest) -> Result<EnhancementResponse> {
        let ai_manager = self.ai_manager.lock().await;
        
        // Set preferred provider if available
        if ai_manager.is_provider_available(self.settings.preferred_provider.clone()) {
            let mut ai_manager_mut = ai_manager;
            ai_manager_mut.set_provider(self.settings.preferred_provider.clone());
            ai_manager_mut.enhance_text(request).await
        } else {
            ai_manager.enhance_text(request).await
        }
    }
    
    pub async fn enhance_with_custom_settings(
        &self,
        content: &str,
        types: Vec<EnhancementType>,
        style: WritingStyle,
        sensitivity: Sensitivity,
    ) -> Result<EnhancementResponse> {
        let request = EnhancementRequest {
            content: content.to_string(),
            enhancement_types: types,
            style,
            sensitivity,
        };
        
        self.enhance_with_request(&request).await
    }
    
    pub fn update_settings(&mut self, settings: EnhancerSettings) {
        self.settings = settings;
    }
    
    pub fn get_settings(&self) -> &EnhancerSettings {
        &self.settings
    }
    
    pub async fn get_available_providers(&self) -> Vec<(AIProviderType, bool)> {
        let ai_manager = self.ai_manager.lock().await;
        vec![
            (AIProviderType::Local, ai_manager.is_provider_available(AIProviderType::Local)),
            (AIProviderType::Cloud, ai_manager.is_provider_available(AIProviderType::Cloud)),
        ]
    }
    
    pub async fn set_preferred_provider(&mut self, provider: AIProviderType) -> Result<()> {
        let ai_manager = self.ai_manager.lock().await;
        if ai_manager.is_provider_available(provider.clone()) {
            self.settings.preferred_provider = provider;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Provider {:?} is not available", provider))
        }
    }
    
    pub fn filter_suggestions_by_confidence(&self, response: &EnhancementResponse) -> EnhancementResponse {
        let filtered_suggestions = response.suggestions.iter()
            .filter(|s| s.confidence >= self.settings.confidence_threshold)
            .cloned()
            .collect();
        
        EnhancementResponse {
            original_content: response.original_content.clone(),
            enhanced_content: response.enhanced_content.clone(),
            suggestions: filtered_suggestions,
            confidence: response.confidence,
            processing_time_ms: response.processing_time_ms,
        }
    }
    
    pub fn apply_selected_suggestions(
        &self,
        response: &EnhancementResponse,
        selected_suggestions: &[usize],
    ) -> String {
        let mut result = response.original_content.clone();
        
        // Apply suggestions in reverse order to maintain correct positions
        let mut sorted_indices = selected_suggestions.to_vec();
        sorted_indices.sort_by(|a, b| b.cmp(a));
        
        for &index in &sorted_indices {
            if let Some(suggestion) = response.suggestions.get(index) {
                result = suggestion.after.clone();
                break; // For simplicity, apply the last suggestion completely
            }
        }
        
        result
    }
    
    pub async fn get_enhancement_preview(
        &self,
        content: &str,
        enhancement_types: &[EnhancementType],
    ) -> Result<Vec<String>> {
        let mut previews = Vec::new();
        
        for enhancement_type in enhancement_types {
            let request = EnhancementRequest {
                content: content.to_string(),
                enhancement_types: vec![enhancement_type.clone()],
                style: self.settings.default_style.clone(),
                sensitivity: self.settings.default_sensitivity.clone(),
            };
            
            match self.enhance_with_request(&request).await {
                Ok(response) => {
                    let preview = self.create_preview_text(&response, enhancement_type);
                    previews.push(preview);
                }
                Err(_) => {
                    previews.push(format!("Preview unavailable for {:?}", enhancement_type));
                }
            }
        }
        
        Ok(previews)
    }
    
    fn create_preview_text(&self, response: &EnhancementResponse, enhancement_type: &EnhancementType) -> String {
        let type_name = match enhancement_type {
            EnhancementType::Clarity => "Clarity",
            EnhancementType::Structure => "Structure",
            EnhancementType::Tags => "Tags",
            EnhancementType::Summarization => "Summary",
            EnhancementType::Grammar => "Grammar",
        };
        
        let preview_length = 100;
        let enhanced_preview = if response.enhanced_content.len() > preview_length {
            format!("{}...", &response.enhanced_content[..preview_length])
        } else {
            response.enhanced_content.clone()
        };
        
        format!("{}: {}", type_name, enhanced_preview)
    }
    
    pub fn get_enhancement_statistics(&self, responses: &[EnhancementResponse]) -> EnhancementStats {
        let total_responses = responses.len();
        let avg_confidence = if total_responses > 0 {
            responses.iter().map(|r| r.confidence).sum::<f32>() / total_responses as f32
        } else {
            0.0
        };
        
        let avg_processing_time = if total_responses > 0 {
            responses.iter().map(|r| r.processing_time_ms).sum::<u64>() / total_responses as u64
        } else {
            0
        };
        
        let total_suggestions = responses.iter().map(|r| r.suggestions.len()).sum();
        
        let enhancement_type_counts = responses.iter()
            .flat_map(|r| r.suggestions.iter())
            .fold(std::collections::HashMap::new(), |mut acc, s| {
                *acc.entry(s.suggestion_type.clone()).or_insert(0) += 1;
                acc
            });
        
        EnhancementStats {
            total_enhancements: total_responses,
            average_confidence: avg_confidence,
            average_processing_time_ms: avg_processing_time,
            total_suggestions,
            enhancement_type_counts,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnhancementStats {
    pub total_enhancements: usize,
    pub average_confidence: f32,
    pub average_processing_time_ms: u64,
    pub total_suggestions: usize,
    pub enhancement_type_counts: std::collections::HashMap<EnhancementType, usize>,
}

// Utility functions for enhancement UI
impl NoteEnhancer {
    pub fn get_enhancement_type_description(enhancement_type: &EnhancementType) -> &'static str {
        match enhancement_type {
            EnhancementType::Clarity => "Improves readability and makes the text clearer",
            EnhancementType::Structure => "Adds proper markdown formatting and organization",
            EnhancementType::Tags => "Suggests relevant tags based on content",
            EnhancementType::Summarization => "Creates a summary of the main points",
            EnhancementType::Grammar => "Fixes spelling and grammar errors",
        }
    }
    
    pub fn get_style_description(style: &WritingStyle) -> &'static str {
        match style {
            WritingStyle::Formal => "Professional and formal tone",
            WritingStyle::Casual => "Conversational and relaxed tone",
            WritingStyle::Technical => "Precise and technical language",
        }
    }
    
    pub fn get_sensitivity_description(sensitivity: &Sensitivity) -> &'static str {
        match sensitivity {
            Sensitivity::Low => "Minimal changes, preserve original style",
            Sensitivity::Medium => "Moderate improvements and restructuring",
            Sensitivity::High => "Significant enhancements and reorganization",
        }
    }
}