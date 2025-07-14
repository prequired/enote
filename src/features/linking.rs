use regex::Regex;
use std::collections::HashMap;

pub struct LinkProcessor {
    wiki_link_regex: Regex,
    markdown_link_regex: Regex,
}

impl LinkProcessor {
    pub fn new() -> Self {
        Self {
            wiki_link_regex: Regex::new(r"\[\[([^\]]+)\]\]").unwrap(),
            markdown_link_regex: Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap(),
        }
    }
    
    pub fn extract_wiki_links(&self, content: &str) -> Vec<WikiLink> {
        let mut links = Vec::new();
        
        for cap in self.wiki_link_regex.captures_iter(content) {
            if let Some(link_match) = cap.get(0) {
                if let Some(text_match) = cap.get(1) {
                    let link = WikiLink {
                        text: text_match.as_str().to_string(),
                        start: link_match.start(),
                        end: link_match.end(),
                        full_match: link_match.as_str().to_string(),
                    };
                    links.push(link);
                }
            }
        }
        
        links
    }
    
    pub fn extract_markdown_links(&self, content: &str) -> Vec<MarkdownLink> {
        let mut links = Vec::new();
        
        for cap in self.markdown_link_regex.captures_iter(content) {
            if let Some(link_match) = cap.get(0) {
                if let (Some(text_match), Some(url_match)) = (cap.get(1), cap.get(2)) {
                    let link = MarkdownLink {
                        text: text_match.as_str().to_string(),
                        url: url_match.as_str().to_string(),
                        start: link_match.start(),
                        end: link_match.end(),
                        full_match: link_match.as_str().to_string(),
                    };
                    links.push(link);
                }
            }
        }
        
        links
    }
    
    pub fn resolve_wiki_links(&self, content: &str, note_lookup: &HashMap<String, String>) -> String {
        let mut result = content.to_string();
        let links = self.extract_wiki_links(content);
        
        // Process links in reverse order to maintain correct positions
        for link in links.iter().rev() {
            if let Some(note_id) = note_lookup.get(&link.text.to_lowercase()) {
                let replacement = format!("[{}](note://{})", link.text, note_id);
                result.replace_range(link.start..link.end, &replacement);
            }
        }
        
        result
    }
    
    pub fn create_backlinks_map(&self, notes: &[(String, String)]) -> HashMap<String, Vec<String>> {
        let mut backlinks: HashMap<String, Vec<String>> = HashMap::new();
        
        for (note_id, content) in notes {
            let wiki_links = self.extract_wiki_links(content);
            
            for link in wiki_links {
                // Find notes that match this link text
                for (other_note_id, _) in notes {
                    if other_note_id != note_id {
                        // This is simplified - in practice you'd look up by title
                        if link.text.to_lowercase().contains(&other_note_id.to_lowercase()) {
                            backlinks
                                .entry(other_note_id.clone())
                                .or_insert_with(Vec::new)
                                .push(note_id.clone());
                        }
                    }
                }
            }
        }
        
        backlinks
    }
    
    pub fn highlight_links_in_content(&self, content: &str) -> String {
        let mut result = content.to_string();
        
        // Highlight wiki links
        result = self.wiki_link_regex.replace_all(&result, |caps: &regex::Captures| {
            let link_text = &caps[1];
            format!("<span class=\"wiki-link\">[[{}]]</span>", link_text)
        }).to_string();
        
        // Highlight markdown links
        result = self.markdown_link_regex.replace_all(&result, |caps: &regex::Captures| {
            let link_text = &caps[1];
            let url = &caps[2];
            format!("<a href=\"{}\" class=\"markdown-link\">{}</a>", url, link_text)
        }).to_string();
        
        result
    }
    
    pub fn suggest_links(&self, content: &str, available_notes: &[String]) -> Vec<LinkSuggestion> {
        let mut suggestions = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();
        
        for note_title in available_notes {
            let title_words: Vec<&str> = note_title.split_whitespace().collect();
            
            // Look for exact title matches
            for window in words.windows(title_words.len()) {
                if window.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>() == 
                   title_words.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>() {
                    
                    suggestions.push(LinkSuggestion {
                        text: note_title.clone(),
                        suggested_link: format!("[[{}]]", note_title),
                        confidence: 1.0,
                        reason: "Exact title match".to_string(),
                    });
                }
            }
            
            // Look for partial matches
            for word in &words {
                if note_title.to_lowercase().contains(&word.to_lowercase()) && word.len() > 3 {
                    suggestions.push(LinkSuggestion {
                        text: note_title.clone(),
                        suggested_link: format!("[[{}]]", note_title),
                        confidence: 0.5,
                        reason: format!("Contains word: {}", word),
                    });
                }
            }
        }
        
        // Remove duplicates and sort by confidence
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        suggestions.dedup_by(|a, b| a.text == b.text);
        
        suggestions
    }
}

#[derive(Debug, Clone)]
pub struct WikiLink {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub full_match: String,
}

#[derive(Debug, Clone)]
pub struct MarkdownLink {
    pub text: String,
    pub url: String,
    pub start: usize,
    pub end: usize,
    pub full_match: String,
}

#[derive(Debug, Clone)]
pub struct LinkSuggestion {
    pub text: String,
    pub suggested_link: String,
    pub confidence: f32,
    pub reason: String,
}

impl Default for LinkProcessor {
    fn default() -> Self {
        Self::new()
    }
}