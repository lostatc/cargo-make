#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    array_into_iter,
    bindings_with_variant_name,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incomplete_features,
    indirect_structural_match,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_patterns,
    path_statements,
    patterns_in_fns_without_body,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolon,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    intra_doc_link_resolution_failure,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # cargo-make
//!
//! Rust task runner and build tool.<br>
//! The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
//! A task is a command or a script to execute.<br>
//! Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
//! With a simple toml based configuration file, you can define a multi platform build script that can run build, test,
//! documentation generation, bench tests execution, security validations and more by running a single command.
//!
//! ## Installation
//! In order to install, just run the following command
//!
//! ```sh
//! cargo install cargo-make
//! ```
//!
//! This will install cargo-make in your ~/.cargo/bin.<br>
//! Make sure to add ~/.cargo/bin directory to your PATH variable.
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/cargo-make/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/cargo-make/blob/master/LICENSE) open source license.
//!

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod test;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

// make types public for docs
pub mod types;

mod cache;
mod cli;
mod cli_commands;
mod command;
mod condition;
mod config;
mod descriptor;
mod environment;
mod execution_plan;
mod functions;
mod installer;
mod io;
mod legacy;
mod logger;
mod profile;
mod runner;
mod scriptengine;
mod storage;
mod toolchain;
mod version;

/// Handles the command line arguments and executes the runner.
pub fn run_cli(command_name: String, sub_command: bool) {
    cli::run_cli(command_name, sub_command)
}
