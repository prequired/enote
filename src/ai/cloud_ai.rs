use super::{AIProvider, EnhancementRequest, EnhancementResponse, EnhancementType, Suggestion};
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::json;
use std::time::Instant;

pub struct CloudAI {
    client: Client,
    api_key: Option<String>,
    base_url: String,
}

impl CloudAI {
    pub async fn new() -> Result<Self> {
        let client = Client::new();
        let api_key = std::env::var("XAI_API_KEY").ok();
        let base_url = "https://api.x.ai/v1".to_string();
        
        Ok(Self {
            client,
            api_key,
            base_url,
        })
    }
    
    async fn call_api(&self, request: &EnhancementRequest) -> Result<EnhancementResponse> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow!("XAI API key not configured"))?;
        
        let start_time = Instant::now();
        
        let prompt = self.build_prompt(request);
        
        let payload = json!({
            "model": "grok-beta",
            "messages": [
                {
                    "role": "system",
                    "content": "You are an AI assistant that helps improve note-taking content. Provide enhanced versions of notes while maintaining the original meaning."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 2000,
            "temperature": 0.3
        });
        
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("API request failed: {}", response.status()));
        }
        
        let response_data: serde_json::Value = response.json().await?;
        
        let enhanced_content = response_data["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid API response format"))?
            .to_string();
        
        // Parse the response to extract suggestions
        let suggestions = self.parse_suggestions(&request.content, &enhanced_content, &request.enhancement_types);
        
        Ok(EnhancementResponse {
            original_content: request.content.clone(),
            enhanced_content,
            suggestions,
            confidence: 0.9,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }
    
    fn build_prompt(&self, request: &EnhancementRequest) -> String {
        let mut prompt = String::new();
        
        prompt.push_str("Please enhance the following note content according to these requirements:\n\n");
        
        // Add enhancement type instructions
        for enhancement_type in &request.enhancement_types {
            match enhancement_type {
                EnhancementType::Clarity => {
                    prompt.push_str("- Improve clarity and readability\n");
                }
                EnhancementType::Structure => {
                    prompt.push_str("- Add proper markdown structure with headers and formatting\n");
                }
                EnhancementType::Tags => {
                    prompt.push_str("- Suggest relevant tags (format: #tag)\n");
                }
                EnhancementType::Summarization => {
                    prompt.push_str("- Add a summary section at the beginning\n");
                }
                EnhancementType::Grammar => {
                    prompt.push_str("- Fix grammar and spelling errors\n");
                }
            }
        }
        
        // Add style instructions
        prompt.push_str(&format!("\nWriting style: {:?}\n", request.style));
        prompt.push_str(&format!("Enhancement sensitivity: {:?}\n", request.sensitivity));
        
        prompt.push_str("\nOriginal content:\n");
        prompt.push_str(&request.content);
        
        prompt.push_str("\n\nPlease provide the enhanced version:");
        
        prompt
    }
    
    fn parse_suggestions(&self, original: &str, enhanced: &str, types: &[EnhancementType]) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // Create suggestions based on what was requested and changed
        for enhancement_type in types {
            let suggestion = Suggestion {
                suggestion_type: enhancement_type.clone(),
                description: match enhancement_type {
                    EnhancementType::Clarity => "Improved text clarity and flow".to_string(),
                    EnhancementType::Structure => "Added markdown structure and formatting".to_string(),
                    EnhancementType::Tags => "Generated relevant tags".to_string(),
                    EnhancementType::Summarization => "Added content summary".to_string(),
                    EnhancementType::Grammar => "Fixed grammar and spelling".to_string(),
                },
                before: original.to_string(),
                after: enhanced.to_string(),
                confidence: 0.9,
                applied: true,
            };
            suggestions.push(suggestion);
        }
        
        suggestions
    }
    
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }
    
    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }
}

impl AIProvider for CloudAI {
    async fn enhance_text(&mut self, request: &EnhancementRequest) -> Result<EnhancementResponse> {
        if !self.is_available() {
            return Err(anyhow!("Cloud AI is not available - API key not configured"));
        }
        
        // Try API call, fall back to simple enhancement if it fails
        match self.call_api(request).await {
            Ok(response) => Ok(response),
            Err(e) => {
                log::warn!("Cloud AI API call failed: {}. Using fallback enhancement.", e);
                self.fallback_enhancement(request).await
            }
        }
    }
    
    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
    
    fn get_name(&self) -> &str {
        "Cloud AI (xAI Grok)"
    }
}

impl CloudAI {
    async fn fallback_enhancement(&self, request: &EnhancementRequest) -> Result<EnhancementResponse> {
        let start_time = Instant::now();
        let mut enhanced_content = request.content.clone();
        let mut suggestions = Vec::new();
        
        // Apply basic enhancements when API is unavailable
        for enhancement_type in &request.enhancement_types {
            match enhancement_type {
                EnhancementType::Clarity => {
                    enhanced_content = self.improve_clarity_offline(&enhanced_content);
                    suggestions.push(Suggestion {
                        suggestion_type: EnhancementType::Clarity,
                        description: "Applied basic clarity improvements (offline)".to_string(),
                        before: request.content.clone(),
                        after: enhanced_content.clone(),
                        confidence: 0.6,
                        applied: true,
                    });
                }
                EnhancementType::Structure => {
                    enhanced_content = self.add_structure_offline(&enhanced_content);
                    suggestions.push(Suggestion {
                        suggestion_type: EnhancementType::Structure,
                        description: "Added basic structure (offline)".to_string(),
                        before: request.content.clone(),
                        after: enhanced_content.clone(),
                        confidence: 0.7,
                        applied: true,
                    });
                }
                EnhancementType::Tags => {
                    let tags = self.generate_tags_offline(&enhanced_content);
                    if !tags.is_empty() {
                        enhanced_content.push_str(&format!("\n\n{}", tags.join(" ")));
                        suggestions.push(Suggestion {
                            suggestion_type: EnhancementType::Tags,
                            description: "Generated basic tags (offline)".to_string(),
                            before: request.content.clone(),
                            after: enhanced_content.clone(),
                            confidence: 0.5,
                            applied: true,
                        });
                    }
                }
                EnhancementType::Grammar => {
                    enhanced_content = self.fix_grammar_offline(&enhanced_content);
                    suggestions.push(Suggestion {
                        suggestion_type: EnhancementType::Grammar,
                        description: "Applied basic grammar fixes (offline)".to_string(),
                        before: request.content.clone(),
                        after: enhanced_content.clone(),
                        confidence: 0.6,
                        applied: true,
                    });
                }
                EnhancementType::Summarization => {
                    let summary = self.create_summary_offline(&enhanced_content);
                    enhanced_content = format!("## Summary\n{}\n\n{}", summary, enhanced_content);
                    suggestions.push(Suggestion {
                        suggestion_type: EnhancementType::Summarization,
                        description: "Added basic summary (offline)".to_string(),
                        before: request.content.clone(),
                        after: enhanced_content.clone(),
                        confidence: 0.5,
                        applied: true,
                    });
                }
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
    
    fn improve_clarity_offline(&self, content: &str) -> String {
        let mut improved = content.to_string();
        improved = improved.replace("very good", "excellent");
        improved = improved.replace("very bad", "poor");
        improved = improved.replace("a lot of", "many");
        improved = improved.replace("  ", " ");
        improved
    }
    
    fn add_structure_offline(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return content.to_string();
        }
        
        let mut structured = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i == 0 && !line.trim().starts_with('#') && !line.trim().is_empty() {
                structured.push(format!("# {}", line.trim()));
            } else {
                structured.push(line.to_string());
            }
        }
        
        structured.join("\n")
    }
    
    fn generate_tags_offline(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let content_lower = content.to_lowercase();
        
        let keywords = [
            ("meeting", "#meeting"),
            ("project", "#project"),
            ("task", "#task"),
            ("idea", "#idea"),
            ("work", "#work"),
        ];
        
        for (keyword, tag) in &keywords {
            if content_lower.contains(keyword) {
                tags.push(tag.to_string());
            }
        }
        
        tags
    }
    
    fn fix_grammar_offline(&self, content: &str) -> String {
        let mut fixed = content.to_string();
        fixed = fixed.replace(" i ", " I ");
        fixed = fixed.replace("  ", " ");
        fixed = fixed.replace(" .", ".");
        fixed = fixed.replace(" ,", ",");
        fixed
    }
    
    fn create_summary_offline(&self, content: &str) -> String {
        let first_sentence = content.lines()
            .next()
            .unwrap_or("Content summary")
            .trim();
        
        if first_sentence.len() > 100 {
            format!("{}...", &first_sentence[..100])
        } else {
            first_sentence.to_string()
        }
    }
}