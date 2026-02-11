mod cli;

use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Gemini API request structures
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

// Gemini API response structures
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

// Gemini API error response
#[derive(Deserialize)]
struct GeminiErrorResponse {
    error: GeminiError,
}

#[derive(Deserialize)]
struct GeminiError {
    code: u16,
    message: String,
    status: String,
}

async fn call_gemini_api(message: &str) -> Result<String, String> {
    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY not found in environment variables".to_string())?;

    // Try different models - Use actual available models from API
    // Models that support generateContent: gemini-2.5-flash, gemini-flash-latest, gemini-pro-latest, etc.
    let models = ["gemini-2.5-flash", "gemini-flash-latest", "gemini-pro-latest", "gemini-2.0-flash"];
    let api_versions = ["v1beta", "v1"];

    // Create HTTP client with timeout
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: message.to_string(),
            }],
        }],
    };

    // Try different API versions and models
    for api_version in &api_versions {
        for model in &models {
            let url = format!(
                "https://generativelanguage.googleapis.com/{}/models/{}:generateContent?key={}",
                api_version, model, api_key
            );

            let response = match client
                .post(&url)
                .json(&request_body)
                .send()
                .await
            {
                Ok(resp) => {
                    eprintln!("Trying: {} (model: {})", api_version, model);
                    resp
                }
                Err(e) => {
                    eprintln!("Failed to send request to {}: {}", url, e);
                    continue;
                }
            };

            let status = response.status();
            let status_code = status.as_u16();

            if status.is_success() {
                match response.json::<GeminiResponse>().await {
                    Ok(gemini_response) => {
                        if let Some(text) = gemini_response
                            .candidates
                            .first()
                            .and_then(|c| c.content.parts.first())
                            .map(|p| p.text.clone())
                        {
                            eprintln!("Success with {} / {}", api_version, model);
                            return Ok(text);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse response from {}: {}", url, e);
                        continue;
                    }
                }
            } else {
                // Read response text first (can only consume once)
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

                // Try to parse as structured error
                if let Ok(error_response) = serde_json::from_str::<GeminiErrorResponse>(&error_text) {
                    eprintln!(
                        "API error from {} / {}: {} ({}): {}",
                        api_version, model, error_response.error.status, error_response.error.code, error_response.error.message
                    );
                } else {
                    eprintln!("HTTP {} from {} / {}: {}", status_code, api_version, model, error_text);
                }
                // Continue to next model/version
                continue;
            }
        }
    }

    Err("Failed to get response from Gemini API. Please check your API key and model availability.".to_string())
}

async fn chat_handler(Json(payload): Json<ChatRequest>) -> Result<ResponseJson<ChatResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    if payload.message.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ErrorResponse {
                error: "Message cannot be empty".to_string(),
            }),
        ));
    }

    // Validate message length
    if payload.message.len() > 10000 {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ErrorResponse {
                error: "Message is too long (max 10000 characters)".to_string(),
            }),
        ));
    }

    match call_gemini_api(&payload.message).await {
        Ok(response) => Ok(ResponseJson(ChatResponse { response })),
        Err(e) => {
            eprintln!("Error calling Gemini API: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse { error: e }),
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Check for CLI mode
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "chat" || args[1] == "cli" || args[1] == "--chat" || args[1] == "--cli") {
        cli::run_interactive_chat().await;
        return;
    }

    // Verify API key is set
    if env::var("GEMINI_API_KEY").is_err() {
        eprintln!("Warning: GEMINI_API_KEY not found in environment variables");
        eprintln!("   Please create a .env file with your API key");
    }

    // Health check endpoint
    async fn health() -> &'static str {
        "OK"
    }

    // Build the application router
    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/chat", post(chat_handler));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000. Is another server running?");

    println!("🚀 Server running on http://localhost:3000");
    println!("📝 POST to http://localhost:3000/chat with {{ \"message\": \"your message\" }}");
    println!("💡 Health check: http://localhost:3000/health");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
