use std::error::Error;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the existing README.md
    let readme_content = fs::read_to_string("README.md")?;

    // Run the buup list command from the root crate
    let output = Command::new("cargo")
        .args(["run", "--bin", "buup", "--", "list"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to run buup: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Get the output as a string
    let buup_output = String::from_utf8(output.stdout)?;

    // Create a formatted section for the README
    let transformer_section = format!(
        "## Available Transformers\n\n\
        The following transformers are currently available in Buup:\n\n\
        ```bash\n\
        {}\
        ```\n\n",
        buup_output
    );

    // Check if the section already exists in the README
    let section_marker_start = "## Available Transformers";
    let section_marker_end = "## Update README.md with `buup list`";

    let new_readme = if readme_content.contains(section_marker_start) {
        // Replace the existing section
        let before_section = readme_content
            .split(section_marker_start)
            .next()
            .unwrap_or("");

        let after_section = readme_content
            .split(section_marker_end)
            .nth(1)
            .unwrap_or("");

        format!(
            "{}{}{}{}",
            before_section, transformer_section, section_marker_end, after_section
        )
    } else {
        // Insert the new section before "## Update README.md with `buup list`"
        let parts: Vec<&str> = readme_content.split(section_marker_end).collect();
        if parts.len() >= 2 {
            format!(
                "{}{}{}{}",
                parts[0], transformer_section, section_marker_end, parts[1]
            )
        } else {
            // Fallback: just append to the end
            format!("{}\n\n{}", readme_content, transformer_section)
        }
    };

    // Write the updated README
    fs::write("README.md", new_readme)?;

    println!("README.md has been updated with the latest transformer information.");

    Ok(())
}
