use std::fs;
use std::path::Path;
use std::process;

fn main() {
    println!("\n[SENTINEL V4 - RUST EDITION]");
    println!("--- Rust Knowledge Base Integrity Audit ---\n");

    let base_path = Path::new(".");
    let mut errors = Vec::new();

    let required_standards = vec!["README.md", "authoring.md"];

    println!("[STEP 1] Auditing standard documents...");
    for f in required_standards {
        let p = Path::new("docs/standards").join(f);
        if !p.exists() {
            errors.push(format!("Missing standard file: {}", f));
        }
    }

    println!("[STEP 2] Auditing 7-RAK structural integrity...");
    let racks = vec![
        "RAK-01-anatomy",
        "RAK-02-foundation",
        "RAK-03-evolution",
        "RAK-04-core-mechanics",
        "RAK-05-ecosystem",
        "RAK-06-the-underworld",
        "RAK-07-specialization",
    ];

    for rack in &racks {
        let p = base_path.join(rack);
        if !p.exists() {
            errors.push(format!("Missing core RAK: {}", rack));
        } else if !p.join("README.md").exists() {
            errors.push(format!("Missing README.md in {}", rack));
        }
    }

    println!("[STEP 3] Deep recursive validation...");
    if let Err(e) = audit_dir(base_path, &mut errors, &racks) {
        println!("Error walking directory: {}", e);
    }

    println!("\n--- Audit Results ---");
    if errors.is_empty() {
        println!("[PASS] All systems nominal.\n");
    } else {
        println!("[FAIL] Found {} inconsistencies:", errors.len());
        for (i, err) in errors.iter().enumerate() {
            println!("  {:02}. {}", i + 1, err);
        }
        println!("\nRecommendation: Re-align with docs/standards/ documentation.\n");
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

                if dir == Path::new(".") && !racks.contains(&name) {
                    continue;
                }

                if name.starts_with('.')
                    || name == "target"
                    || name == "scripts"
                    || name == "docs"
                    || name == "assets"
                    || name == "examples"
                {
                    continue;
                }

                if name.starts_with("RAK-")
                    || name.starts_with("SR-")
                    || name.starts_with("BK-")
                    || name.starts_with("CH-")
                    || name.starts_with("SEC-")
                {
                    if !path.join("README.md").exists() {
                        errors.push(format!("Missing README.md in {:?}", path));
                    }

                    if name == "RAK-01-anatomy" {
                        for sub_entry in fs::read_dir(&path)? {
                            let sub_name = sub_entry?.file_name().into_string().unwrap();
                            if sub_name.starts_with("SR-") {
                                errors.push(format!("RAK-01 must be flat. Remove SR: {}", sub_name));
                            }
                        }
                    }

                    if name.starts_with("CH-") || name.starts_with("SEC-") {
                        for support_dir in ["assets", "examples"] {
                            let support_path = path.join(support_dir);
                            if support_path.exists() && is_dir_empty(&support_path)? {
                                errors.push(format!("Empty '{}/' in {:?}", support_dir, path));
                            }
                        }
                    }
                }

                audit_dir(&path, errors, racks)?;
            }
        }
    }
    Ok(())
}

fn is_dir_empty(path: &Path) -> std::io::Result<bool> {
    Ok(fs::read_dir(path)?.next().is_none())
}
