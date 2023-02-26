use clippy_utils::{diagnostics::span_lint_and_sugg, ty::implements_trait};
use rustc_errors::Applicability;
use rustc_hir::{intravisit::FnKind, Body, FnDecl, FnRetTy, HirId, TraitItem, TraitItemKind};
use rustc_hir_analysis::hir_ty_to_ty;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;

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
    pub UNNECESSARY_BOX_RETURNS,
    nursery,
    "Needlessly returning a Box"
}
declare_lint_pass!(UnnecessaryBoxReturns => [UNNECESSARY_BOX_RETURNS]);

fn check_fn_decl(cx: &LateContext<'_>, decl: &FnDecl<'_>) {
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
            UNNECESSARY_BOX_RETURNS,
            return_ty_hir.span,
            format!("boxed return of the sized type `{boxed_ty}`").as_str(),
            "try",
            boxed_ty.to_string(),
            // the return value and function callers also needs to be changed, so this can't be MachineApplicable
            Applicability::Unspecified,
        );
    }
}

impl LateLintPass<'_> for UnnecessaryBoxReturns {
    fn check_trait_item(&mut self, cx: &LateContext<'_>, item: &TraitItem<'_>) {
        let TraitItemKind::Fn(signature, _) = &item.kind else { return };
        check_fn_decl(cx, signature.decl);
    }

    fn check_fn(
        &mut self,
        cx: &LateContext<'_>,
        fn_kind: FnKind<'_>,
        decl: &FnDecl<'_>,
        _: &Body<'_>,
        _: Span,
        _: HirId,
    ) {
        // It's unclear what part of a closure you would span, so for now it's ignored.
        // Trait implementations should also not be linted.
        // If this is changed, please also make sure not to call `hir_ty_to_ty` below.
        let FnKind::ItemFn(..) = fn_kind else { return };

        check_fn_decl(cx, decl);
    }
}
