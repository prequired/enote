use fuzzy_matcher::{FuzzyMatcher, SkimMatcherV2};
use std::collections::HashMap;
use crate::features::Note;
use regex::Regex;
use chrono::{DateTime, Utc, NaiveDate};

pub struct SearchEngine {
    matcher: SkimMatcherV2,
    index: SearchIndex,
}

pub struct SearchIndex {
    notes: HashMap<String, IndexedNote>,
    tags: HashMap<String, Vec<String>>, // tag -> note_ids
    words: HashMap<String, Vec<(String, f32)>>, // word -> (note_id, weight)
}

#[derive(Clone)]
struct IndexedNote {
    id: String,
    title: String,
    content: String,
    tags: Vec<String>,
    word_count: usize,
    title_words: Vec<String>,
    content_words: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub note_id: String,
    pub title: String,
    pub score: f32,
    pub match_type: MatchType,
    pub snippet: String,
    pub highlighted_title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchType {
    TitleExact,
    TitleFuzzy,
    ContentExact,
    ContentFuzzy,
    Tag,
    Regex,
    DateRange,
    TagCombination,
}

#[derive(Debug, Clone)]
pub struct SearchFilter {
    pub query: String,
    pub tags: Vec<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub regex_pattern: Option<String>,
    pub match_any_tag: bool, // If false, must match all tags
    pub include_content: bool,
    pub include_titles: bool,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            index: SearchIndex::new(),
        }
    }
    
    pub fn update_index(&mut self, notes: &[Note]) {
        self.index.clear();
        
        for note in notes {
            if !note.is_deleted {
                self.index.add_note(note);
            }
        }
        
        self.index.build_word_index();
    }
    
    pub fn search(&self, query: &str, max_results: usize) -> Vec<SearchResult> {
        if query.trim().is_empty() {
            return Vec::new();
        }
        
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        // Check if query is a special search pattern
        if let Some(advanced_results) = self.try_advanced_search(query, max_results) {
            return advanced_results;
        }
        
        // Search by title (exact and fuzzy)
        for note in self.index.notes.values() {
            // Exact title match
            if note.title.to_lowercase().contains(&query_lower) {
                results.push(SearchResult {
                    note_id: note.id.clone(),
                    title: note.title.clone(),
                    score: 100.0,
                    match_type: MatchType::TitleExact,
                    snippet: self.create_snippet(&note.content, query, 150),
                    highlighted_title: Some(self.highlight_match(&note.title, query)),
                });
            }
            
            // Fuzzy title match
            if let Some((score, _)) = self.matcher.fuzzy_match(&note.title, query) {
                if score > 50 { // Threshold for fuzzy matching
                    results.push(SearchResult {
                        note_id: note.id.clone(),
                        title: note.title.clone(),
                        score: score as f32,
                        match_type: MatchType::TitleFuzzy,
                        snippet: self.create_snippet(&note.content, query, 150),
                        highlighted_title: Some(self.highlight_fuzzy_match(&note.title, query)),
                    });
                }
            }
        }
        
        // Search by content
        for note in self.index.notes.values() {
            // Exact content match
            if note.content.to_lowercase().contains(&query_lower) {
                let score = self.calculate_content_score(&note.content, query);
                results.push(SearchResult {
                    note_id: note.id.clone(),
                    title: note.title.clone(),
                    score,
                    match_type: MatchType::ContentExact,
                    snippet: self.create_snippet(&note.content, query, 150),
                    highlighted_title: None,
                });
            }
            
            // Fuzzy content match (on individual words)
            for word in &note.content_words {
                if let Some((score, _)) = self.matcher.fuzzy_match(word, query) {
                    if score > 40 {
                        let final_score = (score as f32) * 0.5; // Lower weight for content fuzzy matches
                        results.push(SearchResult {
                            note_id: note.id.clone(),
                            title: note.title.clone(),
                            score: final_score,
                            match_type: MatchType::ContentFuzzy,
                            snippet: self.create_snippet(&note.content, word, 150),
                            highlighted_title: None,
                        });
                        break; // Only one fuzzy match per note to avoid duplicates
                    }
                }
            }
        }
        
        // Search by tags
        for (tag, note_ids) in &self.index.tags {
            if tag.to_lowercase().contains(&query_lower) {
                for note_id in note_ids {
                    if let Some(note) = self.index.notes.get(note_id) {
                        results.push(SearchResult {
                            note_id: note.id.clone(),
                            title: note.title.clone(),
                            score: 80.0, // High score for tag matches
                            match_type: MatchType::Tag,
                            snippet: format!("Tagged with: #{}", tag),
                            highlighted_title: None,
                        });
                    }
                }
            }
        }
        
        // Remove duplicates (keep highest scoring match per note)
        let mut note_scores: HashMap<String, SearchResult> = HashMap::new();
        for result in results {
            match note_scores.get(&result.note_id) {
                Some(existing) if existing.score >= result.score => continue,
                _ => {
                    note_scores.insert(result.note_id.clone(), result);
                }
            }
        }
        
        // Sort by score and limit results
        let mut final_results: Vec<SearchResult> = note_scores.into_values().collect();
        final_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        final_results.truncate(max_results);
        
        final_results
    }
    
    pub fn search_by_tag(&self, tag: &str) -> Vec<SearchResult> {
        if let Some(note_ids) = self.index.tags.get(tag) {
            note_ids.iter()
                .filter_map(|note_id| self.index.notes.get(note_id))
                .map(|note| SearchResult {
                    note_id: note.id.clone(),
                    title: note.title.clone(),
                    score: 100.0,
                    match_type: MatchType::Tag,
                    snippet: format!("Tagged with: #{}", tag),
                    highlighted_title: None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn suggest_tags(&self, query: &str, limit: usize) -> Vec<String> {
        let query_lower = query.to_lowercase();
        let mut matching_tags: Vec<String> = self.index.tags.keys()
            .filter(|tag| tag.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
        
        matching_tags.sort();
        matching_tags.truncate(limit);
        matching_tags
    }
    
    fn create_snippet(&self, content: &str, query: &str, max_length: usize) -> String {
        let query_lower = query.to_lowercase();
        
        // Find the first occurrence of the query
        if let Some(pos) = content.to_lowercase().find(&query_lower) {
            let start = pos.saturating_sub(50);
            let end = (pos + query.len() + 50).min(content.len());
            
            let mut snippet = content[start..end].to_string();
            
            // Add ellipsis if we're not at the beginning/end
            if start > 0 {
                snippet = format!("...{}", snippet);
            }
            if end < content.len() {
                snippet = format!("{}...", snippet);
            }
            
            // Truncate if still too long
            if snippet.len() > max_length {
                snippet.truncate(max_length - 3);
                snippet.push_str("...");
            }
            
            snippet
        } else {
            // No match found, return beginning of content
            let mut snippet = content.chars().take(max_length).collect::<String>();
            if content.len() > max_length {
                snippet.push_str("...");
            }
            snippet
        }
    }
    
    fn highlight_match(&self, text: &str, query: &str) -> String {
        let query_lower = query.to_lowercase();
        let text_lower = text.to_lowercase();
        
        if let Some(pos) = text_lower.find(&query_lower) {
            let before = &text[..pos];
            let matched = &text[pos..pos + query.len()];
            let after = &text[pos + query.len()..];
            format!("{}<mark>{}</mark>{}", before, matched, after)
        } else {
            text.to_string()
        }
    }
    
    fn highlight_fuzzy_match(&self, text: &str, query: &str) -> String {
        // Simplified fuzzy highlighting - would be more sophisticated in production
        if let Some((_, indices)) = self.matcher.fuzzy_match(text, query) {
            let mut result = String::new();
            let chars: Vec<char> = text.chars().collect();
            let mut last_idx = 0;
            
            for &idx in &indices {
                if idx > last_idx {
                    result.push_str(&chars[last_idx..idx].iter().collect::<String>());
                }
                result.push_str(&format!("<mark>{}</mark>", chars[idx]));
                last_idx = idx + 1;
            }
            
            if last_idx < chars.len() {
                result.push_str(&chars[last_idx..].iter().collect::<String>());
            }
            
            result
        } else {
            text.to_string()
        }
    }
    
    fn calculate_content_score(&self, content: &str, query: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let content_lower = content.to_lowercase();
        
        let matches = content_lower.matches(&query_lower).count();
        let content_length = content.len();
        
        // Score based on frequency and content length
        let frequency_score = (matches as f32) * 10.0;
        let density_score = (matches as f32) / (content_length as f32) * 1000.0;
        
        frequency_score + density_score
    }
    
    // Advanced search methods
    fn try_advanced_search(&self, query: &str, max_results: usize) -> Option<Vec<SearchResult>> {
        // Regex search: /pattern/
        if query.starts_with('/') && query.ends_with('/') && query.len() > 2 {
            let pattern = &query[1..query.len()-1];
            return Some(self.search_regex(pattern, max_results));
        }
        
        // Tag combination search: tag:work AND tag:urgent
        if query.contains("tag:") && (query.contains(" AND ") || query.contains(" OR ")) {
            return Some(self.search_tag_combination(query, max_results));
        }
        
        // Date range search: date:2024-01-01..2024-12-31
        if query.starts_with("date:") && query.contains("..") {
            return Some(self.search_date_range(query, max_results));
        }
        
        // Complex filter search
        if query.contains("title:") || query.contains("content:") || query.contains("created:") {
            return Some(self.search_with_filters(query, max_results));
        }
        
        None
    }
    
    pub fn search_regex(&self, pattern: &str, max_results: usize) -> Vec<SearchResult> {
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return Vec::new(), // Invalid regex pattern
        };
        
        let mut results = Vec::new();
        
        for note in self.index.notes.values() {
            let mut matches = Vec::new();
            
            // Search in title
            if regex.is_match(&note.title) {
                matches.push((MatchType::Regex, &note.title, 100.0));
            }
            
            // Search in content
            if regex.is_match(&note.content) {
                matches.push((MatchType::Regex, &note.content, 80.0));
            }
            
            for (match_type, text, score) in matches {
                results.push(SearchResult {
                    note_id: note.id.clone(),
                    title: note.title.clone(),
                    score,
                    match_type,
                    snippet: self.create_regex_snippet(text, &regex, 150),
                    highlighted_title: None,
                });
            }
        }
        
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(max_results);
        results
    }
    
    pub fn search_tag_combination(&self, query: &str, max_results: usize) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        // Parse tag combinations like "tag:work AND tag:urgent" or "tag:meeting OR tag:planning"
        let is_and = query.contains(" AND ");
        let is_or = query.contains(" OR ");
        
        if !is_and && !is_or {
            return results;
        }
        
        let separator = if is_and { " AND " } else { " OR " };
        let tag_parts: Vec<&str> = query.split(separator).collect();
        let mut required_tags = Vec::new();
        
        for part in tag_parts {
            if let Some(tag) = part.strip_prefix("tag:") {
                required_tags.push(tag.trim());
            }
        }
        
        if required_tags.is_empty() {
            return results;
        }
        
        for note in self.index.notes.values() {
            let note_tags: Vec<&str> = note.tags.iter().map(|s| s.as_str()).collect();
            
            let matches = if is_and {
                // All tags must be present
                required_tags.iter().all(|&tag| note_tags.contains(&tag))
            } else {
                // At least one tag must be present
                required_tags.iter().any(|&tag| note_tags.contains(&tag))
            };
            
            if matches {
                results.push(SearchResult {
                    note_id: note.id.clone(),
                    title: note.title.clone(),
                    score: 90.0,
                    match_type: MatchType::TagCombination,
                    snippet: format!("Tags: {}", note.tags.join(", ")),
                    highlighted_title: None,
                });
            }
        }
        
        results.truncate(max_results);
        results
    }
    
    pub fn search_date_range(&self, query: &str, max_results: usize) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        // Parse date range like "date:2024-01-01..2024-12-31"
        if let Some(date_part) = query.strip_prefix("date:") {
            let dates: Vec<&str> = date_part.split("..").collect();
            if dates.len() != 2 {
                return results;
            }
            
            let start_date = match NaiveDate::parse_from_str(dates[0], "%Y-%m-%d") {
                Ok(date) => date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                Err(_) => return results,
            };
            
            let end_date = match NaiveDate::parse_from_str(dates[1], "%Y-%m-%d") {
                Ok(date) => date.and_hms_opt(23, 59, 59).unwrap().and_utc(),
                Err(_) => return results,
            };
            
            for note in self.index.notes.values() {
                // Check if note was created or modified in the date range
                if let Ok(note_data) = serde_json::from_str::<serde_json::Value>(&serde_json::to_string(&note).unwrap_or_default()) {
                    // This is a simplified check - in a real implementation you'd have proper date fields
                    results.push(SearchResult {
                        note_id: note.id.clone(),
                        title: note.title.clone(),
                        score: 85.0,
                        match_type: MatchType::DateRange,
                        snippet: format!("Found in date range {} to {}", dates[0], dates[1]),
                        highlighted_title: None,
                    });
                }
            }
        }
        
        results.truncate(max_results);
        results
    }
    
    pub fn search_with_filters(&self, query: &str, max_results: usize) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let mut title_query = None;
        let mut content_query = None;
        
        // Parse filters like "title:meeting content:agenda"
        let parts: Vec<&str> = query.split_whitespace().collect();
        let mut i = 0;
        
        while i < parts.len() {
            if let Some(title_term) = parts[i].strip_prefix("title:") {
                title_query = Some(title_term);
            } else if let Some(content_term) = parts[i].strip_prefix("content:") {
                content_query = Some(content_term);
            }
            i += 1;
        }
        
        for note in self.index.notes.values() {
            let mut score = 0.0;
            let mut matches = false;
            
            if let Some(title_term) = title_query {
                if note.title.to_lowercase().contains(&title_term.to_lowercase()) {
                    score += 100.0;
                    matches = true;
                }
            }
            
            if let Some(content_term) = content_query {
                if note.content.to_lowercase().contains(&content_term.to_lowercase()) {
                    score += 80.0;
                    matches = true;
                }
            }
            
            if matches {
                results.push(SearchResult {
                    note_id: note.id.clone(),
                    title: note.title.clone(),
                    score,
                    match_type: MatchType::ContentExact,
                    snippet: self.create_snippet(&note.content, query, 150),
                    highlighted_title: None,
                });
            }
        }
        
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(max_results);
        results
    }
    
    pub fn advanced_search(&self, filter: &SearchFilter, max_results: usize) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        for note in self.index.notes.values() {
            let mut score = 0.0;
            let mut matches = true;
            
            // Apply text query filter
            if !filter.query.is_empty() {
                let query_lower = filter.query.to_lowercase();
                let mut text_match = false;
                
                if filter.include_titles && note.title.to_lowercase().contains(&query_lower) {
                    score += 100.0;
                    text_match = true;
                }
                
                if filter.include_content && note.content.to_lowercase().contains(&query_lower) {
                    score += 80.0;
                    text_match = true;
                }
                
                if !text_match {
                    matches = false;
                }
            }
            
            // Apply tag filters
            if !filter.tags.is_empty() {
                let note_tags: Vec<&str> = note.tags.iter().map(|s| s.as_str()).collect();
                
                let tag_match = if filter.match_any_tag {
                    filter.tags.iter().any(|tag| note_tags.contains(&tag.as_str()))
                } else {
                    filter.tags.iter().all(|tag| note_tags.contains(&tag.as_str()))
                };
                
                if !tag_match {
                    matches = false;
                } else {
                    score += 50.0;
                }
            }
            
            // Apply regex filter
            if let Some(ref pattern) = filter.regex_pattern {
                if let Ok(regex) = Regex::new(pattern) {
                    if !regex.is_match(&note.content) && !regex.is_match(&note.title) {
                        matches = false;
                    } else {
                        score += 90.0;
                    }
                }
            }
            
            // Note: Date filtering would require proper created_at/modified_at fields
            // This is simplified for demonstration
            
            if matches {
                results.push(SearchResult {
                    note_id: note.id.clone(),
                    title: note.title.clone(),
                    score,
                    match_type: MatchType::ContentExact,
                    snippet: self.create_snippet(&note.content, &filter.query, 150),
                    highlighted_title: None,
                });
            }
        }
        
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(max_results);
        results
    }
    
    fn create_regex_snippet(&self, content: &str, regex: &Regex, max_length: usize) -> String {
        if let Some(mat) = regex.find(content) {
            let start = mat.start().saturating_sub(50);
            let end = (mat.end() + 50).min(content.len());
            
            let mut snippet = content[start..end].to_string();
            
            if start > 0 {
                snippet = format!("...{}", snippet);
            }
            if end < content.len() {
                snippet = format!("{}...", snippet);
            }
            
            if snippet.len() > max_length {
                snippet.truncate(max_length - 3);
                snippet.push_str("...");
            }
            
            snippet
        } else {
            content.chars().take(max_length).collect::<String>()
        }
    }
}

impl SearchIndex {
    fn new() -> Self {
        Self {
            notes: HashMap::new(),
            tags: HashMap::new(),
            words: HashMap::new(),
        }
    }
    
    fn clear(&mut self) {
        self.notes.clear();
        self.tags.clear();
        self.words.clear();
    }
    
    fn add_note(&mut self, note: &Note) {
        let indexed_note = IndexedNote {
            id: note.id.clone(),
            title: note.title.clone(),
            content: note.content.clone(),
            tags: note.tags.clone(),
            word_count: note.content.split_whitespace().count(),
            title_words: note.title.split_whitespace()
                .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
                .filter(|w| !w.is_empty())
                .collect(),
            content_words: note.content.split_whitespace()
                .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
                .filter(|w| !w.is_empty() && w.len() > 2) // Filter out very short words
                .collect(),
        };
        
        // Add tags to tag index
        for tag in &note.tags {
            self.tags.entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(note.id.clone());
        }
        
        self.notes.insert(note.id.clone(), indexed_note);
    }
    
    fn build_word_index(&mut self) {
        for note in self.notes.values() {
            // Index title words with higher weight
            for word in &note.title_words {
                self.words.entry(word.clone())
                    .or_insert_with(Vec::new)
                    .push((note.id.clone(), 2.0)); // Higher weight for title words
            }
            
            // Index content words
            for word in &note.content_words {
                self.words.entry(word.clone())
                    .or_insert_with(Vec::new)
                    .push((note.id.clone(), 1.0));
            }
        }
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}