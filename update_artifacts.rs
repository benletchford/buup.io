use std::error::Error;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Update README.md
    update_readme()?;

    // Generate sitemap.xml
    generate_sitemap()?;

    Ok(())
}

fn update_readme() -> Result<(), Box<dyn Error>> {
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
    let section_marker_end = "### Update README.md with `buup list`";

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

fn generate_sitemap() -> Result<(), Box<dyn Error>> {
    // Get all transformers from the buup library
    let _transformers = buup::all_transformers(); // Unused but kept for clarity
    let categorized = buup::categorized_transformers();

    // Start building the sitemap XML
    let mut sitemap_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
   <url>
      <loc>https://buup.io/</loc>
      <priority>1.0</priority>
      <changefreq>weekly</changefreq>
   </url>
"#
    .to_string();

    // Helper function to get priority based on category
    let get_priority = |transformer: &dyn buup::Transform| -> &str {
        match transformer.category() {
            buup::TransformerCategory::Encoder => "0.9",
            buup::TransformerCategory::Decoder => "0.9",
            buup::TransformerCategory::Formatter => "0.8",
            buup::TransformerCategory::Crypto => "0.8",
            buup::TransformerCategory::Compression => "0.9",
            buup::TransformerCategory::Color => "0.8",
            buup::TransformerCategory::Other => "0.7",
        }
    };

    // Define category order
    let category_order = [
        buup::TransformerCategory::Encoder,
        buup::TransformerCategory::Decoder,
        buup::TransformerCategory::Compression,
        buup::TransformerCategory::Formatter,
        buup::TransformerCategory::Crypto,
        buup::TransformerCategory::Color,
        buup::TransformerCategory::Other,
    ];

    // Add entries in the predefined category order
    for category in &category_order {
        if let Some(transformers) = categorized.get(category) {
            sitemap_content.push_str(&format!("   <!-- {} related transformers -->\n", category));

            // Sort transformers by ID for consistent ordering
            let mut sorted_transformers = transformers.to_vec();
            sorted_transformers.sort_by_key(|t| t.id());

            for transformer in sorted_transformers {
                sitemap_content.push_str(&format!(
                    "   <url>\n      <loc>https://buup.io/#{}</loc>\n      <priority>{}</priority>\n      <changefreq>monthly</changefreq>\n   </url>\n",
                    transformer.id(),
                    get_priority(transformer)
                ));
            }
        }
    }

    // Close the sitemap
    sitemap_content.push_str("</urlset>");

    // Write the sitemap to the buup_web/assets directory
    fs::write("buup_web/assets/sitemap.xml", sitemap_content)?;

    println!("Sitemap has been generated at buup_web/assets/sitemap.xml");

    Ok(())
}
