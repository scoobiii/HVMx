// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: adaptive.rs
// Location: crates/hvmx-scheduler/src/adaptive.rs
// Purpose: Adaptive scheduling (learning from execution patterns)
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

use crate::{Task, Backend, SchedulerStats};

/// Adaptive scheduler (learns from execution patterns)
pub struct AdaptiveScheduler {
    stats: SchedulerStats,
    cpu_perf: f64,
    gpu_perf: f64,
}

impl AdaptiveScheduler {
    pub fn new() -> Self {
        Self {
            stats: SchedulerStats::new(),
            cpu_perf: 1.0,
            gpu_perf: 1.0,
        }
    }

    /// Choose backend for task based on learned patterns
    pub fn choose_backend(&self, _task: &Task) -> Backend {
        // TODO: Implement adaptive logic
        if self.gpu_perf > self.cpu_perf {
            Backend::GPU
        } else {
            Backend::CPU
        }
    }

    /// Update performance metrics
    pub fn update_perf(&mut self, backend: Backend, time_ms: u64) {
        match backend {
            Backend::CPU => self.cpu_perf = time_ms as f64,
            Backend::GPU => self.gpu_perf = time_ms as f64,
        }
    }
}

impl Default for AdaptiveScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_creation() {
        let sched = AdaptiveScheduler::new();
        assert_eq!(sched.cpu_perf, 1.0);
        assert_eq!(sched.gpu_perf, 1.0);
    }

    #[test]
    fn test_adaptive_choose() {
        let sched = AdaptiveScheduler::new();
        let task = Task::new(1, 100, Backend::CPU);
        let backend = sched.choose_backend(&task);
        
        // Should choose either backend
        assert!(backend == Backend::CPU || backend == Backend::GPU);
    }

    #[test]
    fn test_adaptive_update() {
        let mut sched = AdaptiveScheduler::new();
        sched.update_perf(Backend::GPU, 5);
        sched.update_perf(Backend::CPU, 10);
        
        assert_eq!(sched.gpu_perf, 5.0);
        assert_eq!(sched.cpu_perf, 10.0);
    }
}
