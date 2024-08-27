use nix::sched::{unshare, CloneFlags};

pub fn network_namespace() {
    // Create a new network namespace
    match unshare(CloneFlags::CLONE_NEWNET) {
        Ok(()) => {
            // Perform network-related configuration within the new network namespace
            println!("We are in the new network namespcae!")
        }
        Err(err) => {
            eprintln!("Failed to create new network namespace: {:?}", err)
        }
    }
}
