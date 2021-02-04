use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = App::new("FSAI")
        .version("0.1.1")
        .author("Sergey Reshetnikov <shaman@simmirra.com>")
        .about("File System Analytic Interface")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .takes_value(true)
            .help("Specify directory name to analyze"))
        .get_matches();

    let directory_name = matches.value_of("directory").unwrap_or(".");
    println!("{} was specified as working directory", directory_name);
    Ok(())
}
