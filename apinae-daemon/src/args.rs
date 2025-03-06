use clap::Parser;

/// Command line application for starting daemon and reading test configurations.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author="Kjetil Fjellheim")]
pub struct Args {
    /// Input file.
    #[arg(long)]
    pub file: String,

    /// This starts the daemon with the specified test from the file.
    #[arg(long)]
    pub id: Option<String>,

    /// Lists the available tests in the specified file.
    #[arg(long)]
    pub list: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_daemon_args() {
        let args = Args::parse_from(["apinae-daemon", "--file", "test.json", "--id", "1"]);
        assert_eq!(args.file, "test.json");
        assert_eq!(args.id, Some("1".to_string()));
        assert!(!args.list);
    }

    #[test]
    fn test_daemon_args_list() {
        let args = Args::parse_from(["apinae-daemon", "--file", "test.json", "--list"]);
        assert_eq!(args.file, "test.json");
        assert_eq!(args.id, None);
        assert!(args.list);
    }
}
