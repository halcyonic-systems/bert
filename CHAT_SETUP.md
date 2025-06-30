# BERT LLM Chat Integration

## Overview
This document describes the LLM chat integration feature for the BERT application, which allows users to interact with loaded system models through natural language queries.

## Architecture

### Frontend Components
- **File**: `src/leptos_app/components/chat.rs`
- **Purpose**: Reactive chat UI component built with Leptos
- **Key Features**:
  - Environment detection (Tauri desktop vs web browser)
  - Automatic model context loading
  - Real-time message display
  - Proper async state management

### Backend Service
- **File**: `src-tauri/src/chat_service.rs`
- **Purpose**: LLM provider abstraction and model management
- **Key Features**:
  - Multiple LLM provider support (Ollama, OpenAI, Mock)
  - Automatic model data extraction and context management
  - Enhanced factual response generation
  - Graceful fallback handling

### Integration Points
- **File**: `src-tauri/src/lib.rs`
- **Tauri Commands**:
  - `chat_with_model(message, context)` - Main chat endpoint
  - `get_current_model()` - Retrieve loaded model data
  - `update_current_model(data)` - Store model data for chat context
- **File**: `src-tauri/src/data_model/load.rs`
  - Auto-detection and storage of JSON model data during file loading

## LLM Provider Support

### 1. Local LLM (Ollama) - Default & Recommended
```toml
# In src-tauri/Cargo.toml
ollama-rs = "0.2"
tokio = "1.0"
```

**Setup Instructions**:
1. Install Ollama: `curl -fsSL https://ollama.ai/install.sh | sh`
2. Pull model: `ollama pull llama3.2:3b`
3. Start Ollama service: `ollama serve`
4. Build with local LLM support: `cargo tauri dev --features local-llm`

**Benefits**:
- Complete privacy (no data sent to cloud)
- No API costs
- Works offline
- Customizable system prompts

### 2. Cloud API (OpenAI) - Optional
```toml
# In src-tauri/Cargo.toml (when enabled)
async-openai = "0.28"
```

**Setup Instructions**:
1. Get OpenAI API key
2. Set environment variable: `export OPENAI_API_KEY="your-key"`
3. Build with cloud API: `cargo tauri dev --features cloud-api`

**Benefits**:
- Instant responses
- High-quality analysis
- No local compute required

### 3. Mock Provider - Fallback
- Provides enhanced sample responses when no LLM is available
- Includes Bitcoin-specific examples
- Used for development/testing

## Implementation History & Fixes

### Initial Implementation Challenges
1. **Thread Safety Issues**: Fixed mutex handling in async contexts
2. **Leptos API Compatibility**: Updated from deprecated `create_signal()` to `signal()`
3. **WebAssembly Constraints**: Changed from `Action::new` to `Action::new_unsync`, then to direct `spawn_local`
4. **Environment Detection**: Added `is_tauri_environment()` for desktop vs web compatibility

### Model Context Integration
- **Problem**: Chat was using hardcoded sample data instead of loaded JSON models
- **Solution**: 
  - Modified `load_file` command to auto-detect and store JSON model data
  - Added `get_current_model()` / `update_current_model()` commands
  - Integrated model data extraction into chat context

### Response Quality Improvements
- **Problem**: LLM responses were interpretive ("appears to be", "seems like")
- **Solution**: 
  - Enhanced system prompts with explicit factual reporting requirements
  - Banned interpretive language in responses
  - Required structured **System Facts** format
  - Added examples of good vs bad responses

### Async State Management
- **Problem**: Chat UI would hang on "..." while processing
- **Solution**:
  - Replaced complex Action pattern with direct `spawn_local`
  - Improved error handling in async operations
  - Added proper state updates during processing

## Build Configuration

### Feature Flags
```toml
# Default: Local LLM only
cargo tauri dev

# With cloud API support
cargo tauri dev --features cloud-api

# Local LLM specifically
cargo tauri dev --features local-llm
```

### Dependencies
```toml
[dependencies]
# Core async runtime
tokio = { version = "1.0", features = ["full"] }

# Local LLM (Ollama)
ollama-rs = { version = "0.2", optional = true }

# Cloud APIs
async-openai = { version = "0.28", optional = true }

# JSON handling
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }

[features]
default = ["local-llm"]
local-llm = ["dep:ollama-rs"]
cloud-api = ["dep:async-openai"]
```

## Current Status

### ✅ Working Features
- Chat UI with proper async handling
- Environment detection (desktop vs web)
- Automatic model context loading from JSON files
- Ollama integration with llama3.2:3b model
- OpenAI API integration (when enabled)
- Enhanced factual response generation
- Structured system analysis output
- Graceful fallback to mock responses

### ⚠️ Known Issues & Areas for Improvement

#### Response Quality Issues (January 2025)
**Problem**: Despite enhanced system prompts, LLM responses still contain interpretive language:
- Still uses: "appears to be", "seems like", "overall this system appears to be"
- Not following the structured **System Facts** format consistently
- Providing interpretive analysis instead of factual data extraction

**Example Current Response**:
```
"Based on the provided data, it appears to be a complex system for managing and maintaining the Bitcoin blockchain..."
```

**Target Response Format**:
```
**System Facts:**
• **Name**: Bitcoin Network
• **Subsystems**: 4 total
• **Interactions**: 12 total
• **Components**: Protocol, Validating, Mining, Development
• **Flows**: F0.0 Protocol Rules & Parameters, F0.1 Mempool Transactions, etc.
```

#### Root Causes Identified:
1. **System Prompt Effectiveness**: Current prompt may not be strong enough to override LLM's natural interpretive tendencies
2. **Model Training Bias**: llama3.2:3b may be inherently trained to provide interpretive responses
3. **Context Processing**: LLM may not be properly parsing the structured format requirements
4. **Prompt Engineering**: May need more aggressive prompt techniques (few-shot examples, stronger constraints)

#### Targeted Improvements Needed:
1. **Stronger Prompt Engineering**:
   - Add few-shot examples showing exact desired vs undesired responses
   - Use more aggressive language constraints
   - Implement response validation/filtering

2. **Response Post-Processing**:
   - Add automatic detection of interpretive language
   - Implement response rewriting to remove banned phrases
   - Force structured format compliance

3. **Alternative Model Testing**:
   - Test with different local models (llama3.1, mistral, etc.)
   - Compare cloud API responses (OpenAI GPT-4, Claude)
   - Evaluate which models better follow structured instructions

4. **Enhanced Context Extraction**:
   - Improve JSON parsing and summarization
   - Provide more explicit data structure to LLM
   - Pre-format data in the exact output structure desired

### 🔧 Performance Notes
- **2019 MacBook Pro**: 5-15 tokens/second with local LLM
- **Future Apple Silicon**: Expected 50-150+ tokens/second
- **Cloud APIs**: Instant responses but require internet + API costs

### 📋 Response Format
The chat now provides structured, factual analysis:

```
**System Facts:**
• **Name**: Bitcoin Network
• **Subsystems**: 4 total
• **Interactions**: 12 total
• **Components**: Protocol, Validating, Mining, Network Distribution
• **Flows**: block_subsidy, network_difficulty, utxo_set_hash, etc.

**Subsystem Analysis:**
[Exact subsystem names and properties from JSON]

**Interaction Analysis:**
[Exact interaction flows with source→sink mappings]
```

## Usage Examples

### Basic Chat Commands
- "What is this system?" - Get system overview
- "List all components" - Show subsystems and their properties  
- "Show interactions" - Display flows between components
- "Explain the mining process" - Get Bitcoin-specific analysis

### Model Context
The chat automatically uses the currently loaded JSON model data. When you load a `.btc`, `.json`, or other system model file, the chat context updates automatically.

## Troubleshooting

### Ollama Issues
- **Connection Failed**: Ensure `ollama serve` is running
- **Model Not Found**: Run `ollama pull llama3.2:3b`
- **Port Conflicts**: Check if port 11434 is available

### Build Issues
- **Feature Conflicts**: Use only one feature flag at a time
- **Dependency Errors**: Clear `target/` and rebuild
- **Port 1320 Occupied**: Kill existing processes with `lsof -ti:1320 | xargs kill -9`

### Chat UI Issues
- **Hanging on "..."**: Check browser console for errors
- **No Response**: Verify model context is loaded
- **Mock Responses Only**: Check if Ollama is running and model exists

## Future Enhancements

### Immediate Priority (Next Development Session)
1. **Fix Factual Response Issues**:
   - Implement few-shot prompting with explicit good/bad examples
   - Add response post-processing to filter interpretive language
   - Test alternative models for better instruction following
   - Strengthen system prompts with more aggressive constraints

2. **Response Validation**:
   - Automatic detection of banned phrases ("appears to be", "seems like")
   - Force structured format compliance before returning responses
   - Implement response quality scoring

### Medium-Term Improvements
1. **Structured Analysis Frameworks**
   - Leverage points analysis
   - Stock & flow diagrams
   - Systems archetypes identification

2. **Visual Integration**
   - Highlight discussed components in the visual model
   - Generate system diagrams from chat insights
   - Interactive exploration with guided questions

3. **Domain-Specific Intelligence**
   - Bitcoin/cryptocurrency expertise
   - Systems thinking methodologies
   - Comparative analysis capabilities

4. **Performance Optimizations**
   - Model caching for faster responses
   - Streaming responses for real-time feedback
   - Context compression for large models

### Development Notes for Next Session
- **Current Issue**: LLM not following structured format despite enhanced prompts
- **Test Data**: Use Bitcoin model with Protocol, Validating, Mining, Development subsystems
- **Success Criteria**: Responses must start with "**System Facts:**" format and avoid all interpretive language
- **Quick Wins**: Try OpenAI API to compare response quality vs local llama3.2:3b

## Contributing

When working on the chat feature:

1. **Test Both Environments**: Desktop (Tauri) and web browser
2. **Check Model Context**: Ensure loaded JSON data is properly passed
3. **Verify Response Quality**: Responses should be factual, not interpretive
4. **Handle Errors Gracefully**: Provide helpful fallbacks when LLMs are unavailable
5. **Update Documentation**: Keep this file current with any changes

## Dependencies & Versions

- **Leptos**: Frontend framework for reactive UI
- **Tauri**: Desktop app framework for Rust
- **Ollama**: Local LLM inference server
- **OpenAI API**: Cloud-based language model access
- **Tokio**: Async runtime for Rust
- **Serde**: JSON serialization/deserialization

Last Updated: January 2025 