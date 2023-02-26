use clippy_utils::{diagnostics::span_lint_and_sugg, ty::implements_trait};
use rustc_errors::Applicability;
use rustc_hir::{FnDecl, FnRetTy, ImplItemKind, Item, ItemKind, Node, TraitItem, TraitItemKind};
use rustc_hir_analysis::hir_ty_to_ty;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

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
    pedantic,
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

    fn check_impl_item(&mut self, cx: &LateContext<'_>, item: &rustc_hir::ImplItem<'_>) {
        // Ignore implementations of traits, because the lint should be on the
        // trait, not on the implmentation of it.
        let Node::Item(parent) = cx.tcx.hir().get_parent(item.hir_id()) else { return };
        let ItemKind::Impl(parent) = parent.kind else { return };
        if parent.of_trait.is_some() {
            return;
        }

        let ImplItemKind::Fn(signature, ..) = &item.kind else { return };
        check_fn_decl(cx, signature.decl);
    }

    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        let ItemKind::Fn(signature, ..) = &item.kind else { return };
        check_fn_decl(cx, signature.decl);
    }
}
