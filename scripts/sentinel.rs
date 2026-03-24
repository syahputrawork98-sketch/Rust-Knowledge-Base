use std::fs;
use std::path::Path;
use std::process;

fn main() {
    println!("\n\x1b[1;38;5;208m[SENTINEL V4 - RUST EDITION]\x1b[0m");
    println!("--- Fearless Guardian of the Gold Standard ---\n");

    let base_path = Path::new(".");
    let mut errors = Vec::new();

    // 1. Check Standards (The Pillars)
    let required_standards = vec![
        "architecture.md", "conventions.md", "workflow.md", 
        "status-protocol.md", "terminology.md", "aesthetic.md"
    ];
    
    println!("[\x1b[34mSTEP 1\x1b[0m] Auditing Standard Documents...");
    for f in required_standards {
        let p = Path::new("docs/standards").join(f);
        if !p.exists() {
            errors.push(format!("Missing standard file: {}", f));
        }
    }

    // 2. Check 7-RAK Blueprint
    println!("[\x1b[34mSTEP 2\x1b[0m] Auditing 7-RAK Structural Integrity...");
    let racks = vec![
        "RAK-01-anatomy", "RAK-02-foundation", "RAK-03-evolution", 
        "RAK-04-core-mechanics", "RAK-05-ecosystem", "RAK-06-the-underworld", 
        "RAK-07-specialization"
    ];

    for rack in &racks {
        let p = base_path.join(rack);
        if !p.exists() {
            errors.push(format!("Missing core RAK: {}", rack));
        } else if !p.join("README.md").exists() {
            errors.push(format!("Missing README.md in {}", rack));
        }
    }

    // 3. Deep Audit (Recursive)
    println!("[\x1b[34mSTEP 3\x1b[0m] Deep Dive Recursive Validation...");
    if let Err(e) = audit_dir(base_path, &mut errors, &racks) {
        println!("\x1b[31mError walking directory: {}\x1b[0m", e);
    }

    println!("\n--- Audit Results ---");
    if errors.is_empty() {
        println!("\x1b[32m[PASS] All systems nominal. Gold Standard V4 achieved!\x1b[0m\n");
    } else {
        println!("\x1b[31m[FAIL] Found {} inconsistencies:\x1b[0m", errors.len());
        for (i, err) in errors.iter().enumerate() {
            println!("  {:02}. {}", i + 1, err);
        }
        println!("\n\x1b[33mRecommendation: Re-align with docs/standards/ documentation.\x1b[0m\n");
        process::exit(1);
    }
}

fn audit_dir(dir: &Path, errors: &mut Vec<String>, racks: &[&str]) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap();
                
                // Skip non-RAK top-level dirs
                if dir == Path::new(".") && !racks.contains(&name) {
                    continue;
                }

                // Generic Skip logic
                if name.starts_with('.') || name == "target" || name == "scripts" || name == "docs" || name == "assets" || name == "examples" {
                    continue;
                }

                // Structure Rules
                if name.starts_with("RAK-") || name.starts_with("SR-") || name.starts_with("BK-") || name.starts_with("CH-") {
                    if !path.join("README.md").exists() {
                        errors.push(format!("Missing README.md in {:?}", path));
                    }

                    // RAK-01 Flat Structure Rule (RAK -> BK directly)
                    if name == "RAK-01-anatomy" {
                        // Scan for any SR- in RAK-01 (Violates Flat structure)
                        for sub_entry in fs::read_dir(&path)? {
                            let sub_name = sub_entry?.file_name().into_string().unwrap();
                            if sub_name.starts_with("SR-") {
                                errors.push(format!("RAK-01 must be flat. Remove SR: {}", sub_name));
                            }
                        }
                    }

                    // Standard Branching Rules
                    if name.starts_with("CH-") {
                        if !path.join("assets").exists() {
                            errors.push(format!("Missing 'assets/' in {:?}", path));
                        }
                        if !path.join("examples").exists() {
                            errors.push(format!("Missing 'examples/' in {:?}", path));
                        }
                    }
                }
                
                audit_dir(&path, errors, racks)?;
            }
        }
    }
    Ok(())
}
