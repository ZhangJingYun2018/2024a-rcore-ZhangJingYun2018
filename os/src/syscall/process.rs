//! Process management syscalls

use crate::{
    config::MAX_SYSCALL_NUM, mm::VirtAddr, task::{
        alloc_memory, change_program_brk, exit_current_and_run_next, free_memory, get_task_info, map_user_stack, suspend_current_and_run_next, TaskStatus
    }, timer::get_time_us
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();

    let time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let data = unsafe {
        core::slice::from_raw_parts(
            &time as *const TimeVal as *const u8,
            core::mem::size_of::<TimeVal>(),
        )
    };
    map_user_stack(_ts as usize, data);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let (status, syscall_times, time) = get_task_info();
    let task_info = TaskInfo {
        status,
        syscall_times,
        time,
    };

    let data = unsafe {
        core::slice::from_raw_parts(
            &task_info as *const TaskInfo as *const u8,
            core::mem::size_of::<TaskInfo>(),
        )
    };
    map_user_stack(_ti as usize, data);
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if port > 0x7 {
        return -1;
    }
    match port as u8 & 0x7 {
        0 => -1,
        p => {
            let svaddr = VirtAddr::from(start);
            let evaddr = VirtAddr::from(start + len);
            if svaddr.aligned(){
                alloc_memory(svaddr, evaddr, p)
            }else {
                -1
            }
            
        }
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");

    let svaddr = VirtAddr::from(_start);
    let evaddr = VirtAddr::from(_start + _len);
    if svaddr.aligned(){
        free_memory(svaddr, evaddr)
    }else {
        -1
    }
    
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
