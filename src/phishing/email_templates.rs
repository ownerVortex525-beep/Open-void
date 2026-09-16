// CF-VOID Email Template Generator
// Author: CYBER-FORCE
// HTML email templates for phishing campaigns

use crate::cli::banner;

pub struct EmailTemplate {
    pub name: String,
    pub subject: String,
    pub html_body: String,
    pub link_url: String,
}

impl EmailTemplate {
    pub fn birthday_email(name: &str, link: &str) -> Self {
        let subject = format!("Happy Birthday {}! 🎂 Special surprises await!", name);
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f0f2f5; padding: 20px; }}
.container {{ max-width: 600px; margin: 0 auto; background: white; border-radius: 12px; overflow: hidden; }}
.header {{ background: linear-gradient(135deg, #ff6b6b 0%, #ffa502 100%); color: white; padding: 40px; text-align: center; }}
.header h1 {{ font-size: 28px; margin-bottom: 10px; }}
.header p {{ font-size: 16px; opacity: 0.9; }}
.content {{ padding: 30px; }}
.btn {{ display: inline-block; background: #ff6b6b; color: white; text-decoration: none; padding: 14px 32px; border-radius: 30px; font-weight: bold; font-size: 16px; margin: 20px 0; }}
.message {{ font-size: 16px; color: #555; line-height: 1.7; }}
.footer {{ text-align: center; padding: 20px; color: #999; font-size: 12px; }}
</style></head><body>
<div class="container">
<div class="header">
<h1>🎉 Happy Birthday, {}!</h1>
<p>We've prepared something special for you</p>
</div>
<div class="content">
<p class="message">Dear {},</p>
<p class="message">Happy Birthday! Today is all about celebrating you. We've put together some special surprises and exclusive offers just for you!</p>
<p class="message">Click the button below to see your special birthday gifts:</p>
<center><a href="{}" class="btn">🎁 View Your Birthday Gifts</a></center>
<p class="message">Don't miss out on these exclusive offers valid only for today!</p>
</div>
<div class="footer">
This email was sent to you because you're on our birthday celebration list.<br>
<a href="{}">View in browser</a>
</div>
</div>
    </body></html>"#, name, name, link, link);
        Self { name: "birthday".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn love_email(name: &str, link: &str) -> Self {
        let subject = format!("A Special Message For You, {} 💖", name);
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Georgia', serif; background: #fff5f5; padding: 20px; }}
.container {{ max-width: 500px; margin: 0 auto; background: white; border-radius: 16px; box-shadow: 0 4px 20px rgba(0,0,0,0.1); }}
.header {{ text-align: center; padding: 30px; }}
.header .heart {{ font-size: 50px; }}
.message {{ padding: 30px; font-size: 18px; color: #333; line-height: 1.8; text-align: center; }}
.btn {{ display: block; margin: 30px auto; background: #ff4757; color: white; text-decoration: none; padding: 14px 36px; border-radius: 30px; font-weight: bold; font-size: 16px; }}
.footer {{ text-align: center; padding: 15px; color: #999; font-size: 12px; }}
</style></head><body>
<div class="container">
<div class="header">
<div class="heart">💖 💌 💝</div>
</div>
<div class="message">
<p>My dearest {},</p>
<p>There's something I've been wanting to tell you. Something important...</p>
<p>Click below to read my heartfelt message:</p>
<a href="{}" class="btn">💌 Read My Message</a>
</div>
<div class="footer">
You received this message with affection.<br>
<a href="{}">View message</a>
</div>
</div>
    </body></html>"#, name, link, link);
        Self { name: "love".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn offer_email(name: &str, link: &str) -> Self {
        let subject = format!("{} - Exclusive 70% Off Just For You! 🔥", name);
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #0f0f23; padding: 20px; }}
.container {{ max-width: 550px; margin: 0 auto; background: #1a1a2e; border-radius: 12px; overflow: hidden; }}
.header {{ background: linear-gradient(135deg, #ff0000 0%, #ff6b6b 100%); padding: 30px; text-align: center; color: white; }}
.header h1 {{ font-size: 24px; }}
.badge {{ display: inline-block; background: #ffd700; color: #000; padding: 5px 15px; border-radius: 20px; font-weight: bold; margin-top: 10px; }}
.content {{ padding: 30px; }}
.offer {{ font-size: 32px; font-weight: bold; color: #ffd700; text-align: center; margin: 20px 0; }}
.btn {{ display: block; margin: 20px auto; background: #ff6b6b; color: white; text-decoration: none; padding: 14px 32px; border-radius: 30px; font-weight: bold; }}
.details {{ font-size: 14px; color: #aaa; line-height: 1.6; }}
</style></head><body>
<div class="container">
<div class="header">
<h1>Exclusive Offer Just For You!</h1>
<div class="badge">LIMITED TIME - 24 HOURS ONLY</div>
</div>
<div class="content">
<p style="color: #fff; font-size: 16px;">Dear {},</p>
<p class="offer">70% OFF</p>
<p class="details">This exclusive discount is only available for the next 24 hours. Use code <strong style="color:#ffd700;">WELCOME70</strong> at checkout.</p>
<a href="{}" class="btn">🔥 Claim Your Discount Now</a>
<p class="details">Don't miss this limited-time offer. Expires soon.<br>This offer is personalized just for you.</p>
</div>
</div>
</body></html>"#, name, link);
        Self { name: "offer".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn card_email(name: &str, link: &str) -> Self {
        let subject = format!("You've received a card from someone special, {} 💌", name);
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f5f7fa; padding: 20px; }}
.container {{ max-width: 500px; margin: 0 auto; background: white; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1); }}
.header {{ background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%); padding: 40px; text-align: center; }}
.header .icon {{ font-size: 50px; margin-bottom: 10px; }}
.content {{ padding: 30px; text-align: center; }}
.btn {{ display: inline-block; background: #ff6b6b; color: white; text-decoration: none; padding: 12px 28px; border-radius: 25px; font-weight: bold; margin: 20px 0; }}
</style></head><body>
<div class="container">
<div class="header">
<div class="icon">💌📬💝</div>
<h1 style="color: #2d3436;">You've Received a Card!</h1>
</div>
<div class="content">
<p>Hello {},</p>
<p>Someone special has sent you a personalized greeting card!</p>
<a href="{}" class="btn">💌 Open Your Card</a>
<p style="margin-top: 20px; color: #888; font-size: 13px;">This card was specially created for you.</p>
</div>
</div>
</body></html>"#, name, link);
        Self { name: "card".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn prize_email(name: &str, link: &str) -> Self {
        let subject = format!("{} - You Won a Prize! 🎁 Claim Now", name);
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f0f2f5; padding: 20px; }}
.container {{ max-width: 500px; margin: 0 auto; background: white; border-radius: 12px; overflow: hidden; }}
.header {{ background: #4CAF50; padding: 30px; text-align: center; color: white; }}
.header .trophy {{ font-size: 50px; }}
.content {{ padding: 30px; text-align: center; }}
.btn {{ display: inline-block; background: #4CAF50; color: white; text-decoration: none; padding: 14px 30px; border-radius: 8px; font-weight: bold; }}
</style></head><body>
<div class="container">
<div class="header">
<div class="trophy">🏆</div>
<h1>Congratulations, {}!</h1>
</div>
<div class="content">
<p>You've been selected as our lucky winner!</p>
<p style="font-size: 20px; font-weight: bold; color: #4CAF50; margin: 20px 0;">🎁 $500 Gift Card 🎁</p>
<p>Please click the button below to claim your prize:</p>
<a href="{}" class="btn">Claim Your Prize Now</a>
<p style="margin-top: 20px; color: #999; font-size: 12px;">Winner must claim within 24 hours.</p>
</div>
</div>
</body></html>"#, name, link);
        Self { name: "prize".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn invoice_email(name: &str, link: &str) -> Self {
        let subject = format!("Invoice #INV-{} - Payment Required", chrono::Local::now().format("%Y%m%d"));
        let amount = "1,247.50";
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f5f5f5; padding: 20px; }}
.container {{ max-width: 550px; margin: 0 auto; background: white; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
.header {{ background: #1a73e8; color: white; padding: 20px; }}
.table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
.table th {{ background: #f0f0f0; padding: 10px; text-align: left; }}
.table td {{ padding: 10px; border-bottom: 1px solid #eee; }}
.total {{ font-weight: bold; font-size: 18px; }}
.btn {{ display: inline-block; background: #1a73e8; color: white; text-decoration: none; padding: 12px 28px; border-radius: 6px; }}
</style></head><body>
<div class="container">
<div class="header">
<h2>Invoice</h2>
<p>{}</p>
</div>
<table class="table">
<tr><th>Description</th><th>Amount</th></tr>
<tr><td>Professional Services</td><td>${}</td></tr>
<tr><td>Tax (10%)</td><td>${}</td></tr>
<tr><td class="total">Total Due</td><td class="total">${}</td></tr>
</table>
<p>Payment is required within 48 hours.</p>
<a href="{}" class="btn">💳 Pay Now</a>
</div>
</body></html>"#, chrono::Local::now().format("%B %d, %Y"), amount, "247.50", "1,495.00", link);
        Self { name: "invoice".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn shipping_email(name: &str, link: &str) -> Self {
        let subject = format!("{}/ Package Delivery - Action Required", chrono::Local::now().format("%Y%m%d"));
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f8f9fa; padding: 20px; }}
.container {{ max-width: 550px; margin: 0 auto; background: white; border-radius: 8px; }}
.header {{ background: #4CAF50; color: white; padding: 20px; }}
.detail {{ padding: 20px; }}
.detail p {{ margin: 8px 0; }}
.btn {{ display: inline-block; background: #4CAF50; color: white; text-decoration: none; padding: 12px 28px; border-radius: 6px; }}
</style></head><body>
<div class="container">
<div class="header">
<h2>📦 Package Delivery</h2></div>
<div class="detail">
<p><strong>To:</strong> {}</p>
<p><strong>Date:</strong> {}</p>
<p><strong>Tracking:</strong> CF-VOID-{}</p>
<p><strong>Status:</strong> Delivery Attempt Failed</p>
<p>A delivery attempt was made but no one was available to receive the package.</p>
<a href="{}" class="btn">📅 Reschedule Delivery</a>
</div>
</div>
</body></html>"#, name, chrono::Local::now().format("%B %d, %Y"), chrono::Local::now().format("%m%d%H%M"), link);
        Self { name: "shipping".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn bank_email(name: &str, link: &str) -> Self {
        let subject = format!("{} - Security Alert: Login from new device", chrono::Local::now().format("%B %d"));
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f5f5f5; padding: 20px; }}
.container {{ max-width: 500px; margin: 0 auto; background: white; border-radius: 8px; }}
.header {{ background: #e53935; color: white; padding: 20px; }}
.content {{ padding: 20px; }}
.btn {{ display: inline-block; background: #e53935; color: white; text-decoration: none; padding: 12px 28px; border-radius: 6px; font-weight: bold; }}
.detail {{ background: #f5f5f5; padding: 15px; border-radius: 6px; margin: 15px 0; font-family: monospace; font-size: 13px; }}
</style></head><body>
<div class="container">
<div class="header">
<h2>🔒 Security Alert</h2>
<p>Unusual activity detected on your account</p>
</div>
<div class="content">
<p>Dear Customer,</p>
<p>We noticed a login attempt from a new device on {} at {}:</p>
<div class="detail">Device: Unknown<br>Location: {}<br>Browser: Chrome</div>
<p>If this was not you, please verify your identity immediately:</p>
<a href="{}" class="btn">Verify Your Identity</a>
<p style="font-size: 12px; color: #999; margin-top: 20px;">If you don't verify within 24 hours, your account will be temporarily suspended.</p>
</div>
</div>
</body></html>"#, chrono::Local::now().format("%B %d, %Y"), chrono::Local::now().format("%I:%M %p"), "Unknown Location", link);
        Self { name: "bank".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn crypto_email(name: &str, link: &str) -> Self {
        let subject = format!("{} - Crypto Wallet Verification Required", name);
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #0d1117; padding: 20px; }}
.container {{ max-width: 500px; margin: 0 auto; background: #161b22; border-radius: 8px; color: #c9d1d9; }}
.header {{ background: #238636; color: white; padding: 20px; }}
.content {{ padding: 20px; }}
.btn {{ display: inline-block; background: #238636; color: white; text-decoration: none; padding: 12px 28px; border-radius: 6px; font-weight: bold; }}
.qr {{ font-family: monospace; font-size: 14px; background: #0d1117; padding: 10px; border-radius: 6px; margin: 10px 0; }}
</style></head><body>
<div class="container">
<div class="header">
<h2>💰 Wallet Verification</h2>
<p>DeFiChain - Secure Connection</p>
</div>
<div class="content">
<p>Hello {},</p>
<p>We've detected unusual activity on your wallet. Please verify your identity:</p>
<p style="color: #ffd700; font-weight: bold;">Wallet: 0x7Fb68...{}...</p>
<p>A verification process has been started. Please confirm your credentials:</p>
<a href="{}" class="btn">🔐 Verify Wallet</a>
</div>
</div>
</body></html>"#, chrono::Local::now().format("%Y%m%d"), "A4F2", link);
        Self { name: "crypto".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn tax_email(name: &str, link: &str) -> Self {
        let subject = format!("{} - IRS Notice: Refund Available", chrono::Local::now().format("2024"));
        let refund = "2,847.23";
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f0f2f5; padding: 20px; }}
.container {{ max-width: 550px; margin: 0 auto; background: white; border-radius: 8px; }}
.header {{ background: #1a73e8; color: white; padding: 20px; }}
.content {{ padding: 20px; }}
.refund {{ font-size: 28px; font-weight: bold; color: #1a73e8; text-align: center; margin: 20px 0; }}
.btn {{ display: inline-block; background: #1a73e8; color: white; text-decoration: none; padding: 14px 32px; border-radius: 6px; font-weight: bold; }}
</style></head><body>
<div class="container">
<div class="header">
<h2>📄 IRS Notice</h2>
<p>Refund Available: ${}</p>
</div>
<div class="content">
<p>Dear {} {},</p>
<p><strong>Tax Refund ID:</strong> REF-{}</p>
<p><strong>Year:</strong> {}</p>
<p>We have a refund of:</p>
<div class="refund">$ {}</div>
<p>Please verify your information to receive your refund:</p>
<a href="{}" class="btn">💸 Claim Your Refund</a>
<p style="font-size: 12px; color: #999; margin-top: 20px;">Deadline: {} days from today.</p>
</div>
</div>
</body></html>"#, refund, "Valued", name, chrono::Local::now().format("%Y%m%d%H%M"), chrono::Local::now().format("%Y"), refund, link, 30);
        Self { name: "tax".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn meeting_email(name: &str, link: &str) -> Self {
        let subject = format!("{} - Meeting Invitation - {} ", chrono::Local::now().format("%Y%m%d"), "CF-VOID Security");
        let body = format!(r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<style>
body {{ font-family: 'Arial', sans-serif; background: #f8f9fa; padding: 20px; }}
.container {{ max-width: 500px; margin: 0 auto; background: white; border-radius: 8px; }}
.header {{ background: #7b1fa2; color: white; padding: 20px; }}
.content {{ padding: 20px; }}
.detail {{ background: #f3e5f5; padding: 15px; border-radius: 6px; margin: 15px 0; }}
.btn {{ display: inline-block; background: #7b1fa2; color: white; text-decoration: none; padding: 12px 28px; border-radius: 6px; font-weight: bold; }}
</style></head><body>
<div class="container">
<div class="header">
<h2>📅 Meeting Invitation</h2>
<p>CF-VOID Security Team</p>
</div>
<div class="content">
<p>Hi {},</p>
<p>You've been invited to a security review meeting.</p>
<div class="detail">
<strong>Date:</strong> {}<br>
<strong>Time:</strong> {}<br>
<strong>Duration:</strong> 30 minutes<br>
<strong>Location:</strong> {}<br>
</div>
<a href="{}" class="btn">Join Meeting</a>
</div>
</div>
</body></html>"#, name, chrono::Local::now().format("%B %d, %Y"), chrono::Local::now().format("%I:%M %p"), link, link);
        Self { name: "meeting".to_string(), subject, html_body: body, link_url: link.to_string() }
    }

    pub fn generate_all(name: &str, link: &str) -> Vec<EmailTemplate> {
        vec![
            Self::birthday_email(name, link),
            Self::love_email(name, link),
            Self::offer_email(name, link),
            Self::card_email(name, link),
            Self::prize_email(name, link),
            Self::invoice_email(name, link),
            Self::shipping_email(name, link),
            Self::bank_email(name, link),
            Self::crypto_email(name, link),
            Self::tax_email(name, link),
            Self::meeting_email(name, link),
        ]
    }

    pub fn save(&self, output_dir: &str) -> Option<String> {
        let _ = std::fs::create_dir_all(output_dir);
        let filename = format!("{}/{}.html", output_dir, self.name);
        if let Err(e) = std::fs::write(&filename, &self.html_body) {
            banner::error(&format!("Failed to save {}: {}", filename, e));
            return None;
        }
        banner::success(&format!("Email template saved: {}", filename));
        Some(filename)
    }
}

pub fn list_email_templates() {
    banner::info("Available email templates:");
    let templates = [
        ("birthday", "Birthday celebration with special offers"),
        ("love", "Romantic love letter message"),
        ("offer", "Exclusive discount/offer"),
        ("card", "Greeting card from someone special"),
        ("prize", "You won a prize notification"),
        ("invoice", "Fake invoice / payment request"),
        ("shipping", "Package delivery failure notification"),
        ("bank", "Bank security alert"),
        ("crypto", "Crypto wallet verification"),
        ("tax", "Tax refund notification"),
        ("meeting", "Meeting invitation"),
    ];
    for (name, desc) in &templates {
        banner::info(&format!("  {} - {}", name, desc));
    }
}
