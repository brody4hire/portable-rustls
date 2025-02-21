/// pub trait - version with doc - version that includes Send & Sync - supports use with alloc::sync::Arc
#[cfg(not(use_rc_alias))]
macro_rules! pub_api_trait_with_doc {
    ($doc_text: literal, $name:ident, $body:tt) => {
        #[doc = $doc_text]
        pub trait $name: core::fmt::Debug + Send + Sync $body
    }
}

macro_rules! pub_api_trait_with_xxx_meta_xxx {
    ($name:ident, $x:meta, $extra_ignored:tt, $body:tt) => {
        #[$x]
        pub trait $name: core::fmt::Debug + Send + Sync $body
    }
}

/////// XXX TODO REPLACE ALL USE OF THIS MACRO WITH pub_api_trait_with_doc! (with doc fixed) & REMOVE THIS MACRO
/// pub trait - version with no doc - version that includes Send & Sync - supports use with alloc::sync::Arc
#[cfg(not(use_rc_alias))]
macro_rules! pub_api_trait {
    ($name:ident, $body:tt) => {
        pub trait $name: core::fmt::Debug + Send + Sync $body
    }
}

/// pub trait - version with doc - version with no Send / Sync - supports use with alloc::rc::Rc
#[cfg(use_rc_alias)]
macro_rules! pub_api_trait_with_doc {
    ($doc_text: literal, $name:ident, $body:tt) => {
        #[doc = $doc_text]
        pub trait $name: core::fmt::Debug $body
    }
}

/////// XXX TODO REPLACE ALL USE OF THIS MACRO WITH pub_api_trait_with_doc! (with doc fixed) & REMOVE THIS MACRO
/// pub trait - version with no doc - version with no Send / Sync - supports use with alloc::rc::Rc
#[cfg(use_rc_alias)]
macro_rules! pub_api_trait {
    ($name:ident, $body:tt) => {
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
