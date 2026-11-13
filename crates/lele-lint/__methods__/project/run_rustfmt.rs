use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use crate::Error;
use crate::Project;

pub fn run_rustfmt(project: &Project, source: &str) -> Result<String, Error> {
    let mut child = Command::new("rustfmt")
        .arg("--edition")
        .arg(project.crate_edition())
        .args(["--emit", "stdout"])
        .current_dir(&project.root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| Error::RustfmtStart {
            root: project.root.clone(),
            source,
        })?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(source.as_bytes())
            .map_err(|source| Error::RustfmtStart {
                root: project.root.clone(),
                source,
            })?;
    }
    let output = child
        .wait_with_output()
        .map_err(|source| Error::RustfmtStart {
            root: project.root.clone(),
            source,
        })?;
    if !output.status.success() {
        return Err(Error::RustfmtFailed {
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Error;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            root: PathBuf::from("."),
            ..Project::default()
        };
        let formatted = project.run_rustfmt("fn a(){}").unwrap();
        assert_eq!(formatted, "fn a() {}\n");
        let missing = Project {
            root: PathBuf::from("/nonexistent-lele-root"),
            ..Project::default()
        };
        assert!(matches!(
            missing.run_rustfmt("fn a(){}"),
            Err(Error::RustfmtStart { .. })
        ));
        assert!(matches!(
            project.run_rustfmt("fn a( {"),
            Err(Error::RustfmtFailed { .. })
        ));
    }
}
