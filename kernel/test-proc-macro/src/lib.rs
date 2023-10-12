use proc_macro::{TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn unit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    const MSG: &str = "Unit test functions must be of the format `fn <name>() { .. }`";

    let test_func = item.clone();
    
    let mut iter = item.into_iter();
    let mut func_name = None;
    while let Some(tok) = iter.next() {
        let tok = match tok {
            TokenTree::Ident(ident) if ident.to_string().as_str() == "fn" => iter.next(),
            _ => continue,
        };

        let next_token = match tok {
            Some(TokenTree::Ident(ident)) => {
                func_name = Some(ident.to_string());
                iter.next()
            },
            _ => panic!("{MSG}"),
        };

        match next_token {
            Some(t) if t.to_string().as_str() == "()" => break,
            Some(TokenTree::Punct(p)) if p.to_string().as_str() == "<" => panic!("Unit test functions cannot be generic"),
            _ => panic!("{MSG}"),
        }
    }

    let func_name = func_name.unwrap_or_else(|| panic!("{MSG}"));
    let func_name_str = func_name.replace("_", "-");

    let mut output = format!(
"
const _: () = {{
    #[link_section = \".init_array\"]
    static __TEST_POINTER: &crate::test_framework::__UnitTest = {{
        static __TEST: crate::test_framework::__UnitTest = crate::test_framework::__UnitTest::new(
            {func_name},
            \"{func_name_str}\",
        );
        &__TEST
    }};
}};
#[allow(unused)]
"
    ).parse::<TokenStream>().unwrap();
    output.extend(test_func);
    output
}