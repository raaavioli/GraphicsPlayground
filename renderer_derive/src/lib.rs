#![recursion_limit = "128"]

extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn pointerss_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syntax_tree = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_vertex_attrib_pointers(&syntax_tree)
}

fn impl_vertex_attrib_pointers(syntax_tree: &syn::DeriveInput) -> proc_macro::TokenStream {
    let identifier = &syntax_tree.ident;
    let generics = &syntax_tree.generics;
    let where_clause = &syntax_tree.generics.where_clause;
    let vertex_attrib_calls = build_vertex_attrib_pointer_calls(&syntax_tree.data);

    let stream = quote! {
        impl #identifier #generics #where_clause {
            fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = size_of::<#identifier>();
                let mut offset = 0;

                #(#vertex_attrib_calls)*
            }
        }
    };
    stream.into()
}

fn build_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match data {
        syn::Data::Union(_) => panic!("VertexAttribPointers can not be implemented for unions"),
        syn::Data::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        syn::Data::Struct(data_struct) => data_struct
            .fields
            .iter()
            .map(build_vertex_attrib_pointer_call)
            .collect(),
    }
}

fn build_vertex_attrib_pointer_call(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };
    let location_attrib = field
        .attrs
        .iter()
        .find(|a| a.path.segments.iter().any(|ps| ps.ident == "location"))
        .unwrap_or_else(|| panic!("{} is missing #[location = ?] attribute", field_name));

    let location_value = (location_attrib.tokens.clone().into_iter())
        .find_map(|t| match t {
            proc_macro2::TokenTree::Literal(literal) => {
                Some(literal.to_string().parse::<usize>().unwrap())
            }
            _ => None,
        })
        .unwrap();

    let field_type = &field.ty;
    quote! {
        let location = #location_value;
        unsafe {
            #field_type::vertex_attrib_pointer(gl, location, stride, offset);
        }
        offset += ::std::mem::size_of::<#field_type>();
    }
}
