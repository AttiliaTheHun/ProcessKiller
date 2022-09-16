#![windows_subsystem = "windows"]
extern crate winproc;
use winproc::Process;
use core::iter::Iterator;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{thread, time};

fn main() {
	let data: (bool, bool, u64, Vec<String>) = load_data();
	let debug: bool = data.0;
	let autostop: bool = data.1;
	let interval: u64 = data.2;
	let target_process_names: Vec<&str> = data.3.iter().map(String::as_str).collect();
	if debug {
		println!("Target process names: {:?}", &target_process_names);
		println!("AUTOSTOP: {:?}, INTERVAL: {:?}", autostop, interval);	
	}
	
	// We needed the integer value only to initialize this Duration object, so it is safe to overshadow it
	let interval = time::Duration::from_millis(interval);
	
	loop {
		let status: bool = kill_processes(find_process_ids(&target_process_names, &debug));
		println!("Execution status: {:?}", status);
		if autostop {
			break;
		}
		thread::sleep(interval);
	}
	
}

fn find_process_ids(names: &Vec<&str>, debug: &bool) -> Vec<u32> {
	
	let mut file: File = File::create("process_log").unwrap();

	let mut process_ids: Vec<u32> = vec![];
	let mut process_log_done: bool = false;
	for name in names {
		let mut process_iterator = Process::all().unwrap();
		loop {
			match process_iterator.next() {
				Some(process) => {
					let process_name: String = process.name().unwrap().to_lowercase();
					
					if *debug && !process_log_done {
						writeln!(&mut file, "{}: {}\n", process.id(), process_name).unwrap();
					}
					
					if process_name.contains(&name.to_lowercase()) {
						process_ids.push(process.id());
					}
				},
				None => {break;}
			}
		}
		process_log_done = true; // we do not need the processes logged ten times, one is just enough
		if *debug {
			println!("Searching for: {:?}", name);
		}
	}
	if *debug {
		println!("Process ids: {:?}", process_ids);
	}
	process_ids
}

fn kill_processes(pids: Vec<u32>) -> bool {
	const TERMINATION_EXIT_CODE: u32 = 248;
	for pid in pids {
		Process::from_id(pid).unwrap().terminate(TERMINATION_EXIT_CODE);
	}
	true
}

fn load_data() -> (bool, bool, u64, Vec<String>) {
	let mut names: Vec<String> = vec![];
	let mut interval: u64 = 500;
	let mut autostop: bool = true;
	let mut debug: bool = false;
	let filename = "./config";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).expect("Cannot find the config file");
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.starts_with("#") {
			continue;
		}
		if line.starts_with("NAME") {
			names.push(String::from(line.strip_prefix("NAME=").expect("Problem with getting target process from the file").trim()));
		}
		if line.starts_with("AUTOSTOP") {
			autostop = line.strip_prefix("AUTOSTOP=").expect("Problem getting AUTOSTOP value, using default value").trim().to_lowercase().eq("true");
		}
		if line.starts_with("DEBUG") {
			debug = line.strip_prefix("DEBUG=").expect("Not sure if our head aches, I guesss it does not").trim().to_lowercase().eq("true");
		}
		if line.starts_with("INTERVAL") {
			interval = line.strip_prefix("INTERVAL=").expect("Trouble getting check interval, using default value").trim().parse::<u64>().unwrap();
		}
		
    }
	(debug, autostop, interval, names)
}
