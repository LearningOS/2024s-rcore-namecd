//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        current_user_token, get_task_info, current_task_alloc_memory, current_task_dealloc_memory,
    },
    mm::{translated_byte_buffer,},
    timer::get_time_us,
};
use core::mem::size_of;

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
    let mut buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, size_of::<TimeVal>());
    if buffers.len() == 0 {
        return -1;
    }
    let time_us = get_time_us();
    let time_val = TimeVal {
        sec: time_us / 1_000_000,
        usec: time_us % 1_000_000,
    };
    let mut time_val_ptr = &time_val as *const TimeVal as *const u8;
    for  buffer in buffers.iter_mut() {
        unsafe {
            buffer.copy_from_slice(core::slice::from_raw_parts(time_val_ptr, buffer.len()));
            time_val_ptr = time_val_ptr.add(buffer.len());
        } 
    }
    
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let buffers = translated_byte_buffer(current_user_token(), _ti as *const u8, size_of::<TaskInfo>());
    if buffers.len() == 0 {
        return -1;
    }
    let (status, syscall_times, time) = get_task_info();
    let task_info = TaskInfo {
        status,
        syscall_times,
        time,
    };
    let mut task_info_ptr = &task_info as *const TaskInfo as *const u8;
    for buffer in buffers {
        unsafe {
            buffer.copy_from_slice(core::slice::from_raw_parts(task_info_ptr, buffer.len()));
            task_info_ptr = task_info_ptr.add(buffer.len());
        }
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _port & !0x7 != 0 || _port & 0x7 == 0{
        return -1;
    }
    current_task_alloc_memory(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    current_task_dealloc_memory(_start, _len)
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
