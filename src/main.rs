use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use wurfl::{Wurfl, WurflCacheProvider, wurfl_download};

fn main() {
    const DEFAULT_WURFL_PATH: &str = "wurfl.zip";
    // download WURFL file from the ScientiaMobile Snapshot generator to the current directory
    // (replace the sample URL with your own specific customer URL to avoid error 402)
    let download_url = "https://data.scientiamobile.com/xxxxx/wurfl.zip";

    match wurfl_download(download_url, ".") {
        Ok(_) => println!("WURFL file downloaded successfully"),
        Err(e) => println!("WURFL file download FAILED: {}", e),
    }

    let wurfl_path = Path::new(DEFAULT_WURFL_PATH);
    if !wurfl_path.exists() || !wurfl_path.is_file() {
        eprintln!("Invalid WURFL file path. Please specify a correct WURFL file path.");
        exit(1);
    }

    // Creates an instance of the WURFL engine with a 100K elements LRU cache
    let engine = match Wurfl::new(
        wurfl_path.to_str().unwrap_or(DEFAULT_WURFL_PATH),
        None,
        None,
        WurflCacheProvider::LRU,
        Some("100000"),
    ) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Problem initializing wurfl: {:?}", e);
            exit(1);
        }
    };

    // Set updater URL and timeouts
    if let Some(e) = engine.set_updater_data_url(download_url) {
        eprintln!("Updater URL initialization failed: {}", e);
    }
    if let Some(e) = engine.set_updater_data_url_timeout(10, 10) {
        eprintln!("Updater timeout initialization failed: {}", e);
    }
    if let Some(e) = engine.updater_start() {
        eprintln!("Updater start failed: {}", e);
    }

    // Output header
    println!("Device name\tOS\tForm factor");

    // Read lines from the file containing a list of User-Agents and perform a device detection for each line
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ua = match line {
            Ok(ua) => ua,
            Err(_) => continue,
        };

        let device = match engine.lookup_useragent(&ua) {
            Ok(d) => d,
            Err(_) => continue,
        };

        // Helper closure to get a virtual capability or "Unknown"
        let get_cap = |cap: &str| match device.get_virtual_cap(cap) {
            Ok(Some(val)) => val,
            Ok(None) => "Unknown",
            Err(_) => "Unknown",
};


        println!(
            "{}\t{}\t{}",
            get_cap("complete_device_name"),
            get_cap("advertised_device_os"),
            get_cap("form_factor"),
        );
    }

    engine.updater_stop();
}