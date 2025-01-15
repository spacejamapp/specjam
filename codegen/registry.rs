//! test vector registry generator

use anyhow::Result;
use heck::ToUpperCamelCase;
use proc_macro2::Span;
use serde_json::Value;
use std::{
    fs::{self, ReadDir},
    path::Path,
};
use syn::{parse_quote, Expr, Ident, Item, ItemMod, LitStr};

/// The scale sections which contains both tiny and large test vectors
const SCALE_SECTIONS: [&str; 6] = [
    "assurances",
    "authorizations",
    "disputes",
    "reports",
    "safrole",
    "statistics",
];

/// The general sections which have only one set of test vectors
const GENERAL_SECTIONS: &[&str] = &["history", "preimages"];

/// The test vector registry
pub struct Registry<'s> {
    /// The root directory of the test vectors
    root: &'s Path,

    /// The output directory
    output: &'s Path,

    /// The registry
    registry: Vec<Item>,

    /// The test vectors
    tests: ItemMod,
}

impl<'s> Registry<'s> {
    /// Generate the test vector registry
    pub fn new(root: &'s Path, output: &'s Path) -> Self {
        Self {
            root,
            output,
            registry: Vec::new(),
            tests: parse_quote!(
                /// The test vectors
                pub mod tests {}
            ),
        }
    }

    /// generate all tests
    pub fn run(mut self) -> Result<()> {
        self.scale()?;
        self.general()?;
        self.codec()?;
        self.pvm()?;
        self.trie()?;

        // impl the registry
        let all = self.extract_all_tests();
        let tests = self.tests;
        let items = self.registry;
        let registry = quote::quote! {
            #all

            #(#items)*

            #tests
        };

        // write the output to the output directory
        let out = self.output.join("registry.rs");
        fs::write(out, registry.to_string())?;
        Ok(())
    }

    /// Extract all tests from the registry
    fn extract_all_tests(&self) -> syn::Item {
        let mut tests = Vec::new();
        for item in &self.registry {
            if let syn::Item::Const(syn::ItemConst { expr, .. }) = item {
                if let syn::Expr::Array(syn::ExprArray { elems, .. }) = *expr.clone() {
                    tests.extend(elems.into_iter())
                }
            }
        }

        let len = tests.len();
        let len: Expr = parse_quote!(#len);
        parse_quote! {
            #[doc = "The all test vectors"]
            pub const ALL_TESTS: [crate::Test; #len] = [#(#tests),*];
        }
    }

    /// Generate the scale test vectors
    fn scale(&mut self) -> Result<()> {
        for section in SCALE_SECTIONS {
            let path = self.root.join(section);
            let mut tests = Vec::new();
            for scale in ["tiny", "full"] {
                let path = path.join(scale);
                let dir = fs::read_dir(&path)?;
                tests.extend(self.process_base(section, dir, Some(scale.to_string()))?);
            }
            self.embed_namespace(section, tests);
        }

        Ok(())
    }

    /// Generate the general test vectors
    fn general(&mut self) -> Result<()> {
        for section in GENERAL_SECTIONS {
            let path = self.root.join(section).join("data");
            let dir = fs::read_dir(path)?;
            let tests = self.process_base(section, dir, None)?;
            self.embed_namespace(section, tests);
        }

        Ok(())
    }

    /// Generate the codec test vectors
    fn codec(&mut self) -> Result<()> {
        let Some((_, tests)) = self.tests.content.as_mut() else {
            return Err(anyhow::anyhow!("tests already initialized"));
        };

        let mut const_tests = Vec::new();
        for entry in fs::read_dir(self.root.join("codec").join("data"))? {
            let path = entry?.path();
            if path.extension().unwrap_or_default() != "json" {
                continue;
            }

            let bin = hex::encode(fs::read(path.with_extension("bin"))?);
            let parse = move |json: Value| Ok((json.to_string(), bin));
            let test = wrap_test(tests, &None, "codec", &path, parse)?;
            const_tests.push(test);
        }

        self.embed_namespace("codec", const_tests);
        Ok(())
    }

    /// Generate the pvm test vectors
    fn pvm(&mut self) -> Result<()> {
        let Some((_, tests)) = self.tests.content.as_mut() else {
            return Err(anyhow::anyhow!("tests already initialized"));
        };

        let section = "pvm";
        let dir = fs::read_dir(self.root.join(section).join("programs"))?;
        let mut const_tests = Vec::new();
        for entry in dir {
            let path = entry?.path();
            let test = self::wrap_test(tests, &None, section, &path, |json| {
                let input = serde_json::json!({
                    "input": {
                        "initial-regs": json["pre-state"],
                        "initial-pc": json["initial-pc"],
                        "initial-page-map": json["initial-page-map"],
                        "initial-memory": json["initial-memory"],
                        "initial-gas": json["initial-gas"],
                        "program": json["program"],
                    },
                })
                .to_string();

                let output = serde_json::json!({
                    "output": {
                        "expected-status": json["expected-status"],
                        "expected-regs": json["expected-regs"],
                        "expected-pc": json["expected-pc"],
                        "expected-memory": json["expected-memory"],
                        "expected-gas": json["expected-gas"],
                    }
                })
                .to_string();

                Ok((input, output))
            })?;

            const_tests.push(test);
        }

        self.embed_namespace(section, const_tests);
        Ok(())
    }

    /// Generate the trie test vectors
    fn trie(&mut self) -> Result<()> {
        let file = self.root.join("trie").join("trie.json");
        let Some((_, tests)) = self.tests.content.as_mut() else {
            return Err(anyhow::anyhow!("tests already initialized"));
        };

        let test = wrap_test(
            tests,
            &None,
            "trie",
            &file,
            |json| -> Result<(String, String)> {
                let vectors = json
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("invalid trie test"))?;

                let mut input = Vec::new();
                let mut output = Vec::new();
                for vector in vectors {
                    input.push(serde_json::json!({
                        "input": vector["input"],
                    }));
                    output.push(serde_json::json!({
                        "output": vector["output"],
                    }));
                }

                Ok((
                    serde_json::to_string(&input)?,
                    serde_json::to_string(&output)?,
                ))
            },
        )?;

        self.embed_namespace("trie", vec![test]);
        Ok(())
    }

    fn process_base(
        &mut self,
        section: &str,
        dir: ReadDir,
        scale: Option<String>,
    ) -> Result<Vec<syn::Path>> {
        let Some((_, tests)) = self.tests.content.as_mut() else {
            return Err(anyhow::anyhow!("tests already initialized"));
        };

        let mut const_tests = Vec::new();
        for entry in dir {
            let path = entry?.path();
            if path.extension().unwrap_or_default() != "json" {
                continue;
            }

            let test = self::wrap_test(tests, &scale, section, &path, |json| {
                let input = serde_json::json!({
                    "input": json["input"],
                    "pre_state": json["pre_state"],
                })
                .to_string();

                let output = serde_json::json!({
                    "output": json["output"],
                    "post_state": json["post_state"],
                })
                .to_string();

                Ok((input, output))
            })?;

            const_tests.push(test);
        }

        Ok(const_tests)
    }

    /// Embed the namespace into the registry
    fn embed_namespace(&mut self, section: &str, tests: Vec<syn::Path>) {
        let namespace = Ident::new(&section.to_uppercase(), Span::call_site());
        let tests_len = tests.len();
        let tests_len: Expr = parse_quote!(#tests_len);
        let doc = LitStr::new(
            &format!("The test vectors for the {section} section"),
            Span::call_site(),
        );

        self.registry.push(parse_quote! {
                #[doc = #doc]
                pub const #namespace: [crate::Test; #tests_len] = [#(#tests),*];
        });
    }
}

fn wrap_test<P>(
    tests: &mut Vec<Item>,
    scale: &Option<String>,
    section: &str,
    file: &Path,
    parse: P,
) -> Result<syn::Path>
where
    P: FnOnce(Value) -> Result<(String, String)>,
{
    // build the constant Test
    let mut test = file
        .with_extension("")
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("invalid file name"))?
        .to_string_lossy()
        .replace('-', "_");

    let doc = LitStr::new(
        &format!("test vector {test} for {section}"),
        Span::call_site(),
    );

    // read the json file
    let json: Value = serde_json::from_slice(&fs::read(file)?)
        .map_err(|e| anyhow::anyhow!("invalid json {file:?} : {e}"))?;
    let (input, output) = parse(json)?;

    // construct the constant Test
    let test_lower = LitStr::new(&test, Span::call_site());
    test = test.to_uppercase();
    let const_test = {
        let mut test = format!("TEST_{}_{test}", section.to_uppercase());
        if let Some(scale) = &scale {
            test.push_str(&format!("_{}", scale.to_uppercase()));
        }
        Ident::new(&test, Span::call_site())
    };
    let const_input = LitStr::new(&input, Span::call_site());
    let const_output = LitStr::new(&output, Span::call_site());

    // handle enum
    let section_caml = Ident::new(&section.to_upper_camel_case(), Span::call_site());
    let scale: Expr = if let Some(scale) = &scale {
        let ident = Ident::new(&scale.to_upper_camel_case(), Span::call_site());
        parse_quote!(Some(crate::Scale::#ident))
    } else {
        parse_quote!(None)
    };

    tests.push(parse_quote!(
        #[doc = #doc]
        pub const #const_test: crate::Test = crate::Test {
            scale: #scale,
            section: crate::Section::#section_caml,
            name: #test_lower,
            input: #const_input,
            output: #const_output,
        };
    ));

    Ok(parse_quote!(tests::#const_test))
}
