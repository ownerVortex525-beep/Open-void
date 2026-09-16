// CF-VOID AI Prompt Templates
// Author: CYBER-FORCE
// Predefined prompts for AI recon, planning, analysis

pub fn system_prompt() -> &'static str {
    "You are CF-VOID AI, an expert penetration testing assistant created by IND 'CYBER-FORCE'. You specialize in:
- Automated security reconnaissance and vulnerability discovery
- Precise attack execution with clear evidence collection
- Intelligent attack chain planning
- Accurate vulnerability severity assessment

You always respond in valid JSON when asked for structured data.
You always provide actionable security findings with proof of concept evidence.
You are methodical, thorough, and prioritize high-impact vulnerabilities first."
}

pub fn recon_prompt(target: &str) -> String {
    let json_template = r#"{
  "ports": [{"port": 80, "service": "http", "version": "Apache 2.4.41"}, ...],
  "web_tech": {"server": "...", "framework": "...", "database": "...", "language": "...", "cdn": "..."},
  "attack_vectors": [
    {"name": "SQL Injection", "confidence": 0.85, "description": "...", "recommended_order": 1}
  ],
  "subdomain_count": 42,
  "headers": {"Server": "...", "X-Powered-By": "..."}
}
"#;

    format!(
        "Perform a comprehensive reconnaissance on this target:\n\nTarget: {}\nScope: All publicly accessible assets, open ports, web services, subdomains\nDepth: Thorough but time-efficient (5-10 minute scan window)\n\nFor each finding, provide structured JSON output:\n{}\nFocus on identifying:\n1. Open ports and running services\n2. Web application frameworks and technologies\n3. Potential injection points\n4. Authentication mechanisms\n5. File upload or download endpoints\n6. Any exposed admin panels or debug interfaces\n7. Subdomain enumeration results\n\nBe concise but thorough. Output ONLY valid JSON.",
        target, json_template
    )
}

pub fn plan_prompt(recon: &ReconResult) -> String {
    let plan_template = r#"{
  "target": "...",
  "steps": [
    {"id": 1, "module": "XSS Scanner", "description": "...", "params": {}, "depends_on": []}
  ],
  "reasoning": "Why this sequence was chosen",
  "total_steps": 4
}
"#;

    format!(
        "You are planning a penetration test attack sequence based on reconnaissance data.\n\nReconnaissance Summary:\n- Target: {}\n- Open Ports: {}\n- Web Tech: {} {} {}\n- Attack Vectors Identified: {}\n- Subdomains: {}\n\nBased on this data, generate a prioritized attack plan in JSON format:\n{}\nConsider:\n1. Highest impact vulnerabilities first\n2. Easiest to exploit first\n3. Dependencies between modules\n4. Stealth considerations\n\nOutput ONLY valid JSON.",
        recon.target,
        recon.ports.len(),
        recon.web_tech.server,
        recon.web_tech.framework,
        recon.web_tech.database,
        recon.attack_vectors.len(),
        recon.subdomain_count,
        plan_template
    )
}

pub fn analyze_prompt(raw_finding: &str) -> String {
    format!(
        "Analyze this security finding from a penetration test:\n\nFinding Data:\n{}\n\nProvide analysis in JSON format:\n{{\n  \"is_true_positive\": true/false,\n  \"confidence\": 0.0-1.0,\n  \"severity\": \"CRITICAL|HIGH|MEDIUM|LOW|INFO\",\n  \"cvss_estimate\": 0.0-10.0,\n  \"description\": \"Why this is a vulnerability or false positive\",\n  \"proof_of_concept\": \"How to verify this manually\",\n  \"evidence\": \"Key evidence from the finding\",\n  \"recommendations\": [\"immediate fix\", \"long-term improvement\"]\n}}\n\nBe thorough and accurate. Output ONLY valid JSON.",
        raw_finding
    )
}

pub fn exploit_prompt(target: &str, vuln: &str, endpoint: &str) -> String {
    format!(
        "You are an expert exploit developer. Given a confirmed vulnerability, generate a step-by-step exploitation plan.\n\nVulnerability: {}\nTarget: {}\nEndpoint: {}\n\nGenerate an exploitation plan in JSON:\n{{\n  \"payload\": \"exact payload to use\",\n  \"method\": \"GET|POST|PUT|...\",\n  \"headers\": {{\"Content-Type\": \"...\", \"Cookie\": \"...\"}},\n  \"data\": \"request body if POST\",\n  \"expected_result\": \"what success looks like\",\n  \"verification\": \"how to verify exploitation succeeded\"\n}}\n\nBe precise with payloads. Output ONLY valid JSON.",
        vuln, target, endpoint
    )
}

pub fn report_prompt(findings: &str) -> String {
    format!(
        "Generate a comprehensive penetration testing report based on these findings:\n\nFindings:\n{}\n\nWrite a professional security report in Markdown format including:\n1. Executive Summary\n2. Methodology\n3. Detailed Findings (organized by severity)\n4. Technical Details with Proof of Concept\n5. Remediation Recommendations\n6. Risk Assessment\n\nFormat with proper Markdown headers, tables, and code blocks where appropriate.",
        findings
    )
}

// ReconResult type (re-exported for plan_prompt)
use super::engine::ReconResult;
