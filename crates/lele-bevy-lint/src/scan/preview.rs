use atomic_delegate_macros::atomic_delegates;

use crate::scan;

pub struct Preview {
    pub name: String,
    pub kind: scan::PreviewKind,
    pub line: usize,
    pub is_test: bool,
    pub harness: scan::HarnessCall,
    pub scene: scan::SceneUsage,
}

#[atomic_delegates]
impl Preview {
    pub fn missing_subject<'a>(
        previews: &'a [Preview],
        kind: scan::PreviewKind,
        subjects: &[String],
    ) -> Option<&'a Preview> {
    }
}

#[cfg(test)]
mod tests {
    use super::Preview;
    use crate::scan;
    use crate::testing;

    #[test]
    fn test_usage() {
        let subjects = vec![String::from("setup_ui")];
        let copy = [testing::preview(&["spawn_status"])];
        let real = [testing::preview(&["setup_ui"])];
        assert!(Preview::missing_subject(&copy, scan::PreviewKind::Png, &subjects).is_some());
        assert!(Preview::missing_subject(&real, scan::PreviewKind::Png, &subjects).is_none());
        assert!(Preview::missing_subject(&copy, scan::PreviewKind::Mp4, &subjects).is_none());
    }
}
