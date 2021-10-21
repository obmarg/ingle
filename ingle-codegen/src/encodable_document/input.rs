use darling::{FromDeriveInput, FromField};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(ingle), supports(struct_named))]
pub struct EncodableDocumentInput {
    pub(super) ident: syn::Ident,
    pub(super) generics: syn::Generics,
    pub(super) data: darling::ast::Data<(), EncodableDocumentField>,
}

#[derive(FromField, Debug)]
#[darling(attributes(ingle))]
pub struct EncodableDocumentField {
    pub(super) ident: Option<proc_macro2::Ident>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_derive() {
        let parsed = EncodableDocumentInput::from_derive_input(&syn::parse_quote! {
            struct MyDocument {
                field: i8
            }
        })
        .unwrap();

        insta::assert_debug_snapshot!(parsed, @r###"
        EncodableDocumentInput {
            ident: Ident(
                MyDocument,
            ),
            generics: Generics {
                lt_token: None,
                params: [],
                gt_token: None,
                where_clause: None,
            },
            data: Struct(
                Fields {
                    style: Struct,
                    fields: [
                        EncodableDocumentField {
                            ident: Some(
                                Ident(
                                    field,
                                ),
                            ),
                        },
                    ],
                    span: Some(
                        Span,
                    ),
                    __nonexhaustive: (),
                },
            ),
        }
        "###);
    }
}
