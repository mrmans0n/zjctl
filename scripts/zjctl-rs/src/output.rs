use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Table,
    Quiet,
}

pub fn emit<T: Serialize>(value: &T, format: &OutputFormat, table_fn: impl FnOnce(&T)) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(value).unwrap());
        }
        OutputFormat::Table => table_fn(value),
        OutputFormat::Quiet => {}
    }
}

pub fn emit_ok(format: &OutputFormat) {
    emit(&serde_json::json!({"ok": true}), format, |_| {});
}

pub fn emit_dry_run(command: &[&str], format: &OutputFormat) {
    let val = serde_json::json!({
        "dry_run": true,
        "command": command,
    });
    emit(&val, format, |v| {
        println!("{}", serde_json::to_string_pretty(v).unwrap());
    });
}
