use std::collections::HashMap;
use std::process::Command;
use sysinfo::{System, SystemExt};

pub struct SystemInfoFetcher {
    sys: System,
}

fn has_package_manager(package_manager: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("type -p {}", package_manager))
        .status()
        .unwrap()
        .success()
}

fn get_package_count(package_count_command: &str) -> usize {
    let output = Command::new("sh")
        .arg("-c")
        .arg(package_count_command)
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.lines().count()
}

impl SystemInfoFetcher {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self { sys }
    }

    pub fn get_distro(&self) -> String {
        self.sys.name().unwrap()
    }

    // color order: reset, primary, secondary, tertiary, border, header, text
    pub fn get_distro_colors(&self) -> [&'static str; 7] {
        match self.get_distro().as_str() {
            "Arch Linux" => ["\x1B[0m", "\x1B[38;2;0;162;237m", "\x1B[38;2;31;184;255m", "\x1B[38;2;82;200;255m", "\x1B[38;2;200;200;200m", "\x1B[38;2;31;184;255m", "\x1B[38;2;150;150;150m"],
            "Windows" => ["\x1B[0m", "\x1B[38;2;0;162;237m", "\x1B[38;2;31;184;255m", "\x1B[38;2;82;200;255m", "\x1B[38;2;200;200;200m", "\x1B[38;2;31;184;255m", "\x1B[38;2;150;150;150m"],
            _ => ["\x1B[0m", "\x1B[38;2;200;200;200m", "\x1B[38;2;150;150;150m", "\x1B[38;2;100;100;100m", "\x1B[38;2;250;250;250m", "\x1B[38;2;200;200;200m", "\x1B[38;2;150;150;150m"], // default
        }
    }

    pub fn get_distro_nf_glyph(&self) -> String {
        match self.get_distro().as_str() {
            "Arch Linux" => "\u{f303}",
            "Windows" => "\u{e70f}",
            _ => "\u{f128}",
        }.parse().unwrap()
    }

    pub fn get_kernel_version(&self) -> String {
        self.sys.kernel_version().unwrap()
    }

    pub fn get_packages(&self) -> String {

        let mut package_str: String = String::from("");
        let mut packages = HashMap::new();

        // add new line to add package manager support
        packages.insert("pacman", if has_package_manager("pacman-key") { get_package_count("pacman -Qq --color never") } else { 0 });
        packages.insert("scoop", if has_package_manager("scoop") { get_package_count("scoop list") - 5 } else { 0 });

        for (package_manager, package_count) in &packages {
            if package_count > &0_usize {
                package_str.insert_str(package_str.len(), package_manager);
                package_str.insert_str(package_str.len(), format!(" ({}),", package_count).as_str());
            }
        }

        if package_str.len() == 0 {
            package_str.insert_str(0, "no packages detected");
        }

        package_str
    }
}