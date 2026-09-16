// CF-VOID Report Generator Module
// Generate reports in various formats

use anyhow::Result;
use std::fs::File;
use std::io::Write;


use crate::core::finding::ScanResult;

/// Report generator
pub struct ReportGenerator {
    result: ScanResult,
}

impl ReportGenerator {
    /// Create new report generator
    pub fn new(result: ScanResult) -> Self {
        Self { result }
    }

    /// Save report in specified format
    pub fn save(&self, path: &str, format: &str) -> Result<()> {
        match format {
            "json" => self.save_json(path),
            "html" => self.save_html(path),
            "csv" => self.save_csv(path),
            "markdown" => self.save_markdown(path),
            _ => {
                anyhow::bail!("Unsupported format: {}", format);
            }
        }
    }

    /// Save as JSON
    pub fn save_json(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.result.findings)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Save as HTML
    pub fn save_html(&self, path: &str) -> Result<()> {
        let html = self.generate_html();
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }

    /// Save as CSV
    pub fn save_csv(&self, path: &str) -> Result<()> {
        let mut wtr = csv::Writer::from_path(path)?;
        
        // Write header
        wtr.write_record(&["Severity", "Title", "URL", "Evidence", "Module"])?;
        
        // Write findings
        for finding in &self.result.findings {
            wtr.write_record(&[
                finding.severity.as_str(),
                &finding.title,
                &finding.url,
                &finding.evidence,
                &finding.module,
            ])?;
        }
        
        wtr.flush()?;
        Ok(())
    }

    /// Save as Markdown
    pub fn save_markdown(&self, path: &str) -> Result<()> {
        let md = self.generate_markdown();
        let mut file = File::create(path)?;
        file.write_all(md.as_bytes())?;
        Ok(())
    }

    /// Generate HTML report
    fn generate_html(&self) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>CF-VOID Scan Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("h1 { color: #333; }\n");
        html.push_str(".finding { border: 1px solid #ccc; padding: 10px; margin: 10px 0; }\n");
        html.push_str(".critical { border-left: 5px solid #dc3545; }\n");
        html.push_str(".high { border-left: 5px solid #fd7e14; }\n");
        html.push_str(".medium { border-left: 5px solid #ffc107; }\n");
        html.push_str(".low { border-left: 5px solid #17a2b8; }\n");
        html.push_str(".info { border-left: 5px solid #6c757d; }\n");
        html.push_str("</style>\n</head>\n<body>\n");
        html.push_str("<h1>CF-VOID Scan Report</h1>\n");
        html.push_str(&format!("<p>Total Findings: {}</p>\n", self.result.finding_count()));
        
        for finding in &self.result.findings {
            let severity_class = finding.severity.as_str().to_lowercase();
            html.push_str(&format!(
                "<div class=\"finding {}\">\n<h2>{} - {}</h2>\n<p><strong>URL:</strong> {}</p>\n<p><strong>Evidence:</strong> {}</p>\n</div>\n",
                severity_class, finding.title, finding.severity, finding.url, finding.evidence
            ));
        }
        
        html.push_str("</body>\n</html>");
        html
    }

    /// Generate Markdown report
    fn generate_markdown(&self) -> String {
        let mut md = String::new();
        
        md.push_str("# CF-VOID Scan Report\n\n");
        md.push_str(&format!("**Total Findings:** {}\n\n", self.result.finding_count()));
        
        for finding in &self.result.findings {
            md.push_str(&format!("## {} [{}]\n\n", finding.title, finding.severity));
            md.push_str(&format!("**URL:** {}\n\n", finding.url));
            md.push_str(&format!("**Evidence:** {}\n\n", finding.evidence));
            md.push_str("---\n\n");
        }
        
        md
    }
}
