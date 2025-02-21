/// pub trait for rustls API - version that includes Send & Sync - supports use with alloc::sync::Arc
#[cfg(not(use_rc_alias))]
macro_rules! rustls_api_trait {
    ($name:ident, $x:meta, $_extra_separator_to_help_avoid_bad_code_formatting:tt, $body:tt) => {
        // XXX TBD RESOLVE TODO - ??? ???
        // TODO: should only allow unreachable_pub in more limited cases
        #[allow(unreachable_pub)]
        #[$x]
        pub trait $name: core::fmt::Debug + Send + Sync $body
    }
}

/// pub trait for rustls API - version with no Send / Sync - supports use with alloc::rc::Rc
#[cfg(use_rc_alias)]
macro_rules! rustls_api_trait {
    ($name:ident, $x:meta, $_extra_separator_to_help_avoid_bad_code_formatting:tt, $body:tt) => {
        #[allow(unreachable_pub)]
        #[$x]
        pub trait $name: core::fmt::Debug $body
    }
}

/// internal pub(crate) trait that includes Send & Sync - supports use with alloc::sync::Arc
#[cfg(not(use_rc_alias))]
macro_rules! internal_generic_state_trait {
    // XXX QUICK HACKY MACRO API WITH SEPARATE NAME & GENERIC TYPE PARAMETERS
    ($name:ident, $generic_type_parameter:ident, $body:tt) => {
        pub(crate) trait $name<$generic_type_parameter>: Send + Sync $body
    }
}

/// internal pub(crate) trait with no Send / Sync - supports use with alloc::rc::Rc
#[cfg(use_rc_alias)]
macro_rules! internal_generic_state_trait {
    // XXX QUICK HACKY MACRO API WITH SEPARATE NAME & GENERIC TYPE PARAMETERS
    ($name:ident, $generic_type_parameter:ident, $body:tt) => {
        pub(crate) trait $name<$generic_type_parameter> $body
    }
}
