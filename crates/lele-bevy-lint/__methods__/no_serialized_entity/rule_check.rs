use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::rules;
use crate::scan;

pub fn rule_check(
    _self: &rules::no_serialized_entity::NoSerializedEntity,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let mut stack: Vec<&syn::Item> = source.file.items.iter().collect();
        while let Some(item) = stack.pop() {
            match item {
                syn::Item::Struct(struct_def) => {
                    if rules::no_serialized_entity::NoSerializedEntity::is_serialized(
                        &struct_def.attrs,
                    ) && rules::no_serialized_entity::NoSerializedEntity::struct_has_entity(
                        struct_def,
                    ) {
                        violations.push(scan::BevyProject(project).violation(
                            source.origin,
                            source.relative_path,
                            struct_def.ident.span().start().line,
                            rules::no_serialized_entity::NoSerializedEntity::CODE,
                            format!(
                                "`{}` derives `Serialize`/`Deserialize` but holds `Entity` — send the stable domain id (e.g. `PeerId` / `RoomName`) and map it to an `Entity` at runtime",
                                struct_def.ident,
                            ),
                        ));
                    }
                }
                syn::Item::Enum(item_enum) => {
                    if rules::no_serialized_entity::NoSerializedEntity::is_serialized(
                        &item_enum.attrs,
                    ) && rules::no_serialized_entity::NoSerializedEntity::enum_has_entity(
                        item_enum,
                    ) {
                        violations.push(scan::BevyProject(project).violation(
                            source.origin,
                            source.relative_path,
                            item_enum.ident.span().start().line,
                            rules::no_serialized_entity::NoSerializedEntity::CODE,
                            format!(
                                "`{}` derives `Serialize`/`Deserialize` but holds `Entity` — send the stable domain id (e.g. `PeerId` / `RoomName`) and map it to an `Entity` at runtime",
                                item_enum.ident,
                            ),
                        ));
                    }
                }
                syn::Item::Mod(module) => {
                    if let Some((_, inner)) = &module.content {
                        stack.extend(inner.iter());
                    }
                }
                _ => {}
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules;
    use lele_lint::Project;
    use std::path::PathBuf;

    fn messages(source: &str) -> Vec<String> {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("net/snapshot.rs"),
            syn::parse_str(source).unwrap(),
        );
        rule_check(&rules::no_serialized_entity::NoSerializedEntity, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn test_usage() {
        let found = messages("#[derive(Serialize)] pub struct Snapshot { pub owner: Entity }");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("PeerId"));
    }

    #[test]
    fn test_usage_nested_generics_are_violations() {
        for code in [
            "#[derive(Serialize)] pub struct A { pub members: Vec<Entity> }",
            "#[derive(Deserialize)] pub struct B { pub owner: Option<Entity> }",
            "#[derive(Serialize)] pub struct C { pub by_room: std::collections::HashMap<String, Entity> }",
            "#[derive(Serialize)] pub struct D { pub pair: (u32, Entity) }",
            "#[derive(Serialize)] pub struct E { pub members: [Entity; 4] }",
            "#[derive(Serialize)] pub struct F { pub owner: bevy::ecs::entity::Entity }",
        ] {
            assert_eq!(messages(code).len(), 1, "{code}");
        }
    }

    #[test]
    fn test_usage_enum_variant_entity_is_violation() {
        let found = messages(
            "#[derive(Serialize, Deserialize)] pub enum Delta { Joined { owner: Entity }, Left }",
        );
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_usage_unserialized_entity_is_clean() {
        let found = messages("pub struct Live { pub owner: Entity }");
        assert_eq!(found, Vec::<String>::new());
    }

    #[test]
    fn test_usage_serialized_without_entity_is_clean() {
        let found =
            messages("#[derive(Serialize, Deserialize)] pub struct Snapshot { pub owner: PeerId }");
        assert_eq!(found, Vec::<String>::new());
    }
}
