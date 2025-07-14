use pulldown_cmark::{Parser, Event, Tag, Options, html};
use std::collections::HashMap;

pub struct MarkdownProcessor {
    options: Options,
}

impl MarkdownProcessor {
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        
        Self { options }
    }
    
    pub fn to_html(&self, markdown: &str) -> String {
        let parser = Parser::new_ext(markdown, self.options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
    
    pub fn extract_headers(&self, markdown: &str) -> Vec<Header> {
        let parser = Parser::new_ext(markdown, self.options);
        let mut headers = Vec::new();
        let mut current_header: Option<Header> = None;
        let mut in_heading = false;
        
        for event in parser {
            match event {
                Event::Start(Tag::Heading(level, id, _)) => {
                    in_heading = true;
                    current_header = Some(Header {
                        level: level as u8,
                        text: String::new(),
                        id: id.map(|s| s.to_string()),
                    });
                }
                Event::Text(text) if in_heading => {
                    if let Some(ref mut header) = current_header {
                        header.text.push_str(&text);
                    }
                }
                Event::End(Tag::Heading(_, _, _)) => {
                    in_heading = false;
                    if let Some(header) = current_header.take() {
                        headers.push(header);
                    }
                }
                _ => {}
            }
        }
        
        headers
    }
    
    pub fn extract_links(&self, markdown: &str) -> Vec<Link> {
        let parser = Parser::new_ext(markdown, self.options);
        let mut links = Vec::new();
        let mut current_link: Option<Link> = None;
        let mut in_link = false;
        
        for event in parser {
            match event {
                Event::Start(Tag::Link(_, url, title)) => {
                    in_link = true;
                    current_link = Some(Link {
                        text: String::new(),
                        url: url.to_string(),
                        title: if title.is_empty() { None } else { Some(title.to_string()) },
                    });
                }
                Event::Text(text) if in_link => {
                    if let Some(ref mut link) = current_link {
                        link.text.push_str(&text);
                    }
                }
                Event::End(Tag::Link(_, _, _)) => {
                    in_link = false;
                    if let Some(link) = current_link.take() {
                        links.push(link);
                    }
                }
                _ => {}
            }
        }
        
        links
    }
    
    pub fn extract_wiki_links(&self, markdown: &str) -> Vec<WikiLink> {
        let mut wiki_links = Vec::new();
        let regex = regex::Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        
        for cap in regex.captures_iter(markdown) {
            if let Some(link_match) = cap.get(0) {
                if let Some(text_match) = cap.get(1) {
                    let parts: Vec<&str> = text_match.as_str().split('|').collect();
                    let (target, display_text) = if parts.len() > 1 {
                        (parts[0].trim(), Some(parts[1].trim().to_string()))
                    } else {
                        (parts[0].trim(), None)
                    };
                    
                    wiki_links.push(WikiLink {
                        target: target.to_string(),
                        display_text,
                        start: link_match.start(),
                        end: link_match.end(),
                    });
                }
            }
        }
        
        wiki_links
    }
    
    pub fn extract_tags(&self, markdown: &str) -> Vec<String> {
        let regex = regex::Regex::new(r"#(\w+)").unwrap();
        regex.captures_iter(markdown)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    pub fn extract_frontmatter(&self, markdown: &str) -> Option<HashMap<String, String>> {
        if !markdown.starts_with("---\n") {
            return None;
        }
        
        let end_pos = markdown[4..].find("\n---\n")?;
        let frontmatter = &markdown[4..end_pos + 4];
        
        let mut metadata = HashMap::new();
        for line in frontmatter.lines() {
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                metadata.insert(key, value);
            }
        }
        
        Some(metadata)
    }
    
    pub fn remove_frontmatter(&self, markdown: &str) -> String {
        if !markdown.starts_with("---\n") {
            return markdown.to_string();
        }
        
        if let Some(end_pos) = markdown[4..].find("\n---\n") {
            return markdown[end_pos + 8..].to_string();
        }
        
        markdown.to_string()
    }
    
    pub fn add_frontmatter(&self, markdown: &str, metadata: &HashMap<String, String>) -> String {
        let mut result = String::new();
        result.push_str("---\n");
        
        for (key, value) in metadata {
            result.push_str(&format!("{}: {}\n", key, value));
        }
        
        result.push_str("---\n\n");
        result.push_str(&self.remove_frontmatter(markdown));
        
        result
    }
    
    pub fn count_words(&self, markdown: &str) -> usize {
        let content = self.remove_frontmatter(markdown);
        let parser = Parser::new_ext(&content, self.options);
        let mut word_count = 0;
        
        for event in parser {
            if let Event::Text(text) = event {
                word_count += text.split_whitespace().count();
            }
        }
        
        word_count
    }
    
    pub fn estimate_reading_time(&self, markdown: &str) -> usize {
        let word_count = self.count_words(markdown);
        // Assume 200 words per minute reading speed
        std::cmp::max(1, word_count / 200)
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub level: u8,
    pub text: String,
    pub id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WikiLink {
    pub target: String,
    pub display_text: Option<String>,
    pub start: usize,
    pub end: usize,
}

impl Default for MarkdownProcessor {
    fn default() -> Self {
        Self::new()
    }
}