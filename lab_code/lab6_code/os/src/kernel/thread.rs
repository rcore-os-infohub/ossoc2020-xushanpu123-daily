//! 线程相关的内核功能

use super::*;

pub(super) fn sys_gettid() -> SyscallResult {
    SyscallResult::Proceed(PROCESSOR.lock().current_thread().id)
}

