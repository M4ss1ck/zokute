use crate::plugin_manifest::PluginManifest;
use crate::plugin_protocol::{self, PluginOutput, V1Request, PROTOCOL_VERSION};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

pub struct RunnerConfig {
    pub manifest: Arc<PluginManifest>,
    pub manifest_dir: PathBuf,
    pub instance_id: String,
    pub config: toml::Value,
}

pub async fn run_plugin(cfg: &RunnerConfig) -> PluginOutput {
    let request = V1Request {
        protocol: PROTOCOL_VERSION,
        instance_id: cfg.instance_id.clone(),
        config: cfg.config.clone(),
    };
    let request_json = serde_json::to_string(&request).unwrap_or_default();

    let mut cmd = Command::new(&cfg.manifest.command[0]);
    if cfg.manifest.command.len() > 1 {
        cmd.args(&cfg.manifest.command[1..]);
    }
    cmd.current_dir(if let Some(cwd) = &cfg.manifest.cwd {
        PathBuf::from(cwd)
    } else {
        cfg.manifest_dir.clone()
    });
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    cmd.kill_on_drop(true);

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => return PluginOutput::ExitError { code: None, stderr: format!("spawn: {e}") },
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = timeout(Duration::from_secs(2), stdin.write_all(request_json.as_bytes())).await;
        let _ = stdin.shutdown().await;
    }

    let max_output = cfg.manifest.max_output_bytes as usize;
    let result = timeout(Duration::from_secs(cfg.manifest.timeout_seconds), child.wait_with_output()).await;

    match result {
        Ok(Ok(output)) => {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if !output.status.success() {
                return PluginOutput::ExitError { code: output.status.code(), stderr };
            }
            if output.stdout.len() > max_output {
                return PluginOutput::Oversized;
            }
            match plugin_protocol::parse_response(&output.stdout) {
                Ok(response) => PluginOutput::Success(response),
                Err(e) => PluginOutput::ParseError(e),
            }
        }
        Ok(Err(e)) => PluginOutput::ExitError { code: None, stderr: format!("io: {e}") },
        Err(_) => PluginOutput::Timeout,
    }
}
