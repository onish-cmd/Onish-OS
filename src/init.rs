extern crate os_utils;
extern crate sc;
extern crate libc;

fn main() {
    // --- INITIALIZATION PHASE ---
    // We must mount these virtual filesystems first or nothing (like /dev/console or /proc) will work.
    os_utils::mount("proc\0", "/proc\0", "proc\0");
    os_utils::mount("devtmpfs\0", "/dev\0", "devtmpfs\0");
    os_utils::mount("sysfs\0", "/sys\0", "sysfs\0");
    
    os_utils::print("[ OK ] FILESYSTEMS MOUNTED\n");
    os_utils::print("Welcome to Onish-OS\n");
    os_utils::print("--VERSION 0.8 PRE-RELEASE--\n");

    // Find existing shell to avoid a unuseable distro.
    let mut shell_path = "/bin/sh";
    let shells = ["/usr/bin/bash", "/bin/bash", "/bin/sh", "/usr/bin/sh"];
    for path in shells {
        if std::path::Path::new(path).exists() {
            shell_path = path;
            break;
        }
    }

    // Tell shell thats being used.
    println!("DEBUG: Using shell {}", shell_path);

    // We wrap the entire Shell + Power logic in a loop.
    // This prevents the "PID 1 Attempted to Kill" Kernel Panic.
    loop {
        // --- TTY CONFIGURATION ---
        // Claim /dev/console as our Controlling Terminal (TIOCSCTTY).
        os_utils::attach_console();

        // --- FORK THE USER SESSION ---
        // We create a child process. The child becomes Bash, the parent stays as the Manager.
        let pid = os_utils::fork();

        if pid == 0 {
            // --- CHILD PROCESS (The Session) ---
            use std::process::Command;

            // Launching Bash as a 'Login Shell' (-l).
            // This triggers /etc/profile which we will use to fix the network.
            let child = Command::new(shell_path)
                .arg("-l")
                .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
                .env("TERM", "xterm")
                .spawn();

            match child {
                Ok(mut proc) => {
                    // This blocks the child until you type 'exit' in Bash.
                    proc.wait().expect("Bash crashed");
                }
                Err(_) => {
                    os_utils::print("FATAL: /bin/sh not found in RootFS\n");
                }
            }
            // If Bash ends, this child process MUST die or it will try to act like PID 1.
            os_utils::suicide(0);
        } else {
            // --- PARENT PROCESS (The Monitor) ---
            unsafe {
                let mut status = 0;
                // Wait specifically for our Bash PID. 
                // This ignores orphans (like background tasks) so we don't reboot early.
                libc::waitpid(pid as i32, &mut status, 0);
            }

            // --- POWER MANAGEMENT ---
            // After Bash closes, we ask what to do next.
            let ans = os_utils::input("\nSession Ended. reboot / shutdown / shell? ");
            let choice = ans.to_ascii_lowercase();

            if choice == "shutdown" {
                os_utils::shutdown();
            } else if choice == "reboot" {
                os_utils::reboot();
            } 
            // If they type 'shell' or anything else, the loop repeats and restarts Bash.
            os_utils::print("Restarting session...\n");
        }
    }
} // Is my english good? 