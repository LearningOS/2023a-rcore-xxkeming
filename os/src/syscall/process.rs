//! Process management syscalls
use crate::{
    config::{ MAX_SYSCALL_NUM, PAGE_SIZE },
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, get_current_task_info,
    },
    mm::translated_byte_ptr, timer::get_time_us,
    task::mmap,
    task::munmap
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
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let time = get_time_us();
    let ts = translated_byte_ptr(current_user_token(), ts);
    *ts = TimeVal {
        sec: time / 1_000_000,
        usec: time % 1_000_000
    };
    
    //println!("kernel: sys_get_time {:?}", ts);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    //-1

    let (status, syscall_times, time) = get_current_task_info();
    let ti = translated_byte_ptr(current_user_token(), ti);
    *ti = TaskInfo { status, syscall_times, time: ((time / 1_000_000) & 0xffff) * 1000 + (time % 1_000_000 ) / 1000 };

    //println!("kernel: sys_task_info {:?}", ti.time);
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    //-1
    if start %  PAGE_SIZE != 0 || port & !0x7 != 0 || port &0x7 == 0 {
        return -1;
    }
    mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    //-1
    if start %  PAGE_SIZE != 0 {
        return -1;
    }
    munmap(start, len)
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
