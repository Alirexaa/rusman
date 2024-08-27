use core::slice;
use std::{ffi::CString, process};

use nix::{
    sched::{clone, CloneFlags},
    sys::signal::{self, Signal},
    unistd::execvp,
};
use rusman::container_runtime::namespaces::{
    mount::mount_namespace, network::network_namespace, pid::pid_namespace,
};

fn main() {
    // Define the stack size for the child process
    let stack_size = 1024 * 1024; // 1MB stack size for the child process

    // Alocate memory for the child process
    let mut child_stack = vec![0; stack_size];

    // Define the flags for the close system call
    let flags = CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNET | CloneFlags::CLONE_NEWNS;

    // Execute the clone system call to create a new process
    match unsafe {
        // Obtain a mutable pointer to the child stack
        let child_stack_ptr = child_stack.as_mut_ptr();

        // Create a slice from the child stack pointer and the stack size
        let child_stack_slice = slice::from_raw_parts_mut(child_stack_ptr, stack_size);

        // Call the clone system call with the chile function, child stack, flags, and None for the closure argument
        clone(Box::new(child_function), child_stack_slice, flags, None)
    } {
        /*
        On success, the PID of the child process is returned in the
        parent, and 0 is returned in the child.  On failure, -1 is
        returned in the parent, no child process is created, and errno is
        set to indicate the error.
        https://man7.org/linux/man-pages/man2/fork.2.html
        and
        https://man7.org/linux/man-pages/man2/clone.2.html
        */
        // According to up document comment 0 pid means that we are in parent proccess
        Ok(child_pid) => {
            // Check if we are in the parent or child process
            if child_pid == nix::unistd::Pid::from_raw(0) {
                // Parent Process
                // Wait for the child process to terminate
                match nix::sys::wait::waitpid(child_pid, None) {
                    Ok(_) => println!("Child process terminated"),
                    Err(err) => eprintln!("Faild wait for child process: {:?}", err),
                }
            } else {
                // Child process
                unsafe {
                    nix::sys::signal::signal(Signal::SIGCHLD, signal::SigHandler::SigIgn)
                        .expect("Faild to set SIGHLD handler");
                }
            }
        }
        Err(err) => {
            eprintln!("Faild to create new proccess: {:?}", err)
        }
    }
}

/// Function to be executed in the child process.
/// When this function returns, the child process terminates.
fn child_function() -> isize {
    network_namespace();
    pid_namespace();
    mount_namespace();

    // Execute an interactive shell within the namespace
    let program = CString::new("/bin/sh").unwrap();
    let args = [CString::new("/bin/sh").unwrap()];
    execvp(&program, &args).expect("Faild to execute program");

    // The execvp call replaces the cureent process,so this line should not reached.
    println!("Execvp failed!");

    // Exit the child process
    process::exit(1)
}
