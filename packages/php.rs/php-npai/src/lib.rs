//! PHP Node-API export library.
//!
//! This crate does three things only: declare N-API objects, move arguments across
//! the bridge, and map pure Rust results from Boost `language.php` back to the host.
//! The parser and language semantics belong to Boost `data.text.php`; do not add a
//! parsing or highlighting bypass here.

#![deny(missing_docs)]

use data_text_core::{
    parser::session::ParseSession,
    tree::{GreenNode, GreenTree},
};
use data_text_php::{PhpLanguage, PhpLexer, PhpParser};
use language_php::{
    PhpHighlighter,
    lsp::highlighter::{HighlightKind, Highlighter},
};
use napi_derive::napi;

/// Kind and byte span of a lexical or syntax leaf.
#[napi(object)]
pub struct TokenSpan {
    /// Leaf kind name (the debug name of `PhpTokenType`).
    pub kind: String,
    /// Start byte offset.
    pub start: u32,
    /// End byte offset (exclusive).
    pub end: u32,
}

/// Syntax tree node exposing only kind and byte span.
#[napi(object)]
pub struct SyntaxNode {
    /// Node kind name (the debug name of `PhpElementType` or `PhpTokenType`).
    pub kind: String,
    /// Start byte offset.
    pub start: u32,
    /// End byte offset (exclusive).
    pub end: u32,
    /// Child nodes; empty for leaves.
    pub children: Vec<SyntaxNode>,
}

/// Parse result.
#[napi(object)]
pub struct ParseOutcome {
    /// Whether a syntax tree was produced.
    pub ok: bool,
    /// Fatal error (present only when `ok = false`).
    pub error: Option<String>,
    /// Non-fatal diagnostics.
    pub diagnostics: Vec<String>,
    /// Syntax tree root.
    pub root: Option<SyntaxNode>,
    /// Number of leaves.
    pub token_count: u32,
}

/// Highlight span.
#[napi(object)]
pub struct HighlightSpan {
    /// Start byte offset.
    pub start: u32,
    /// End byte offset (exclusive).
    pub end: u32,
    /// Highlight kind: `keyword` / `string` / `number` / `comment` / `identifier`.
    pub kind: String,
}

struct Parsed {
    root: SyntaxNode,
    tokens: Vec<TokenSpan>,
    diagnostics: Vec<String>,
}

fn parse_source(source: &str) -> Result<Parsed, (String, Vec<String>)> {
    let language = PhpLanguage::default();
    let parser = PhpParser::new(&language);
    let lexer = PhpLexer::new(&language);
    let mut cache = ParseSession::<PhpLanguage>::default();
    let output = data_text_core::parser::parse(&parser, &lexer, source, &[], &mut cache);
    let diagnostics = output.diagnostics.iter().map(|error| format!("{error:?}")).collect::<Vec<_>>();

    match output.result {
        Ok(green) => {
            let mut tokens = Vec::new();
            let root = to_syntax_node(green, 0, &mut tokens);
            Ok(Parsed { root, tokens, diagnostics })
        }
        Err(error) => Err((format!("{error:?}"), diagnostics)),
    }
}

fn to_syntax_node(node: &GreenNode<'_, PhpLanguage>, start: u32, tokens: &mut Vec<TokenSpan>) -> SyntaxNode {
    let mut cursor = start;
    let mut children = Vec::with_capacity(node.children.len());

    for child in node.children {
        match child {
            GreenTree::Node(child) => {
                children.push(to_syntax_node(child, cursor, tokens));
                cursor += child.byte_length;
            }
            GreenTree::Leaf(leaf) => {
                let end = cursor + leaf.length;
                let kind = format!("{:?}", leaf.kind);
                tokens.push(TokenSpan { kind: kind.clone(), start: cursor, end });
                children.push(SyntaxNode { kind, start: cursor, end, children: Vec::new() });
                cursor = end;
            }
        }
    }

    SyntaxNode { kind: format!("{:?}", node.kind), start, end: start + node.byte_length, children }
}

fn highlight_kind(kind: HighlightKind) -> &'static str {
    match kind {
        HighlightKind::Keyword => "keyword",
        HighlightKind::String => "string",
        HighlightKind::Number => "number",
        HighlightKind::Comment => "comment",
        HighlightKind::Identifier => "identifier",
    }
}

/// npm package version (published with the crate version).
#[napi]
pub fn php_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Parse PHP source and return the syntax tree.
#[napi]
pub fn php_parse(source: String) -> ParseOutcome {
    match parse_source(&source) {
        Ok(parsed) => ParseOutcome {
            ok: true,
            error: None,
            diagnostics: parsed.diagnostics,
            token_count: parsed.tokens.len() as u32,
            root: Some(parsed.root),
        },
        Err((error, diagnostics)) => ParseOutcome { ok: false, error: Some(error), diagnostics, root: None, token_count: 0 },
    }
}

/// Lexical leaves in source order.
#[napi]
pub fn php_tokens(source: String) -> Vec<TokenSpan> {
    parse_source(&source).map(|parsed| parsed.tokens).unwrap_or_default()
}

/// Diagnostics with the fatal error first.
#[napi]
pub fn php_check(source: String) -> Vec<String> {
    match parse_source(&source) {
        Ok(parsed) => parsed.diagnostics,
        Err((error, diagnostics)) => {
            let mut all = Vec::with_capacity(diagnostics.len() + 1);
            all.push(error);
            all.extend(diagnostics);
            all
        }
    }
}

/// PHP highlighting backed by Boost `language.php`.
#[napi]
pub fn php_highlight(source: String) -> Vec<HighlightSpan> {
    let highlighter = PhpHighlighter::default();
    highlighter
        .highlight(&source)
        .into_iter()
        .map(|(start, end, kind)| HighlightSpan {
            start: start as u32,
            end: end as u32,
            kind: highlight_kind(kind).to_string(),
        })
        .collect()
}
