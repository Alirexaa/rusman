use nix::sched::{unshare, CloneFlags};

pub fn pid_namespace() {
    // Create a new PID namespace
    match unshare(CloneFlags::CLONE_NEWPID) {
        Ok(()) => {
            // Perform PID-related configuration within the new PID namespace
            println!("We are in the new PID namespcae!")
        }
        Err(err) => {
            eprintln!("Failed to create new PID namespace: {:?}", err)
        }
    }
}
