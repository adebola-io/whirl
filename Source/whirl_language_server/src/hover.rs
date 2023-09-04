use std::sync::MutexGuard;

use tower_lsp::lsp_types::{Hover, HoverContents, LanguageString, MarkedString};
use whirl_ast::{ASTVisitor, FunctionSignature};

/// Information shown during hovering.
pub struct HoverInfo {
    pub contents: HoverContents,
}

impl From<&str> for HoverInfo {
    fn from(value: &str) -> Self {
        HoverInfo {
            contents: HoverContents::Scalar(MarkedString::String(value.to_owned())),
        }
    }
}

impl From<MutexGuard<'_, FunctionSignature>> for HoverInfo {
    fn from(value: MutexGuard<'_, FunctionSignature>) -> Self {
        let mut info = vec![];

        // Construct function signature.
        let mut string = String::new();

        if value.is_async {
            string.push_str("async ");
        }
        string.push_str("function ");
        string.push_str(&value.name.name);
        string.push('(');

        for (index, parameter) in value.params.iter().enumerate() {
            string.push_str(&parameter.to_string());
            if index < value.params.len() - 1 {
                string.push_str(", ");
            }
        }

        string.push(')');

        info.push(MarkedString::LanguageString(LanguageString {
            language: String::from("wrl"),
            value: string,
        }));

        // Documentation?

        if let Some(ref docs) = value.info {
            let mut documentation = String::new();
            for line in docs {
                documentation.push_str(line);
                documentation.push('\n')
            }
            info.push(MarkedString::String(documentation))
        }

        HoverInfo {
            contents: HoverContents::Array(info),
        }
    }
}

impl From<HoverInfo> for Hover {
    fn from(value: HoverInfo) -> Self {
        Hover {
            contents: value.contents,
            range: None,
        }
    }
}

pub struct HoverFinder;

impl ASTVisitor<[u32; 2], Option<HoverInfo>> for HoverFinder {
    fn visit_function(
        &self,
        function: &whirl_ast::FunctionDeclaration,
        args: &[u32; 2],
    ) -> Option<HoverInfo> {
        let signature = function.signature.lock().unwrap();
        let body = &function.body;

        // Hovering over the function name.
        if signature.name.span.contains(*args) {
            return Some(HoverInfo::from(signature));
        }
        // Hovering over something in the function's body.
        if !body.span.contains(*args) {
            return None;
        }
        for statement in &body.statements {
            let hover_info = self.visit_statement(statement, args);
            if hover_info.is_some() {
                return hover_info;
            }
        }
        return None;
    }
}
