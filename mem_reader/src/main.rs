use mach::port::{mach_port_name_t, MACH_PORT_NULL};
use mach::vm_types::{mach_vm_address_t, mach_vm_size_t};
use mach::kern_return::KERN_SUCCESS;
use libc::{c_int};

use std::env;
use std::io;

fn read_mem(pid: usize, addr: usize, buf: &mut [u8]) -> io::Result<()> {
    let port = task_for_pid(pid)?;
    let mut read_len = buf.len() as mach_vm_size_t;
    let result = unsafe {
        mach::vm::mach_vm_read_overwrite(
            port,
            addr as mach_vm_address_t,
            buf.len() as mach_vm_size_t,
            buf.as_mut_ptr() as mach_vm_address_t,
            &mut read_len,
        )
    };

    if read_len != buf.len() as mach_vm_size_t {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Mismatched read sizes for `vm_read` (expected {}, got {})",
                buf.len(),
                read_len
            ),
        ));
    }

    if result != KERN_SUCCESS {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn task_for_pid(pid: usize) -> io::Result<mach_port_name_t> {
    let mut task: mach_port_name_t = MACH_PORT_NULL;

    unsafe {
        let result =
            mach::traps::task_for_pid(mach::traps::mach_task_self(), pid as c_int, &mut task);
        if result != KERN_SUCCESS {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(task)
}

fn main() {
    let pid = env::args().nth(1).unwrap().parse::<usize>().unwrap();
    let addr = usize::from_str_radix(&env::args().nth(2).unwrap(), 16).unwrap();
    let size = env::args().nth(3).unwrap().parse::<usize>().unwrap();
    let mut buf = Vec::new();
    buf.resize(size, 0);

    let mut curr_values = Vec::new();
    loop {
        read_mem(pid, addr, &mut buf).unwrap();
        if buf != curr_values {
            println!("current values: {:?}", buf);
            curr_values = buf.clone();
        }
    }
}