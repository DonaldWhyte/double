#[macro_use] extern crate maplit;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


const MIN_ARGS: usize = 1;
const MAX_ARGS: usize = 12;


fn generate_matcher_macro(max_args: usize) -> String {
    assert!(max_args >= MIN_ARGS && max_args <= MAX_ARGS);

    let arg_nums: Vec<usize> = (MIN_ARGS..MAX_ARGS).collect();
    let macro_cases: Vec<String> = arg_nums.iter().map(
        |&i| generate_matcher_macro_case_n(i)
    ).collect();
    format!(
        "#[macro_export]\nmacro_rules! matcher {{\n{}\n\n}}",
        macro_cases.join("\n"))
}

fn generate_matcher_macro_case_n(n_args: usize) -> String {
    let arg_nums: Vec<usize> = (MIN_ARGS..n_args + 1).collect();
    let case_args: Vec<String> = arg_nums.iter().map(
        |&i| format!("$m{}:expr", i.to_string())
    ).collect();
    let match_impl_func_args: Vec<String> = arg_nums.iter().map(
        |&i| format!("$m{}", i.to_string())
    ).collect();

    format!("
    ({}) => (
        &|args| -> bool {{ match_impl_{}(args, ({})) }}
    );",
        case_args.join(", "),
        n_args.to_string(),
        match_impl_func_args.join(", "))
}

fn generate_match_impls(max_args: usize) -> String {
    assert!(max_args >= MIN_ARGS && max_args <= MAX_ARGS);

    let arg_nums: Vec<usize> = (MIN_ARGS..MAX_ARGS).collect();
    let match_impls: Vec<String> = arg_nums.iter().map(
        |&i| generate_match_impl_n(i)
    ).collect();
    match_impls.join("\n")
}

fn generate_match_impl_n(n_args: usize) -> String {
    let arg_num_to_generic_type = hashmap!(
        0usize => "A",
        1usize => "B",
        2usize => "C",
        3usize => "D",
        4usize => "E",
        5usize => "F",
        6usize => "G",
        7usize => "H",
        8usize => "I",
        9usize => "J",
        10usize => "K",
        11usize => "J"
    );
    assert!(arg_num_to_generic_type.len() == MAX_ARGS);

    // We need a special case for one argument. The rust compile won't treat
    // the input arg as a one-tuple and will treat it is a single arg instead.
    if n_args == 1 {
        return "
pub fn match_impl_1<A>(arg: &A, arg_matcher: &Fn(&A) -> bool) -> bool {
    arg_matcher(arg)
}".to_owned();
    }

    let arg_number_range: Vec<usize> = (0..n_args).collect();
    let type_param_names: Vec<String> = arg_number_range.iter().map(
        |&i| arg_num_to_generic_type.get(&i)
            .expect("not enough num -> type name mappings")
            .to_owned()
            .to_owned()
    ).collect();

    let matcher_params: Vec<String> = type_param_names.iter().map(
        |ref t| format!("&Fn(&{}) -> bool", t)
    ).collect();

    let matcher_invocations: Vec<String> = arg_number_range.iter().map(
        |&i| format!(
            "arg_matchers.{}(&args.{})",
            i.to_string(),
            i.to_string())
    ).collect();

    format!("
pub fn match_impl_{}<{}>(args: &(
        {}
    ),
    arg_matchers: (
        {}
    )) -> bool {{
    let matches = vec!(
        {}
    );
    !matches.iter().any(|is_match| !is_match)
}}",
        n_args.to_string(),
        type_param_names.join(","),
        type_param_names.join(",\n        "),
        matcher_params.join(",\n        "),
        matcher_invocations.join(",\n        "))
}

fn main() {
    let file_contents = format!(
        "{}\n\n{}\n",
        generate_matcher_macro(MAX_ARGS),
        generate_match_impls(MAX_ARGS));

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("matcher_generated.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(file_contents.as_bytes()).unwrap();
}
