extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, parse_str, Block, FnArg, ItemFn, Path, ReturnType, Signature, Stmt,
    Visibility,
};

#[proc_macro]
pub fn benchmark(_: TokenStream) -> TokenStream {
    let mut input = ItemFn {
        attrs: vec![],
        vis: Visibility::Inherited,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: Ident::new("benchmark", Span::call_site()),
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: Default::default(),
            variadic: None,
            output: ReturnType::Default,
        },
        block: Box::new(Block {
            brace_token: Default::default(),
            stmts: vec![],
        }),
    };

    let criterion: FnArg = parse_str("c: &mut Criterion").unwrap();
    input.sig.inputs.push_value(criterion);

    for day in 1..=25 {
        if !std::path::Path::exists(format!("./src/days/day_{:02}.rs", day).as_ref()) {
            continue;
        }
        let puzzle_input: Stmt = parse_str(&*format!(
            "let input = std::fs::read_to_string(\"data/inputs/day_{:02}.txt\").unwrap();",
            day
        ))
        .unwrap();
        input.block.stmts.push(puzzle_input);
        for part in ['a', 'b'] {
            let bench: Stmt =
                parse_str(&*format!("c.bench_function(\"Day {:02} | Part {:02}\", |b| b.iter(|| advent_of_code_rust_criterion::days::day_{:02}::part_{}(&*input)));", day, part.to_uppercase(), day, part))
                    .unwrap();
            input.block.stmts.push(bench);
        }
    }

    quote!(#input).into()
}

#[proc_macro]
pub fn day_function_vec(input: TokenStream) -> TokenStream {
    let vec_name = parse_macro_input!(input as Ident);

    let mut days = vec![];
    for day in 1..=25 {
        if !std::path::Path::exists(format!("./src/days/day_{:02}.rs", day).as_ref()) {
            continue;
        }

        let mut parts = vec![];
        for part in ['a', 'b'] {
            let part: Path =
                parse_str(&*format!("crate::days::day_{:02}::part_{}", day, part)).unwrap();
            parts.push(part)
        }
        days.push(parts);
    }

    let num_days = days.len();
    quote!(
        pub const #vec_name: [[fn(&str) -> Option<Box<dyn Display>>; 2]; #num_days] = [
            #([#(|input| #days(input).map(|o| Box::new(o) as _)),*]),*
        ];
    )
    .into()
}
