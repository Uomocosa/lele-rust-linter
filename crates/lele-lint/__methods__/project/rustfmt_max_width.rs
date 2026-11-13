use crate::Project;

pub fn rustfmt_max_width(project: &Project) -> usize {
    let text = project.root.ancestors().find_map(|dir| {
        ["rustfmt.toml", ".rustfmt.toml"]
            .iter()
            .find_map(|name| std::fs::read_to_string(dir.join(name)).ok())
    });
    text.and_then(|text| {
        text.lines().find_map(|line| {
            let (key, value) = line.split_once('=')?;
            if key.trim() == "max_width" {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
    })
    .unwrap_or(100)
}

#[cfg(test)]
mod tests {
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        std::fs::write(
            dir.path().join("rustfmt.toml"),
            "edition = \"2021\"\nmax_width = 120\n",
        )
        .unwrap();
        assert_eq!(project.rustfmt_max_width(), 120);
        let bare = tempfile::tempdir().unwrap();
        std::fs::write(bare.path().join(".rustfmt.toml"), "tab_spaces = 2\n").unwrap();
        let no_key = Project {
            root: bare.path().to_path_buf(),
            ..Project::default()
        };
        assert_eq!(no_key.rustfmt_max_width(), 100);
    }
}
