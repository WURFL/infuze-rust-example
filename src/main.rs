use std::{env, io};
use std::process::exit;
use wurfl::{Wurfl, WurflCacheProvider};
use std::path::Path;
use std::io::{BufRead, Write};

fn main() {

    const DEFAULT_WURFL_PATH: &str = "/usr/share/wurfl/wurfl.zip";

    let args: Vec<String> = env::args().collect();
    let wurfl_path_str = DEFAULT_WURFL_PATH;
    // check if wurfl exists in default path
    let mut wurfl_path = Path::new(wurfl_path_str);
    if !wurfl_path.exists() || !wurfl_path.is_file() {
        // WURFL default path does not contain a path, let's check if it has been passed from command line
        let wurfl_path_opt = args.get(2);
        if wurfl_path_opt.is_none() {
            println!("Invalid WURFL file path. Please specify a WURFL file path. \n Usage: \n infuze-rust-examples [-w path_of_WURFL_file]");
            exit(1);
        }
        else {
            wurfl_path = Path::new(wurfl_path_opt.unwrap().as_str());
        }
    }

    // Creates an instance of the WURFL engine with a 100K elements LRU cache
    let wurfl_res = Wurfl::new(wurfl_path.to_str().unwrap(), None, None,
                               WurflCacheProvider::LRU, Some("100000"));
    let engine = match wurfl_res {
        Ok(engine) => engine,
        Err(error) => panic!("Problem initializing wurfl: {:?}", error),
    };

    let stdin = io::stdin();

    let _ = io::stdout().write_all("Device name/tOS/tForm factor\n".as_bytes());
    // Read lines from the file containing a list of User-Agents and perform a device detection
    for line in stdin.lock().lines() {
        let device_res = engine.lookup_useragent(line.unwrap().as_str());
        if device_res.is_err() {
            // skip some unlikely errors
            continue;
        }
        let device = device_res.unwrap();
        let device_name = device.get_virtual_capability("complete_device_name");
        let is_tablet = device.get_virtual_capability("advertised_device_os");
        let form_factor = device.get_virtual_capability("form_factor");
        let _ = io::stdout().write_all(device_name.unwrap().as_bytes());
        let _ = io::stdout().write_all("\t".as_bytes());
        let _ = io::stdout().write_all(is_tablet.unwrap().as_bytes());
        let _ = io::stdout().write_all("\t".as_bytes());
        let _ = io::stdout().write_all(form_factor.unwrap().as_bytes());
        let _ = io::stdout().write_all("\n".as_bytes());
    }
}
