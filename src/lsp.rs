use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;


use crate::parser;
use crate::analysis::determinism::{DeterminismAnalyzer, SymbolTable};
use crate::analysis::taint::TaintAnalyzer;
use crate::analysis::scope::ScopeAnalyzer;

#[derive(Debug)]
pub struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "Zet LSP başlatıldı!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: params.text_document.language_id,
        }).await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.content_changes.pop().map(|c| c.text).unwrap_or_default();
        self.on_change(TextDocumentItem {
            uri,
            text,
            version: params.text_document.version,
            language_id: "zet".to_string(),
        }).await
    }
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        let mut diagnostics = vec![];
        
        let content = params.text.trim_start_matches('\u{feff}');
        
        match parser::parse_program(content) {
            Ok((_, toplevels)) => {
                let mut all_functions = Vec::new();
                crate::extract_functions(&toplevels, &mut all_functions, Vec::new());
                
                let mut func_map = HashMap::new();
                for f in &all_functions {
                    func_map.insert(f.name.clone(), (*f).clone());
                }
                
                let symbols = SymbolTable { functions: func_map.clone() };
                
                for f in &all_functions {
                    if let Err(e) = DeterminismAnalyzer::check(f, &symbols) {
                        diagnostics.push(Diagnostic {
                            range: Range::default(), 
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: format!("Determinism Hatası: {}", e),
                            ..Default::default()
                        });
                    }
                    let mut scope_pass = ScopeAnalyzer::new();
                    if let Err(e) = scope_pass.analyze(f) {
                        diagnostics.push(Diagnostic {
                            range: Range::default(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: format!("Scope/Tip Hatası: {}", e),
                            ..Default::default()
                        });
                    }
                    if let Err(e) = TaintAnalyzer::check(f, &symbols) {
                        diagnostics.push(Diagnostic {
                            range: Range::default(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: format!("Zero Trust İhlali: {}", e),
                            ..Default::default()
                        });
                    }
                }
            },
            Err(e) => {
                diagnostics.push(Diagnostic {
                    range: Range::default(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Syntax Hatası:\n{:?}", e),
                    ..Default::default()
                });
            }
        }
        
        self.client.publish_diagnostics(params.uri, diagnostics, Some(params.version)).await;
    }
}

pub async fn run_lsp() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
