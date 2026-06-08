//! USS Language Server - Main Entry Point
//!
//! A Language Server Protocol implementation for Unity Style Sheets (USS)
//! providing completion, diagnostics, hover, and formatting support.

mod completion;
mod diagnostics;
mod document;
mod hover;
mod uss_data;

use dashmap::DashMap;
use document::Document;
use log::info;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// USS Language Server backend
pub struct UssLanguageServer {
    /// LSP client for sending notifications
    client: Client,
    /// Open documents indexed by URI
    documents: DashMap<String, Document>,
}

impl UssLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for UssLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("USS Language Server initializing...");

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![
                        ".".to_string(),
                        "#".to_string(),
                        ":".to_string(),
                        "-".to_string(),
                        "/".to_string(),
                        "(".to_string(),
                    ]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                color_provider: Some(ColorProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "uss-language-server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("USS Language Server initialized!");
        self.client
            .log_message(MessageType::INFO, "USS Language Server ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("USS Language Server shutting down...");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        let version = params.text_document.version;

        let doc = Document::new(text, version);
        self.documents.insert(uri.clone(), doc);

        // Publish diagnostics for the opened document
        self.publish_diagnostics(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if let Some(mut doc) = self.documents.get_mut(&uri) {
            for change in params.content_changes {
                if let Some(range) = change.range {
                    doc.apply_change(range, &change.text);
                } else {
                    // Full document sync
                    doc.set_content(change.text);
                }
            }
            doc.version = params.text_document.version;
        }

        // Publish diagnostics for the changed document
        self.publish_diagnostics(&uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.documents.remove(&uri);

        // Clear diagnostics
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.publish_diagnostics(&uri).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;

        if let Some(doc) = self.documents.get(&uri) {
            let completions = completion::get_completions(&doc, position);
            return Ok(Some(CompletionResponse::Array(completions)));
        }

        Ok(None)
    }

    async fn completion_resolve(&self, item: CompletionItem) -> Result<CompletionItem> {
        // Add additional documentation for resolved completions
        Ok(completion::resolve_completion(item))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(&uri) {
            return Ok(hover::get_hover(&doc, position));
        }

        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.to_string();

        if let Some(doc) = self.documents.get(&uri) {
            let formatted = document::format_document(&doc, &params.options);
            return Ok(Some(formatted));
        }

        Ok(None)
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.to_string();

        if let Some(doc) = self.documents.get(&uri) {
            let formatted = document::format_range(&doc, params.range, &params.options);
            return Ok(Some(formatted));
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(&uri) {
            // Find variable definition (USS custom properties)
            if let Some(location) = document::find_definition(&doc, position, &uri) {
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }

        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;

        if let Some(doc) = self.documents.get(&uri) {
            let refs = document::find_references(&doc, position, &uri);
            if !refs.is_empty() {
                return Ok(Some(refs));
            }
        }

        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        if let Some(doc) = self.documents.get(&uri) {
            if let Some(edit) = document::rename(&doc, position, &new_name, &uri) {
                return Ok(Some(edit));
            }
        }

        Ok(None)
    }

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        let uri = params.text_document.uri.to_string();

        if let Some(doc) = self.documents.get(&uri) {
            return Ok(document::get_colors(&doc));
        }

        Ok(vec![])
    }

    async fn color_presentation(
        &self,
        params: ColorPresentationParams,
    ) -> Result<Vec<ColorPresentation>> {
        let color = params.color;
        Ok(document::get_color_presentations(color))
    }
}

impl UssLanguageServer {
    async fn publish_diagnostics(&self, uri: &str) {
        if let Some(doc) = self.documents.get(uri) {
            let diagnostics = diagnostics::get_diagnostics(&doc);

            if let Ok(url) = uri.parse() {
                self.client
                    .publish_diagnostics(url, diagnostics, Some(doc.version))
                    .await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(UssLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
