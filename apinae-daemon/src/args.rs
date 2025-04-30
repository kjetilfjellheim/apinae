use std::error::Error;

use clap::Parser;

/// Command line application for starting daemon and reading test configurations.
#[derive(Parser, Debug, Clone)]
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

    /// Parameter values for the test. Multiple parameters can be specified.
    /// This is a key-value pair separated by `=`. For example: `key=value`.
    #[arg(long, value_parser = parse_key_val::<String, String>)]
    pub param: Vec<(String, String)>,

    /// List all parameters for the test.
    #[arg(long)]
    pub list_params: bool,

    /// Verify daemon initialization. Stops the server after initialization.
    /// This is useful for testing the daemon without running it.
    #[arg(long)]
    pub verify: bool,

    
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s.find('=').ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
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
