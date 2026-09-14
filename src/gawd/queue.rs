// SUSI Substrate Pulse Queue
// Mandate 30: Non-Blocking Pulse Ingestion & Mandate 31: Serialized Pulse Execution

use std::sync::{Mutex, OnceLock};
use std::collections::VecDeque;
use std::path::PathBuf;
use tracing::info;
use crate::error::EaiResult;

#[derive(Debug, Clone)]
pub struct PulseEntry {
    pub intent: String,
    pub workspace: PathBuf,
    pub version: String,
    pub priority: u8, // 0: Normal, 1: Correction, 2: High
}

pub struct SubstratePulseQueue {
    queue: Mutex<VecDeque<PulseEntry>>,
}

impl SubstratePulseQueue {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<SubstratePulseQueue> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Non-Blocking Ingestion (Aspiration 31)
    pub fn ingest(&self, intent: &str, workspace: &PathBuf, version: &str) -> EaiResult<()> {
        info!(intent = %intent, "Ingesting new pulse into substrate queue");
        let mut queue = self.queue.lock().unwrap();

        let priority = if intent.to_lowercase().contains("stop") || intent.to_lowercase().contains("wait") || intent.to_lowercase().contains("correction") {
            1
        } else {
            0
        };

        let entry = PulseEntry {
            intent: intent.to_string(),
            workspace: workspace.clone(),
            version: version.to_string(),
            priority,
        };

        if priority > 0 {
            // Priority Insertion: Insert at the front (or after other high-priority entries)
            queue.push_front(entry);
        } else {
            queue.push_back(entry);
        }

        Ok(())
    }

    pub fn pop(&self) -> Option<PulseEntry> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }

    pub fn clear(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.clear();
    }
}
