// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

pub(crate) static LINTS: &[&crate::LintInfo] = &[
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::clippy_lints_internal::CLIPPY_LINTS_INTERNAL_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::collapsible_calls::COLLAPSIBLE_SPAN_LINT_CALLS_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::compiler_lint_functions::COMPILER_LINT_FUNCTIONS_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::if_chain_style::IF_CHAIN_STYLE_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::interning_defined_symbol::INTERNING_DEFINED_SYMBOL_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::interning_defined_symbol::UNNECESSARY_SYMBOL_STR_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::invalid_paths::INVALID_PATHS_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::lint_without_lint_pass::DEFAULT_DEPRECATION_REASON_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::lint_without_lint_pass::DEFAULT_LINT_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::lint_without_lint_pass::INVALID_CLIPPY_VERSION_ATTRIBUTE_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::lint_without_lint_pass::LINT_WITHOUT_LINT_PASS_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::lint_without_lint_pass::MISSING_CLIPPY_VERSION_ATTRIBUTE_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::msrv_attr_impl::MISSING_MSRV_ATTR_IMPL_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::outer_expn_data_pass::OUTER_EXPN_EXPN_DATA_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::produce_ice::PRODUCE_ICE_INFO,
    #[cfg(feature = "internal")]
    crate::utils::internal_lints::unnecessary_def_path::UNNECESSARY_DEF_PATH_INFO,
    crate::allow_attributes::ALLOW_ATTRIBUTES_INFO,
    crate::almost_complete_range::ALMOST_COMPLETE_RANGE_INFO,
    crate::approx_const::APPROX_CONSTANT_INFO,
    crate::as_conversions::AS_CONVERSIONS_INFO,
    crate::asm_syntax::INLINE_ASM_X86_ATT_SYNTAX_INFO,
    crate::asm_syntax::INLINE_ASM_X86_INTEL_SYNTAX_INFO,
    crate::assertions_on_constants::ASSERTIONS_ON_CONSTANTS_INFO,
    crate::assertions_on_result_states::ASSERTIONS_ON_RESULT_STATES_INFO,
    crate::async_yields_async::ASYNC_YIELDS_ASYNC_INFO,
    crate::attrs::ALLOW_ATTRIBUTES_WITHOUT_REASON_INFO,
    crate::attrs::BLANKET_CLIPPY_RESTRICTION_LINTS_INFO,
    crate::attrs::DEPRECATED_CFG_ATTR_INFO,
    crate::attrs::DEPRECATED_SEMVER_INFO,
    crate::attrs::EMPTY_LINE_AFTER_OUTER_ATTR_INFO,
    crate::attrs::INLINE_ALWAYS_INFO,
    crate::attrs::MISMATCHED_TARGET_OS_INFO,
    crate::attrs::USELESS_ATTRIBUTE_INFO,
    crate::await_holding_invalid::AWAIT_HOLDING_INVALID_TYPE_INFO,
    crate::await_holding_invalid::AWAIT_HOLDING_LOCK_INFO,
    crate::await_holding_invalid::AWAIT_HOLDING_REFCELL_REF_INFO,
    crate::blocks_in_if_conditions::BLOCKS_IN_IF_CONDITIONS_INFO,
    crate::bool_assert_comparison::BOOL_ASSERT_COMPARISON_INFO,
    crate::bool_to_int_with_if::BOOL_TO_INT_WITH_IF_INFO,
    crate::booleans::NONMINIMAL_BOOL_INFO,
    crate::booleans::OVERLY_COMPLEX_BOOL_EXPR_INFO,
    crate::borrow_deref_ref::BORROW_DEREF_REF_INFO,
    crate::box_default::BOX_DEFAULT_INFO,
    crate::cargo::CARGO_COMMON_METADATA_INFO,
    crate::cargo::MULTIPLE_CRATE_VERSIONS_INFO,
    crate::cargo::NEGATIVE_FEATURE_NAMES_INFO,
    crate::cargo::REDUNDANT_FEATURE_NAMES_INFO,
    crate::cargo::WILDCARD_DEPENDENCIES_INFO,
    crate::casts::AS_PTR_CAST_MUT_INFO,
    crate::casts::AS_UNDERSCORE_INFO,
    crate::casts::BORROW_AS_PTR_INFO,
    crate::casts::CAST_ABS_TO_UNSIGNED_INFO,
    crate::casts::CAST_ENUM_CONSTRUCTOR_INFO,
    crate::casts::CAST_ENUM_TRUNCATION_INFO,
    crate::casts::CAST_LOSSLESS_INFO,
    crate::casts::CAST_NAN_TO_INT_INFO,
    crate::casts::CAST_POSSIBLE_TRUNCATION_INFO,
    crate::casts::CAST_POSSIBLE_WRAP_INFO,
    crate::casts::CAST_PRECISION_LOSS_INFO,
    crate::casts::CAST_PTR_ALIGNMENT_INFO,
    crate::casts::CAST_REF_TO_MUT_INFO,
    crate::casts::CAST_SIGN_LOSS_INFO,
    crate::casts::CAST_SLICE_DIFFERENT_SIZES_INFO,
    crate::casts::CAST_SLICE_FROM_RAW_PARTS_INFO,
    crate::casts::CHAR_LIT_AS_U8_INFO,
    crate::casts::FN_TO_NUMERIC_CAST_INFO,
    crate::casts::FN_TO_NUMERIC_CAST_ANY_INFO,
    crate::casts::FN_TO_NUMERIC_CAST_WITH_TRUNCATION_INFO,
    crate::casts::PTR_AS_PTR_INFO,
    crate::casts::UNNECESSARY_CAST_INFO,
    crate::checked_conversions::CHECKED_CONVERSIONS_INFO,
    crate::cognitive_complexity::COGNITIVE_COMPLEXITY_INFO,
    crate::collapsible_if::COLLAPSIBLE_ELSE_IF_INFO,
    crate::collapsible_if::COLLAPSIBLE_IF_INFO,
    crate::collection_is_never_read::COLLECTION_IS_NEVER_READ_INFO,
    crate::comparison_chain::COMPARISON_CHAIN_INFO,
    crate::copies::BRANCHES_SHARING_CODE_INFO,
    crate::copies::IFS_SAME_COND_INFO,
    crate::copies::IF_SAME_THEN_ELSE_INFO,
    crate::copies::SAME_FUNCTIONS_IN_IF_CONDITION_INFO,
    crate::copy_iterator::COPY_ITERATOR_INFO,
    crate::crate_in_macro_def::CRATE_IN_MACRO_DEF_INFO,
    crate::create_dir::CREATE_DIR_INFO,
    crate::dbg_macro::DBG_MACRO_INFO,
    crate::default::DEFAULT_TRAIT_ACCESS_INFO,
    crate::default::FIELD_REASSIGN_WITH_DEFAULT_INFO,
    crate::default_instead_of_iter_empty::DEFAULT_INSTEAD_OF_ITER_EMPTY_INFO,
    crate::default_numeric_fallback::DEFAULT_NUMERIC_FALLBACK_INFO,
    crate::default_union_representation::DEFAULT_UNION_REPRESENTATION_INFO,
    crate::dereference::EXPLICIT_AUTO_DEREF_INFO,
    crate::dereference::EXPLICIT_DEREF_METHODS_INFO,
    crate::dereference::NEEDLESS_BORROW_INFO,
    crate::dereference::REF_BINDING_TO_REFERENCE_INFO,
    crate::derivable_impls::DERIVABLE_IMPLS_INFO,
    crate::derive::DERIVED_HASH_WITH_MANUAL_EQ_INFO,
    crate::derive::DERIVE_ORD_XOR_PARTIAL_ORD_INFO,
    crate::derive::DERIVE_PARTIAL_EQ_WITHOUT_EQ_INFO,
    crate::derive::EXPL_IMPL_CLONE_ON_COPY_INFO,
    crate::derive::UNSAFE_DERIVE_DESERIALIZE_INFO,
    crate::disallowed_macros::DISALLOWED_MACROS_INFO,
    crate::disallowed_methods::DISALLOWED_METHODS_INFO,
    crate::disallowed_names::DISALLOWED_NAMES_INFO,
    crate::disallowed_script_idents::DISALLOWED_SCRIPT_IDENTS_INFO,
    crate::disallowed_types::DISALLOWED_TYPES_INFO,
    crate::doc::DOC_LINK_WITH_QUOTES_INFO,
    crate::doc::DOC_MARKDOWN_INFO,
    crate::doc::MISSING_ERRORS_DOC_INFO,
    crate::doc::MISSING_PANICS_DOC_INFO,
    crate::doc::MISSING_SAFETY_DOC_INFO,
    crate::doc::NEEDLESS_DOCTEST_MAIN_INFO,
    crate::doc::UNNECESSARY_SAFETY_DOC_INFO,
    crate::double_parens::DOUBLE_PARENS_INFO,
    crate::drop_forget_ref::DROP_COPY_INFO,
    crate::drop_forget_ref::DROP_NON_DROP_INFO,
    crate::drop_forget_ref::DROP_REF_INFO,
    crate::drop_forget_ref::FORGET_COPY_INFO,
    crate::drop_forget_ref::FORGET_NON_DROP_INFO,
    crate::drop_forget_ref::FORGET_REF_INFO,
    crate::drop_forget_ref::UNDROPPED_MANUALLY_DROPS_INFO,
    crate::duplicate_mod::DUPLICATE_MOD_INFO,
    crate::else_if_without_else::ELSE_IF_WITHOUT_ELSE_INFO,
    crate::empty_drop::EMPTY_DROP_INFO,
    crate::empty_enum::EMPTY_ENUM_INFO,
    crate::empty_structs_with_brackets::EMPTY_STRUCTS_WITH_BRACKETS_INFO,
    crate::entry::MAP_ENTRY_INFO,
    crate::enum_clike::ENUM_CLIKE_UNPORTABLE_VARIANT_INFO,
    crate::enum_variants::ENUM_VARIANT_NAMES_INFO,
    crate::enum_variants::MODULE_INCEPTION_INFO,
    crate::enum_variants::MODULE_NAME_REPETITIONS_INFO,
    crate::equatable_if_let::EQUATABLE_IF_LET_INFO,
    crate::escape::BOXED_LOCAL_INFO,
    crate::eta_reduction::REDUNDANT_CLOSURE_INFO,
    crate::eta_reduction::REDUNDANT_CLOSURE_FOR_METHOD_CALLS_INFO,
    crate::excessive_bools::FN_PARAMS_EXCESSIVE_BOOLS_INFO,
    crate::excessive_bools::STRUCT_EXCESSIVE_BOOLS_INFO,
    crate::exhaustive_items::EXHAUSTIVE_ENUMS_INFO,
    crate::exhaustive_items::EXHAUSTIVE_STRUCTS_INFO,
    crate::exit::EXIT_INFO,
    crate::explicit_write::EXPLICIT_WRITE_INFO,
    crate::extra_unused_type_parameters::EXTRA_UNUSED_TYPE_PARAMETERS_INFO,
    crate::fallible_impl_from::FALLIBLE_IMPL_FROM_INFO,
    crate::float_literal::EXCESSIVE_PRECISION_INFO,
    crate::float_literal::LOSSY_FLOAT_LITERAL_INFO,
    crate::floating_point_arithmetic::IMPRECISE_FLOPS_INFO,
    crate::floating_point_arithmetic::SUBOPTIMAL_FLOPS_INFO,
    crate::fn_null_check::FN_NULL_CHECK_INFO,
    crate::format::USELESS_FORMAT_INFO,
    crate::format_args::FORMAT_IN_FORMAT_ARGS_INFO,
    crate::format_args::TO_STRING_IN_FORMAT_ARGS_INFO,
    crate::format_args::UNINLINED_FORMAT_ARGS_INFO,
    crate::format_args::UNUSED_FORMAT_SPECS_INFO,
    crate::format_impl::PRINT_IN_FORMAT_IMPL_INFO,
    crate::format_impl::RECURSIVE_FORMAT_IMPL_INFO,
    crate::format_push_string::FORMAT_PUSH_STRING_INFO,
    crate::formatting::POSSIBLE_MISSING_COMMA_INFO,
    crate::formatting::SUSPICIOUS_ASSIGNMENT_FORMATTING_INFO,
    crate::formatting::SUSPICIOUS_ELSE_FORMATTING_INFO,
    crate::formatting::SUSPICIOUS_UNARY_OP_FORMATTING_INFO,
    crate::from_over_into::FROM_OVER_INTO_INFO,
    crate::from_raw_with_void_ptr::FROM_RAW_WITH_VOID_PTR_INFO,
    crate::from_str_radix_10::FROM_STR_RADIX_10_INFO,
    crate::functions::DOUBLE_MUST_USE_INFO,
    crate::functions::IMPL_TRAIT_IN_PARAMS_INFO,
    crate::functions::MISNAMED_GETTERS_INFO,
    crate::functions::MUST_USE_CANDIDATE_INFO,
    crate::functions::MUST_USE_UNIT_INFO,
    crate::functions::NOT_UNSAFE_PTR_ARG_DEREF_INFO,
    crate::functions::RESULT_LARGE_ERR_INFO,
    crate::functions::RESULT_UNIT_ERR_INFO,
    crate::functions::TOO_MANY_ARGUMENTS_INFO,
    crate::functions::TOO_MANY_LINES_INFO,
    crate::future_not_send::FUTURE_NOT_SEND_INFO,
    crate::if_let_mutex::IF_LET_MUTEX_INFO,
    crate::if_not_else::IF_NOT_ELSE_INFO,
    crate::if_then_some_else_none::IF_THEN_SOME_ELSE_NONE_INFO,
    crate::implicit_hasher::IMPLICIT_HASHER_INFO,
    crate::implicit_return::IMPLICIT_RETURN_INFO,
    crate::implicit_saturating_add::IMPLICIT_SATURATING_ADD_INFO,
    crate::implicit_saturating_sub::IMPLICIT_SATURATING_SUB_INFO,
    crate::inconsistent_struct_constructor::INCONSISTENT_STRUCT_CONSTRUCTOR_INFO,
    crate::index_refutable_slice::INDEX_REFUTABLE_SLICE_INFO,
    crate::indexing_slicing::INDEXING_SLICING_INFO,
    crate::indexing_slicing::OUT_OF_BOUNDS_INDEXING_INFO,
    crate::infinite_iter::INFINITE_ITER_INFO,
    crate::infinite_iter::MAYBE_INFINITE_ITER_INFO,
    crate::inherent_impl::MULTIPLE_INHERENT_IMPL_INFO,
    crate::inherent_to_string::INHERENT_TO_STRING_INFO,
    crate::inherent_to_string::INHERENT_TO_STRING_SHADOW_DISPLAY_INFO,
    crate::init_numbered_fields::INIT_NUMBERED_FIELDS_INFO,
    crate::inline_fn_without_body::INLINE_FN_WITHOUT_BODY_INFO,
    crate::instant_subtraction::MANUAL_INSTANT_ELAPSED_INFO,
    crate::instant_subtraction::UNCHECKED_DURATION_SUBTRACTION_INFO,
    crate::int_plus_one::INT_PLUS_ONE_INFO,
    crate::invalid_upcast_comparisons::INVALID_UPCAST_COMPARISONS_INFO,
    crate::invalid_utf8_in_unchecked::INVALID_UTF8_IN_UNCHECKED_INFO,
    crate::items_after_statements::ITEMS_AFTER_STATEMENTS_INFO,
    crate::iter_not_returning_iterator::ITER_NOT_RETURNING_ITERATOR_INFO,
    crate::large_const_arrays::LARGE_CONST_ARRAYS_INFO,
    crate::large_enum_variant::LARGE_ENUM_VARIANT_INFO,
    crate::large_include_file::LARGE_INCLUDE_FILE_INFO,
    crate::large_stack_arrays::LARGE_STACK_ARRAYS_INFO,
    crate::len_zero::COMPARISON_TO_EMPTY_INFO,
    crate::len_zero::LEN_WITHOUT_IS_EMPTY_INFO,
    crate::len_zero::LEN_ZERO_INFO,
    crate::let_if_seq::USELESS_LET_IF_SEQ_INFO,
    crate::let_underscore::LET_UNDERSCORE_FUTURE_INFO,
    crate::let_underscore::LET_UNDERSCORE_LOCK_INFO,
    crate::let_underscore::LET_UNDERSCORE_MUST_USE_INFO,
    crate::let_underscore::LET_UNDERSCORE_UNTYPED_INFO,
    crate::let_with_type_underscore::LET_WITH_TYPE_UNDERSCORE_INFO,
    crate::lifetimes::EXTRA_UNUSED_LIFETIMES_INFO,
    crate::lifetimes::NEEDLESS_LIFETIMES_INFO,
    crate::literal_representation::DECIMAL_LITERAL_REPRESENTATION_INFO,
    crate::literal_representation::INCONSISTENT_DIGIT_GROUPING_INFO,
    crate::literal_representation::LARGE_DIGIT_GROUPS_INFO,
    crate::literal_representation::MISTYPED_LITERAL_SUFFIXES_INFO,
    crate::literal_representation::UNREADABLE_LITERAL_INFO,
    crate::literal_representation::UNUSUAL_BYTE_GROUPINGS_INFO,
    crate::loops::EMPTY_LOOP_INFO,
    crate::loops::EXPLICIT_COUNTER_LOOP_INFO,
    crate::loops::EXPLICIT_INTO_ITER_LOOP_INFO,
    crate::loops::EXPLICIT_ITER_LOOP_INFO,
    crate::loops::FOR_KV_MAP_INFO,
    crate::loops::ITER_NEXT_LOOP_INFO,
    crate::loops::MANUAL_FIND_INFO,
    crate::loops::MANUAL_FLATTEN_INFO,
    crate::loops::MANUAL_MEMCPY_INFO,
    crate::loops::MISSING_SPIN_LOOP_INFO,
    crate::loops::MUT_RANGE_BOUND_INFO,
    crate::loops::NEEDLESS_RANGE_LOOP_INFO,
    crate::loops::NEVER_LOOP_INFO,
    crate::loops::SAME_ITEM_PUSH_INFO,
    crate::loops::SINGLE_ELEMENT_LOOP_INFO,
    crate::loops::WHILE_IMMUTABLE_CONDITION_INFO,
    crate::loops::WHILE_LET_LOOP_INFO,
    crate::loops::WHILE_LET_ON_ITERATOR_INFO,
    crate::macro_use::MACRO_USE_IMPORTS_INFO,
    crate::main_recursion::MAIN_RECURSION_INFO,
    crate::manual_assert::MANUAL_ASSERT_INFO,
    crate::manual_async_fn::MANUAL_ASYNC_FN_INFO,
    crate::manual_bits::MANUAL_BITS_INFO,
    crate::manual_clamp::MANUAL_CLAMP_INFO,
    crate::manual_is_ascii_check::MANUAL_IS_ASCII_CHECK_INFO,
    crate::manual_let_else::MANUAL_LET_ELSE_INFO,
    crate::manual_main_separator_str::MANUAL_MAIN_SEPARATOR_STR_INFO,
    crate::manual_non_exhaustive::MANUAL_NON_EXHAUSTIVE_INFO,
    crate::manual_rem_euclid::MANUAL_REM_EUCLID_INFO,
    crate::manual_retain::MANUAL_RETAIN_INFO,
    crate::manual_string_new::MANUAL_STRING_NEW_INFO,
    crate::manual_strip::MANUAL_STRIP_INFO,
    crate::map_unit_fn::OPTION_MAP_UNIT_FN_INFO,
    crate::map_unit_fn::RESULT_MAP_UNIT_FN_INFO,
    crate::match_result_ok::MATCH_RESULT_OK_INFO,
    crate::matches::COLLAPSIBLE_MATCH_INFO,
    crate::matches::INFALLIBLE_DESTRUCTURING_MATCH_INFO,
    crate::matches::MANUAL_FILTER_INFO,
    crate::matches::MANUAL_MAP_INFO,
    crate::matches::MANUAL_UNWRAP_OR_INFO,
    crate::matches::MATCH_AS_REF_INFO,
    crate::matches::MATCH_BOOL_INFO,
    crate::matches::MATCH_LIKE_MATCHES_MACRO_INFO,
    crate::matches::MATCH_ON_VEC_ITEMS_INFO,
    crate::matches::MATCH_OVERLAPPING_ARM_INFO,
    crate::matches::MATCH_REF_PATS_INFO,
    crate::matches::MATCH_SAME_ARMS_INFO,
    crate::matches::MATCH_SINGLE_BINDING_INFO,
    crate::matches::MATCH_STR_CASE_MISMATCH_INFO,
    crate::matches::MATCH_WILDCARD_FOR_SINGLE_VARIANTS_INFO,
    crate::matches::MATCH_WILD_ERR_ARM_INFO,
    crate::matches::NEEDLESS_MATCH_INFO,
    crate::matches::REDUNDANT_PATTERN_MATCHING_INFO,
    crate::matches::REST_PAT_IN_FULLY_BOUND_STRUCTS_INFO,
    crate::matches::SIGNIFICANT_DROP_IN_SCRUTINEE_INFO,
    crate::matches::SINGLE_MATCH_INFO,
    crate::matches::SINGLE_MATCH_ELSE_INFO,
    crate::matches::TRY_ERR_INFO,
    crate::matches::WILDCARD_ENUM_MATCH_ARM_INFO,
    crate::matches::WILDCARD_IN_OR_PATTERNS_INFO,
    crate::mem_forget::MEM_FORGET_INFO,
    crate::mem_replace::MEM_REPLACE_OPTION_WITH_NONE_INFO,
    crate::mem_replace::MEM_REPLACE_WITH_DEFAULT_INFO,
    crate::mem_replace::MEM_REPLACE_WITH_UNINIT_INFO,
    crate::methods::BIND_INSTEAD_OF_MAP_INFO,
    crate::methods::BYTES_COUNT_TO_LEN_INFO,
    crate::methods::BYTES_NTH_INFO,
    crate::methods::CASE_SENSITIVE_FILE_EXTENSION_COMPARISONS_INFO,
    crate::methods::CHARS_LAST_CMP_INFO,
    crate::methods::CHARS_NEXT_CMP_INFO,
    crate::methods::CLONED_INSTEAD_OF_COPIED_INFO,
    crate::methods::CLONE_DOUBLE_REF_INFO,
    crate::methods::CLONE_ON_COPY_INFO,
    crate::methods::CLONE_ON_REF_PTR_INFO,
    crate::methods::COLLAPSIBLE_STR_REPLACE_INFO,
    crate::methods::ERR_EXPECT_INFO,
    crate::methods::EXPECT_FUN_CALL_INFO,
    crate::methods::EXPECT_USED_INFO,
    crate::methods::EXTEND_WITH_DRAIN_INFO,
    crate::methods::FILETYPE_IS_FILE_INFO,
    crate::methods::FILTER_MAP_IDENTITY_INFO,
    crate::methods::FILTER_MAP_NEXT_INFO,
    crate::methods::FILTER_NEXT_INFO,
    crate::methods::FLAT_MAP_IDENTITY_INFO,
    crate::methods::FLAT_MAP_OPTION_INFO,
    crate::methods::FROM_ITER_INSTEAD_OF_COLLECT_INFO,
    crate::methods::GET_FIRST_INFO,
    crate::methods::GET_LAST_WITH_LEN_INFO,
    crate::methods::GET_UNWRAP_INFO,
    crate::methods::IMPLICIT_CLONE_INFO,
    crate::methods::INEFFICIENT_TO_STRING_INFO,
    crate::methods::INSPECT_FOR_EACH_INFO,
    crate::methods::INTO_ITER_ON_REF_INFO,
    crate::methods::IS_DIGIT_ASCII_RADIX_INFO,
    crate::methods::ITERATOR_STEP_BY_ZERO_INFO,
    crate::methods::ITER_CLONED_COLLECT_INFO,
    crate::methods::ITER_COUNT_INFO,
    crate::methods::ITER_KV_MAP_INFO,
    crate::methods::ITER_NEXT_SLICE_INFO,
    crate::methods::ITER_NTH_INFO,
    crate::methods::ITER_NTH_ZERO_INFO,
    crate::methods::ITER_ON_EMPTY_COLLECTIONS_INFO,
    crate::methods::ITER_ON_SINGLE_ITEMS_INFO,
    crate::methods::ITER_OVEREAGER_CLONED_INFO,
    crate::methods::ITER_SKIP_NEXT_INFO,
    crate::methods::ITER_WITH_DRAIN_INFO,
    crate::methods::MANUAL_FILTER_MAP_INFO,
    crate::methods::MANUAL_FIND_MAP_INFO,
    crate::methods::MANUAL_OK_OR_INFO,
    crate::methods::MANUAL_SATURATING_ARITHMETIC_INFO,
    crate::methods::MANUAL_SPLIT_ONCE_INFO,
    crate::methods::MANUAL_STR_REPEAT_INFO,
    crate::methods::MAP_CLONE_INFO,
    crate::methods::MAP_COLLECT_RESULT_UNIT_INFO,
    crate::methods::MAP_ERR_IGNORE_INFO,
    crate::methods::MAP_FLATTEN_INFO,
    crate::methods::MAP_IDENTITY_INFO,
    crate::methods::MAP_UNWRAP_OR_INFO,
    crate::methods::MUT_MUTEX_LOCK_INFO,
    crate::methods::NAIVE_BYTECOUNT_INFO,
    crate::methods::NEEDLESS_COLLECT_INFO,
    crate::methods::NEEDLESS_OPTION_AS_DEREF_INFO,
    crate::methods::NEEDLESS_OPTION_TAKE_INFO,
    crate::methods::NEEDLESS_SPLITN_INFO,
    crate::methods::NEW_RET_NO_SELF_INFO,
    crate::methods::NONSENSICAL_OPEN_OPTIONS_INFO,
    crate::methods::NO_EFFECT_REPLACE_INFO,
    crate::methods::OBFUSCATED_IF_ELSE_INFO,
    crate::methods::OK_EXPECT_INFO,
    crate::methods::OPTION_AS_REF_DEREF_INFO,
    crate::methods::OPTION_FILTER_MAP_INFO,
    crate::methods::OPTION_MAP_OR_NONE_INFO,
    crate::methods::OR_FUN_CALL_INFO,
    crate::methods::OR_THEN_UNWRAP_INFO,
    crate::methods::PATH_BUF_PUSH_OVERWRITE_INFO,
    crate::methods::RANGE_ZIP_WITH_LEN_INFO,
    crate::methods::REPEAT_ONCE_INFO,
    crate::methods::RESULT_MAP_OR_INTO_OPTION_INFO,
    crate::methods::SEARCH_IS_SOME_INFO,
    crate::methods::SEEK_FROM_CURRENT_INFO,
    crate::methods::SEEK_TO_START_INSTEAD_OF_REWIND_INFO,
    crate::methods::SHOULD_IMPLEMENT_TRAIT_INFO,
    crate::methods::SINGLE_CHAR_ADD_STR_INFO,
    crate::methods::SINGLE_CHAR_PATTERN_INFO,
    crate::methods::SKIP_WHILE_NEXT_INFO,
    crate::methods::STABLE_SORT_PRIMITIVE_INFO,
    crate::methods::STRING_EXTEND_CHARS_INFO,
    crate::methods::SUSPICIOUS_COMMAND_ARG_SPACE_INFO,
    crate::methods::SUSPICIOUS_MAP_INFO,
    crate::methods::SUSPICIOUS_SPLITN_INFO,
    crate::methods::SUSPICIOUS_TO_OWNED_INFO,
    crate::methods::UNINIT_ASSUMED_INIT_INFO,
    crate::methods::UNIT_HASH_INFO,
    crate::methods::UNNECESSARY_FILTER_MAP_INFO,
    crate::methods::UNNECESSARY_FIND_MAP_INFO,
    crate::methods::UNNECESSARY_FOLD_INFO,
    crate::methods::UNNECESSARY_JOIN_INFO,
    crate::methods::UNNECESSARY_LAZY_EVALUATIONS_INFO,
    crate::methods::UNNECESSARY_SORT_BY_INFO,
    crate::methods::UNNECESSARY_TO_OWNED_INFO,
    crate::methods::UNWRAP_OR_ELSE_DEFAULT_INFO,
    crate::methods::UNWRAP_USED_INFO,
    crate::methods::USELESS_ASREF_INFO,
    crate::methods::VEC_RESIZE_TO_ZERO_INFO,
    crate::methods::VERBOSE_FILE_READS_INFO,
    crate::methods::WRONG_SELF_CONVENTION_INFO,
    crate::methods::ZST_OFFSET_INFO,
    crate::minmax::MIN_MAX_INFO,
    crate::misc::SHORT_CIRCUIT_STATEMENT_INFO,
    crate::misc::TOPLEVEL_REF_ARG_INFO,
    crate::misc::USED_UNDERSCORE_BINDING_INFO,
    crate::misc::ZERO_PTR_INFO,
    crate::misc_early::BUILTIN_TYPE_SHADOW_INFO,
    crate::misc_early::DOUBLE_NEG_INFO,
    crate::misc_early::DUPLICATE_UNDERSCORE_ARGUMENT_INFO,
    crate::misc_early::MIXED_CASE_HEX_LITERALS_INFO,
    crate::misc_early::REDUNDANT_PATTERN_INFO,
    crate::misc_early::SEPARATED_LITERAL_SUFFIX_INFO,
    crate::misc_early::UNNEEDED_FIELD_PATTERN_INFO,
    crate::misc_early::UNNEEDED_WILDCARD_PATTERN_INFO,
    crate::misc_early::UNSEPARATED_LITERAL_SUFFIX_INFO,
    crate::misc_early::ZERO_PREFIXED_LITERAL_INFO,
    crate::mismatching_type_param_order::MISMATCHING_TYPE_PARAM_ORDER_INFO,
    crate::missing_assert_message::MISSING_ASSERT_MESSAGE_INFO,
    crate::missing_const_for_fn::MISSING_CONST_FOR_FN_INFO,
    crate::missing_doc::MISSING_DOCS_IN_PRIVATE_ITEMS_INFO,
    crate::missing_enforced_import_rename::MISSING_ENFORCED_IMPORT_RENAMES_INFO,
    crate::missing_inline::MISSING_INLINE_IN_PUBLIC_ITEMS_INFO,
    crate::missing_trait_methods::MISSING_TRAIT_METHODS_INFO,
    crate::mixed_read_write_in_expression::DIVERGING_SUB_EXPRESSION_INFO,
    crate::mixed_read_write_in_expression::MIXED_READ_WRITE_IN_EXPRESSION_INFO,
    crate::module_style::MOD_MODULE_FILES_INFO,
    crate::module_style::SELF_NAMED_MODULE_FILES_INFO,
    crate::multi_assignments::MULTI_ASSIGNMENTS_INFO,
    crate::multiple_unsafe_ops_per_block::MULTIPLE_UNSAFE_OPS_PER_BLOCK_INFO,
    crate::mut_key::MUTABLE_KEY_TYPE_INFO,
    crate::mut_mut::MUT_MUT_INFO,
    crate::mut_reference::UNNECESSARY_MUT_PASSED_INFO,
    crate::mutable_debug_assertion::DEBUG_ASSERT_WITH_MUT_CALL_INFO,
    crate::mutex_atomic::MUTEX_ATOMIC_INFO,
    crate::mutex_atomic::MUTEX_INTEGER_INFO,
    crate::needless_arbitrary_self_type::NEEDLESS_ARBITRARY_SELF_TYPE_INFO,
    crate::needless_bool::BOOL_COMPARISON_INFO,
    crate::needless_bool::NEEDLESS_BOOL_INFO,
    crate::needless_borrowed_ref::NEEDLESS_BORROWED_REFERENCE_INFO,
    crate::needless_continue::NEEDLESS_CONTINUE_INFO,
    crate::needless_for_each::NEEDLESS_FOR_EACH_INFO,
    crate::needless_late_init::NEEDLESS_LATE_INIT_INFO,
    crate::needless_parens_on_range_literals::NEEDLESS_PARENS_ON_RANGE_LITERALS_INFO,
    crate::needless_pass_by_value::NEEDLESS_PASS_BY_VALUE_INFO,
    crate::needless_question_mark::NEEDLESS_QUESTION_MARK_INFO,
    crate::needless_update::NEEDLESS_UPDATE_INFO,
    crate::neg_cmp_op_on_partial_ord::NEG_CMP_OP_ON_PARTIAL_ORD_INFO,
    crate::neg_multiply::NEG_MULTIPLY_INFO,
    crate::new_without_default::NEW_WITHOUT_DEFAULT_INFO,
    crate::no_effect::NO_EFFECT_INFO,
    crate::no_effect::NO_EFFECT_UNDERSCORE_BINDING_INFO,
    crate::no_effect::UNNECESSARY_OPERATION_INFO,
    crate::no_mangle_with_rust_abi::NO_MANGLE_WITH_RUST_ABI_INFO,
    crate::non_copy_const::BORROW_INTERIOR_MUTABLE_CONST_INFO,
    crate::non_copy_const::DECLARE_INTERIOR_MUTABLE_CONST_INFO,
    crate::non_expressive_names::JUST_UNDERSCORES_AND_DIGITS_INFO,
    crate::non_expressive_names::MANY_SINGLE_CHAR_NAMES_INFO,
    crate::non_expressive_names::SIMILAR_NAMES_INFO,
    crate::non_octal_unix_permissions::NON_OCTAL_UNIX_PERMISSIONS_INFO,
    crate::non_send_fields_in_send_ty::NON_SEND_FIELDS_IN_SEND_TY_INFO,
    crate::nonstandard_macro_braces::NONSTANDARD_MACRO_BRACES_INFO,
    crate::octal_escapes::OCTAL_ESCAPES_INFO,
    crate::only_used_in_recursion::ONLY_USED_IN_RECURSION_INFO,
    crate::operators::ABSURD_EXTREME_COMPARISONS_INFO,
    crate::operators::ARITHMETIC_SIDE_EFFECTS_INFO,
    crate::operators::ASSIGN_OP_PATTERN_INFO,
    crate::operators::BAD_BIT_MASK_INFO,
    crate::operators::CMP_NAN_INFO,
    crate::operators::CMP_OWNED_INFO,
    crate::operators::DOUBLE_COMPARISONS_INFO,
    crate::operators::DURATION_SUBSEC_INFO,
    crate::operators::EQ_OP_INFO,
    crate::operators::ERASING_OP_INFO,
    crate::operators::FLOAT_ARITHMETIC_INFO,
    crate::operators::FLOAT_CMP_INFO,
    crate::operators::FLOAT_CMP_CONST_INFO,
    crate::operators::FLOAT_EQUALITY_WITHOUT_ABS_INFO,
    crate::operators::IDENTITY_OP_INFO,
    crate::operators::INEFFECTIVE_BIT_MASK_INFO,
    crate::operators::INTEGER_ARITHMETIC_INFO,
    crate::operators::INTEGER_DIVISION_INFO,
    crate::operators::MISREFACTORED_ASSIGN_OP_INFO,
    crate::operators::MODULO_ARITHMETIC_INFO,
    crate::operators::MODULO_ONE_INFO,
    crate::operators::NEEDLESS_BITWISE_BOOL_INFO,
    crate::operators::OP_REF_INFO,
    crate::operators::PTR_EQ_INFO,
    crate::operators::SELF_ASSIGNMENT_INFO,
    crate::operators::VERBOSE_BIT_MASK_INFO,
    crate::option_env_unwrap::OPTION_ENV_UNWRAP_INFO,
    crate::option_if_let_else::OPTION_IF_LET_ELSE_INFO,
    crate::overflow_check_conditional::OVERFLOW_CHECK_CONDITIONAL_INFO,
    crate::panic_in_result_fn::PANIC_IN_RESULT_FN_INFO,
    crate::panic_unimplemented::PANIC_INFO,
    crate::panic_unimplemented::TODO_INFO,
    crate::panic_unimplemented::UNIMPLEMENTED_INFO,
    crate::panic_unimplemented::UNREACHABLE_INFO,
    crate::partial_pub_fields::PARTIAL_PUB_FIELDS_INFO,
    crate::partialeq_ne_impl::PARTIALEQ_NE_IMPL_INFO,
    crate::partialeq_to_none::PARTIALEQ_TO_NONE_INFO,
    crate::pass_by_ref_or_value::LARGE_TYPES_PASSED_BY_VALUE_INFO,
    crate::pass_by_ref_or_value::TRIVIALLY_COPY_PASS_BY_REF_INFO,
    crate::pattern_type_mismatch::PATTERN_TYPE_MISMATCH_INFO,
    crate::permissions_set_readonly_false::PERMISSIONS_SET_READONLY_FALSE_INFO,
    crate::precedence::PRECEDENCE_INFO,
    crate::ptr::CMP_NULL_INFO,
    crate::ptr::INVALID_NULL_PTR_USAGE_INFO,
    crate::ptr::MUT_FROM_REF_INFO,
    crate::ptr::PTR_ARG_INFO,
    crate::ptr_offset_with_cast::PTR_OFFSET_WITH_CAST_INFO,
    crate::pub_use::PUB_USE_INFO,
    crate::question_mark::QUESTION_MARK_INFO,
    crate::question_mark_used::QUESTION_MARK_USED_INFO,
    crate::ranges::MANUAL_RANGE_CONTAINS_INFO,
    crate::ranges::RANGE_MINUS_ONE_INFO,
    crate::ranges::RANGE_PLUS_ONE_INFO,
    crate::ranges::REVERSED_EMPTY_RANGES_INFO,
    crate::rc_clone_in_vec_init::RC_CLONE_IN_VEC_INIT_INFO,
    crate::read_zero_byte_vec::READ_ZERO_BYTE_VEC_INFO,
    crate::redundant_async_block::REDUNDANT_ASYNC_BLOCK_INFO,
    crate::redundant_clone::REDUNDANT_CLONE_INFO,
    crate::redundant_closure_call::REDUNDANT_CLOSURE_CALL_INFO,
    crate::redundant_else::REDUNDANT_ELSE_INFO,
    crate::redundant_field_names::REDUNDANT_FIELD_NAMES_INFO,
    crate::redundant_pub_crate::REDUNDANT_PUB_CRATE_INFO,
    crate::redundant_slicing::DEREF_BY_SLICING_INFO,
    crate::redundant_slicing::REDUNDANT_SLICING_INFO,
    crate::redundant_static_lifetimes::REDUNDANT_STATIC_LIFETIMES_INFO,
    crate::ref_option_ref::REF_OPTION_REF_INFO,
    crate::reference::DEREF_ADDROF_INFO,
    crate::regex::INVALID_REGEX_INFO,
    crate::regex::TRIVIAL_REGEX_INFO,
    crate::return_self_not_must_use::RETURN_SELF_NOT_MUST_USE_INFO,
    crate::returns::LET_AND_RETURN_INFO,
    crate::returns::NEEDLESS_RETURN_INFO,
    crate::same_name_method::SAME_NAME_METHOD_INFO,
    crate::self_named_constructors::SELF_NAMED_CONSTRUCTORS_INFO,
    crate::semicolon_block::SEMICOLON_INSIDE_BLOCK_INFO,
    crate::semicolon_block::SEMICOLON_OUTSIDE_BLOCK_INFO,
    crate::semicolon_if_nothing_returned::SEMICOLON_IF_NOTHING_RETURNED_INFO,
    crate::serde_api::SERDE_API_MISUSE_INFO,
    crate::shadow::SHADOW_REUSE_INFO,
    crate::shadow::SHADOW_SAME_INFO,
    crate::shadow::SHADOW_UNRELATED_INFO,
    crate::significant_drop_tightening::SIGNIFICANT_DROP_TIGHTENING_INFO,
    crate::single_char_lifetime_names::SINGLE_CHAR_LIFETIME_NAMES_INFO,
    crate::single_component_path_imports::SINGLE_COMPONENT_PATH_IMPORTS_INFO,
    crate::size_of_in_element_count::SIZE_OF_IN_ELEMENT_COUNT_INFO,
    crate::size_of_ref::SIZE_OF_REF_INFO,
    crate::slow_vector_initialization::SLOW_VECTOR_INITIALIZATION_INFO,
    crate::std_instead_of_core::ALLOC_INSTEAD_OF_CORE_INFO,
    crate::std_instead_of_core::STD_INSTEAD_OF_ALLOC_INFO,
    crate::std_instead_of_core::STD_INSTEAD_OF_CORE_INFO,
    crate::strings::STRING_ADD_INFO,
    crate::strings::STRING_ADD_ASSIGN_INFO,
    crate::strings::STRING_FROM_UTF8_AS_BYTES_INFO,
    crate::strings::STRING_LIT_AS_BYTES_INFO,
    crate::strings::STRING_SLICE_INFO,
    crate::strings::STRING_TO_STRING_INFO,
    crate::strings::STR_TO_STRING_INFO,
    crate::strings::TRIM_SPLIT_WHITESPACE_INFO,
    crate::strlen_on_c_strings::STRLEN_ON_C_STRINGS_INFO,
    crate::suspicious_operation_groupings::SUSPICIOUS_OPERATION_GROUPINGS_INFO,
    crate::suspicious_trait_impl::SUSPICIOUS_ARITHMETIC_IMPL_INFO,
    crate::suspicious_trait_impl::SUSPICIOUS_OP_ASSIGN_IMPL_INFO,
    crate::suspicious_xor_used_as_pow::SUSPICIOUS_XOR_USED_AS_POW_INFO,
    crate::swap::ALMOST_SWAPPED_INFO,
    crate::swap::MANUAL_SWAP_INFO,
    crate::swap_ptr_to_ref::SWAP_PTR_TO_REF_INFO,
    crate::tabs_in_doc_comments::TABS_IN_DOC_COMMENTS_INFO,
    crate::temporary_assignment::TEMPORARY_ASSIGNMENT_INFO,
    crate::to_digit_is_some::TO_DIGIT_IS_SOME_INFO,
    crate::trailing_empty_array::TRAILING_EMPTY_ARRAY_INFO,
    crate::trait_bounds::TRAIT_DUPLICATION_IN_BOUNDS_INFO,
    crate::trait_bounds::TYPE_REPETITION_IN_BOUNDS_INFO,
    crate::transmute::CROSSPOINTER_TRANSMUTE_INFO,
    crate::transmute::TRANSMUTES_EXPRESSIBLE_AS_PTR_CASTS_INFO,
    crate::transmute::TRANSMUTE_BYTES_TO_STR_INFO,
    crate::transmute::TRANSMUTE_FLOAT_TO_INT_INFO,
    crate::transmute::TRANSMUTE_INT_TO_BOOL_INFO,
    crate::transmute::TRANSMUTE_INT_TO_CHAR_INFO,
    crate::transmute::TRANSMUTE_INT_TO_FLOAT_INFO,
    crate::transmute::TRANSMUTE_INT_TO_NON_ZERO_INFO,
    crate::transmute::TRANSMUTE_NULL_TO_FN_INFO,
    crate::transmute::TRANSMUTE_NUM_TO_BYTES_INFO,
    crate::transmute::TRANSMUTE_PTR_TO_PTR_INFO,
    crate::transmute::TRANSMUTE_PTR_TO_REF_INFO,
    crate::transmute::TRANSMUTE_UNDEFINED_REPR_INFO,
    crate::transmute::TRANSMUTING_NULL_INFO,
    crate::transmute::UNSOUND_COLLECTION_TRANSMUTE_INFO,
    crate::transmute::USELESS_TRANSMUTE_INFO,
    crate::transmute::WRONG_TRANSMUTE_INFO,
    crate::types::BORROWED_BOX_INFO,
    crate::types::BOX_COLLECTION_INFO,
    crate::types::LINKEDLIST_INFO,
    crate::types::OPTION_OPTION_INFO,
    crate::types::RC_BUFFER_INFO,
    crate::types::RC_MUTEX_INFO,
    crate::types::REDUNDANT_ALLOCATION_INFO,
    crate::types::TYPE_COMPLEXITY_INFO,
    crate::types::VEC_BOX_INFO,
    crate::undocumented_unsafe_blocks::UNDOCUMENTED_UNSAFE_BLOCKS_INFO,
    crate::undocumented_unsafe_blocks::UNNECESSARY_SAFETY_COMMENT_INFO,
    crate::unicode::INVISIBLE_CHARACTERS_INFO,
    crate::unicode::NON_ASCII_LITERAL_INFO,
    crate::unicode::UNICODE_NOT_NFC_INFO,
    crate::uninit_vec::UNINIT_VEC_INFO,
    crate::unit_return_expecting_ord::UNIT_RETURN_EXPECTING_ORD_INFO,
    crate::unit_types::LET_UNIT_VALUE_INFO,
    crate::unit_types::UNIT_ARG_INFO,
    crate::unit_types::UNIT_CMP_INFO,
    crate::unnamed_address::FN_ADDRESS_COMPARISONS_INFO,
    crate::unnamed_address::VTABLE_ADDRESS_COMPARISONS_INFO,
    crate::unnecessary_box_returns::UNNECESSARY_BOX_RETURNS_INFO,
    crate::unnecessary_owned_empty_strings::UNNECESSARY_OWNED_EMPTY_STRINGS_INFO,
    crate::unnecessary_self_imports::UNNECESSARY_SELF_IMPORTS_INFO,
    crate::unnecessary_struct_initialization::UNNECESSARY_STRUCT_INITIALIZATION_INFO,
    crate::unnecessary_wraps::UNNECESSARY_WRAPS_INFO,
    crate::unnested_or_patterns::UNNESTED_OR_PATTERNS_INFO,
    crate::unsafe_removed_from_name::UNSAFE_REMOVED_FROM_NAME_INFO,
    crate::unused_async::UNUSED_ASYNC_INFO,
    crate::unused_io_amount::UNUSED_IO_AMOUNT_INFO,
    crate::unused_peekable::UNUSED_PEEKABLE_INFO,
    crate::unused_rounding::UNUSED_ROUNDING_INFO,
    crate::unused_self::UNUSED_SELF_INFO,
    crate::unused_unit::UNUSED_UNIT_INFO,
    crate::unwrap::PANICKING_UNWRAP_INFO,
    crate::unwrap::UNNECESSARY_UNWRAP_INFO,
    crate::unwrap_in_result::UNWRAP_IN_RESULT_INFO,
    crate::upper_case_acronyms::UPPER_CASE_ACRONYMS_INFO,
    crate::use_self::USE_SELF_INFO,
    crate::useless_conversion::USELESS_CONVERSION_INFO,
    crate::vec::USELESS_VEC_INFO,
    crate::vec_init_then_push::VEC_INIT_THEN_PUSH_INFO,
    crate::wildcard_imports::ENUM_GLOB_USE_INFO,
    crate::wildcard_imports::WILDCARD_IMPORTS_INFO,
    crate::write::PRINTLN_EMPTY_STRING_INFO,
    crate::write::PRINT_LITERAL_INFO,
    crate::write::PRINT_STDERR_INFO,
    crate::write::PRINT_STDOUT_INFO,
    crate::write::PRINT_WITH_NEWLINE_INFO,
    crate::write::USE_DEBUG_INFO,
    crate::write::WRITELN_EMPTY_STRING_INFO,
    crate::write::WRITE_LITERAL_INFO,
    crate::write::WRITE_WITH_NEWLINE_INFO,
    crate::zero_div_zero::ZERO_DIVIDED_BY_ZERO_INFO,
    crate::zero_sized_map_values::ZERO_SIZED_MAP_VALUES_INFO,
];
