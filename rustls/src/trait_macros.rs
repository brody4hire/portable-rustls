// XXX XXX
macro_rules! pub_api_trait_with_doc {
    ($doc_text: literal, $name:ident, $body:tt) => {
        #[doc = $doc_text]
        pub trait $name: core::fmt::Debug $body
    }
}

// XXX XXX
macro_rules! pub_api_trait {
    ($name:ident, $body:tt) => {
        pub trait $name: core::fmt::Debug $body
    }
}

// XXX XXX
macro_rules! internal_generic_state_trait {
    // XXX QUICK HACKY MACRO API WITH SEPARATE NAME & GENERIC TYPE PARAMETERS
    ($name:ident, $generic_type_parameter:ident, $body:tt) => {
        pub(crate) trait $name<$generic_type_parameter> $body
    }
}
