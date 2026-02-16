use colored::Colorize;
use similar::{ChangeTag, TextDiff};

pub fn print_colored_diff(local_content: &str, merged_content: &str) {
    eprintln!("\n{}", "=== Changes applied ===".bold());

    let diff = TextDiff::from_lines(local_content, merged_content);

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                eprint!("{}", format!("- {change}").red());
            }
            ChangeTag::Insert => {
                eprint!("{}", format!("+ {change}").green());
            }
            ChangeTag::Equal => {
                eprint!("  {change}");
            }
        }
    }

    eprintln!("{}", "=== End of changes ===".bold());
}
