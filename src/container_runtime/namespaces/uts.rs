use nix::sched::{unshare, CloneFlags};

pub fn uts_namespace() {
    // Create a new UTS namespace
    match unshare(CloneFlags::CLONE_NEWUTS) {
        Ok(()) => {
            // Perform UTS-related configuration within the new UTS namespace
            println!("We are in the new UTS namespcae!")
        }
        Err(err) => {
            eprintln!("Failed to create new UTS namespace: {:?}", err)
        }
    }
}
