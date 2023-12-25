//! demo macro lib to generate special result error structs

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;

#[proc_macro]
/// takes tokenstream input to generate result error token stream
///
/// # Arguments
///
/// * `input` - provided token stream
///
/// # Returns
///
/// generated tokenstream of ResultError struct
pub fn make_result_error(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let idents = idents_from_input(input_str.clone());

    let (err_match, impl_froms) = gen_components(idents);

    to_stream(input_str, err_match, impl_froms)
}

/// takes input string to generate HashMap of token strings
///
/// # Arguments
///
/// * `input_str` - provided token stream as string
///
/// # Returns
///
/// generated hashmap of token strings
fn idents_from_input(input_str: String) -> HashMap<String, String> {
    let mut idents = HashMap::new();
    let parts = input_str.split(", ");
    for i in parts {
        let mut parts2 = i.split('(');
        let id = parts2.next().unwrap();
        let sid = parts2.next().unwrap();
        idents.insert(id.to_string(), sid[0..sid.len() - 1].to_string());
    }
    idents
}

/// generate component strings from hashmap values
///
/// # Arguments
///
/// * `idents` - provided hashmap identifier token strings
///
/// # Returns
///
/// err_match and impl_froms component strings
fn gen_components(idents: HashMap<String, String>) -> (String, String) {
    let mut err_match = "".to_owned();
    let mut impl_froms = "".to_owned();
    for (i, v) in idents {
        err_match.push_str(&format!(
            "ResultError::{}(a) => write!(f, \"{{}}\", a),\n",
            i
        ));
        impl_froms.push_str(&format!(
            "impl From<{}> for ResultError {{
            fn from(err: {}) -> Self {{
                ResultError::{}(err)
            }}
        }}\n",
            v, v, i
        ));
    }
    (err_match, impl_froms)
}

/// generate token stream from `input_str`, `err_match`, `impl_froms`
///
/// # Arguments
///
/// * `input_str` - provided token stream as string
/// * `err_match` - err_match component string
/// * `impl_froms` - impl_froms component string
///
/// # Returns
///
/// final token stream for ResultError macro
fn to_stream(input_str: String, err_match: String, impl_froms: String) -> TokenStream {
    format!(
        "#[derive(Debug)]
        /// container for ResultError error types
        pub enum ResultError {{
            {} 
        }}

        impl std::error::Error for ResultError {{}}

        impl Display for ResultError {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                match self {{
                    {}
                }}
            }}
        }}

        {}",
        input_str, err_match, impl_froms
    )
    .parse()
    .unwrap()
}
