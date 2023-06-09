// Adapted from
// https://github.com/pksunkara/cargo-up/blob/1d085f19020c41e02d067247ff267f2483f825b5/cargo-up/rust-visitor/src/lib.rs
// We need to keep it here because cargo-up follows an outdated version of r-a.

use paste::paste;
use ra_ap_hir::{db::HirDatabase, PathResolution, Semantics};
use ra_ap_ide::SourceRoot;
use ra_ap_syntax::{
    ast::{self, AstNode},
    SyntaxKind, SyntaxNode,
};

// TODO: this is a terrible name.
pub(crate) struct SymbolVisitor<'db_, 'db, DB> {
    found: Vec<PathResolution>,
    semantics: &'db_ Semantics<'db, DB>,
}

impl<'db_, 'db, DB> SymbolVisitor<'db_, 'db, DB> {
    fn new(semantics: &'db_ Semantics<'db, DB>) -> Self {
        SymbolVisitor {
            found: Vec::new(),
            semantics,
        }
    }

    pub(crate) fn visit_crate(
        root: &SourceRoot,
        semantics: &Semantics<'db, DB>,
    ) -> Vec<PathResolution>
    where
        DB: HirDatabase,
    {
        let mut visitor = SymbolVisitor::new(semantics);

        root.iter().for_each(|file| {
            let source = semantics.parse(file);
            visitor.walk(source.syntax());
        });

        visitor.found
    }
}

impl<'db, 'db_, DB> Visitor for SymbolVisitor<'db, 'db_, DB>
where
    DB: HirDatabase,
{
    fn visit_path(&mut self, node: &ast::Path) {
        let resolution = match self.semantics.resolve_path(node) {
            Some(resolution) => resolution,
            // TODO: Do we want to emit a warning at some point ?
            None => return,
        };

        self.found.push(resolution);
    }
}

pub(crate) const INTERNAL_ERR: &'static str = "Failed to rowan cast";

pub use ra_ap_syntax;

macro_rules! visiting {
    () => {};
    ($($kind:ident,)*) => {
        trait Visitable: Sized {
            fn accept<T: Visitor>(&self, visitor: &mut T);
        }

        impl Visitable for SyntaxNode {
            fn accept<T: Visitor>(&self, visitor: &mut T) {
                visitor.pre_visit(self);

                paste! {
                    match self.kind() {
                        $(SyntaxKind::$kind => visitor.[<visit_ $kind:lower>](
                            &ast::[<$kind:camel>]::cast((*self).clone()).expect(INTERNAL_ERR),
                        ),)*
                        _ => panic!("Found weird stuff"),
                    };
                };

                for child in self.children() {
                    child.accept(visitor);
                }

                visitor.post_visit(self);
            }
        }

        pub trait Visitor: Sized {
            /// Call this method to perform a in-order traversal on `node` and its children.
            fn walk(&mut self, node: &SyntaxNode) {
                node.accept(self);
            }

            /// This method is called before visiting a node.
            fn pre_visit(&mut self, _node: &SyntaxNode) {
            }

            /// This method is called after visiting a node.
            fn post_visit(&mut self, _node: &SyntaxNode) {
            }

            paste! {
                $(
                    #[doc = " This method is called when visiting a `" $kind "` node."]
                    fn [<visit_ $kind:lower>](&mut self, _node: &ast::[<$kind:camel>]) {}
                )*
            }
        }
    };
}

visiting!(
    SOURCE_FILE,
    STRUCT,
    UNION,
    ENUM,
    FN,
    RET_TYPE,
    EXTERN_CRATE,
    MODULE,
    USE,
    STATIC,
    CONST,
    TRAIT,
    IMPL,
    TYPE_ALIAS,
    MACRO_CALL,
    MACRO_RULES,
    TOKEN_TREE,
    MACRO_DEF,
    PAREN_TYPE,
    TUPLE_TYPE,
    MACRO_TYPE,
    NEVER_TYPE,
    PATH_TYPE,
    PTR_TYPE,
    ARRAY_TYPE,
    SLICE_TYPE,
    REF_TYPE,
    INFER_TYPE,
    FN_PTR_TYPE,
    FOR_TYPE,
    IMPL_TRAIT_TYPE,
    DYN_TRAIT_TYPE,
    OR_PAT,
    PAREN_PAT,
    REF_PAT,
    BOX_PAT,
    IDENT_PAT,
    WILDCARD_PAT,
    REST_PAT,
    PATH_PAT,
    RECORD_PAT,
    RECORD_PAT_FIELD_LIST,
    RECORD_PAT_FIELD,
    TUPLE_STRUCT_PAT,
    TUPLE_PAT,
    SLICE_PAT,
    RANGE_PAT,
    LITERAL_PAT,
    MACRO_PAT,
    CONST_BLOCK_PAT,
    TUPLE_EXPR,
    ARRAY_EXPR,
    PAREN_EXPR,
    PATH_EXPR,
    CLOSURE_EXPR,
    IF_EXPR,
    WHILE_EXPR,
    LOOP_EXPR,
    FOR_EXPR,
    CONTINUE_EXPR,
    BREAK_EXPR,
    LABEL,
    BLOCK_EXPR,
    STMT_LIST,
    RETURN_EXPR,
    YIELD_EXPR,
    YEET_EXPR,
    LET_EXPR,
    UNDERSCORE_EXPR,
    MACRO_EXPR,
    MATCH_EXPR,
    MATCH_ARM_LIST,
    MATCH_ARM,
    MATCH_GUARD,
    RECORD_EXPR,
    RECORD_EXPR_FIELD_LIST,
    RECORD_EXPR_FIELD,
    BOX_EXPR,
    CALL_EXPR,
    INDEX_EXPR,
    METHOD_CALL_EXPR,
    FIELD_EXPR,
    AWAIT_EXPR,
    TRY_EXPR,
    CAST_EXPR,
    REF_EXPR,
    PREFIX_EXPR,
    RANGE_EXPR,
    BIN_EXPR,
    EXTERN_BLOCK,
    EXTERN_ITEM_LIST,
    VARIANT,
    RECORD_FIELD_LIST,
    RECORD_FIELD,
    TUPLE_FIELD_LIST,
    TUPLE_FIELD,
    VARIANT_LIST,
    ITEM_LIST,
    ASSOC_ITEM_LIST,
    ATTR,
    META,
    USE_TREE,
    USE_TREE_LIST,
    PATH,
    PATH_SEGMENT,
    LITERAL,
    RENAME,
    VISIBILITY,
    WHERE_CLAUSE,
    WHERE_PRED,
    ABI,
    NAME,
    NAME_REF,
    LET_STMT,
    LET_ELSE,
    EXPR_STMT,
    GENERIC_PARAM_LIST,
    GENERIC_PARAM,
    LIFETIME_PARAM,
    TYPE_PARAM,
    CONST_PARAM,
    GENERIC_ARG_LIST,
    LIFETIME,
    LIFETIME_ARG,
    TYPE_ARG,
    ASSOC_TYPE_ARG,
    CONST_ARG,
    PARAM_LIST,
    PARAM,
    SELF_PARAM,
    ARG_LIST,
    TYPE_BOUND,
    TYPE_BOUND_LIST,
    MACRO_ITEMS,
    MACRO_STMTS,
);
