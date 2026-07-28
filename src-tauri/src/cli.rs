use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

pub enum CliCommand {
    Show, Hide, Toggle, Edit, Reload, Status,
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
        _ => None,
    }
}

pub fn socket_path() -> PathBuf {
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(runtime).join("zokute/control.sock")
}

pub fn send_command(cmd: &CliCommand) -> Result<String, String> {
    let payload = match cmd {
        CliCommand::Show => "show",
        CliCommand::Hide => "hide",
        CliCommand::Toggle => "toggle",
        CliCommand::Edit => "edit",
        CliCommand::Reload => "reload",
        CliCommand::Status => "status",
    };
    let path = socket_path();
    let mut socket = UnixStream::connect(&path).map_err(|e| format!("connect: {e}"))?;
    socket.write_all(payload.as_bytes()).map_err(|e| format!("write: {e}"))?;
    socket.shutdown(std::net::Shutdown::Write).map_err(|e| format!("shutdown: {e}"))?;
    let mut response = String::new();
    socket.read_to_string(&mut response).map_err(|e| format!("read: {e}"))?;
    Ok(response.trim().to_string())
}
