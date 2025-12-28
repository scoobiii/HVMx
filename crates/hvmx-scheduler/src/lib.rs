// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: lib.rs
// Location: crates/hvmx-scheduler/src/lib.rs
// Purpose: Heterogeneous scheduler root module
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

pub mod partition;
pub mod adaptive;

pub use partition::{Partition, PartitionStrategy};
pub use adaptive::AdaptiveScheduler;

use thiserror::Error;

/// Scheduler errors
#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("No workers available")]
    NoWorkers,
    
    #[error("Task queue full")]
    QueueFull,
    
    #[error("Invalid partition")]
    InvalidPartition,
}

/// Execution backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Backend {
    CPU,
    GPU,
}

/// Task descriptor
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub size: usize,
    pub backend: Backend,
}

impl Task {
    pub fn new(id: u64, size: usize, backend: Backend) -> Self {
        Self { id, size, backend }
    }
}

/// Scheduler statistics
#[derive(Debug, Clone, Default)]
pub struct SchedulerStats {
    pub tasks_scheduled: u64,
    pub tasks_cpu: u64,
    pub tasks_gpu: u64,
    pub total_time_ms: u64,
}

impl SchedulerStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn record_task(&mut self, task: &Task, time_ms: u64) {
        self.tasks_scheduled += 1;
        self.total_time_ms += time_ms;
        
        match task.backend {
            Backend::CPU => self.tasks_cpu += 1,
            Backend::GPU => self.tasks_gpu += 1,
        }
    }
    
    pub fn avg_time_ms(&self) -> f64 {
        if self.tasks_scheduled == 0 {
            0.0
        } else {
            self.total_time_ms as f64 / self.tasks_scheduled as f64
        }
    }
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, 1024, Backend::GPU);
        assert_eq!(task.id, 1);
        assert_eq!(task.size, 1024);
        assert_eq!(task.backend, Backend::GPU);
    }

    #[test]
    fn test_scheduler_stats() {
        let mut stats = SchedulerStats::new();
        let task = Task::new(1, 100, Backend::CPU);
        
        stats.record_task(&task, 10);
        assert_eq!(stats.tasks_scheduled, 1);
        assert_eq!(stats.tasks_cpu, 1);
        assert_eq!(stats.total_time_ms, 10);
    }

    #[test]
    fn test_stats_avg_time() {
        let mut stats = SchedulerStats::new();
        stats.record_task(&Task::new(1, 100, Backend::GPU), 10);
        stats.record_task(&Task::new(2, 200, Backend::GPU), 20);
        
        assert_eq!(stats.avg_time_ms(), 15.0);
    }

    #[test]
    fn test_backend_types() {
        assert_ne!(Backend::CPU, Backend::GPU);
    }

    #[test]
    fn test_stats_multiple_backends() {
        let mut stats = SchedulerStats::new();
        stats.record_task(&Task::new(1, 100, Backend::CPU), 5);
        stats.record_task(&Task::new(2, 200, Backend::GPU), 10);
        
        assert_eq!(stats.tasks_cpu, 1);
        assert_eq!(stats.tasks_gpu, 1);
    }
}
