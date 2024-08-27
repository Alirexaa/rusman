use nix::sched::{unshare, CloneFlags};

pub fn user_namespace() {
    // Create a new user namespace
    match unshare(CloneFlags::CLONE_NEWUTS) {
        Ok(()) => {
            // Perform user-related configuration within the new user namespace
            println!("We are in the new user namespcae!")
        }
        Err(err) => {
            eprintln!("Failed to create new user namespace: {:?}", err)
        }
    }
}
