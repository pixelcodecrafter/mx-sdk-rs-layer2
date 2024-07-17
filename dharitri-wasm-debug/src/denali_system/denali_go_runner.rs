use std::{path::Path, process::Command};

/// Runs the Andes executable,
/// which reads parses and executes one or more denali tests.
pub fn denali_go<P: AsRef<Path>>(relative_path: P) {
    if cfg!(not(feature = "denali-go-tests")) {
        return;
    }

    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);

    let output = Command::new("denali-test")
        .arg(absolute_path)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
    } else {
        panic!(
            "Denali-go output:\n{}\n{}",
            String::from_utf8_lossy(output.stdout.as_slice()),
            String::from_utf8_lossy(output.stderr.as_slice())
        );
    }
}
