#![recursion_limit="128"]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, PartialEq)]
enum PathToRegexError {
    MissingLeadingForwardSlash,
    NonAsciiChars,
    InvalidIdentifier(String),
    InvalidTrailingSlash,
}

fn path_to_regex(path: &str) -> Result<String, PathToRegexError> {
    enum ParseState {
        Initial,
        Static,
        VarName(String),
    };

    if !path.is_ascii() {
        return Err(PathToRegexError::NonAsciiChars);
    }

    let ident_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();

    let mut regex = "".to_string();
    let mut parse_state = ParseState::Initial;

    for byte in path.chars() {
        match parse_state {
            ParseState::Initial => {
                if byte != '/' {
                    return Err(PathToRegexError::MissingLeadingForwardSlash);
                }

                regex += "^/";

                parse_state = ParseState::Static;
            }
            ParseState::Static => {
                if byte == ':' {
                    parse_state = ParseState::VarName("".to_string());
                } else {
                    regex.push(byte);
                    parse_state = ParseState::Static;
                }
            }
            ParseState::VarName(mut name) => {
                if byte == '/' {
                    // Validate 'name' as a Rust identifier
                    if !ident_regex.is_match(&name) {
                        println!("checking name: {}\tbad!", name);
                        return Err(PathToRegexError::InvalidIdentifier(name));
                    } else {
                        println!("checking name: {}\tgood!", name);
                    }

                    regex += &format!("(?P<{}>[^/]+)/", name);
                    parse_state = ParseState::Static;
                } else {
                    name.push(byte);
                    parse_state = ParseState::VarName(name);
                }
            }
        };
    }

    if let ParseState::VarName(name) = parse_state {
        regex += &format!("(?P<{}>[^/]+)", name);
    }

    if regex.ends_with("/") {
        return Err(PathToRegexError::InvalidTrailingSlash);
    }

    regex += "$";

    Ok(regex)
}

#[test]
fn test_path_to_regex() {
    let regex = path_to_regex("/p/:project_id/exams/:exam_id/submissions_expired").unwrap();
    assert_eq!(
        regex,
        r"^/p/(?P<project_id>[^/]+)/exams/(?P<exam_id>[^/]+)/submissions_expired$"
    );
}

#[test]
fn test_path_to_regex_no_path_params() {
    let regex = path_to_regex("/p/exams/submissions_expired").unwrap();
    assert_eq!(regex, r"^/p/exams/submissions_expired$");
}

#[test]
fn test_path_to_regex_no_leading_slash() {
    let regex = path_to_regex("p/exams/submissions_expired");
    assert_eq!(regex, Err(PathToRegexError::MissingLeadingForwardSlash));
}

#[test]
fn test_path_to_regex_non_ascii_chars() {
    let regex = path_to_regex("🥖p🥖:project_id🥖exams🥖:exam_id🥖submissions_expired");
    assert_eq!(regex, Err(PathToRegexError::NonAsciiChars));
}

#[test]
fn test_path_to_regex_invalid_ident() {
    let regex = path_to_regex("/p/:project_id/exams/:exam*ID/submissions_expired");
    assert_eq!(
        regex,
        Err(PathToRegexError::InvalidIdentifier("exam*ID".to_string()))
    );

    let regex = path_to_regex("/p/:project_id/exams/:_exam_id/submissions_expired");
    assert_eq!(
        regex,
        Err(PathToRegexError::InvalidIdentifier("_exam_id".to_string()))
    );
}

#[test]
fn test_path_to_regex_invalid_ending() {
    let regex = path_to_regex("/p/:project_id/exams/:exam_id/submissions_expired/");
    assert_eq!(regex, Err(PathToRegexError::InvalidTrailingSlash));
}

fn get_string_attr(name: &str, attrs: &Vec<syn::Attribute>) -> Option<String> {
    for attr in attrs {
        let attr = attr.parse_meta();

        if let Ok(syn::Meta::List(ref list)) = attr {
            if list.ident == name {
                for thing in &list.nested {
                    if let syn::NestedMeta::Literal(syn::Lit::Str(str_lit)) = thing {
                        return Some(str_lit.value());
                    }
                }
            }
        }
    }

    None
}

fn has_flag_attr(name: &str, attrs: &Vec<syn::Attribute>) -> bool {
    for attr in attrs {
        let attr = attr.parse_meta();

        if let Ok(syn::Meta::Word(ref ident)) = attr {
            if ident == name {
                return true;
            }
        }
    }

    false
}

fn get_struct_fields(data: &syn::Data) -> Vec<syn::Field> {
    match data {
        syn::Data::Struct(data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(ref named_fields) => {
                    named_fields
                        .named
                        .iter()
                        .cloned()
                        .collect()
                }
                _ => panic!("Struct fields must be named")
            }
        }
        _ => panic!("AppPath derive is only supported for structs")
    }
}

fn field_is_option(field: &syn::Field) -> bool {
    match field.ty {
        syn::Type::Path(ref type_path) => {
            type_path
                .path
                .segments
                .iter()
                .last()
                .map(|segment| segment.ident == "Option")
                .unwrap_or(false)
        }
        _ => false
    }
}

#[proc_macro_derive(AppPath, attributes(path, query))]
pub fn app_path_derive(input: TokenStream) -> TokenStream {
    println!("AppPath Struct:");

    let input = parse_macro_input!(input as DeriveInput);

    let struct_fields = get_struct_fields(&input.data);

    let (path_fields, query_fields): (Vec<_>, Vec<_>) = struct_fields.into_iter().partition(|f| {
        !has_flag_attr("query", &f.attrs)
    });

    let name = &input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let path_string = get_string_attr("path", &input.attrs);

    let url_path = path_string.expect("derive(AppPath) requires a #[path(\"/your/path/here\")] attribute on the struct");

    let path_regex = path_to_regex(&url_path).unwrap();

    // TODO - validate path_regex and make sure struct and path have matching fields

    let path_field_assignments = path_fields.clone().into_iter().map(|f| {
        let f_ident = f.ident.unwrap();
        let f_ident_str = f_ident.to_string();

        quote! {
            #f_ident: captures[#f_ident_str].parse().ok()?
        }
    });

    let query_field_assignments = query_fields.into_iter().map(|f| {
        let is_option = field_is_option(&f);
        let f_ident = f.ident.unwrap();

        if is_option {
            quote! {
                #f_ident: query_string.and_then(|q| qs::from_str(q).ok())
            }
        } else {
            quote! {
                #f_ident: qs::from_str(query_string?).ok()?
            }
        }
    });

    let path_field_parsers = quote! {
        #(
            #path_field_assignments
        ),*
    };

    let query_field_parsers = quote! {
        #(
            #query_field_assignments
        ),*
    };

    let expanded = quote! {
        impl #impl_generics rs_frame::AppPath for #name #ty_generics #where_clause {

            fn path_pattern() -> String {
                #path_regex.to_string()
            }

            fn from_str(app_path: &str) -> Option<Self> {
                use serde_qs as qs;

                let question_pos = app_path.find('?');
                let just_path = &app_path[..(question_pos.unwrap_or_else(|| app_path.len()))];

                // TODO - store this in lazy_static
                let path_pattern = Regex::new(&Self::path_pattern()).ok()?;
                let captures = path_pattern.captures(just_path)?;

                let query_string = question_pos.map(|question_pos| {
                    let mut query_string = &app_path[question_pos..];

                    if query_string.starts_with('?') {
                        query_string = &query_string[1..];
                    }

                    query_string
                });

                Some(ExpiredSubmissionsPath {
                    #path_field_parsers,
                    #query_field_parsers
                })
            }

            fn query_string(&self) -> Option<String> {
                // TODO - implement
                //        Be sure to remove duplicates because
                //        there could be multiple fields with
                //        a #[query] attribute that have common fields
                None
            }

            fn to_string(&self) -> String {
                format!(
                    "/p/{}/exams/{}/submissions_expired",
                    self.project_id, self.exam_id
                )
            }
        }
    };

    expanded.into()
}
