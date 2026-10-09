use regex::Regex;
use std::collections::HashSet;

/// Rust reserved keywords that require r# prefix if used as identifiers.
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "async", "await", "dyn", "box", "macro", "try", "yield",
    "abstract", "become", "final", "override", "priv", "typeof", "unsized", "virtual",
];

pub fn is_valid_ident(ident: &str) -> bool {
    let clean = ident.trim().trim_start_matches("r#");
    if clean.is_empty() {
        return false;
    }
    let mut chars = clean.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn sanitize_ident(ident: &str) -> String {
    let clean = ident.trim().trim_start_matches('*').trim();
    if clean.is_empty() {
        return "unnamed".to_string();
    }
    if clean.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        format!("_{}", clean)
    } else if RUST_KEYWORDS.contains(&clean) {
        format!("r#{}", clean)
    } else {
        clean.to_string()
    }
}

/// Cleans C numeric literals (stripping u, U, l, L, f, F suffixes).
pub fn clean_c_literal(lit: &str) -> String {
    let t = lit.trim();
    // Strip simple float suffixes: 1.0f -> 1.0
    if t.ends_with('f') || t.ends_with('F') {
        return t[..t.len() - 1].trim().to_string();
    }
    // Strip integer suffixes: 100u, 100UL, 100L, 100LL
    let mut s = t.to_string();
    while s.ends_with('u') || s.ends_with('U') || s.ends_with('l') || s.ends_with('L') {
        s.pop();
    }
    if s.is_empty() {
        "0".to_string()
    } else {
        s
    }
}

/// Maps standard C/C++ scalar and pointer types to idiomatic Rust types.
pub fn map_c_type(c_type: &str, in_array: bool) -> String {
    let cleaned_c_type = c_type.replace("<struct ", "<").replace("<class ", "<");
    let mut t = cleaned_c_type.as_str().trim();

    // Strip const, volatile, struct, class keywords
    while t.starts_with("const ") || t.starts_with("struct ") || t.starts_with("class ") || t.starts_with("volatile ") {
        if let Some(rest) = t.strip_prefix("const ") {
            t = rest.trim();
        } else if let Some(rest) = t.strip_prefix("struct ") {
            t = rest.trim();
        } else if let Some(rest) = t.strip_prefix("class ") {
            t = rest.trim();
        } else if let Some(rest) = t.strip_prefix("volatile ") {
            t = rest.trim();
        }
    }

    if let Some(rest) = t.strip_prefix("blender::") {
        t = rest.trim();
    }

    if t.starts_with("std::optional<") || t.starts_with("optional<") {
        let inner = t.trim_start_matches("std::optional<").trim_start_matches("optional<").trim_end_matches('>').trim();
        return format!("Option<{}>", map_c_type(inner, false));
    }
    if t.starts_with("std::vector<") || t.starts_with("vector<") || t.starts_with("Vector<") {
        let inner = t.trim_start_matches("std::vector<").trim_start_matches("vector<").trim_start_matches("Vector<").trim_end_matches('>').trim();
        return format!("Vec<{}>", map_c_type(inner, false));
    }
    if t.starts_with("std::unique_ptr<") || t.starts_with("unique_ptr<") {
        let inner = t.trim_start_matches("std::unique_ptr<").trim_start_matches("unique_ptr<").trim_end_matches('>').trim();
        return format!("Box<{}>", map_c_type(inner, false));
    }

    match t {
        "int" | "signed int" | "bContextDataResult" | "int32_t" | "int32" => "i32".to_string(),
        "unsigned int" | "uint" | "uint32_t" | "uint32" => "u32".to_string(),
        "short" | "signed short" | "short int" | "int16_t" | "int16" => "i16".to_string(),
        "unsigned short" | "ushort" | "uint16_t" | "uint16" => "u16".to_string(),
        "char" | "int8_t" | "schar" | "int8" => {
            if in_array {
                "u8".to_string()
            } else {
                "i8".to_string()
            }
        }
        "unsigned char" | "uchar" | "uint8_t" | "byte" | "uint8" => "u8".to_string(),
        "long" | "long int" | "int64_t" | "int64" => "i64".to_string(),
        "unsigned long" | "uint64_t" | "uint64" | "size_t" | "uintptr_t" | "ulong" => "usize".to_string(),
        "float" => "f32".to_string(),
        "double" => "f64".to_string(),
        "bool" => "bool".to_string(),
        "void" => "()".to_string(),
        "void*" | "void *" => "*mut core::ffi::c_void".to_string(),
        "char*" | "char *" | "const char*" | "const char *" => "*mut i8".to_string(),
        "std::string" | "string" | "UString" => "String".to_string(),
        "StringRef" | "StringRefNull" => "String".to_string(),
        other => {
            if other.ends_with('*') {
                "*mut core::ffi::c_void".to_string()
            } else {
                sanitize_ident(other)
            }
        }
    }
}

/// Strips C and C++ comments and disabled blocks from source code while preserving clean syntax.
pub fn strip_comments(source: &str) -> String {
    let if0_re = Regex::new(r"(?s)#if\s+0\b.*?#endif").unwrap();
    let cleaned_if0 = if0_re.replace_all(source, " ");
    let block_re = Regex::new(r"(?s)/\*.*?\*/").unwrap();
    let cleaned = block_re.replace_all(&cleaned_if0, " ");
    let line_re = Regex::new(r"//.*").unwrap();
    line_re.replace_all(&cleaned, "").to_string()
}

/// Fast-path deterministic parser for C/C++ enum declarations.
/// Emits transparent newtype structs with associated constants, completely avoiding
/// duplicate discriminant, missing symbol, or bitflag enum errors in Rust.
pub fn parse_c_enums(content: &str) -> Vec<String> {
    let mut results = Vec::new();
    let clean = strip_comments(content);

    let enum_re = Regex::new(
        r"(?s)(?:typedef\s+)?enum(?:\s+class)?\s*(\w+)?(?:\s*:\s*(\w+))?\s*\{([^}]+)\}\s*(\w+)?\s*;"
    ).unwrap();

    let mut seen_enums = HashSet::new();

    for cap in enum_re.captures_iter(&clean) {
        let name_from_body = cap.get(1).map(|m| m.as_str());
        let name_from_typedef = cap.get(4).map(|m| m.as_str());
        let raw_name = name_from_typedef.or(name_from_body).unwrap_or("UnnamedEnum");
        let enum_name = sanitize_ident(raw_name);

        if seen_enums.contains(&enum_name) || enum_name == "UnnamedEnum" {
            continue;
        }
        seen_enums.insert(enum_name.clone());

        let repr_spec = cap.get(2).map(|m| m.as_str()).unwrap_or("i32");
        let rust_repr = match repr_spec {
            "short" | "int16_t" => "i16",
            "unsigned short" | "uint16_t" => "u16",
            "int" | "int32_t" => "i32",
            "uint32_t" | "unsigned int" | "uint" => "u32",
            "int8_t" | "char" => "i8",
            "uint8_t" | "uchar" => "u8",
            "int64_t" | "long" => "i64",
            "uint64_t" | "size_t" => "u64",
            _ => "i32",
        };

        let body = cap.get(3).map(|m| m.as_str()).unwrap_or("");
        let mut constants = Vec::new();
        let mut cur_val = 0i64;

        // Strip preprocessor lines in body
        let clean_body: String = body.lines()
            .filter(|l| !l.trim().starts_with('#'))
            .collect::<Vec<&str>>()
            .join(" ");

        for item in clean_body.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }

            if let Some((name, val)) = item.split_once('=') {
                let vname = sanitize_ident(name.trim());
                let clean_val = clean_c_literal(val.trim());

                if !vname.is_empty() && is_valid_ident(&vname) {
                    let is_safe_expr = clean_val.chars().all(|c| c.is_ascii_digit() || c == '-' || c == 'x' || c == 'X' || c == '<' || c == '>' || c == '|' || c == '&' || c == '^' || c == ' ' || c == '(' || c == ')');
                    if is_safe_expr && !clean_val.is_empty() {
                        constants.push(format!("    pub const {}: Self = Self(({}) as {});", vname, clean_val, rust_repr));
                    } else {
                        constants.push(format!("    pub const {}: Self = Self({} as {});", vname, cur_val, rust_repr));
                    }
                    cur_val += 1;
                }
            } else {
                let vname = sanitize_ident(item);
                if !vname.is_empty() && is_valid_ident(&vname) {
                    constants.push(format!("    pub const {}: Self = Self({} as {});", vname, cur_val, rust_repr));
                    cur_val += 1;
                }
            }
        }

        let rust_enum = format!(
            "#[repr(transparent)]\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]\npub struct {}(pub {});\n\nimpl {} {{\n{}\n}}\n",
            enum_name,
            rust_repr,
            enum_name,
            constants.join("\n")
        );
        results.push(rust_enum);
    }

    results
}

/// Fast-path deterministic parser for C/C++ struct declarations.
pub fn parse_c_structs(content: &str) -> Vec<String> {
    let mut results = Vec::new();
    let clean = strip_comments(content);

    let struct_re = Regex::new(
        r"(?s)(?:typedef\s+)?struct(?:\s+alignas\([^\)]+\))?\s+(\w+)(?:\s*:\s*(?:public\s+)?(\w+))?\s*\{([^}]+)\}\s*(\w+)?\s*;"
    ).unwrap();

    let ptr_to_array_re = Regex::new(r"\(\*+(\w+)\)\s*((?:\[[^\]]+\])+)").unwrap();
    let decl_re = Regex::new(r"(\*+)?\s*(\w+)\s*((?:\[[^\]]+\])+)?").unwrap();
    let bracket_clean_re = Regex::new(r"\[\s*([^\]]+?)\s*\]").unwrap();
    let braced_init_re = Regex::new(r"(?s)=\s*\{[^}]*\}").unwrap();
    let simple_init_re = Regex::new(r"=\s*[^,;]+").unwrap();

    let mut seen_structs = HashSet::new();

    for cap in struct_re.captures_iter(&clean) {
        let name_from_body = cap.get(1).map(|m| m.as_str());
        let name_from_typedef = cap.get(4).map(|m| m.as_str());
        let base_struct = cap.get(2).map(|m| m.as_str());
        let struct_name = name_from_typedef.or(name_from_body).unwrap_or("UnnamedStruct");

        if seen_structs.contains(struct_name) || struct_name == "UnnamedStruct" {
            continue;
        }
        seen_structs.insert(struct_name.to_string());

        let raw_body = cap.get(3).map(|m| m.as_str()).unwrap_or("");
        // 1. Strip braced initializers (= { ... })
        let no_braced = braced_init_re.replace_all(raw_body, "");
        // 2. Strip simple initializers (= 0, = nullptr, = "")
        let no_inits = simple_init_re.replace_all(&no_braced, "");
        // 3. Normalize brackets: [ 64 ] -> [64]
        let norm_body = bracket_clean_re.replace_all(&no_inits, "[$1]");

        let mut fields = Vec::new();
        let mut seen_fields = HashSet::new();
        let mut field_idx = 0;

        // If inheriting from a base struct, embed base as first field
        if let Some(base) = base_struct {
            fields.push(format!("    pub base: {},", sanitize_ident(base)));
            seen_fields.insert("base".to_string());
        }

        // Split statements by ';'
        for stmt in norm_body.split(';') {
            let stmt = stmt.trim();
            // Skip empty, preprocessor, macros, and C++ methods (functions with parentheses)
            if stmt.is_empty() || stmt.starts_with('#') || stmt.starts_with("DNA_") || stmt.starts_with("MEM_") || stmt.contains('(') || stmt.contains(')') {
                continue;
            }

            // Split comma-separated declarations (e.g. "int a, b", "float rotAxis[3], rotAngle")
            let parts: Vec<&str> = stmt.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            if parts.is_empty() {
                continue;
            }

            // First part contains base type + first declarator
            let first_tokens: Vec<&str> = parts[0].split_whitespace().collect();
            if first_tokens.len() < 2 {
                continue;
            }

            let base_type = first_tokens[..first_tokens.len() - 1].join(" ");
            let mut decls = vec![first_tokens.last().unwrap().to_string()];
            for p in &parts[1..] {
                decls.push(p.to_string());
            }

            for d in decls {
                let d_clean = d.trim();
                if d_clean.is_empty() {
                    continue;
                }

                // Check for pointer to array: float (*disps)[3]
                if let Some(caps) = ptr_to_array_re.captures(d_clean) {
                    let raw_vname = caps.get(1).map(|m| m.as_str()).unwrap_or("field");
                    let dims = caps.get(2).map(|m| m.as_str()).unwrap_or("[1]");
                    let dim_val = clean_c_literal(dims.trim_matches(|c| c == '[' || c == ']'));
                    let elem_type = map_c_type(&base_type, true);

                    let mut vname = sanitize_ident(raw_vname);
                    if seen_fields.contains(&vname) {
                        field_idx += 1;
                        vname = format!("{}_{}", vname, field_idx);
                    }
                    seen_fields.insert(vname.clone());

                    fields.push(format!("    pub {}: *mut [{}; {}],", vname, elem_type, dim_val));
                    continue;
                }

                // Check for standard declarator: *var, var[64], var
                if let Some(caps) = decl_re.captures(d_clean) {
                    let is_ptr = caps.get(1).is_some();
                    let raw_vname = caps.get(2).map(|m| m.as_str()).unwrap_or("field");
                    let array_dims = caps.get(3).map(|m| m.as_str());

                    let mut vname = sanitize_ident(raw_vname);
                    if seen_fields.contains(&vname) {
                        field_idx += 1;
                        vname = format!("{}_{}", vname, field_idx);
                    }
                    seen_fields.insert(vname.clone());

                    if let Some(dims) = array_dims {
                        // Handle multidimensional arrays: e.g. [4][4] or [3]
                        let raw_dims: Vec<&str> = dims.split(']').filter_map(|s| {
                            let s = s.trim_start_matches('[').trim();
                            if s.is_empty() { None } else { Some(s) }
                        }).collect();

                        let elem_type = map_c_type(&base_type, true);
                        let mut final_type = elem_type;
                        for dim in raw_dims.iter().rev() {
                            let clean_dim = clean_c_literal(if dim.is_empty() || *dim == "0" { "0" } else { dim });
                            final_type = format!("[{}; {}]", final_type, clean_dim);
                        }
                        fields.push(format!("    pub {}: {},", vname, final_type));
                    } else if is_ptr {
                        fields.push(format!("    pub {}: *mut core::ffi::c_void,", vname));
                    } else {
                        let rust_type = map_c_type(&base_type, false);
                        fields.push(format!("    pub {}: {},", vname, rust_type));
                    }
                }
            }
        }

        let clean_sname = sanitize_ident(struct_name);
        let rust_struct = format!(
            "#[derive(Debug, Clone, PartialEq)]\n#[repr(C)]\npub struct {} {{\n{}\n}}\n\nimpl Default for {} {{\n    fn default() -> Self {{\n        unsafe {{ core::mem::zeroed() }}\n    }}\n}}\n",
            clean_sname,
            fields.join("\n"),
            clean_sname
        );
        results.push(rust_struct);
    }

    results
}

/// Tries to transpile a header file completely via deterministic AST (<5ms).
/// Returns Some(rust_code) if definitions were parsed cleanly, or None if neural LLM is required.
pub fn try_ast_fast_path(header_content: &str) -> Option<String> {
    let enums = parse_c_enums(header_content);
    let structs = parse_c_structs(header_content);

    if enums.is_empty() && structs.is_empty() {
        return None;
    }

    let mut out = String::from("//! Mechanically generated via blender-cluster AST zero-token fast-path\n\n");
    out.push_str("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]\n\n");
    out.push_str("#[allow(unused_imports)]\nuse crate::*;\n\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype int32_t = i32;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype uint32_t = u32;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype int16_t = i16;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype uint16_t = u16;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype int64_t = i64;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype uint64_t = u64;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype int8_t = i8;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype uint8_t = u8;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype uchar = u8;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype ushort = u16;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype uint = u32;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype ulong = u64;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype int = i32;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype UString = String;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype PropertyFlag = u32;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype PropertyOverrideFlag = u32;\n");
    out.push_str("#[allow(non_camel_case_types)]\ntype ParameterFlag = u32;\n\n");


    for e in enums {
        out.push_str(&e);
        out.push('\n');
    }
    for s in structs {
        out.push_str(&s);
        out.push('\n');
    }

    Some(out)
}
