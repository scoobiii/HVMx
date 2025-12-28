// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: partition.rs
// Location: crates/hvmx-scheduler/src/partition.rs
// Purpose: Task partitioning strategies
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

use crate::{Task, Backend};

/// Partition strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionStrategy {
    /// All tasks on CPU
    AllCPU,
    
    /// All tasks on GPU
    AllGPU,
    
    /// Split by task size threshold
    SizeThreshold(usize),
    
    /// Round-robin between backends
    RoundRobin,
}

/// Task partition result
#[derive(Debug, Clone)]
pub struct Partition {
    pub cpu_tasks: Vec<Task>,
    pub gpu_tasks: Vec<Task>,
}

impl Partition {
    pub fn new() -> Self {
        Self {
            cpu_tasks: Vec::new(),
            gpu_tasks: Vec::new(),
        }
    }
    
    pub fn total_tasks(&self) -> usize {
        self.cpu_tasks.len() + self.gpu_tasks.len()
    }
}

impl Default for Partition {
    fn default() -> Self {
        Self::new()
    }
}

/// Partition tasks according to strategy
pub fn partition_tasks(tasks: &[Task], strategy: PartitionStrategy) -> Partition {
    let mut partition = Partition::new();
    
    match strategy {
        PartitionStrategy::AllCPU => {
            partition.cpu_tasks = tasks.to_vec();
        }
        PartitionStrategy::AllGPU => {
            partition.gpu_tasks = tasks.to_vec();
        }
        PartitionStrategy::SizeThreshold(threshold) => {
            for task in tasks {
                if task.size >= threshold {
                    partition.gpu_tasks.push(task.clone());
                } else {
                    partition.cpu_tasks.push(task.clone());
                }
            }
        }
        PartitionStrategy::RoundRobin => {
            for (i, task) in tasks.iter().enumerate() {
                if i % 2 == 0 {
                    partition.cpu_tasks.push(task.clone());
                } else {
                    partition.gpu_tasks.push(task.clone());
                }
            }
        }
    }
    
    partition
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_all_cpu() {
        let tasks = vec![
            Task::new(1, 100, Backend::CPU),
            Task::new(2, 200, Backend::CPU),
        ];
        
        let partition = partition_tasks(&tasks, PartitionStrategy::AllCPU);
        assert_eq!(partition.cpu_tasks.len(), 2);
        assert_eq!(partition.gpu_tasks.len(), 0);
    }

    #[test]
    fn test_partition_all_gpu() {
        let tasks = vec![
            Task::new(1, 100, Backend::GPU),
            Task::new(2, 200, Backend::GPU),
        ];
        
        let partition = partition_tasks(&tasks, PartitionStrategy::AllGPU);
        assert_eq!(partition.cpu_tasks.len(), 0);
        assert_eq!(partition.gpu_tasks.len(), 2);
    }

    #[test]
    fn test_partition_size_threshold() {
        let tasks = vec![
            Task::new(1, 50, Backend::CPU),
            Task::new(2, 150, Backend::GPU),
            Task::new(3, 200, Backend::GPU),
        ];
        
        let partition = partition_tasks(&tasks, PartitionStrategy::SizeThreshold(100));
        assert_eq!(partition.cpu_tasks.len(), 1);
        assert_eq!(partition.gpu_tasks.len(), 2);
    }

    #[test]
    fn test_partition_round_robin() {
        let tasks = vec![
            Task::new(1, 100, Backend::CPU),
            Task::new(2, 100, Backend::GPU),
            Task::new(3, 100, Backend::CPU),
            Task::new(4, 100, Backend::GPU),
        ];
        
        let partition = partition_tasks(&tasks, PartitionStrategy::RoundRobin);
        assert_eq!(partition.cpu_tasks.len(), 2);
        assert_eq!(partition.gpu_tasks.len(), 2);
    }

    #[test]
    fn test_partition_total_tasks() {
        let tasks = vec![
            Task::new(1, 100, Backend::CPU),
            Task::new(2, 200, Backend::GPU),
        ];
        
        let partition = partition_tasks(&tasks, PartitionStrategy::AllCPU);
        assert_eq!(partition.total_tasks(), 2);
    }
}
