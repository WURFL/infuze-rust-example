# Infuze Rust Examples

This project is a simple console application that reads a list of User-Agents from the standard input, performs a device detection on each one and, for each device detected, 
it outputs three device capabilities in TSV format.

### Prerequisites

- Rust 1.76.0 or above
- Cargo
- Infuze libwurfl (see: https://docs.scientiamobile.com/documentation/infuze/infuze-c-api-user-guide)

Build the application using `cargo build` from this project root directory.
The application executable is usually generated in `target/debug`. 

Sample app execution: 

```
cat 100_ua.txt | target/debug/infuze-rust-examples  > outfile.tsv
```

The file `100_ua.txt` is provided so that you can easily try app execution.

As you can see the command sends the file to the standard input via the `cat` command and dumps the standard output result to the `outfile.tsv`

The example app code downloads a fresh version of the WURFL file from WURFL Snapshot server into the current directory, and will use it to initialize the WURFL engine. 

**Important note:** replace the placeholder URL ("https://data.scientiamobile.com/xxxxx/wurfl.zip") in the `main.rs` file with your own customer WURFL Snapshot URL.