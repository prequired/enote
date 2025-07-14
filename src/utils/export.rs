use anyhow::Result;
use crate::features::Note;
use crate::utils::MarkdownProcessor;
use std::collections::HashMap;
use std::path::Path;

pub struct Exporter {
    markdown_processor: MarkdownProcessor,
}

impl Exporter {
    pub fn new() -> Self {
        Self {
            markdown_processor: MarkdownProcessor::new(),
        }
    }
    
    pub fn export_to_html(&self, note: &Note, include_style: bool) -> Result<String> {
        let mut html = String::new();
        
        if include_style {
            html.push_str(&self.get_html_style());
        }
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("<title>{}</title>\n", note.title));
        
        if include_style {
            html.push_str("<style>\n");
            html.push_str(&self.get_css_styles());
            html.push_str("</style>\n");
        }
        
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("<div class=\"container\">\n");
        
        // Add metadata
        html.push_str("<div class=\"metadata\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", note.title));
        html.push_str(&format!("<p class=\"date\">Created: {}</p>\n", 
            note.created_at.format("%Y-%m-%d %H:%M")));
        html.push_str(&format!("<p class=\"date\">Modified: {}</p>\n", 
            note.modified_at.format("%Y-%m-%d %H:%M")));
        
        if !note.tags.is_empty() {
            html.push_str("<div class=\"tags\">\n");
            for tag in &note.tags {
                html.push_str(&format!("<span class=\"tag\">#{}</span> ", tag));
            }
            html.push_str("</div>\n");
        }
        html.push_str("</div>\n");
        
        // Convert markdown content to HTML
        let content_html = self.markdown_processor.to_html(&note.content);
        html.push_str("<div class=\"content\">\n");
        html.push_str(&content_html);
        html.push_str("</div>\n");
        
        html.push_str("</div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        Ok(html)
    }
    
    pub fn export_to_markdown(&self, note: &Note, include_frontmatter: bool) -> Result<String> {
        let mut content = String::new();
        
        if include_frontmatter {
            let mut metadata = HashMap::new();
            metadata.insert("id".to_string(), note.id.clone());
            metadata.insert("title".to_string(), note.title.clone());
            metadata.insert("created".to_string(), note.created_at.to_rfc3339());
            metadata.insert("modified".to_string(), note.modified_at.to_rfc3339());
            
            if !note.tags.is_empty() {
                metadata.insert("tags".to_string(), format!("[{}]", note.tags.join(", ")));
            }
            
            content = self.markdown_processor.add_frontmatter(&note.content, &metadata);
        } else {
            content = note.content.clone();
        }
        
        Ok(content)
    }
    
    pub fn export_multiple_to_html(&self, notes: &[Note], title: &str) -> Result<String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("<title>{}</title>\n", title));
        html.push_str("<style>\n");
        html.push_str(&self.get_css_styles());
        html.push_str("</style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("<div class=\"container\">\n");
        
        html.push_str(&format!("<h1 class=\"collection-title\">{}</h1>\n", title));
        html.push_str("<div class=\"table-of-contents\">\n");
        html.push_str("<h2>Table of Contents</h2>\n");
        html.push_str("<ul>\n");
        
        for note in notes {
            let slug = self.create_slug(&note.title);
            html.push_str(&format!("<li><a href=\"#{}\">{}</a></li>\n", slug, note.title));
        }
        
        html.push_str("</ul>\n");
        html.push_str("</div>\n");
        
        for note in notes {
            let slug = self.create_slug(&note.title);
            html.push_str(&format!("<div class=\"note\" id=\"{}\">\n", slug));
            html.push_str(&format!("<h2>{}</h2>\n", note.title));
            
            html.push_str("<div class=\"note-metadata\">\n");
            html.push_str(&format!("<span class=\"date\">Created: {}</span> | ", 
                note.created_at.format("%Y-%m-%d")));
            html.push_str(&format!("<span class=\"date\">Modified: {}</span>", 
                note.modified_at.format("%Y-%m-%d")));
            
            if !note.tags.is_empty() {
                html.push_str(" | <span class=\"tags\">");
                for tag in &note.tags {
                    html.push_str(&format!("<span class=\"tag\">#{}</span> ", tag));
                }
                html.push_str("</span>");
            }
            html.push_str("</div>\n");
            
            let content_html = self.markdown_processor.to_html(&note.content);
            html.push_str("<div class=\"note-content\">\n");
            html.push_str(&content_html);
            html.push_str("</div>\n");
            html.push_str("</div>\n");
            html.push_str("<hr>\n");
        }
        
        html.push_str("</div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        Ok(html)
    }
    
    pub fn export_to_json(&self, notes: &[Note]) -> Result<String> {
        let json = serde_json::to_string_pretty(notes)?;
        Ok(json)
    }
    
    pub fn export_to_plain_text(&self, note: &Note) -> Result<String> {
        let mut text = String::new();
        
        text.push_str(&format!("Title: {}\n", note.title));
        text.push_str(&format!("Created: {}\n", note.created_at.format("%Y-%m-%d %H:%M")));
        text.push_str(&format!("Modified: {}\n", note.modified_at.format("%Y-%m-%d %H:%M")));
        
        if !note.tags.is_empty() {
            text.push_str(&format!("Tags: {}\n", note.tags.join(", ")));
        }
        
        text.push_str("\n");
        text.push_str("---\n\n");
        
        // Remove markdown formatting for plain text
        let plain_content = self.markdown_to_plain_text(&note.content);
        text.push_str(&plain_content);
        
        Ok(text)
    }
    
    fn markdown_to_plain_text(&self, markdown: &str) -> String {
        // Simple markdown to plain text conversion
        let mut text = markdown.to_string();
        
        // Remove headers
        text = regex::Regex::new(r"^#{1,6}\s+").unwrap().replace_all(&text, "").to_string();
        
        // Remove emphasis
        text = regex::Regex::new(r"\*\*(.*?)\*\*").unwrap().replace_all(&text, "$1").to_string();
        text = regex::Regex::new(r"\*(.*?)\*").unwrap().replace_all(&text, "$1").to_string();
        text = regex::Regex::new(r"_(.*?)_").unwrap().replace_all(&text, "$1").to_string();
        
        // Remove links but keep text
        text = regex::Regex::new(r"\[([^\]]+)\]\([^)]+\)").unwrap().replace_all(&text, "$1").to_string();
        text = regex::Regex::new(r"\[\[([^\]]+)\]\]").unwrap().replace_all(&text, "$1").to_string();
        
        // Remove code blocks
        text = regex::Regex::new(r"```[^`]*```").unwrap().replace_all(&text, "[Code Block]").to_string();
        text = regex::Regex::new(r"`([^`]+)`").unwrap().replace_all(&text, "$1").to_string();
        
        // Clean up list items
        text = regex::Regex::new(r"^[-*+]\s+").unwrap().replace_all(&text, "• ").to_string();
        
        text
    }
    
    fn create_slug(&self, title: &str) -> String {
        title.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
    
    fn get_html_style(&self) -> &str {
        r#"
        <meta name="generator" content="Edison Note">
        <meta name="description" content="Note exported from Edison Note">
        "#
    }
    
    fn get_css_styles(&self) -> &str {
        r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #fff;
        }
        
        .container {
            background: white;
            padding: 2em;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        
        .metadata {
            border-bottom: 1px solid #eee;
            padding-bottom: 1em;
            margin-bottom: 2em;
        }
        
        .metadata h1 {
            margin: 0 0 0.5em 0;
            color: #0078D4;
        }
        
        .date {
            color: #666;
            font-size: 0.9em;
            margin: 0.2em 0;
        }
        
        .tags {
            margin-top: 0.5em;
        }
        
        .tag {
            background: #00CC6A;
            color: white;
            padding: 0.2em 0.5em;
            border-radius: 4px;
            font-size: 0.8em;
            margin-right: 0.5em;
        }
        
        .content {
            line-height: 1.8;
        }
        
        .content h1, .content h2, .content h3 {
            color: #0078D4;
            margin-top: 1.5em;
        }
        
        .content code {
            background: #f5f5f5;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-family: 'Monaco', 'Consolas', monospace;
        }
        
        .content pre {
            background: #f5f5f5;
            padding: 1em;
            border-radius: 5px;
            overflow-x: auto;
        }
        
        .content blockquote {
            border-left: 4px solid #00CC6A;
            margin: 1em 0;
            padding-left: 1em;
            color: #666;
        }
        
        .collection-title {
            text-align: center;
            color: #0078D4;
            border-bottom: 2px solid #00CC6A;
            padding-bottom: 0.5em;
        }
        
        .table-of-contents {
            background: #f9f9f9;
            padding: 1em;
            border-radius: 5px;
            margin: 2em 0;
        }
        
        .table-of-contents ul {
            list-style-type: none;
            padding-left: 0;
        }
        
        .table-of-contents li {
            margin: 0.5em 0;
        }
        
        .table-of-contents a {
            color: #0078D4;
            text-decoration: none;
        }
        
        .table-of-contents a:hover {
            text-decoration: underline;
        }
        
        .note {
            margin: 2em 0;
        }
        
        .note-metadata {
            color: #666;
            font-size: 0.9em;
            margin-bottom: 1em;
        }
        
        .note-content {
            margin-left: 1em;
        }
        
        @media (max-width: 600px) {
            body {
                padding: 10px;
            }
            
            .container {
                padding: 1em;
            }
        }
        "#
    }
}

impl Default for Exporter {
    fn default() -> Self {
        Self::new()
    }
}