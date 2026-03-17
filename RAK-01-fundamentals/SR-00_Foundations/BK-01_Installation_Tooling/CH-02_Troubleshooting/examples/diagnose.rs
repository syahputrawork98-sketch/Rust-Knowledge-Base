use std::process::Command;

fn main() {
    println!("--- Diagnostik Sistem Rust ---");

    // 1. Cek rustc
    check_tool("rustc", "--version");

    // 2. Cek cargo
    check_tool("cargo", "--version");

    // 3. Cek Linker (C compiler)
    #[cfg(windows)]
    check_tool("cl", ""); // Check for MSVC
    
    #[cfg(not(windows))]
    check_tool("cc", "--version"); // Check for standard CC

    println!("-------------------------------");
    println!("Jika semua 'OK', bengkel Anda siap digunakan!");
}

fn check_tool(name: &str, arg: &str) {
    let status = if arg.is_empty() {
        Command::new(name).status()
    } else {
        Command::new(name).arg(arg).status()
    };

    match status {
        Ok(s) if s.success() => println!("[OK] {} ditemukan.", name),
        _ => println!("[FAIL] {} TIDAK ditemukan atau bermasalah.", name),
    }
}
