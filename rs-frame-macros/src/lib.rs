extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// #[proc_macro]
// pub fn example_macro_fn(_: TokenStream) -> TokenStream {
// 	"struct Stuff;".parse().unwrap()
// }

#[proc_macro_derive(Controller, attributes(hash, route, trigger))]
pub fn controller_derive(input: TokenStream) -> TokenStream {
	// let args = input.clone();
	// let args = parse_macro_input!(args as AttributeArgs);
	let input = parse_macro_input!(input as DeriveInput);


	// println!("args: {:#?}", args);
	println!("input: {:#?}", input);
	"".parse().unwrap()
}

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
	let out = input.clone();
	// let args = input.clone();
	// let args = parse_macro_input!(args as AttributeArgs);
	let input = parse_macro_input!(input as DeriveInput);


	println!("args: {:#?}", args);
	println!("input: {:#?}", input);
	
	out
}
