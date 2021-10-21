use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Path;

mod input;

use input::EncodableDocumentInput;

pub fn encodable_document_derive(input: syn::DeriveInput) -> TokenStream {
    use darling::FromDeriveInput;

    let parsed = EncodableDocumentInput::from_derive_input(&input);
    if let Err(e) = parsed {
        return e.write_errors();
    }
    let input = parsed.unwrap();

    let analyzed = analyze(input);
    if let Err(e) = analyzed {
        return e;
    }
    let document = analyzed.unwrap();

    codegen(document)
}

struct EncodableDocument {
    ident: Ident,
    name: String,
    fields: Vec<EncodableField>,
    generics: syn::Generics,
}

struct EncodableField {
    ident: Ident,
    name: String,
}

fn analyze(input: EncodableDocumentInput) -> Result<EncodableDocument, TokenStream> {
    let mut fields = Vec::new();
    for field in input.data.take_struct().unwrap().fields {
        let ident = field.ident.unwrap();
        fields.push(EncodableField {
            name: ident.to_string(),
            ident,
        });
    }

    Ok(EncodableDocument {
        name: input.ident.to_string(),
        ident: input.ident,
        generics: input.generics,
        fields,
    })
}

fn codegen(document: EncodableDocument) -> TokenStream {
    use syn::{LitInt, LitStr};

    let ident = &document.ident;
    let name = LitStr::new(&document.name, document.ident.span());
    let (impl_generics, ty_generics, where_clause) = document.generics.split_for_impl();
    let num_fields = LitInt::new(&document.fields.len().to_string(), Span::call_site());

    let fields = document.fields.iter().map(|f| &f.ident);
    let field_strings = document
        .fields
        .iter()
        .map(|f| syn::LitStr::new(&f.name, f.ident.span()));

    quote! {
        impl #impl_generics ::ingle::EncodableDocument for #ident #ty_generics #where_clause {
            fn encode(&self) -> Result<
                ::ingle::values::DocumentValues,
                ::ingle::values::EncodingError
            > {
                let mut encoder = ::ingle::values::encode::document::encode_struct(#name, #num_fields)?;
                #(
                    encoder.encode_entry(#field_strings.to_string(), &self.#fields)?;
                )*
                encoder.end()
            }
        }

        impl #impl_generics ::ingle::EncodableValue for #ident #ty_generics #where_clause {
            fn encode(&self, encoder: ::ingle::values::encode::ValueEncoder) -> Result<
                ::ingle::values::Value,
                ::ingle::values::EncodingError
            > {
                <Self as ::ingle::EncodableDocument>::encode(self).map(|doc| doc.into_value())
            }
        }
    }
}
