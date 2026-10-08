use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use anyhow::{Context, Result};
use colored::*;

static CARGO_MUTEX: Mutex<()> = Mutex::new(());

pub struct CrateVerifier {
    root_dir: PathBuf,
}

impl CrateVerifier {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }

    pub fn register_module(&self, target_crate: &str, module_name: &str) -> Result<()> {
        let lib_rs = self.root_dir.join("crates").join(target_crate).join("src").join("lib.rs");
        if !lib_rs.exists() {
            return Ok(());
        }
        let content = std::fs::read_to_string(&lib_rs)?;
        let mod_stmt = format!("pub mod {};", module_name);
        let use_stmt = format!("pub use {}::*;", module_name);
        if !content.contains(&mod_stmt) {
            let mut updated = content.trim_end().to_string();
            updated.push('\n');
            updated.push_str(&mod_stmt);
            updated.push('\n');
            updated.push_str(&use_stmt);
            updated.push('\n');
            std::fs::write(&lib_rs, updated)?;
        }
        Ok(())
    }

    pub fn unregister_module(&self, target_crate: &str, module_name: &str) -> Result<()> {
        let lib_rs = self.root_dir.join("crates").join(target_crate).join("src").join("lib.rs");
        if !lib_rs.exists() {
            return Ok(());
        }
        let content = std::fs::read_to_string(&lib_rs)?;
        let mod_stmt = format!("pub mod {};", module_name);
        let use_stmt = format!("pub use {}::*;", module_name);
        let updated = content.replace(&mod_stmt, "").replace(&use_stmt, "");
        std::fs::write(&lib_rs, updated)?;
        Ok(())
    }

    pub fn write_and_verify(
        &self,
        target_crate: &str,
        module_name: &str,
        rust_code: &str,
        source_rel: &str,
    ) -> Result<bool> {
        let _guard = CARGO_MUTEX.lock().unwrap();

        let target_dir = self.root_dir.join("crates").join(target_crate).join("src");
        std::fs::create_dir_all(&target_dir)?;
        let target_file = target_dir.join(format!("{}.rs", module_name));

        // Save file & register
        std::fs::write(&target_file, rust_code)?;
        self.register_module(target_crate, module_name)?;

        // Run cargo check on specific crate
        let status = Command::new("cargo")
            .arg("check")
            .arg("-p")
            .arg(target_crate)
            .current_dir(&self.root_dir)
            .output()
            .context("Failed to run cargo check")?;

        if status.status.success() {
            // Commit to git
            let _ = Command::new("git")
                .args(["add", target_file.to_str().unwrap()])
                .current_dir(&self.root_dir)
                .output();

            let lib_rs = target_dir.join("lib.rs");
            let _ = Command::new("git")
                .args(["add", lib_rs.to_str().unwrap()])
                .current_dir(&self.root_dir)
                .output();

            let msg = format!("port({}): port {} via blender-cluster ({})", target_crate, module_name, source_rel);
            let _ = Command::new("git")
                .args(["commit", "-m", &msg])
                .current_dir(&self.root_dir)
                .output();

            Ok(true)
        } else {
            let err_msg = String::from_utf8_lossy(&status.stderr);
            let error_lines: Vec<&str> = err_msg.lines().filter(|l| l.trim().starts_with("error") || l.contains("error[E")).take(10).collect();
            println!("    {} Compilation error:\n    {}", "✗".red(), error_lines.join("\n    "));

            // Roll back to keep crate 100% green
            let _ = self.unregister_module(target_crate, module_name);
            let _ = std::fs::remove_file(&target_file);
            Ok(false)
        }
    }
}
