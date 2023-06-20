use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan};
use std::env;

fn main() {

    // User & hostname
    // let user = Command::new("whoami")
    //     .output()
    //     .expect("Failed to get user");

    // let user = String::from_utf8_lossy(&user.stdout);

    // let hostname = Command::new("uname")
    //     .args(["-n"])
    //     .output()
    //     .expect("Failed to get hostname");



   // let hostname = String::from_utf8_lossy(&hostname.stdout);
    let user = env::var("USER");

    let mut hostname = String::new();
    {
        let hostname_file = File::open("/etc/hostname").expect("u forgor the /etc/hostname file u arch-using moronbox");
        let mut hostname_reader = BufReader::new(hostname_file);
        hostname_reader.read_line(&mut hostname).expect("Failed string conversion... EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE");
    }

    // Read release file, AKA get OS name
    let mut release_distro = String::new();
    {
        let release_file = File::open("/etc/os-release").expect("Failed to find release file.");
        let mut release_reader = BufReader::new(release_file);
        release_reader.read_line(&mut release_distro).expect("Failed string conversion.");
    }

    // Release file processing, huge credit to ChatGPT for this one!
   let mut distro_name: String = release_distro[5..release_distro.len() - 1].to_string();
   distro_name = distro_name.replace('\"', "");
    // println!("OS: {}", Red.paint(distro_name.clone()));

    let figlet = Command::new("figlet")
        .args(["-f", "smslant", &distro_name])
        .output();

   if let Ok(output) = figlet {
        let output = String::from_utf8_lossy(&output.stdout);
        print!("{}", output);
   }
    // Print all the things we've been saving.
    println!("{}@{}", user.unwrap(), hostname);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("┐ OS: {}", Red.paint(distro_name));

    // I know it's terrible, but it works.
    let mut kernel = String::new();
    {
        let kernel_file = File::open("/proc/version").expect("Read the README.md you dumbass");
        let mut kernel_reader = BufReader::new(kernel_file);
        kernel_reader.read_line(&mut kernel).expect("Failed string conversion");
    }

    let mut kernel_name: String = (kernel[14..kernel.len()]).to_string();
    kernel_name = kernel_name.split_whitespace()
        .next()
        .unwrap()
        .to_string();

    println!("│ Kernel: {}", Yellow.paint(kernel_name.trim()));

    // Read memfile
    if let Ok(file) = File::open("/proc/meminfo") {
        // Reader & Iterator
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Vars
        let mut ramtotal: u32 = 0;
        let mut ramavail: u32 = 0;

        // Read 1st & 2nd line
        if let Some(Ok(line)) = lines.next() {
            if let Some(idx) = line.find(char::is_whitespace) {
                // Reading & Parsing
                let mut line_processed = line[idx..].trim();
                line_processed = &line_processed[0..line_processed.len() - 3];
                // mafs
                let mut ram_gb: u32 = line_processed.parse().unwrap();
                ram_gb /= 1048576;
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
                ram_gb /= 1048576;
                ramavail = ram_gb;
            }
        }

        let ramused = ramtotal - ramavail;

        println!("│ Mem: {}/{} GB ({} GB Available)", Green.paint(ramused.to_string()), Green.paint(ramtotal.to_string()), Green.paint(ramavail.to_string()));

    }

    {
        // This took me unusually long.

        // Generic file stuff
        let mut uptime = String::new();
        let uptime_file = File::open("/proc/uptime").expect(":skull:");
        let mut uptime_reader = BufReader::new(uptime_file);
        uptime_reader.read_line(&mut uptime).expect("what");
        let mut iterator = uptime.split_whitespace();
        uptime = iterator.next().expect("*screeches at the top of his lungs*").to_string();

        // was never expecting rounding to be this difficult
        let uptimeint = uptime.parse::<f32>();
        let roundeduptimeint: u32 = uptimeint.expect("phoque").round() as u32;
        let uptimemins: u32 = roundeduptimeint / 60;
        println!("│ Uptime: {} minutes", Blue.paint(uptimemins.to_string()))

    }

    // Get shell
    let shell_raw = env::var("SHELL").expect("Could not read $SHELL variable");

    // Split the path using '/' as the separator
    // Thanks ChatGPT
    let parts: Vec<&str> = shell_raw.rsplitn(2, '/').collect();

    // Check if the path contains at least one '/'
    if parts.len() > 1 {
        let shell = parts[0];
        println!("│ Shell: {}", Cyan.paint(shell));
    } else {
        println!("Invalid path format.");
    }
    // Time for a challenge, Get CPU model!
    {
        let file = File::open("/proc/cpuinfo").expect("Could not read /proc/cpuinfo");
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Read up until the 5th line
        let mut i = 1;
        while i < 5 {
            lines.next();
            i += 1;

        }
        let mut model: String = String::new();

        if let Some(Ok(line)) = lines.next() {
            model = line.split(':').nth(1).expect("Failed to parse CPU Info").trim().to_string();
       }

        println!("┘ CPU: {}", Purple.paint(model));

    }

    // Colours
   println!("\n{} {} {} {} {} {}", Red.paint("◆"), Green.paint("◆"), Yellow.paint("◆"), Blue.paint("◆"), Purple.paint("◆"), Cyan.paint("◆"));

}
