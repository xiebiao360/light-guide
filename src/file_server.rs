use std::{fs::File, io::Read};

use crate::FsRunArgs;
use anyhow::Result;
use axum::Router;
use daemonize::Daemonize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const PID_FILE: &str = "/tmp/guide-fs.pid";

pub fn run_server(args: &FsRunArgs) -> Result<()> {
    if args.detach {
        // 配置守护进程
        let stdout = File::create("/tmp/guide-fs.out").unwrap();
        let stderr = File::create("/tmp/guide-fs.err").unwrap();
        let daemonize = Daemonize::new()
            .pid_file(PID_FILE) // PID 文件
            .chown_pid_file(true) // 是否修改 PID 文件的所有者
            .working_directory("/tmp")
            .stdout(stdout) // 标准输出重定向
            .stderr(stderr); // 标准错误重定向

        match daemonize.start() {
            Ok(_) => {
                print!("Start light-guide-fs successfully!");
                start_server(&args)
            }
            Err(e) => return Err(e.into()),
        }
    } else {
        start_server(&args)
    }
}

#[tokio::main]
async fn start_server(args: &FsRunArgs) -> Result<()> {
    let app = Router::new().nest_service("/", ServeDir::new(&args.path));
    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&addr).await?;
    print!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

pub fn stop_server() -> Result<()> {
    if let Ok(mut file) = File::open(PID_FILE) {
        let mut pid = String::new();
        if file.read_to_string(&mut pid).is_ok() {
            if let Ok(pid) = pid.trim().parse::<i32>() {
                // stop pid process use Command
                let output = std::process::Command::new("kill")
                    .arg("-TERM")
                    .arg(format!("{}", pid))
                    .output()?;
                if output.status.success() {
                    print!("Stop light-guide-web successfully!");
                } else {
                    return Err(anyhow::anyhow!(
                        "Failed to stop light-guide-web. Error: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
        }
    }
    Ok(())
}
