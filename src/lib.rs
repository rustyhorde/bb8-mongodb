// Copyright (c) 2021 bb8-mongodb developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! A [`bb8`](https://docs.rs/bb8) connection manager for a [`MongoDB`](https://www.mongodb.com/) connection pool
//!
//! # Example
//! ```
//! # use anyhow::Result;
//! # use bb8::Pool;
//! # use bb8_mongodb::MongodbConnectionManager;
//! # use mongodb::options::{ClientOptions, Credential};
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! // Setup the MongoDB `ClientOptions`
//! let mut client_options = ClientOptions::parse("mongodb://somedburi:27017").await?;
//! client_options.app_name = Some("app".to_string());
//! client_options.credential = Some(
//!     Credential::builder()
//!         .username(Some("dbuser".to_string()))
//!         .password(Some("dbpass".to_string()))
//!         .source(Some("dbauthsource".to_string()))
//!         .build(),
//! );
//!
//! // Setup the `bb8-mongodb` connection manager
//! let connection_manager = MongodbConnectionManager::new(client_options, "db");
//!
//! // Setup the `bb8` connection pool
//! let pool = Pool::builder().build(connection_manager).await?;
//!
//! // Get a connection from the pool
//! let conn = pool.get().await?;
//! # Ok(())
//! # }
//! ```
//!

// rustc lints
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    array_into_iter,
    asm_sub_register,
    bad_asm_style,
    bare_trait_objects,
    bindings_with_variant_name,
    // box_pointers,
    break_with_label_and_loop,
    clashing_extern_declarations,
    coherence_leak_check,
    confusable_idents,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_cfg_attr_crate_type_name,
    deprecated_in_future,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    forbidden_lint_groups,
    function_item_references,
    // fuzzy_provenance_casts,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_doc_attributes,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    // lossy_provenance_casts,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    mixed_script_confusables,
    // must_not_suspend,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    // non_exhaustive_omitted_patterns,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    overlapping_range_endpoints,
    path_statements,
    pointer_structural_match,
    redundant_semicolons,
    renamed_and_removed_lints,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    stable_features,
    suspicious_auto_trait_impls,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unexpected_cfgs,
    // unfulfilled_lint_expectations,
    uninhabited_static,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macro_rules,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_results,
    unused_unsafe,
    unused_variables,
    variant_size_differences,
    where_clauses_object_safety,
    while_true,
)]
// nightly only lints
#![cfg_attr(
    nightly_lints,
    deny(
        ffi_unwind_calls,
        named_arguments_used_positionally,
        repr_transparent_external_private_fields
    )
)]
// nightly or beta only lints
// #![cfg_attr(any(beta_lints, nightly_lints), deny())]
// beta only lints
// #![cfg_attr(beta_lints, deny())]
// beta or stable only lints
// #![cfg_attr( any(beta_lints, stable_lints), deny())]
// stable only lints
#![cfg_attr(stable_lints, deny(cenum_impl_drop_cast))]
// clippy lints
#![deny(clippy::all, clippy::pedantic)]
// #![allow(clippy::default_trait_access)]
// #![cfg_attr(nightly_lints, allow(clippy::nonstandard_macro_braces))]
// rustdoc lints
#![deny(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::missing_crate_level_docs,
    // rustdoc::missing_doc_code_examples,
    // rustdoc::private_doc_tests,
    rustdoc::private_intra_doc_links,
)]

mod error;
mod manager;

// used in doctests
use anyhow as _;
use tokio as _;

pub use error::Error;
pub use manager::Mongodb as MongodbConnectionManager;
