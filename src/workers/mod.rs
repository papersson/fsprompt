use crossbeam::channel::{Receiver, Sender};
use std::path::PathBuf;

pub mod generator;

#[derive(Debug, Clone)]
pub enum WorkerCommand {
    GenerateOutput {
        root_path: PathBuf,
        selected_files: Vec<PathBuf>,
        format: crate::ui::OutputFormat,
        include_tree: bool,
        ignore_patterns: String,
    },
    Cancel,
}

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    Progress {
        stage: ProgressStage,
        current: usize,
        total: usize,
    },
    OutputReady {
        content: String,
        token_count: usize,
    },
    Error(String),
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressStage {
    ScanningFiles,
    ReadingFiles,
    BuildingOutput,
}

#[derive(Debug)]
pub struct WorkerHandle {
    sender: Sender<WorkerCommand>,
    receiver: Receiver<WorkerEvent>,
}

impl WorkerHandle {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = crossbeam::channel::unbounded();
        let (event_tx, event_rx) = crossbeam::channel::unbounded();

        // Spawn the worker thread
        std::thread::spawn(move || {
            generator::run_worker(cmd_rx, event_tx);
        });

        Self {
            sender: cmd_tx,
            receiver: event_rx,
        }
    }

    pub fn send_command(
        &self,
        command: WorkerCommand,
    ) -> Result<(), crossbeam::channel::SendError<WorkerCommand>> {
        self.sender.send(command)
    }

    pub fn try_recv_event(&self) -> Option<WorkerEvent> {
        self.receiver.try_recv().ok()
    }
}
