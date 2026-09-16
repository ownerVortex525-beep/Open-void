// CF-VOID Output Module
// Formatted terminal output

use colored::*;
use std::io::{self, Write};

/// Print a progress bar
pub fn print_progress_bar(current: usize, total: usize, prefix: &str) {
    let width = 40;
    let progress = (current as f64 / total as f64 * width as f64) as usize;
    let bar: String = "█".repeat(progress) + &"░".repeat(width - progress);
    let percent = (current as f64 / total as f64 * 100.0) as usize;
    
    print!("\r  {} [{}] {}% ({}/{})", 
        prefix.bright_cyan(), 
        bar.bright_green(), 
        percent.to_string().bright_yellow(),
        current.to_string().bright_white(),
        total.to_string().bright_white()
    );
    io::stdout().flush().unwrap_or(());
}

/// Print a spinner
pub fn print_spinner(message: &str) {
    static SPINNERS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    static mut INDEX: usize = 0;
    
    unsafe {
        let spinner = SPINNERS[INDEX % SPINNERS.len()];
        INDEX = (INDEX + 1) % SPINNERS.len();
        print!("\r  {} {}", spinner.bright_cyan(), message.bright_white());
        io::stdout().flush().unwrap_or(());
    }
}

/// Print a table
pub fn print_table(headers: &[String], rows: &[Vec<String>]) {
    if headers.is_empty() || rows.is_empty() {
        return;
    }
    
    // Calculate column widths
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }
    
    // Print header
    print!("  ");
    for (i, header) in headers.iter().enumerate() {
        print!("{:<width$} ", header.bright_white().bold(), width = widths[i]);
    }
    println!();
    
    // Print separator
    print!("  ");
    for width in &widths {
        print!("{} ", "─".repeat(*width).bright_cyan());
    }
    println!();
    
    // Print rows
    for row in rows {
        print!("  ");
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                print!("{:<width$} ", cell.bright_green(), width = widths[i]);
            }
        }
        println!();
    }
}

/// Clear current line
pub fn clear_line() {
    print!("\r{}\r", " ".repeat(80));
    io::stdout().flush().unwrap_or(());
}

/// Print a separator line
pub fn print_separator() {
    println!("{}", "─".repeat(60).bright_cyan());
}

/// Print a section header
pub fn print_section(title: &str) {
    println!();
    println!("  {}", format!("═══ {} ═══", title).bright_magenta().bold());
    println!();
}

/// Print info message
pub fn print_info(message: &str) {
    println!("  {} {}", "[*]".bright_cyan().bold(), message.bright_cyan());
}

/// Print error message
pub fn print_error(message: &str) {
    println!("  {} {}", "[-]".bright_red().bold(), message.bright_red());
}

/// Print success message
pub fn print_success(message: &str) {
    println!("  {} {}", "[+]".bright_green().bold(), message.bright_green());
}

/// Print warning message
pub fn print_warning(message: &str) {
    println!("  {} {}", "[!]".bright_yellow().bold(), message.bright_yellow());
}
