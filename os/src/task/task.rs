//! Types related to task management

use crate::syscall::SYSCALL_CMD_ARRAY;

use super::TaskContext;

/// sys_call max num
pub const SYSCALL_MAX_NUM: usize = SYSCALL_CMD_ARRAY.len();
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task trace vector
    pub task_trace_vec: [usize; SYSCALL_MAX_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
