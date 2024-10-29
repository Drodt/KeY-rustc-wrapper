use paste::paste;

use super::*;

macro_rules! create_visitor_traits {
    ($($name:ident : $ty:ty), *) => {
        /// Syntax tree traversal to walk a shared borrow of a [Term] syntax tree.
        pub trait Visit<'ast> {
            $(
                paste!{
                    fn [< visit_ $name >] (&mut self, t: &'ast $ty) {
                        [< visit_ $name >](self, t)
                    }
                }
            )*
        }

        $(
            impl Visitable for $ty {
                fn visit<'ast, V: Visit<'ast>>(&'ast self, v: &mut V) {
                    paste!{
                        v.[< visit_ $name >](self)
                    }
                }
            }
        )*
    };
}

/// Conveniance trait for traversal of a shared borrow of a syntax tree.
pub trait Visitable {
    fn visit<'ast, V: Visit<'ast>>(&'ast self, v: &mut V);
}

// Only necessary fns for collecting types rn.
create_visitor_traits! {
  mod: Mod,
  item: Item,
  item_kind: ItemKind,
  body: Body,
  impl: Impl,
  expr: Expr
}

fn visit_mod<'a, V: Visit<'a> + ?Sized>(v: &mut V, x: &'a Mod) {
    for i in &x.items {
        v.visit_item(i);
    }
}

fn visit_item<'a, V: Visit<'a> + ?Sized>(v: &mut V, x: &'a Item) {
    v.visit_item_kind(&x.kind);
}

fn visit_item_kind<'a, V: Visit<'a> + ?Sized>(v: &mut V, x: &'a ItemKind) {
    match x {
        ItemKind::ExternCrate(_) => {}
        ItemKind::Use(..) => {}
        ItemKind::Static(_, _, body) => {
            v.visit_body(body);
        }
        ItemKind::Const(_, _, body) => {
            v.visit_body(body);
        }
        ItemKind::Fn(_, _, body) => {
            v.visit_body(body);
        }
        ItemKind::Mod(m) => v.visit_mod(m),
        ItemKind::TyAlias(..) => {}
        ItemKind::Enum(..) => {}
        ItemKind::Struct(..) => {}
        ItemKind::Union(..) => {}
        ItemKind::Trait(..) => {}
        ItemKind::TraitAlias(..) => {}
        ItemKind::Impl(i) => v.visit_impl(i),
    }
}

fn visit_body<'a, V: Visit<'a> + ?Sized>(v: &mut V, x: &'a Body) {
    v.visit_expr(&x.value);
}

fn visit_impl<'a, V: Visit<'a> + ?Sized>(_v: &mut V, _x: &'a Impl) {}

fn visit_expr<'a, V: Visit<'a> + ?Sized>(_v: &mut V, _x: &'a Expr) {
    // TODO: Get to any bodies, e.g. in closures
}
