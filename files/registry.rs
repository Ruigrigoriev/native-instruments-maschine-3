//! Windows registry read helpers for Native Instruments Maschine 3.

pub const APP_KEY: &str = "Software\\Native\\Native Instruments Maschine 3";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Native Instruments Maschine 3";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Native Instruments Maschine 3") },
        RegValue { name: "Version",     value: String::from("v2.7.18") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Native Instruments Maschine 3") },
        RegValue { name: "Publisher",       value: String::from("Native") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Native Instruments Maschine 3\\uninstall.exe\"") },
    ]
}
