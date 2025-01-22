use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Item {
    pub ident: Ident,
    pub owner_id: OwnerId,
    pub kind: ItemKind,
    pub span: Span,
    pub vis_span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "serde_tag")]
pub enum ItemKind {
    ExternCrate {
        symbol: Option<Symbol>,
    },
    Use {
        path: UsePath,
        use_kind: UseKind,
    },
    Static {
        ty: HirTy,
        r#const: bool,
        body: Body,
    },
    Const {
        ty: HirTy,
        generics: Generics,
        #[serde(skip)]
        body_id: rustc_hir::BodyId,
        body: Body,
    },
    Fn {
        sig: FnSig,
        generics: Generics,
        #[serde(skip)]
        body_id: rustc_hir::BodyId,
        body: Body,
    },
    //Macro(MacroDef, MacroKind),
    Mod {
        r#mod: Mod,
    },
    /* ForeignMod {
        abi: Abi,
        items: [ForeignItemRef],
    }, */
    //GlobalAsm(InlineAsm),
    TyAlias {
        ty: HirTy,
        generics: Generics,
    },
    //OpaqueTy(OpaqueTy),
    Enum {
        def: EnumDef,
        generics: Generics,
    },
    Struct {
        data: VariantData,
        generics: Generics,
    },
    Union {
        data: VariantData,
        generics: Generics,
    },
    Trait {
        field1: bool,
        field2: bool,
        generics: Generics,
        bounds: GenericBounds,
        refs: Vec<TraitItemRef>,
    },
    TraitAlias {
        generics: Generics,
        bounds: GenericBounds,
    },
    Impl {
        r#impl: Impl,
    },
    Macro {
        def: MacroDef,
        kind: MacroKind,
    },
    ForeignMod,
    GlobalAsm,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MacroDef {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Body {
    pub params: Vec<Param>,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Param {
    pub hir_id: HirId,
    pub pat: Pat,
    pub ty_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FnSig {
    pub header: FnHeader,
    pub decl: FnDecl,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FnHeader {
    pub safety: bool,
    pub constness: bool,
    pub asyncness: bool,
    //pub abi: Abi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Mod {
    pub spans: ModSpans,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModSpans {
    pub inner_span: Span,
    pub inject_use_span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EnumDef {
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Variant {
    pub ident: Ident,
    pub hir_id: HirId,
    pub def_id: LocalDefId,
    pub data: VariantData,
    pub disr_expr: Option<AnonConst>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum VariantData {
    Struct {
        fields: Vec<FieldDef>,
        recovered: bool,
    },
    Tuple(Vec<FieldDef>, HirId, LocalDefId),
    Unit(HirId, LocalDefId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FieldDef {
    pub span: Span,
    pub vis_span: Span,
    pub ident: Ident,
    pub hir_id: HirId,
    pub def_id: LocalDefId,
    pub ty: HirTy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraitItemRef {
    pub id: TraitItemId,
    pub ident: Ident,
    pub kind: AssocItemKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraitItemId {
    pub owner_id: OwnerId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Impl {
    pub constness: bool,
    pub safety: bool,
    pub polarity: ImplPolarity,
    pub defaultness: Defaultness,
    pub defaultness_span: Option<Span>,
    pub generics: Generics,
    pub of_trait: Option<TraitRef>,
    pub self_ty: HirTy,
    pub items: Vec<ImplItemRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ImplPolarity {
    Positive,
    Negative(Span),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImplItemRef {
    pub id: ImplItemId,
    pub ident: Ident,
    pub kind: AssocItemKind,
    pub span: Span,
    pub trait_item_def_id: Option<DefId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "serde_tag")]
pub enum HirTyKind {
    InferDelegation {
        def_id: DefId,
        kind: InferDelegationKind,
    },
    Slice {
        ty: HirTy,
    },
    Array {
        ty: HirTy,
        len: ArrayLen,
    },
    Ptr {
        ty: MutHirTy,
    },
    Ref {
        lifetime: Lifetime,
        ty: MutHirTy,
    },
    BareFn {
        ty: BareFnHirTy,
    },
    Never,
    Tup {
        tys: Vec<HirTy>,
    },
    AnonAdt {
        item: Item,
    },
    Path {
        path: QPath,
    },
    // OpaqueDef(Item, Vec<GenericArg>, bool),
    TraitObject {
        refs: Vec<PolyTraitRef>,
        lifetime: Lifetime,
        syntax: TraitObjectSyntax,
    },
    Typeof {
        r#const: AnonConst,
    },
    Infer,
    Err,
    Pat {
        ty: HirTy,
        pat: Pat,
    },
    OpaqueDef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum AssocItemKind {
    Const,
    Fn { has_self: bool },
    Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TraitObjectSyntax {
    Dyn,
    DynStar,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum InferDelegationKind {
    Input(usize),
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ArrayLen {
    Infer(InferArg),
    Body(ConstArg),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MutHirTy {
    pub ty: HirTy,
    pub mutbl: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BareFnHirTy {
    pub safety: bool,
    //pub abi: Abi,
    pub generic_params: Vec<GenericParam>,
    pub decl: FnDecl,
    pub param_names: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FnDecl {
    pub inputs: Vec<HirTy>,
    pub output: FnRetTy,
    pub c_variadic: bool,
    pub implicit_self: ImplicitSelfKind,
    pub lifetime_elision_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum FnRetTy {
    DefaultReturn(Span),
    Return(HirTy),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ImplicitSelfKind {
    Imm,
    Mut,
    RefImm,
    RefMut,
    None,
}

pub type UsePath = Path<Vec<Res>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum UseKind {
    Single,
    Glob,
    ListStem,
}
