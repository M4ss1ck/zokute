use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

pub enum CliCommand {
    Show, Hide, Toggle, Edit, Reload, Status,
    ConfigValidate(Option<String>),
    ConfigShow { redact_plugin_config: bool },
    Diagnostics { output: Option<String> },
}

pub fn parse_args(args: &[String]) -> Option<CliCommand> {
    let cmd = args.get(1)?;
    match cmd.as_str() {
        "show" => Some(CliCommand::Show),
        "hide" => Some(CliCommand::Hide),
        "toggle" => Some(CliCommand::Toggle),
        "edit" => Some(CliCommand::Edit),
        "reload" => Some(CliCommand::Reload),
        "status" => Some(CliCommand::Status),
        "config" => parse_config_subcommand(args.get(2), args.get(3)),
        "diagnostics" => {
            let output = if args.get(2) == Some(&"--output".to_string()) {
                args.get(3).cloned()
            } else { None };
            Some(CliCommand::Diagnostics { output })
        }
        _ => None,
    }
}

fn parse_config_subcommand(sub: Option<&String>, arg: Option<&String>) -> Option<CliCommand> {
    match sub?.as_str() {
        "validate" => Some(CliCommand::ConfigValidate(arg.cloned())),
        "show" => {
            let redact = arg == Some(&"--redact-plugin-config".to_string());
            Some(CliCommand::ConfigShow { redact_plugin_config: redact })
        }
        _ => None,
    }
}

pub fn socket_path() -> PathBuf {
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(runtime).join("zokute/control.sock")
}

pub fn send_command(cmd: &CliCommand) -> Result<String, String> {
    let payload: String = match cmd {
        CliCommand::Show => "show".into(),
        CliCommand::Hide => "hide".into(),
        CliCommand::Toggle => "toggle".into(),
        CliCommand::Edit => "edit".into(),
        CliCommand::Reload => "reload".into(),
        CliCommand::Status => "status".into(),
        CliCommand::ConfigValidate(path) => {
            return crate::diagnostics::validate_config(path.as_deref());
        }
        CliCommand::ConfigShow { redact_plugin_config } => {
            serde_json::json!({
                "command": "config_show",
                "redact_plugin_config": redact_plugin_config,
            }).to_string()
        }
        CliCommand::Diagnostics { output: _ } => {
            serde_json::json!({
                "command": "diagnostics",
            }).to_string()
        }
    };
    let path = socket_path();
    let mut socket = UnixStream::connect(&path).map_err(|e| format!("connect: {e}"))?;
    socket.write_all(payload.as_bytes()).map_err(|e| format!("write: {e}"))?;
    socket.shutdown(std::net::Shutdown::Write).map_err(|e| format!("shutdown: {e}"))?;
    let mut response = String::new();
    socket.read_to_string(&mut response).map_err(|e| format!("read: {e}"))?;
    Ok(response.trim().to_string())
}
