use nix::sched::{unshare, CloneFlags};

pub fn cgroup_namespace() {
    // Create a new cgroup namespace
    match unshare(CloneFlags::CLONE_NEWCGROUP) {
        Ok(()) => {
            // Perform cgroup-related configuration within the new cgroup namespace
            println!("We are in the new cgroup namespcae!")
        }
        Err(err) => {
            eprintln!("Failed to create new cgroup namespace: {:?}", err)
        }
    }
}
