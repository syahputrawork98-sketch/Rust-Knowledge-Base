use std::fs;
use std::path::Path;
use std::process;

fn main() {
    println!("--- Sentinel Audit (Rust Edition: Fearless Guardian) ---");
    let base_path = Path::new("."); // Assuming run from root
    println!("Auditing: {:?}", fs::canonicalize(base_path).unwrap());

    let mut errors = Vec::new();

    // 1. Check Standards
    let required_files = vec![
        "architecture.md", "conventions.md", "workflow.md", 
        "status-protocol.md", "contribution.md", "core-contribution.md"
    ];
    for f in required_files {
        let p = Path::new("docs/standards").join(f);
        if !p.exists() {
            errors.push(format!("Missing standard file: {}", f));
        }
    }

    // 2. Audit Structure
    if let Err(e) = audit_dir(base_path, &mut errors) {
        println!("Error walking directory: {}", e);
    }

    if errors.is_empty() {
        println!("[PASS] Everything is perfectly standardized! (Memory Safe & Gold Standard)");
    } else {
        println!("[FAIL] Found {} inconsistencies:", errors.len());
        for err in errors {
            println!(" - {}", err);
        }
        process::exit(1);
    }
}

fn audit_dir(dir: &Path, errors: &mut Vec<String>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap();
                if name.starts_with('.') || name == "target" || name == "scripts" || name == "docs" {
                    continue;
                }

                if name.starts_with("RAK-") || name.starts_with("SR-") || name.starts_with("BK-") || name.starts_with("CH-") {
                    if !path.join("README.md").exists() {
                        errors.push(format!("Missing README.md in {:?}", path));
                    }
                    if name.starts_with("CH-") {
                        if !path.join("assets").exists() {
                            errors.push(format!("Missing 'assets/' folder in {:?}", path));
                        }
                        if !path.join("examples").exists() {
                            errors.push(format!("Missing 'examples/' folder in {:?}", path));
                        }
                    }
                }
                audit_dir(&path, errors)?;
            }
        }
    }
    Ok(())
}
