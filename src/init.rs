extern crate os_utils;
extern crate sc;
extern crate libc;

use std::{io, fs};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn set_panic_timeout(seconds: &str) {
    if let Err(e) = fs::write("/proc/sys/kernel/panic", seconds) {
        println!("Failed to set panic timeout! Error: {}", e)
    } else {
        println!("[{}{} OK {}] Set panic timout to {} seconds", GREEN, BOLD, RESET, seconds)
    }
}

fn main() {
    // --- INITIALIZATION PHASE ---
    // We must mount these virtual filesystems first or nothing (like /dev/console or /proc) will work.
    os_utils::mount("proc\0", "/proc\0", "proc\0");
    os_utils::mount("devtmpfs\0", "/dev\0", "devtmpfs\0");
    os_utils::mount("sysfs\0", "/sys\0", "sysfs\0");
    
    println!("[{}{} OK {}] FILESYSTEMS MOUNTED", BOLD, GREEN, RESET);
    println!("-------------------------------");
    println!("         Onish-OS 1.0          ");
    println!("-------------------------------");
    set_panic_timeout("10");

    // Find user configured shell.
    let shell_file = fs::read_to_string("etc/shell.txt");
    let configured_shell = match &shell_file {
        Ok(s) => s.trim(),
        Err(_) => "/bin/sh",
    };


    println!("[ INFO ] Bringing up network for self-healing...");
    let ip_paths = ["/sbin/ip", "/usr/sbin/ip", "/bin/ip", "/usr/bin/ip"];
    let mut ip_path = "/sbin/ip";
    for path in ip_paths {
        if Path::new(path).exists() {
            println!("Found IP at {}", path);
            ip_path = path;
            break;
        }
    }
    Command::new(ip_path).args(["link", "set", "eth0", "up"])
    .stdout(Stdio::inherit()) // <--- Send logs to terminal
    .stderr(Stdio::inherit())
    .status()
    .expect("Failed IP");

    let udhcpc_paths = ["/sbin/udhcpc", "/usr/sbin/udhcpc", "/bin/udhcpc"];
    let mut udhcpc = "/sbin/udhcpc";
    for path in udhcpc_paths {
        if Path::new(path).exists() {
            udhcpc = path;
            println!("Found udhcpc at {}", udhcpc);
        }
    }
    let status = Command::new(udhcpc).args(["-i", "eth0", "-t", "5", "-T", "4", "-s" , "/usr/share/udhcpc/default.script", "-q"])
    .stdout(Stdio::inherit()) // <--- Send logs to terminal
    .stderr(Stdio::inherit())
    .status();


    let mut repair = false;
    println!("Set repair to {}", repair);
    match status {
    Ok(s) if s.success() => {
        println!("[{}{} OK {}] DHCP Lease obtained!", BOLD, GREEN, RESET);
        repair = true;
    },
    _ => {
        println!("[{}{} ERR {}] DHCP Failed - check if script exists", BOLD, RED, RESET);
        repair = false;
    },
}

    // Package manager repair.
    if !Path::new("/lib/apk/db/installed").exists() {
        println!("Repairing apk!");
        fs::create_dir_all("/lib/apk/db").ok();
        fs::File::create("/lib/apk/db/installed").ok();
    }

    if !Path::new("/etc/apk/world").exists() {
        fs::File::create("/etc/apk/world").ok();

        let repo_path = "etc/apk/repositories";
        let content = "http://dl-cdn.alpinelinux.org/alpine/v3.20/main\n\
        http://dl-cdn.alpinelinux.org/alpine/v3.20/community\n";
        fs::write(repo_path, content).expect("Failed to do setup repos");
    }

    // Search for APK for self-repair
    let apk_paths = ["/bin/apk", "/usr/bin/apk", "/sbin/apk"];
    let mut apk_path = "/sbin/apk";
    for path in apk_paths {
        if Path::new(path).exists() {
            apk_path = path;
            println!("APK exists at {}", apk_path);
            break;
        }
    }

    if repair == true {
    if !Path::new("/etc/ssl/certs/ca-certificates.crt").exists() {
        println!("SSL missing installing now!");
        
        let status = Command::new(apk_path)
        .args([
            "add",
            "ca-certificates",
            "alpine-keys",
            "--repository", "http://dl-cdn.alpinelinux.org/alpine/v3.20/main",
            "--allow-untrusted"
        ])
        .status();
    
    match status {
        Ok(s) => if s.success() { println!("Sucessful SSL Certificates are installed.") }
        Err(e) => println!("Failed to install SSL Error {}", e),
    }
    }
    let update = Command::new(apk_path).arg("update").status();
    match update {
        Ok(s) => if s.success() { println!("Sucessfully updated database.") },
        Err(e) => println!("Failed to update database Error: {}", e)
    }
    }

    // Find existing shell to avoid a unuseable distro.
    let shell_path = if Path::new(&configured_shell).is_file() {
        configured_shell
    } else {
        println!("[{}{} ERROR {}] Configured shell not found falling back to /bin/sh", BOLD, RED, RESET);
        "/bin/sh"
    };

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
                .env("HOME", "/root")
                .env("TERM", "xterm")
                .spawn();

            match child {
                Ok(mut proc) => {
                    // This blocks the child until you type 'exit' in Bash.
                    proc.wait().expect("Bash crashed");
                }
                Err(e) => {
                    println!("{}FATAL: {} {} failed to run Error: {} \n", RED, RESET, shell_path, e);
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

                while libc::waitpid(-1, &mut status, libc::WNOHANG) > 0 {
                    // Keep reaping until all zombies are dead.
                }
            }

            // --- POWER MANAGEMENT ---
            // After Bash closes, we ask what to do next.
            let _ = Command::new("/bin/stty")
            .args(["-F", "/dev/tty", "sane", "echo", "icanon"])
            .status();
            unsafe {
                libc::kill(-(pid as i32), libc::SIGKILL);
                let mut status = 0;
                while libc::waitpid(-1, &mut status, libc::WNOHANG) > 0 {}
                libc::tcflush(libc::STDIN_FILENO, libc::TCIOFLUSH);
            }

            print!("\x1b[?1049l\x1b[?25h\x1b[0m\x1b[2J\x1b[H");
            let _ = io::stdout().flush();

            let ans = os_utils::input("\nSession Ended. reboot / shutdown / shell? ");
            let choice = ans.to_ascii_lowercase();

            if choice == "shutdown" {
                os_utils::shutdown();
            } else if choice == "reboot" {
                os_utils::reboot();
            } else if choice == "goodbye" {
                os_utils::suicide(93)
            }
            // If they type 'shell' or anything else, the loop repeats and restarts Bash.
            os_utils::print("Restarting session...\n");
        }
    }
}
