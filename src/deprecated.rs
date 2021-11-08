//! Module containing all deprecated API that will be removed in the next major version.

use crate::{
    Dynamic, Engine, EvalAltResult, ImmutableString, NativeCallContext, RhaiResult, Scope, AST,
};

#[cfg(feature = "no_std")]
use std::prelude::v1::*;

impl Engine {
    /// Evaluate a file, but throw away the result and only return error (if any).
    /// Useful for when you don't need the result, but still need to keep track of possible errors.
    ///
    /// Not available under `no_std` or `WASM`.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`run_file`][Engine::run_file] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `run_file` instead")]
    #[cfg(not(feature = "no_std"))]
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    #[inline(always)]
    pub fn consume_file(&self, path: std::path::PathBuf) -> Result<(), Box<EvalAltResult>> {
        self.run_file(path)
    }

    /// Evaluate a file with own scope, but throw away the result and only return error (if any).
    /// Useful for when you don't need the result, but still need to keep track of possible errors.
    ///
    /// Not available under `no_std` or `WASM`.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`run_file_with_scope`][Engine::run_file_with_scope] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `run_file_with_scope` instead")]
    #[cfg(not(feature = "no_std"))]
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    #[inline(always)]
    pub fn consume_file_with_scope(
        &self,
        scope: &mut Scope,
        path: std::path::PathBuf,
    ) -> Result<(), Box<EvalAltResult>> {
        self.run_file_with_scope(scope, path)
    }

    /// Evaluate a string, but throw away the result and only return error (if any).
    /// Useful for when you don't need the result, but still need to keep track of possible errors.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`run`][Engine::run] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `run` instead")]
    #[inline(always)]
    pub fn consume(&self, script: &str) -> Result<(), Box<EvalAltResult>> {
        self.run(script)
    }

    /// Evaluate a string with own scope, but throw away the result and only return error (if any).
    /// Useful for when you don't need the result, but still need to keep track of possible errors.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`run_with_scope`][Engine::run_with_scope] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `run_with_scope` instead")]
    #[inline(always)]
    pub fn consume_with_scope(
        &self,
        scope: &mut Scope,
        script: &str,
    ) -> Result<(), Box<EvalAltResult>> {
        self.run_with_scope(scope, script)
    }

    /// Evaluate an AST, but throw away the result and only return error (if any).
    /// Useful for when you don't need the result, but still need to keep track of possible errors.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`run_ast`][Engine::run_ast] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `run_ast` instead")]
    #[inline(always)]
    pub fn consume_ast(&self, ast: &AST) -> Result<(), Box<EvalAltResult>> {
        self.run_ast(ast)
    }

    /// Evaluate an [`AST`] with own scope, but throw away the result and only return error (if any).
    /// Useful for when you don't need the result, but still need to keep track of possible errors.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`run_ast_with_scope`][Engine::run_ast_with_scope] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `run_ast_with_scope` instead")]
    #[inline(always)]
    pub fn consume_ast_with_scope(
        &self,
        scope: &mut Scope,
        ast: &AST,
    ) -> Result<(), Box<EvalAltResult>> {
        self.run_ast_with_scope(scope, ast)
    }
}

impl Dynamic {
    /// Convert the [`Dynamic`] into a [`String`] and return it.
    /// If there are other references to the same string, a cloned copy is returned.
    /// Returns the name of the actual type if the cast fails.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`into_string`][Dynamic::into_string] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `into_string` instead")]
    #[inline(always)]
    pub fn as_string(self) -> Result<String, &'static str> {
        self.into_string()
    }

    /// Convert the [`Dynamic`] into an [`ImmutableString`] and return it.
    /// Returns the name of the actual type if the cast fails.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`into_immutable_string`][Dynamic::into_immutable_string] instead.
    ///
    /// This method will be removed in the next major version.
    #[deprecated(since = "1.1.0", note = "use `into_immutable_string` instead")]
    #[inline(always)]
    pub fn as_immutable_string(self) -> Result<ImmutableString, &'static str> {
        self.into_immutable_string()
    }
}

impl NativeCallContext<'_> {
    /// Call a function inside the call context.
    ///
    /// # WARNING
    ///
    /// All arguments may be _consumed_, meaning that they may be replaced by `()`.
    /// This is to avoid unnecessarily cloning the arguments.
    ///
    /// Do not use the arguments after this call. If they are needed afterwards,
    /// clone them _before_ calling this function.
    ///
    /// If `is_method` is [`true`], the first argument is assumed to be passed
    /// by reference and is not consumed.
    #[deprecated(since = "1.2.0", note = "use `call_fn_raw` instead")]
    #[inline(always)]
    pub fn call_fn_dynamic_raw(
        &self,
        fn_name: impl AsRef<str>,
        is_method_call: bool,
        args: &mut [&mut Dynamic],
    ) -> RhaiResult {
        self.call_fn_raw(fn_name.as_ref(), is_method_call, is_method_call, args)
    }
}

#[allow(useless_deprecated)]
#[deprecated(since = "1.2.0", note = "explicitly wrap `EvalAltResult` in `Err`")]
impl<T> From<EvalAltResult> for Result<T, Box<EvalAltResult>> {
    #[inline(always)]
    fn from(err: EvalAltResult) -> Self {
        Err(err.into())
    }
}