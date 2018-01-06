use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn generate_matcher_macro(n_args: u32) -> String {
    assert!(n_args >= 2);

    String result = "macro_rules! matcher {";
    result
}

fn generate_matcher_macro_case_n(n: u32) -> String {

}

fn generate_match_impl_n(n: u32) -> String {
    assert!(n_args >= 2);

    let type_param_names = range(n).iter().map().collect().join(",");

    String result = "fn match_impl_" + n.to_string() + "<";
    result.push_str(type_param_names.join(","));
    result.push_str(">");
    result.push_str("(args: &()");
    result.push_str(type_param_names.join(","));
    result.push_str(")");

    result.push_str("{\n");
    result.push_str("    let matches = vec!(");
    for i in 0..n {
        let i_str = i.to_string();
        result.push_str(
            "        arg_matchers." + i_str + "(&args." + i_str + "),");
    }
    result.push_str("    );");
    result.push_str("!matches.iter().any(|is_match| !is_match)");
    result.push_str("}\n");
    result
}

fn match_impl2<A, B>(
    args: &(
        A,
        B,
    ),
    arg_matchers: (
        &Fn(&A) -> bool,
        &Fn(&B) -> bool,
    )) -> bool
{
    let matches = vec!(
        arg_matchers.0(&args.0),
        arg_matchers.1(&args.1),
    );
    !matches.iter().any(|is_match| !is_match)
}




fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("matcher_generated.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b"
macro_rules! matcher {
    // TODO
}
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ").unwrap();
}
