use anyhow::{Context, Result};
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, InitializeParams, MessageType, Url},
        Request,
    },
    register_plugin, LapcePlugin, PLUGIN_RPC,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Default)]
pub struct State;

register_plugin!(State);

#[derive(Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Options {
    pub path: Option<String>,
    pub args: Option<Vec<String>>,
}

fn initialize(params: InitializeParams) -> Result<()> {
    #[cfg(debug_assertions)]
    PLUGIN_RPC.window_show_message(
        MessageType::WARNING,
        format!(
            "Nushell LSP plugin init params: {:?}",
            params.initialization_options
        ),
    );

    let document_selector = vec![DocumentFilter {
        // TODO: if Nushell is implemented in Lapce, require it here, not in pattern
        language: None,
        scheme: None,
        pattern: Some("*.nu".to_owned()),
    }];

    let options: Options = params
        .initialization_options
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();

    let path = options.path.context("Nushell LSP path is not configured")?;
    let server_uri = Url::parse(&format!("urn:{}", path))?;
    let server_args = options.args.unwrap_or_default();

    #[cfg(debug_assertions)]
    {
        PLUGIN_RPC.window_show_message(
            MessageType::WARNING,
            format!("Nushell LSP debug server_uri: {:?}", server_uri),
        );
        PLUGIN_RPC.window_show_message(
            MessageType::WARNING,
            format!("Nushell LSP debug server_args: {:?}", server_args),
        );
    }

    PLUGIN_RPC.start_lsp(server_uri, server_args, document_selector, Some(json!({})));

    Ok(())
}

impl State {
    fn dispatch_request(&mut self, _id: u64, method: String, params: Value) -> Result<()> {
        match method.as_str() {
            Initialize::METHOD => {
                let params = serde_json::from_value(params)
                    .expect("initialize method should have `InitializeParams` params");
                initialize(params)
            }
            _ => Ok(()),
        }
    }
}

impl LapcePlugin for State {
    fn handle_request(&mut self, id: u64, method: String, params: Value) {
        if let Err(err) = self.dispatch_request(id, method, params) {
            PLUGIN_RPC.window_show_message(
                MessageType::ERROR,
                format!("Nushell LSP plugin error: {err:?}"),
            );
        }
    }
}
