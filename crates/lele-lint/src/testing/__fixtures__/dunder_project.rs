use crate::DunderPath;
use crate::Project;

pub(crate) fn dunder_project() -> Project {
    Project {
        dunder_paths: DunderPath::defaults(),
        ..Project::default()
    }
}

#[cfg(test)]
mod tests {
    use super::dunder_project;
    use crate::DunderPath;

    #[test]
    fn test_usage() {
        assert_eq!(
            dunder_project().dunder_paths.len(),
            DunderPath::defaults().len()
        );
    }
}
