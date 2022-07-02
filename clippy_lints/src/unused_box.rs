use clippy_utils::{diagnostics::span_lint_and_sugg, ty::implements_trait};
use rustc_errors::Applicability;
use rustc_hir::{intravisit::FnKind, Body, FnDecl, FnRetTy, HirId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;
use rustc_typeck::hir_ty_to_ty;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks for a return type containing a `Box<T>` where `T` implements `Sized`
    ///
    /// ### Why is this bad?
    ///
    /// It's better to just return `T` in these cases. The caller may not need
    /// the value to be boxed, and it's expensive to free the memory once the
    /// `Box<T>` been dropped.
    ///
    /// ### Example
    /// ```rust
    /// fn foo() -> Box<String> {
    ///     Box::new(String::from("Hello, world!"))
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// fn foo() -> String {
    ///     String::from("Hello, world!")
    /// }
    /// ```
    #[clippy::version = "1.64.0"]
    pub UNUSED_BOX,
    nursery,
    "Needlessly returning a Box"
}
declare_lint_pass!(UnusedBox => [UNUSED_BOX]);

impl LateLintPass<'_> for UnusedBox {
    fn check_fn(
        &mut self,
        cx: &LateContext<'_>,
        fn_kind: FnKind<'_>,
        decl: &FnDecl<'_>,
        _: &Body<'_>,
        _: Span,
        _: HirId,
    ) {
        // it's unclear what part of a closure you would span, so for now it's ignored
        // if this is changed, please also make sure not to call `hir_ty_to_ty` below
        if matches!(fn_kind, FnKind::Closure) {
            return;
        }

        let FnRetTy::Return(return_ty_hir) = &decl.output else { return };

        // this is safe, since we're not in a body
        let return_ty = hir_ty_to_ty(cx.tcx, return_ty_hir);

        if !return_ty.is_box() {
            return;
        }

        let boxed_ty = return_ty.boxed_ty();
        let Some(sized_trait) = cx.tcx.lang_items().sized_trait() else { return };

        // it's sometimes useful to return Box<T> if T is unsized, so don't lint those
        if implements_trait(cx, boxed_ty, sized_trait, &[]) {
            span_lint_and_sugg(
                cx,
                UNUSED_BOX,
                return_ty_hir.span,
                format!("function returns `Box<{0}>` when `{0}` implements `Sized`", boxed_ty).as_str(),
                "change the return type to",
                boxed_ty.to_string(),
                // the return value also needs to be changed, so this can't be MachineApplicable
                Applicability::HasPlaceholders,
            );
        }
    }
}
