use crate::core::types::{CanonicalPath, OutputFormat, PatternString, ProgressCount, TokenCount};
use crossbeam::channel::{Receiver, Sender};

/// Output generation worker
pub mod generator;

/// Commands sent to worker threads
#[derive(Debug, Clone)]
pub enum WorkerCommand {
    /// Generate output from selected files
    GenerateOutput {
        /// Root directory path
        root_path: CanonicalPath,
        /// List of selected files
        selected_files: Vec<CanonicalPath>,
        /// Output format
        format: OutputFormat,
        /// Whether to include directory tree
        include_tree: bool,
        /// Ignore patterns (comma-separated)
        ignore_patterns: PatternString,
    },
    /// Cancel current operation
    Cancel,
}

/// Events sent from worker threads
#[derive(Debug, Clone)]
pub enum WorkerEvent {
    /// Progress update
    Progress {
        /// Current stage
        stage: ProgressStage,
        /// Progress count
        progress: ProgressCount,
    },
    /// Output generation complete
    OutputReady {
        /// Generated content
        content: String,
        /// Estimated token count
        token_count: TokenCount,
    },
    /// Error occurred
    Error(String),
    /// Operation cancelled
    Cancelled,
}

/// Progress stages for output generation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressStage {
    /// Scanning filesystem
    ScanningFiles,
    /// Reading file contents
    ReadingFiles,
    /// Building final output
    BuildingOutput,
}

/// Handle for communicating with worker thread
#[derive(Debug)]
pub struct WorkerHandle {
    sender: Sender<WorkerCommand>,
    receiver: Receiver<WorkerEvent>,
}

impl WorkerHandle {
    /// Create a new worker handle and spawn worker thread
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

    /// Send command to worker thread
    pub fn send_command(
        &self,
        command: WorkerCommand,
    ) -> Result<(), crossbeam::channel::SendError<WorkerCommand>> {
        self.sender.send(command)
    }

    /// Try to receive event from worker thread
    pub fn try_recv_event(&self) -> Option<WorkerEvent> {
        self.receiver.try_recv().ok()
    }
}
