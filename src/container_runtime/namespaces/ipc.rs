use nix::sched::{unshare, CloneFlags};

pub fn ipc_namespace() {
    // Create a new IPC namespace
    match unshare(CloneFlags::CLONE_NEWIPC) {
        Ok(()) => {
            // Perform IPC-related configuration within the new IPC namespace
            println!("We are in the new IPC namespcae!")
        }
        Err(err) => {
            eprintln!("Failed to create new IPC namespace: {:?}", err)
        }
    }
}
