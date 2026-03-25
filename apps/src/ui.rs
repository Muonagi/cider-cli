#![allow(dead_code)]

use std::borrow::Cow;

use console::{Style, style};
use indicatif::{ProgressBar, ProgressStyle};

/// Print a status message: `  • message`
pub fn status(msg: impl Into<Cow<'static, str>>) {
    let prefix = style("•").cyan().bold();
    println!("  {prefix} {}", msg.into());
}

/// Print a success message: `  ✔ message`
pub fn success(msg: impl Into<Cow<'static, str>>) {
    let prefix = style("✔").green().bold();
    println!("  {prefix} {}", msg.into());
}

/// Print a warning message: `  ⚠ message`
pub fn warn(msg: impl Into<Cow<'static, str>>) {
    let prefix = style("⚠").yellow().bold();
    println!("  {prefix} {}", msg.into());
}

/// Print a header/section message: `▸ message`
pub fn header(msg: impl Into<Cow<'static, str>>) {
    println!("{} {}", style("▸").bold(), style(msg.into()).bold());
}

/// Create a spinner with a message. Call `.finish_with_message()` or `.finish_and_clear()` when done.
pub fn spinner(msg: impl Into<Cow<'static, str>>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("  {spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb.set_message(msg.into());
    pb
}

/// Finish a spinner with a success check mark.
pub fn finish_spinner(pb: &ProgressBar, msg: impl Into<Cow<'static, str>>) {
    let check = Style::new().green().bold().apply_to("✔");
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {msg}")
            .unwrap(),
    );
    pb.finish_with_message(format!("{check} {}", msg.into()));
}

/// Finish a spinner with a warning icon.
pub fn finish_spinner_warn(pb: &ProgressBar, msg: impl Into<Cow<'static, str>>) {
    let icon = Style::new().yellow().bold().apply_to("⚠");
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {msg}")
            .unwrap(),
    );
    pb.finish_with_message(format!("{icon} {}", msg.into()));
}

/// Create a progress bar for installation (0-100%).
pub fn progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  {spinner:.cyan} Installing [{bar:30.cyan/dim}] {pos}%")
            .unwrap()
            .progress_chars("━╸─"),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}
