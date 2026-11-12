use std::io::Write;

use crate::Location;
use crate::RuleViolation;

pub fn print_github(d: &RuleViolation, w: &mut impl Write) {
    let _ = match d.location {
        Location::File => writeln!(
            w,
            "::error file={file},line=1,title={code}::{message}",
            file = d.file.display(),
            code = d.code,
            message = d.message
        ),
        Location::Site { line, col } => writeln!(
            w,
            "::error file={file},line={line},col={col},title={code}::{message}",
            file = d.file.display(),
            code = d.code,
            message = d.message
        ),
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::print_github;
    use crate::Location;
    use crate::RuleViolation;

    #[test]
    fn test_usage() {
        let site = RuleViolation {
            file: PathBuf::from("a.rs"),
            location: Location::Site { line: 3, col: 4 },
            code: "LELE001",
            message: "m".to_string(),
        };
        let mut out: Vec<u8> = Vec::new();
        print_github(&site, &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "::error file=a.rs,line=3,col=4,title=LELE001::m\n"
        );
        let file_level = RuleViolation {
            location: Location::File,
            ..site
        };
        let mut out: Vec<u8> = Vec::new();
        print_github(&file_level, &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "::error file=a.rs,line=1,title=LELE001::m\n"
        );
    }
}
