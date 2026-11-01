// SPDX-License-Identifier: Apache-2.0

use crate::business::Business;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::http::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
    pub id: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

pub async fn handler(state: AppState, body: &str) -> impl IntoResponse {
    match serde_json::from_str::<JsonRpcRequest>(body) {
        Ok(request) => {
            let response = dispatch(&state, &request).await;
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError { code: -32700, message: "Parse error".to_string(), data: None }),
                id: None,
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

async fn dispatch(state: &AppState, request: &JsonRpcRequest) -> JsonRpcResponse {
    match request.method.as_str() {
        "initialize" => initialize_handler(request),
        "tools/list" => tools_list_handler(request),
        "tools/call" => tools_call_handler(state, request).await,
        "resources/list" => resources_list_handler(request),
        _ => error_response(-32601, &format!("Method not found: {}", request.method), request.id.clone()),
    }
}

fn initialize_handler(request: &JsonRpcRequest) -> JsonRpcResponse {
    let result = json!({
        "protocolVersion": "2024-01-28",
        "capabilities": { "tools": {}, "resources": {} },
        "serverInfo": { "name": "service-business", "version": "0.1.0" }
    });
    JsonRpcResponse { jsonrpc: "2.0".to_string(), result: Some(result), error: None, id: request.id.clone() }
}

fn tools_list_handler(request: &JsonRpcRequest) -> JsonRpcResponse {
    let tools = json!({
        "tools": [
            {
                "name": "business.append",
                "description": "Append a business record",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "category": { "type": "string" },
                        "place_id": { "type": ["string", "null"], "description": "UUID of a service-places Place" },
                        "registration_number": { "type": ["string", "null"] }
                    },
                    "required": ["name", "category"]
                }
            },
            {
                "name": "business.lookup",
                "description": "Look up a business by UUID or exact name",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query_type": { "type": "string", "enum": ["uuid", "name"] },
                        "value": { "type": "string" }
                    },
                    "required": ["query_type", "value"]
                }
            },
            {
                "name": "business.list_by_place",
                "description": "List all businesses occupying a given service-places Place",
                "inputSchema": {
                    "type": "object",
                    "properties": { "place_id": { "type": "string" } },
                    "required": ["place_id"]
                }
            }
        ]
    });
    JsonRpcResponse { jsonrpc: "2.0".to_string(), result: Some(tools), error: None, id: request.id.clone() }
}

async fn tools_call_handler(state: &AppState, request: &JsonRpcRequest) -> JsonRpcResponse {
    let params = match &request.params {
        Some(Value::Object(obj)) => obj,
        _ => return error_response(-32602, "Invalid params: expected object", request.id.clone()),
    };
    let tool_name = match params.get("name").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return error_response(-32602, "Missing tool name", request.id.clone()),
    };
    let arguments = params.get("arguments").cloned().unwrap_or(Value::Object(Default::default()));

    match tool_name {
        "business.append" => append_tool_handler(state, &arguments, request).await,
        "business.lookup" => lookup_tool_handler(state, &arguments, request).await,
        "business.list_by_place" => list_by_place_tool_handler(state, &arguments, request).await,
        _ => error_response(-32602, &format!("Unknown tool: {}", tool_name), request.id.clone()),
    }
}

async fn append_tool_handler(state: &AppState, arguments: &Value, request: &JsonRpcRequest) -> JsonRpcResponse {
    let args = match arguments.as_object() {
        Some(obj) => obj,
        None => return error_response(-32602, "Invalid arguments", request.id.clone()),
    };
    let name = match args.get("name").and_then(|v| v.as_str()) {
        Some(n) => n,
        None => return error_response(-32602, "Missing 'name' argument", request.id.clone()),
    };
    let category = match args.get("category").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return error_response(-32602, "Missing 'category' argument", request.id.clone()),
    };
    let place_id = match args.get("place_id") {
        Some(Value::String(s)) => match Uuid::parse_str(s) {
            Ok(id) => Some(id),
            Err(_) => return error_response(-32602, "Invalid 'place_id' UUID format", request.id.clone()),
        },
        _ => None,
    };

    let mut business = Business::new(name, category, place_id);
    if let Some(Value::String(reg)) = args.get("registration_number") {
        business = business.with_registration_number(reg);
    }

    match state.ledger_backing.append_business(&business) {
        Ok(cursor) => {
            let _ = state.business_store.append(business.clone());
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!({ "cursor": cursor, "business_id": business.id.to_string() })),
                error: None,
                id: request.id.clone(),
            }
        }
        Err(e) => error_response(-32603, &format!("Failed to append to ledger: {}", e), request.id.clone()),
    }
}

async fn lookup_tool_handler(state: &AppState, arguments: &Value, request: &JsonRpcRequest) -> JsonRpcResponse {
    let args = match arguments.as_object() {
        Some(obj) => obj,
        None => return error_response(-32602, "Invalid arguments", request.id.clone()),
    };
    let query_type = match args.get("query_type").and_then(|v| v.as_str()) {
        Some(qt) => qt,
        None => return error_response(-32602, "Missing 'query_type' argument", request.id.clone()),
    };
    let value = match args.get("value").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => return error_response(-32602, "Missing 'value' argument", request.id.clone()),
    };

    let result = match query_type {
        "uuid" => match Uuid::parse_str(value) {
            Ok(id) => state.business_store.lookup_by_id(id),
            Err(_) => return error_response(-32602, "Invalid UUID format", request.id.clone()),
        },
        "name" => state.business_store.lookup_by_name(value),
        _ => return error_response(-32602, "Invalid query_type (must be 'uuid' or 'name')", request.id.clone()),
    };

    match result {
        Ok(business) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::to_value(&business).unwrap_or(Value::Null)),
            error: None,
            id: request.id.clone(),
        },
        Err(e) => error_response(-32603, &format!("Business not found: {}", e), request.id.clone()),
    }
}

async fn list_by_place_tool_handler(state: &AppState, arguments: &Value, request: &JsonRpcRequest) -> JsonRpcResponse {
    let args = match arguments.as_object() {
        Some(obj) => obj,
        None => return error_response(-32602, "Invalid arguments", request.id.clone()),
    };
    let place_id = match args.get("place_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()) {
        Some(id) => id,
        None => return error_response(-32602, "Missing or invalid 'place_id' argument", request.id.clone()),
    };

    let businesses = state.business_store.list_by_place(place_id);
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(json!({ "businesses": businesses })),
        error: None,
        id: request.id.clone(),
    }
}

fn resources_list_handler(request: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse { jsonrpc: "2.0".to_string(), result: Some(json!({ "resources": [] })), error: None, id: request.id.clone() }
}

fn error_response(code: i32, message: &str, id: Option<Value>) -> JsonRpcResponse {
    JsonRpcResponse { jsonrpc: "2.0".to_string(), result: None, error: Some(JsonRpcError { code, message: message.to_string(), data: None }), id }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_returns_protocol_version() {
        let request = JsonRpcRequest { jsonrpc: "2.0".to_string(), method: "initialize".to_string(), params: None, id: Some(Value::Number(1.into())) };
        let response = initialize_handler(&request);
        assert!(response.result.unwrap().get("protocolVersion").is_some());
    }

    #[test]
    fn tools_list_includes_all_three_tools() {
        let request = JsonRpcRequest { jsonrpc: "2.0".to_string(), method: "tools/list".to_string(), params: None, id: Some(Value::Number(1.into())) };
        let response = tools_list_handler(&request);
        let tools = response.result.unwrap();
        assert_eq!(tools.get("tools").unwrap().as_array().unwrap().len(), 3);
    }

    #[test]
    fn resources_list_returns_empty_array() {
        let request = JsonRpcRequest { jsonrpc: "2.0".to_string(), method: "resources/list".to_string(), params: None, id: Some(Value::Number(1.into())) };
        let response = resources_list_handler(&request);
        assert_eq!(response.result.unwrap().get("resources").unwrap().as_array().unwrap().len(), 0);
    }

    #[test]
    fn unknown_method_returns_error() {
        let response = error_response(-32601, "Method not found: bogus", Some(Value::Number(1.into())));
        assert_eq!(response.error.unwrap().code, -32601);
    }
}
