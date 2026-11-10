use crate::rules;
use crate::ErrorFormat;
use crate::Linter;
use crate::Rule;

pub fn default() -> Linter {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    rules::atomic_file::AtomicFile::register(&mut rules);
    rules::snake_case_files::SnakeCaseFiles::register(&mut rules);
    rules::method_file_co_location::MethodFileCoLocation::register(&mut rules);
    rules::method_visibility::MethodVisibility::register(&mut rules);
    rules::no_cross_domain_reexport::NoCrossDomainReexport::register(&mut rules);
    rules::no_dunder_tests::NoDunderTests::register(&mut rules);
    rules::test_inline::TestInline::register(&mut rules);
    rules::helpers_are_methods::HelpersAreMethods::register(&mut rules);
    rules::test_usage::TestUsage::register(&mut rules);
    rules::no_positional::NoPositional::register(&mut rules);
    rules::no_stuttered_path::NoStutteredPath::register(&mut rules);
    rules::no_stuttered_type::NoStutteredType::register(&mut rules);
    rules::no_super_imports::NoSuperImports::register(&mut rules);
    rules::no_trivial_accessors::NoTrivialAccessors::register(&mut rules);
    rules::root_reexport::RootReexport::register(&mut rules);
    rules::domain_import::DomainImport::register(&mut rules);
    rules::atomic_delegates::AtomicDelegates::register(&mut rules);
    rules::constructor_no_skip::ConstructorNoSkip::register(&mut rules);
    rules::constants_placement::ConstantsPlacement::register(&mut rules);
    rules::container_placement::ContainerPlacement::register(&mut rules);
    rules::no_inner_fns::NoInnerFns::register(&mut rules);
    rules::single_field_newtype::SingleFieldNewtype::register(&mut rules);
    rules::mod_rs_purity::ModRsPurity::register(&mut rules);
    rules::no_allow_attributes::NoAllowAttributes::register(&mut rules);
    rules::no_crate_paths::NoCratePaths::register(&mut rules);
    rules::no_collection_newtype::NoCollectionNewtype::register(&mut rules);
    rules::impl_item_order::ImplItemOrder::register(&mut rules);
    rules::clippy_config_cargo::ClippyConfigCargo::register(&mut rules);
    rules::clippy_config_clippy::ClippyConfigClippy::register(&mut rules);
    rules::methods_layout::MethodsLayout::register(&mut rules);
    rules::no_comments::NoComments::register(&mut rules);
    rules::delegate_macro::DelegateMacro::register(&mut rules);
    rules::boundary_imports::BoundaryImports::register(&mut rules);
    rules::no_free_functions::NoFreeFunctions::register(&mut rules);
    rules::methods_are_delegates::MethodsAreDelegates::register(&mut rules);
    rules::conversion_is_from::ConversionIsFrom::register(&mut rules);
    rules::constructor_naming::ConstructorNaming::register(&mut rules);
    rules::dunder_module_name::DunderModuleName::register(&mut rules);
    rules::no_stringified_errors::NoStringifiedErrors::register(&mut rules);
    rules::error_message_omits_source::ErrorMessageOmitsSource::register(&mut rules);
    Linter {
        rules,
        error_format: ErrorFormat::Clippy,
    }
}

#[cfg(test)]
mod tests {
    use super::default;

    #[test]
    fn test_usage() {
        let linter = default();
        assert!(linter
            .rules
            .iter()
            .any(|rule| rule.name() == "no_free_functions"));
    }
}
