use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan};
use std::env;

fn main() {

    // User & hostname
    let user = Command::new("whoami")
        .output()
        .expect("Failed to get user");

    let user = String::from_utf8_lossy(&user.stdout);

    let hostname = Command::new("uname")
        .args(&["-n"])
        .output()
        .expect("Failed to get hostname");

    let hostname = String::from_utf8_lossy(&hostname.stdout);

    // Don't print user and hostname just yet.

    // println!("{}@{}", user.trim(), hostname.trim());
    // println!("══════════════════════════════════");

    // drop(user);
    //drop(hostname);
    
    
    // Read release file, AKA get OS name
    let mut release_distro = String::new();
    {
        let release_file = File::open("/etc/os-release").expect("Failed to find release file.");
        let mut release_reader = BufReader::new(release_file);
        release_reader.read_line(&mut release_distro).expect("Failed string conversion.");
    }

    // Release file processing, huge credit to ChatGPT for this one!
    let mut distro_name: String = (&release_distro[5..release_distro.len() - 1]).to_string();
    distro_name = distro_name.replace("\"", "");
    // println!("OS: {}", Red.paint(distro_name.clone()));
    
    let figlet = Command::new("figlet")
        .args(["-f", "smslant", &distro_name.clone()])
        .output();
    
    let figlet = figlet.unwrap();
    let figlet = String::from_utf8_lossy(&figlet.stdout);

    // Print all the things we've been saving.
    print!("{}", figlet);
    println!("{}@{}", user.clone().trim(), hostname.clone().trim());
    println!("------------------------");
    println!("OS: {}", Red.paint(distro_name.clone()));

    // Free up RAM
    drop(release_distro);
    drop(distro_name);

    // Kernel Version
    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Couldn't get kernel version!");

    let kernel = String::from_utf8_lossy(&kernel.stdout);

    println!("Kernel: {}", Yellow.paint(kernel.trim()));

    // Read memfile
    if let Ok(file) = File::open("/proc/meminfo") {
        // Reader & Iterator
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Vars
        let mut ramtotal: u32 = 0;
        let mut ramavail: u32 = 0;
        let mut ramused: u32 = 0;
        
        // Read 1st & 2nd line
        if let Some(Ok(line)) = lines.next() {
            if let Some(idx) = line.find(char::is_whitespace) {
                // Reading & Parsing
                let mut line_processed = line[idx..].trim();
                line_processed = &line_processed[0..line_processed.len() - 3];
                // mafs
                let mut ram_gb: u32 = line_processed.parse().unwrap();
                ram_gb = ram_gb / 1048576;
                ramtotal = ram_gb;
            }
        }

        lines.next();

        if let Some(Ok(line)) = lines.next() {
             if let Some(idx) = line.find(char::is_whitespace) {
                // Reading & Parsing
                let mut line_processed = line[idx..].trim();
                line_processed = &line_processed[0..line_processed.len() - 3];
                // mafs
                let mut ram_gb: u32 = line_processed.parse().unwrap();
                ram_gb = ram_gb / 1048576;
                ramavail = ram_gb;
            }
        }

        ramused = ramtotal - ramavail;

        println!("Mem: {}/{} GB ({} GB Available)", Green.paint(ramused.to_string()), Green.paint(ramtotal.to_string()), Green.paint(ramavail.to_string()));

        // Read Uptime

        let uptime = Command::new("uptime")
            .output()
            .expect("Failed to get uptime");

        let mut uptime = String::from_utf8_lossy(&uptime.stdout).trim().to_string();
        uptime = uptime.split_whitespace().nth(2).unwrap_or("").to_string();
        uptime = uptime[0..uptime.len() - 1].to_string();
        println!("Uptime: {} (H:MM)", Blue.paint(uptime));
    } else {
        panic!("Cannot find /proc/meminfo");
    }
    
    // Get shell
    let shell = env::var("SHELL").expect("Could not read $SHELL variable");
    println!("Shell: {}", Cyan.paint(shell));

    // Time for a challenge, Get CPU model!
    {
        let file = File::open("/proc/cpuinfo").expect("Could not read /proc/cpuinfo");
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Read up until the 5th line
        let mut i = 1;
        while i < 5 {
            lines.next();
            i = i + 1;

        }

        drop(i);
        let mut model: String = String::new();

        if let Some(Ok(line)) = lines.next() {
            model = line.split(":").nth(1).expect("Failed to parse CPU Info").trim().to_string();   
        }

        println!("CPU: {}", Purple.paint(model));
    }
}
