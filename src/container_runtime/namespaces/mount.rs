use std::fs;

use nix::{
    mount::{self, mount, MsFlags},
    sched::{unshare, CloneFlags},
    unistd::chroot,
};

pub fn mount_namespace() {
    // Create a new mount namespace
    match unshare(CloneFlags::CLONE_NEWNS) {
        Ok(()) => {
            // Perform mount-related configuration within the new mount namespace

            // Create a new directory to be used as new root
            fs::create_dir_all("/tmp/newroot").expect("Failed to create /tem/newroot directory");
            // Make the mounts in the new mount namesapce private
            mount(
                None::<&str>,
                "/",
                None::<&str>,
                MsFlags::MS_PRIVATE | MsFlags::MS_REC,
                None::<&str>,
            )
            .expect("Failed to make mounts private");

            // Mount the /proc file system as private within the new mount namesapce

            mount(
                Some("proc"),
                "/proc",
                Some("proc"),
                MsFlags::MS_PRIVATE,
                None::<&str>,
            )
            .expect("Failed to make mounts private");

            // Set the new root as the cureent root
            chroot("/").expect("Faild to change root");
            println!("We are in the new mount namespcae!");
        }
        Err(err) => {
            eprintln!("Failed to create new mount namespace: {:?}", err)
        }
    }
}
