// CF-VOID Finding Module
// Vulnerability findings data structure

use serde::{Deserialize, Serialize};

/// Vulnerability severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    /// Parse severity from string
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "CRITICAL" => Severity::Critical,
            "HIGH" => Severity::High,
            "MEDIUM" => Severity::Medium,
            "LOW" => Severity::Low,
            "INFO" => Severity::Info,
            _ => Severity::Info,
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub title: String,
    pub severity: Severity,
    pub url: String,
    pub evidence: String,
    pub module: String,
    pub details: Option<String>,
    pub remediation: Option<String>,
}

impl Finding {
    /// Create new finding
    pub fn new(
        title: &str,
        severity: Severity,
        url: &str,
        evidence: &str,
        module: &str,
    ) -> Self {
        Self {
            title: title.to_string(),
            severity,
            url: url.to_string(),
            evidence: evidence.to_string(),
            module: module.to_string(),
            details: None,
            remediation: None,
        }
    }

    /// Create finding with details
    pub fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }

    /// Create finding with remediation
    pub fn with_remediation(mut self, remediation: &str) -> Self {
        self.remediation = Some(remediation.to_string());
        self
    }
}

/// Scan result containing findings and metadata
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub findings: Vec<Finding>,
    pub requests_made: usize,
    pub duration_secs: f64,
}

impl ScanResult {
    /// Create new scan result
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            requests_made: 0,
            duration_secs: 0.0,
        }
    }

    /// Add finding
    pub fn add_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }

    /// Get findings count
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Get findings by severity
    pub fn findings_by_severity(&self, severity: &Severity) -> Vec<&Finding> {
        self.findings.iter()
            .filter(|f| &f.severity == severity)
            .collect()
    }

    /// Merge another result into this one
    pub fn merge(&mut self, other: ScanResult) {
        self.findings.extend(other.findings);
        self.requests_made += other.requests_made;
    }
}
