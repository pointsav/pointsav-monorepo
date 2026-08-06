// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct JsonRpcRequest {
    #[allow(dead_code)]
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    // Fable-audit finding (2026-08-02): this was `pub id: Value` with no default,
    // making the field mandatory. A JSON-RPC *notification* (e.g. the
    // "notifications/initialized" message every real MCP client sends right after
    // `initialize`) legally omits `id` entirely -- deserializing one failed here,
    // and the handler replied with a -32700 "Parse error", which is both the wrong
    // error (this wasn't malformed JSON) and a spec violation (notifications must
    // get no response at all). `#[serde(default)]` lets the field be absent;
    // `mcp_handler` checks for `None` to suppress the response body entirely.
    #[serde(default)]
    pub id: Option<Value>,
}

#[derive(Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Value,
}

#[derive(Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}
