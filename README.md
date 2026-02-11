# 🤖 Rust AI Chatbot

A **proof-of-concept (POC)** AI chatbot built with Rust that integrates with Google's Gemini API. The application supports both **HTTP server mode** and **interactive CLI mode** for seamless AI-powered conversations.

## ✨ Features

- 🚀 **Dual Mode Operation**: Run as an HTTP server or interactive CLI
- 🤖 **Gemini API Integration**: Powered by Google's Gemini AI models
- ⚡ **Fast & Efficient**: Built with Rust for maximum performance
- 🔄 **Automatic Fallback**: Tries multiple API versions and models for reliability
- 🛡️ **Input Validation**: Message length limits and error handling
- 🌐 **RESTful API**: Easy-to-use HTTP endpoints for integration

## �️ Technology Stack

- **Language**: Rust (Edition 2021)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) - Modern, ergonomic web framework
- **Async Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
- **HTTP Client**: [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for API calls
- **Serialization**: [Serde](https://serde.rs/) - JSON serialization/deserialization
- **AI Integration**: Google Gemini API (v1/v1beta)

## �📋 Prerequisites

- **Rust** (1.70 or later) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Google Gemini API Key** - [Get API Key](https://makersuite.google.com/app/apikey)

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd rust-ai-chatbot
```

### 2. Set Up Environment Variables

Create a `.env` file in the project root:

```env
GEMINI_API_KEY=your_api_key_here
```

### 3. Install Dependencies

```bash
cargo build
```

### 4. Run the Application

#### Server Mode (HTTP API)

```bash
cargo run
```

The server will start on `http://localhost:3000`

#### CLI Mode (Interactive Chat)

```bash
cargo run chat
```

Or use alternative commands:
```bash
cargo run cli
cargo run --chat
cargo run --cli
```

## 📡 API Endpoints

### Health Check
```bash
GET http://localhost:3000/health
```

Response: `OK`

### Chat Endpoint
```bash
POST http://localhost:3000/chat
Content-Type: application/json

{
  "message": "Hello, how are you?"
}
```

Response:
```json
{
  "response": "I'm doing great! How can I help you today?"
}
```

### Example with cURL

```bash
curl -X POST http://localhost:3000/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "What is Rust programming language?"}'
```

## 🎯 Usage Examples

### Server Mode
```bash
# Terminal 1: Start the server
cargo run

# Terminal 2: Send requests
curl -X POST http://localhost:3000/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Explain quantum computing"}'
```

### CLI Mode
```bash
cargo run chat

# Interactive session
You: What is machine learning?
Bot: Machine learning is a subset of artificial intelligence...

You: exit
👋 Goodbye!
```

## 🎬 Video Demo

> **Note**: To add a video demo to this README:
> 
> 1. **Record your demo** using screen recording software (OBS, QuickTime, etc.)
> 2. **Upload the video** to a platform like:
>    - YouTube
>    - GitHub (add to repository as a release asset)
>    - Cloud storage (Google Drive, Dropbox, etc.)
> 3. **Embed the video** using one of these methods:

### Option 1: Video Link (YouTube)
```markdown
[![Video Demo](https://img.youtube.com/vi/YOUR_VIDEO_ID/maxresdefault.jpg)](https://www.youtube.com/watch?v=YOUR_VIDEO_ID)
```

### Option 2: GIF Recording
```markdown
![Demo](path/to/demo.gif)
```

### Option 3: Direct Video File (GitHub)
```markdown
https://user-images.githubusercontent.com/YOUR_USERNAME/video.mp4
```

## 🏗️ Project Structure

```
rust-ai-chatbot/
├── src/
│   ├── main.rs          # Server mode & API handlers
│   └── cli.rs           # Interactive CLI mode
├── Cargo.toml           # Project dependencies
├── .env                 # Environment variables (not committed)
├── .gitignore          
└── README.md
```

## 🔧 Configuration

### Supported Gemini Models
The application automatically tries the following models:
- `gemini-2.5-flash`
- `gemini-flash-latest`
- `gemini-pro-latest`
- `gemini-2.0-flash`

### API Versions
- `v1beta`
- `v1`

### Message Limits
- Maximum message length: **10,000 characters**
- Empty messages are rejected

## 🛠️ Built With

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [Serde](https://serde.rs/) - Serialization/deserialization
- [dotenv](https://github.com/dotenv-rs/dotenv) - Environment variable management

## 🚨 Error Handling

The application includes comprehensive error handling:
- ✅ API key validation
- ✅ Network timeout (30 seconds)
- ✅ Multiple model fallback
- ✅ Detailed error messages
- ✅ Input validation

## 📝 Development

### Run in Debug Mode
```bash
cargo run
```

### Build for Production
```bash
cargo build --release
./target/release/rust-ai-chatbot
```

### Run Tests
```bash
cargo test
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is a **proof-of-concept (POC)** and is provided as-is for educational purposes.

## 🙏 Acknowledgments

- Google Gemini API for AI capabilities
- Rust community for excellent libraries and tools

## 📧 Contact

For questions or feedback, please open an issue in the repository.

---

**Made with ❤️ using Rust and Gemini AI**
