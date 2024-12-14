use std::{sync::Mutex, thread};

use once_cell::sync::Lazy;
use tokio::{runtime::Runtime, time};
use tracing::error;

use crate::web_server::AppState;

static RUNNERS: Lazy<Mutex<Vec<Box<dyn Runner>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub trait Runner: Sync + Send {
    fn run(&self, state: &AppState) -> anyhow::Result<()>;
}

pub fn register_runner(runner: Box<dyn Runner>) {
    RUNNERS.lock().unwrap().push(runner);
}

pub fn start_background(state: AppState) -> anyhow::Result<()> {
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let state_for_thread = state.clone();
            loop {
                if let Err(e) = run_all(&state_for_thread).await {
                    error!("Failed to run all: {:?}", e);
                }
                time::sleep(time::Duration::from_secs(5)).await;
            }
        });
    });

    Ok(())
}

async fn run_all(state: &AppState) -> anyhow::Result<()> {
    let runners = RUNNERS.lock().unwrap();
    for runner in runners.iter() {
        runner.run(state)?;
    }
    Ok(())
}
