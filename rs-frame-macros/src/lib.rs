extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};

// #[proc_macro]
// pub fn example_macro_fn(_: TokenStream) -> TokenStream {
// 	"struct Stuff;".parse().unwrap()
// }

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

#[proc_macro_derive(AppPath, attributes(path, query))]
pub fn app_path_derive(input: TokenStream) -> TokenStream {
    println!("AppPath Struct:");

    // let args = input.clone();
    // let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    for attr in &input.attrs {
        println!("attrs: {:#?}", attr);
    }

    // println!("args: {:#?}", args);
    println!("input: {:#?}", input);
    "".parse().unwrap()
}

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("New Struct:");

    // let out = input.clone();
    // let args = input.clone();
    // let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    let args = parse_macro_input!(args as AttributeArgs);

    println!("args: {:#?}", args);
    // for thing in args {
    // 	match thing {
    // 		proc_macro::TokenTree::Literal(lit) => println!("lit is {}", lit),
    // 		_ => {}
    // 	}
    // }

    // println!("input: {:#?}", input);

    // out
    let output = quote! { #input };
    output.into()
}
