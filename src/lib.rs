//!
//! # Maybe-Async-Cfg Procedure Macro
//!
//! **Why bother writing similar code twice for blocking and async code?**
//!
//! [![Build Status](https://github.com/nvksv/maybe-async-cfg/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/nvksv/maybe-async-cfg/actions)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Latest Version](https://img.shields.io/crates/v/maybe-async-cfg.svg)](https://crates.io/crates/maybe-async-cfg)
//! [![maybe-async](https://docs.rs/maybe-async-cfg/badge.svg)](https://docs.rs/maybe-async-cfg)
//!
//! When implementing both sync and async versions of API in a crate, most API of the two version
//! are almost the same except for some async/await keyword.
//!
//! `maybe-async-cfg` help unifying async and sync implementation by **procedural macro**.
//! - Write async code with normal `async`, `await`, and let `maybe_async_cfg` handles those `async`
//! and `await` when you need a blocking code.
//! - Add `maybe` attributes and specify feature-based conditions under which sync or async code 
//! should be generated.
//! - Use `only_if` (or `remove_if`) to keep code in specified version if necessary.
//!
//! The `maybe` procedural macro can be applied to the following codes:
//! - use declaration
//! - trait declaration
//! - trait implementation
//! - function definition
//! - struct and enum definition
//!
//! **RECOMMENDATION**: Enable **resolver ver2** in your crate, which is introduced in Rust 1.51. If
//! not, two crates in dependency with conflict version (one async and another blocking) can fail
//! complilation.
//!
//!
//! ## Motivation
//!
//! The async/await language feature alters the async world of rust. Comparing with the map/and_then
//! style, now the async code really resembles sync version code.
//!
//! In many crates, the async and sync version of crates shares the same API, but the minor
//! difference that all async code must be awaited prevent the unification of async and sync code.
//! In other words, we are forced to write an async and an sync implementation repectively.
//!
//!
//! ## Macros in Detail
//!
//! To use `maybe-async-cfg`, we must know which block of codes is only used on sync implementation,
//! and which on async. These two versions of the implementation should share the same function
//! signatures except for async/await keywords.
//!
//! Use `maybe` macro for code that is the same in both async and sync versions except for
//! async/await keywords. Specify in the macro parameters the conditions (based on features) under
//! which async and/or sync versions of the code should appear.
//!
//! - attribute macro **`maybe`**
//!
//!     Offers a unified way to provide sync and async conversion on demand depending on features,
//! enabled for your crate, with **async first** policy.
//!
//!     ```toml
//!     [dependencies]
//!     maybe_async_cfg = "0.2"
//!
//!     [features]
//!     use_sync = []
//!     use_async = []
//!     ```
//!
//!     In this and all the following examples, we use two features. But you can use any conditions
//! that are convenient for you, for example, replacing `feature="use_sync"` with
//! `not(feature="use_async")` everywhere. Feel free, `maybe-async-cfg` does not analyze the
//! conditions in any way, just substituting them as is.
//!
//!     Add the `maybe` attribute before all the items that need to be changed in different versions
//! of the code (sync or async).
//!
//!     Want to keep async code? Specify the `async` parameter with the condition (based on
//! features) when your code should be async.
//!
//!     Wanna convert async code to sync? Specify the `sync` parameter with the condition when the
//! sync code should be generated.
//!
//!     ```rust, no_run
//!     #[maybe_async_cfg::maybe(
//!         idents(Foo),
//!         sync(feature="use_sync"),
//!         async(feature="use_async")
//!     )]
//!     struct Struct {
//!         f: Foo,
//!     }
//!
//!     ```
//!     After convertation:
//!     ```rust, no_run
//!     #[cfg(feature="use_sync")]
//!     struct StructSync {
//!         f: FooSync,
//!     }
//!     #[cfg(feature="use_async")]
//!     struct StructAsync {
//!         f: FooAsync,
//!     }
//!     ```
//!
//! - procedural macro **`content`**
//!
//!     The `content` macro allows you to specify common parameters for many `maybe` macros. Use the
//! internal `default` attribute with the required parameters inside the `content` macro.
//!
//!     ```rust, no_run
//!     maybe_async_cfg::content!{
//!     #![maybe_async_cfg::default(
//!         idents(Foo, Bar),
//!     )]
//!
//!     #[maybe_async_cfg::maybe(
//!         sync(feature="use_sync"), 
//!         async(feature="use_async")
//!     )]
//!     struct Struct {
//!         f: Foo,
//!     }
//!
//!     #[maybe_async_cfg::maybe(
//!         sync(feature="use_sync"), 
//!         async(feature="use_async")
//!     )]
//!     async fn func(b: Bar) {
//!         todo!()
//!     }
//!     } // content!
//!     ```
//!     After convertation:
//!     ```rust, no_run
//!     #[cfg(feature="use_sync")]
//!     struct StructSync {
//!         f: FooSync,
//!     }
//!     #[cfg(feature="use_async")]
//!     struct StructAsync {
//!         f: FooAsync,
//!     }
//!
//!     #[cfg(feature="use_sync")]
//!     fn func_sync(b: BarSync) {
//!         todo!()
//!     }
//!     #[cfg(feature="use_async")]
//!     async fn func_async(b: BarAsync) {
//!         todo!()
//!     }
//!     ```
//!     
//!
//! ## Examples
//!
//! ### rust client for services
//!
//! When implementing rust client for any services, like awz3. The higher level API of async and
//! sync version is almost the same, such as creating or deleting a bucket, retrieving an object and
//! etc.
//!
//! The example `service_client` is a proof of concept that `maybe_async_cfg` can actually free us
//! from writing almost the same code for sync and async. We can toggle between a sync AWZ3 client
//! and async one by `is_sync` feature gate when we add `maybe-async-cfg` to dependency.
//!
//! 
//! ## Аppreciations
//! 
//! This crate is a redesigned fork of these wonderful crates:
//! 
//! - [fMeow/maybe-async-rs](https://github.com/fMeow/maybe-async-rs)
//! 
//! - [marioortizmanero/maybe-async-rs](https://github.com/marioortizmanero/maybe-async-rs)
//! 
//! Thanks!
//!
//! 
//! # License
//! MIT

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod macros;
mod params;
mod utils;
mod visit_ext;
mod visitor_async;
mod visitor_content;

mod doctests;

mod debug;

const DEFAULT_CRATE_NAME: &'static str = "maybe_async_cfg";
const MACRO_MAYBE_NAME: &'static str = "maybe";
const MACRO_ONLY_IF_NAME: &'static str = "only_if";
const MACRO_REMOVE_IF_NAME: &'static str = "remove_if";
const MACRO_NOOP_NAME: &'static str = "noop";
const MACRO_REMOVE_NAME: &'static str = "remove";
const MACRO_DEFAULT_NAME: &'static str = "default";

const STANDARD_MACROS: &'static [&'static str] = &[
    "dbg",
    "print",
    "println",
    "assert",
    "assert_eq",
    "assert_ne",
];

/// Marks the code that can be presented in several versions. 
/// 
/// ### The `maybe` macro has the following parameters:
///
/// - `disable` 
/// 
///     The macro with `disable` parameter will do nothing, like `noop`. Use it to write and debug 
/// initial async code.
///
/// - `prefix` 
/// 
///     The name of `maybe-async-cfg` crate. If not set, `"maybe_async_cfg"` will be used.
///
/// - `sync`, `async` 
/// 
///     Defines versions of the code: the item to which the attribute `maybe` refers will be 
/// replaced with multiple copies (one for each version), which will be modified according to 
/// the version kind and its parameters.
///
///     For the `sync` version, the item will be converted from async to sync code by deleting
/// the `async` and `await` keywords. The types `Future<Output=XXX>` will also be replaced with just
/// `XXX`. For the  `async` version, the item will be left async.
///
///     In any case, the item will be converted according to all the parameters described below. For
/// functions, structs/enums and traits, the name will be changed as if it is mentioned in the
/// `idents` list (if it is not explicitly specified there and if `keep_self` is not present).
///
/// - All other parameters will be passed to all versions (with merging).
///
///     Therefore, those parts of the version parameters that match in all versions can be specified
/// here. For example, this is the expected behavior for the `idents` list.
///
/// ### Every version has the following parameters:
///
/// - `key`
///
///     Defines unique name of the version to use it in `only_if`/`remove_if` conditions. If 
/// omitted, `sync`/`async` will be used.
///
///     ```rust, no_run
///     #[maybe_async_cfg::maybe(
///         sync(key="foo", feature="use_sync"),
///         async(key="bar", feature="use_async"),
///     )]
///     struct Struct {
///         f: usize,
///         
///         // This field will only be present in sync version
///         #[maybe_async_cfg::only_if(key="foo")]
///         sync_only_field: bool,
///     }
///     ```
///     After convertation:
///     ```rust, no_run 
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: usize,
///         sync_only_field: bool,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: usize,
///     }
///     ```
/// 
/// - `cfg`
/// 
///     Defines the condition (based on features), under which the current version should appear.
/// 
///     Note: conditions like `feature = "..."`, `not(...)`, `all(...)`, `any(...)` will be 
/// processed correctly, even if the `cfg(...)` was omitted.
///
///     ```rust, no_run
///     #[maybe_async_cfg::maybe(
///         sync(cfg(feature="use_sync")),
///         async(feature="use_async")
///     )]
///     struct Struct {
///         f: Foo,
///     }
///
///     ```
///     After convertation:
///     ```rust, no_run
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: Foo,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: Foo,
///     }
///     ```
///  
/// - `idents` 
/// 
///     Defines a list of identifiers that should be renamed depending on the version of the code.
///
///     Each identifier can have the following clarifying parameters:
///
///     - `fn`
///
///         means that this is the name of the function and it should be converted by adding the
/// suffixes `"_sync"`/`"_async"` (otherwise, the suffixes `"Sync"`/`"Async"` will be used).
///
///     - `use`  
/// 
///         in `use` lists, using this identifier will result in renaming via the `as` expression, 
/// rather than a simple replacement as is. In other cases, a simple replacement will be used.
///
///     - `keep`
///
///         this identifier will not be converted anywhere
///
///     - `sync`, `async`
///
///         specifies the name that will be used in the corresponding version of the code. Overrides
/// the standard scheme of suffixes used by default. If the parameter value is omitted, 
/// the identifier will not be renamed in this case.
///
///     ```rust, no_run
///     #[maybe_async_cfg::maybe(
///         idents(
///             Foo,
///             Bar,
///             baz(fn),
///             Qux(use),
///             waldo(sync, async="async_waldo"),
///             xyzzy(fn, use, sync="xizzy_the_sync_func"),
///         ),
///         sync(feature="use_sync"),
///         async(feature="use_async"),
///     )]
///     async fn func() {
///         struct Foo {}
///         use extcrate::{
///             Bar,
///             baz,
///             Qux,
///             waldo::{
///                 plugh,
///                 xyzzy
///             }
///         };
///         let _ = baz( Foo {}, Bar::new() ).await;
///         let _ = xizzy( Qux::flob(b).await );
///     }
///     ```
///     After convertation:
///     ```rust, no_run
///     #[cfg(feature="use_sync")]
///     fn func_sync() {
///         struct FooSync {}
///         use extcrate::{
///             BarSync,
///             baz_sync,
///             Qux as QuxSync,
///             waldo::{
///                 plugh,
///                 xyzzy as xizzy_the_sync_func
///             }
///         };         
///         let _ = baz_sync( FooSync {}, BarSync::new() );
///         let _ = xizzy_the_sync_func( QuxSync::flob() );
///     }
///     #[cfg(feature="use_async")]
///     async fn func_async() {
///         struct FooAsync {}
///         use extcrate::{
///             BarAsync,
///             baz_async,
///             Qux as QuxAsync,
///             async_waldo::{
///                 plugh,
///                 xyzzy as xyzzy_async
///             }
///         };
///         let _ = baz_async( FooAsync {}, BarAsync::new() ).await;
///         let _ = xyzzy_async( QuxAsync::flob().await );     
///     }
///     ```
///
/// - `keep_self`
///
///     Do not change name of item to which attribute `maybe` refers.
///
/// - `self`
/// 
///     Defines the name that will be assigned to the item in this variant.
/// 
/// - `send`
///
///     If `send = "Send"` or `send = "true"` is present, the attribute
/// `#[async_trait::async_trait]` will be added before the async code. If `send = "?Send"` or
/// `send = "false"` then `#[async_trait::async_trait(?Send)]` will be added.  
/// 
/// - `drop_attrs`
///
///     Remove any attributes with specified names.
///
///     ```rust, no_run
///     #[maybe_async_cfg::maybe(
///         sync(feature="use_sync", drop_attrs(attr)),
///         async(feature="use_async"),
///     )]
///     struct Struct {
///         f: usize,
///
///         // This attribute will be removed in sync version
///         #[attr(param)]
///         field1: bool,
///     }
///     ```
///     After convertation:
///     ```rust, no_run
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: usize,
///         field1: bool,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: usize,
///         #[attr(param)]
///         field1: bool,
///     }
///     ```
///
/// - `replace_features`
///
///     Replace one feature name with another.
///
///     ```rust, no_run
///     #[maybe_async_cfg::maybe(
///         sync(feature="use_sync", replace_features(secure(secure_sync))),
///         async(feature="use_async"),
///     )]
///     struct Struct {
///         f: usize,
///         // In sync version "secure" feature will be replaced with "secure_sync" feature
///         #[cfg(feature="secure")]
///         field: bool,
///     }
///     ```
///     After convertation:
///     ```rust, no_run
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: usize,
///         #[cfg(feature="secure_sync")]
///         field: bool,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: usize,
///         #[cfg(feature="secure")]
///         field: bool,
///     }
///     ```
///
/// - `inner`, `outer`
///
///     Adds some attributes to the generated code. Inner attributes will appear below attribute 
/// `#[cfg(...)]`, outer attributes will appear above it.
/// 
///     Note: if the version parameter is not parsed as a parameter of some other type, it will be 
/// interpreted as an inner attribute.
/// 
///     Useful for testing: just write `test` in version parameters.
///
///     ```rust, no_run
///     #[maybe_async_cfg::maybe(
///         sync(feature="secure_sync", test, "resource(path = \"/foo/bar\")", outer(xizzy)),
///         async(feature="secure_sync", inner(baz(qux), async_attributes::test)),
///     )]
///     async fn test_func() {
///         todo!()
///     }
///     ```
///     After convertation:
///     ```rust, no_run
///     #[xizzy]
///     #[cfg(feature="use_sync")]
///     #[test]
///     #[resource(path = "/foo/bar")]
///     fn test_func_sync() {
///         todo!()
///     }
///     #[cfg(feature="use_async")]
///     #[baz(qux)]
///     #[async_attributes::test]
///     async fn test_func_async() {
///         todo!()
///     }
///     ```
/// 
/// - In other cases, the following rules apply:
///     
///     - name-value pairs (`xxx = "yyy"`) with a name other than `key`, `prefix`, `send` and
/// `feature` will produce an error.
///     
///     - `feature = "..."`, `not(...)`, `all(...)`, `any(...)` will be interpreted as condition for
/// current version (as wrapped in `cfg(...)`).
/// 
///     - all another parameters will be interpreted as inner attribute for current version (as 
/// wrapped in `inner(...)`).
/// 
#[proc_macro_error]
#[proc_macro_attribute]
pub fn maybe(args: TokenStream, input: TokenStream) -> TokenStream {
    macros::maybe(args, input)
}

/// Marks conditional content that should only be used in the specified version of the code.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn only_if(_: TokenStream, body: TokenStream) -> TokenStream {
    body
}

/// Marks conditional content that should be used in all versions of the code except the specified 
/// one.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn remove_if(_: TokenStream, body: TokenStream) -> TokenStream {
    body
}

/// Does nothing (leaves content intact).
#[proc_macro_error]
#[proc_macro_attribute]
pub fn noop(_: TokenStream, body: TokenStream) -> TokenStream {
    body
}

/// Removes marked content.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn remove(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// A wrapper for code with common `maybe` parameters
/// 
/// The `content` macro allows you to specify common parameters for many `maybe` macros. Use the
/// internal `default` attribute with the required parameters inside the `content` macro.
///
/// ```rust, no_run
/// maybe_async_cfg::content!{
/// #![maybe_async_cfg::default(
///     idents(Foo, Bar),
/// )]
///
/// #[maybe_async_cfg::maybe(sync(feature="use_sync"), async(feature="use_async"))]
/// struct Struct {
///     f: Foo,
/// }
///
/// #[maybe_async_cfg::maybe(sync(feature="use_sync"), async(feature="use_async"))]
/// async fn func(b: Bar) {
///     todo!()
/// }
/// } // content!
/// ```
/// After convertation:
/// ```rust, no_run
/// #[cfg(feature="use_sync")]
/// struct StructSync {
///     f: FooSync,
/// }
/// #[cfg(feature="use_async")]
/// struct StructAsync {
///     f: FooAsync,
/// }
///
/// #[cfg(feature="use_sync")]
/// fn func_sync(b: BarSync) {
///     todo!()
/// }
/// #[cfg(feature="use_async")]
/// async fn func_async(b: BarAsync) {
///     todo!()
/// }
/// ```
/// 
#[proc_macro_error]
#[proc_macro]
pub fn content(body: TokenStream) -> TokenStream {
    macros::content(body)
}
