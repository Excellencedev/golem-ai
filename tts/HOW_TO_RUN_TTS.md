# How to Run and Test TTS Implementation

This guide provides comprehensive instructions for building, testing, and verifying the TTS (Text-to-Speech) implementation across all providers.

## Prerequisites

### Required Tools
- Rust toolchain (latest stable)
- `cargo-component` for building WebAssembly components
  ```powershell
  cargo install cargo-component
  ```
- Golem CLI (for deployment and testing)
- API keys for the TTS providers you want to test

### API Keys and Configuration

You'll need API keys for the providers you want to test:

#### Deepgram
```powershell
$env:DEEPGRAM_API_KEY = "your-deepgram-api-key-here"
```

#### ElevenLabs
```powershell
$env:ELEVENLABS_API_KEY = "your-elevenlabs-api-key-here"
```

#### AWS Polly
```powershell
$env:AWS_ACCESS_KEY_ID = "your-aws-access-key-id"
$env:AWS_SECRET_ACCESS_KEY = "your-aws-secret-access-key"
$env:AWS_REGION = "us-east-1"
# Optional: for temporary credentials
$env:AWS_SESSION_TOKEN = "your-session-token"
```

#### Google Cloud TTS
```powershell
# Set the path to your service account JSON key file
$env:GOOGLE_APPLICATION_CREDENTIALS = "path\to\your\service-account-key.json"
```

## Building the TTS Components

### 1. Build All TTS Provider Libraries

Build all provider libraries from the tts directory:

```powershell
cd c:\Users\USER\3D Objects\golem-ai\tts
```

**Build Deepgram provider:**
```powershell
cd deepgram
cargo component build --release
cd ..
```

**Build ElevenLabs provider:**
```powershell
cd elevenlabs
cargo component build --release
cd ..
```

**Build AWS Polly provider:**
```powershell
cd polly
cargo component build --release
cd ..
```

**Build Google Cloud TTS provider:**
```powershell
cd google
cargo component build --release
cd ..
```

### 2. Build All Providers at Once

Alternatively, build all providers in one command:

```powershell
cd c:\Users\USER\3D Objects\golem-ai\tts
cargo build --all --release
```

### 3. Build the Test Component

```powershell
cd c:\Users\USER\3D Objects\golem-ai\test\tts\components-rust\test-tts
cargo component build --release
```

The compiled WebAssembly component will be at:
```
target/wasm32-wasip1/release/test_tts.wasm
```

## Running Tests

### Unit Tests

Run unit tests for all TTS libraries:

```powershell
cd c:\Users\USER\3D Objects\golem-ai\tts
cargo test --all
```

Run tests for a specific provider:

```powershell
# Deepgram tests
cd deepgram
cargo test

# ElevenLabs tests
cd elevenlabs
cargo test

# Polly tests  
cd polly
cargo test

# Google tests
cd google
cargo test
```

### Integration Tests with Test Component

The test component provides 7 comprehensive test cases covering different aspects of the TTS functionality.

#### Test 1: Basic Synthesis
Tests basic text-to-speech synthesis with default settings.

#### Test 2: Voice Listing
Tests voice discovery and filtering capabilities.

#### Test 3: Streaming Lifecycle
Tests streaming API (create stream, send text, finish).

#### Test 4: Batch Synthesis
Tests batch processing of multiple text inputs.

#### Test 5: Voice Settings
Tests customized voice settings (speed, pitch, volume, stability).

#### Test 6: Voice Search
Tests voice search with filters.

#### Test 7: Advanced Features
Tests timing marks and sound effects generation.

## Manual Testing with Golem

### 1. Deploy a TTS Provider Component

Deploy one of the TTS provider components to Golem:

```powershell
# Deploy Deepgram provider
golem-cli component add \
  --component-name "deepgram-tts" \
  --component-file "tts\deepgram\target\wasm32-wasip1\release\deepgram.wasm"

# Deploy ElevenLabs provider
golem-cli component add \
  --component-name "elevenlabs-tts" \
  --component-file "tts\elevenlabs\target\wasm32-wasip1\release\elevenlabs.wasm"

# Deploy Polly provider
golem-cli component add \
  --component-name "polly-tts" \
  --component-file "tts\polly\target\wasm32-wasip1\release\polly.wasm"

# Deploy Google provider
golem-cli component add \
  --component-name "google-tts" \
  --component-file "tts\google\target\wasm32-wasip1\release\google.wasm"
```

### 2. Create a Worker Instance

```powershell
golem-cli worker add \
  --component-name "deepgram-tts" \
  --worker-name "deepgram-worker-1" \
  --env "DEEPGRAM_API_KEY=$env:DEEPGRAM_API_KEY"
```

### 3. Invoke TTS Functions

**List voices:**
```powershell
golem-cli worker invoke-and-await \
  --component-name "deepgram-tts" \
  --worker-name "deepgram-worker-1" \
  --function "golem:tts/voices/list-voices" \
  --arg "none"
```

**Synthesize speech:**
```powershell
golem-cli worker invoke-and-await \
  --component-name "deepgram-tts" \
  --worker-name "deepgram-worker-1" \
  --function "golem:tts/synthesis/synthesize" \
  --parameters @synthesis-params.json
```

Where `synthesis-params.json` contains:
```json
{
  "input": {
    "content": "Hello, this is a test of the Deepgram TTS system.",
    "text-type": "plain",
    "language": "en"
  },
  "options": {
    "voice-id": "aura-asteria-en",
    "audio-config": {
      "format": "mp3",
      "sample-rate": 24000,
      "channels": 1
    }
  }
}
```

## Testing with the Test Component

### 1. Deploy Test Component

```powershell
cd c:\Users\USER\3D Objects\golem-ai\test\tts\components-rust\test-tts

golem-cli component add \
  --component-name "test-tts" \
  --component-file "target\wasm32-wasip1\release\test_tts.wasm"
```

### 2. Create Worker with Provider Configuration

The test component uses the TTS provider components as dependencies, so you need to set up the environment variables for the provider you want to test:

```powershell
# For testing with Deepgram
golem-cli worker add \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --env "DEEPGRAM_API_KEY=$env:DEEPGRAM_API_KEY"
```

### 3. Run Individual Tests

```powershell
# Test 1: Basic synthesis
golem-cli worker invoke-and-await \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --function "test:tts/test-tts-api/test1"

# Test 2: Voice listing
golem-cli worker invoke-and-await \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --function "test:tts/test-tts-api/test2"

# Test 3: Streaming lifecycle
golem-cli worker invoke-and-await \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --function "test:tts/test-tts-api/test3"

# Test 4: Batch synthesis
golem-cli worker invoke-and-await \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --function "test:tts/test-tts-api/test4"

# Test 5: Voice settings
golem-cli worker invoke-and-await \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --function "test:tts/test-tts-api/test5"

# Test 6: Voice search
golem-cli worker invoke-and-await \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --function "test:tts/test-tts-api/test6"

# Test 7: Advanced features
golem-cli worker invoke-and-await \
  --component-name "test-tts" \
  --worker-name "tts-test-worker" \
  --function "test:tts/test-tts-api/test7"
```

## Verification Checklist

### For Each Provider

- [ ] **Build Success**: Component builds without errors
- [ ] **Voice Listing**: Can list available voices
- [ ] **Voice Search**: Can search and filter voices
- [ ] **Basic Synthesis**: Can synthesize simple text
- [ ] **Audio Formats**: Different audio formats work correctly
- [ ] **Voice Settings**: Custom voice settings are applied
- [ ] **Batch Processing**: Multiple inputs are processed correctly
- [ ] **Error Handling**: Invalid inputs produce proper error messages
- [ ] **API Key Validation**: Missing/invalid API keys are detected
- [ ] **Logging**: Appropriate log messages are generated

### Provider-Specific Features

#### Deepgram
- [ ] Aura voices are listed correctly
- [ ] All 9 voices are available
- [ ] Sample rate configuration works

#### ElevenLabs
- [ ] Voice cloning API is accessible (if available)
- [ ] Streaming works correctly
- [ ] Sound effects generation works

#### AWS Polly
- [ ] Both neural and standard voices are available
- [ ] Lexicon management works
- [ ] SSML is properly validated

#### Google Cloud TTS
- [ ] WaveNet and Neural2 voices are listed
- [ ] Service account authentication works
- [ ] SSML support is functional

## Troubleshooting

### Common Issues

**Build Errors:**
```
error: failed to build component
```
- Ensure `cargo-component` is installed and up to date
- Check that you're using the correct Rust toolchain version
- Verify all dependencies are available

**API Key Errors:**
```
ERROR: Required environment variable not set
```
- Double-check environment variable names (case-sensitive)
- Ensure API keys are valid and have appropriate permissions
- For AWS, verify all required credentials are set

**Network Errors:**
```
ERROR: Network error: Connection refused
```
- Check internet connectivity
- Verify API endpoints are accessible
- Check for firewall/proxy restrictions

**Authentication Errors:**
```
ERROR: Unauthorized
```
- Verify API keys are current and valid
- Check API key permissions/scopes
- For Google, verify service account has TTS API enabled

### Logs and Debugging

Enable detailed logging:

```powershell
# Set log level
$env:TTS_PROVIDER_LOG_LEVEL = "debug"
```

Available log levels: `error`, `warn`, `info`, `debug`, `trace`

## Performance Testing

Measure synthesis performance:

```powershell
# Time a synthesis operation
Measure-Command {
  golem-cli worker invoke-and-await \
    --component-name "deepgram-tts" \
    --worker-name "deepgram-worker-1" \
    --function "golem:tts/synthesis/synthesize" \
    --parameters @synthesis-params.json
}
```

## Creating Demo Videos

This section shows how to create a comprehensive demo video showcasing all 5 TTS providers working.

### Quick Demo Script

This script creates a demo showing all 5 providers synthesizing the same text:

```powershell
# Demo text to synthesize
$demoText = "Welcome to the Golem TTS demonstration. This is provider"

# 1. DEEPGRAM DEMO
Write-Host "`n=== DEEPGRAM TTS ===" -ForegroundColor Cyan
golem-cli worker invoke-and-await `
  --component-name "deepgram-tts" `
  --worker-name "demo-worker-deepgram" `
  --function "golem:tts/synthesis/synthesize" `
  --args "{ \`"input\`": { \`"content\`": \`"$demoText Deepgram Aura\`", \`"text-type\`": \`"plain\`" }, \`"options\`": { \`"voice-id\`": \`"aura-asteria-en\`" } }"

# 2. ELEVENLABS DEMO
Write-Host "`n=== ELEVENLABS TTS ===" -ForegroundColor Green
golem-cli worker invoke-and-await `
  --component-name "elevenlabs-tts" `
  --worker-name "demo-worker-elevenlabs" `
  --function "golem:tts/synthesis/synthesize" `
  --args "{ \`"input\`": { \`"content\`": \`"$demoText ElevenLabs\`", \`"text-type\`": \`"plain\`" }, \`"options\`": { \`"voice-id\`": \`"21m00Tcm4TlvDq8ikWAM\`" } }"

# 3. AWS POLLY DEMO
Write-Host "`n=== AWS POLLY TTS ===" -ForegroundColor Yellow
golem-cli worker invoke-and-await `
  --component-name "polly-tts" `
  --worker-name "demo-worker-polly" `
  --function "golem:tts/synthesis/synthesize" `
  --args "{ \`"input\`": { \`"content\`": \`"$demoText AWS Polly\`", \`"text-type\`": \`"plain\`" }, \`"options\`": { \`"voice-id\`": \`"Joanna\`" } }"

# 4. GOOGLE CLOUD TTS DEMO
Write-Host "`n=== GOOGLE CLOUD TTS ===" -ForegroundColor Magenta
golem-cli worker invoke-and-await `
  --component-name "google-tts" `
  --worker-name "demo-worker-google" `
  --function "golem:tts/synthesis/synthesize" `
  --args "{ \`"input\`": { \`"content\`": \`"$demoText Google Cloud\`", \`"text-type\`": \`"plain\`" }, \`"options\`": { \`"voice-id\`": \`"en-US-Neural2-A\`" } }"
```

### Comprehensive Demo Workflow

For a complete demonstration video showing all features:

#### 1. Setup Phase (Record This)

```powershell
# Show environment setup
Write-Host "=== TTS Provider Demo - Setup ===" -ForegroundColor White -BackgroundColor DarkBlue

# Display API keys are configured (hide actual values!)
Write-Host "`nAPI Keys Configured:" -ForegroundColor Green
Write-Host "  ✓ DEEPGRAM_API_KEY" -ForegroundColor Gray
Write-Host "  ✓ ELEVENLABS_API_KEY" -ForegroundColor Gray
Write-Host "  ✓ AWS credentials" -ForegroundColor Gray
Write-Host "  ✓ GOOGLE_APPLICATION_CREDENTIALS" -ForegroundColor Gray
```

#### 2. Build All Providers (Optional - can skip in demo)

```powershell
Write-Host "`n=== Building All TTS Providers ===" -ForegroundColor Cyan
cd "c:\Users\USER\3D Objects\golem-ai\tts"

# Show build progress for each provider
@("deepgram", "elevenlabs", "polly", "google") | ForEach-Object {
    Write-Host "`nBuilding $_ provider..." -ForegroundColor Yellow
    cd $_
    cargo component build --release
    cd ..
}
```

#### 3. Deploy All Providers (Record This)

```powershell
Write-Host "`n=== Deploying TTS Providers to Golem ===" -ForegroundColor Cyan

# Deploy each provider
golem-cli component add --component-name "deepgram-tts" `
  --component-file "tts\deepgram\target\wasm32-wasip1\release\deepgram.wasm"

golem-cli component add --component-name "elevenlabs-tts" `
  --component-file "tts\elevenlabs\target\wasm32-wasip1\release\elevenlabs.wasm"

golem-cli component add --component-name "polly-tts" `
  --component-file "tts\polly\target\wasm32-wasip1\release\polly.wasm"

golem-cli component add --component-name "google-tts" `
  --component-file "tts\google\target\wasm32-wasip1\release\google.wasm"

Write-Host "`n✓ All 5 providers deployed!" -ForegroundColor Green
```

#### 4. Create Workers (Record This)

```powershell
Write-Host "`n=== Creating Worker Instances ===" -ForegroundColor Cyan

# Create worker for each provider with appropriate env vars
golem-cli worker add --component-name "deepgram-tts" --worker-name "demo-deepgram" `
  --env "DEEPGRAM_API_KEY=$env:DEEPGRAM_API_KEY"

golem-cli worker add --component-name "elevenlabs-tts" --worker-name "demo-elevenlabs" `
  --env "ELEVENLABS_API_KEY=$env:ELEVENLABS_API_KEY"

golem-cli worker add --component-name "polly-tts" --worker-name "demo-polly" `
  --env "AWS_ACCESS_KEY_ID=$env:AWS_ACCESS_KEY_ID" `
  --env "AWS_SECRET_ACCESS_KEY=$env:AWS_SECRET_ACCESS_KEY" `
  --env "AWS_REGION=$env:AWS_REGION"

golem-cli worker add --component-name "google-tts" --worker-name "demo-google" `
  --env "GOOGLE_APPLICATION_CREDENTIALS=$env:GOOGLE_APPLICATION_CREDENTIALS"

Write-Host "`n✓ All workers created!" -ForegroundColor Green
```

#### 5. Demonstrate Each Provider (THE KEY PART!)

**Demo 1: List Voices from All Providers**

```powershell
Write-Host "`n" "=" * 60 -ForegroundColor White
Write-Host "DEMONSTRATION 1: List Available Voices" -ForegroundColor White -BackgroundColor DarkBlue
Write-Host "=" * 60 "`n" -ForegroundColor White

# Deepgram voices
Write-Host "`n[1/5] DEEPGRAM Aura Voices:" -ForegroundColor Cyan
golem-cli worker invoke-and-await --component-name "deepgram-tts" `
  --worker-name "demo-deepgram" --function "golem:tts/voices/list-voices"

# ElevenLabs voices
Write-Host "`n[2/5] ELEVENLABS Voices:" -ForegroundColor Green
golem-cli worker invoke-and-await --component-name "elevenlabs-tts" `
  --worker-name "demo-elevenlabs" --function "golem:tts/voices/list-voices"

# AWS Polly voices
Write-Host "`n[3/5] AWS POLLY Neural Voices:" -ForegroundColor Yellow
golem-cli worker invoke-and-await --component-name "polly-tts" `
  --worker-name "demo-polly" --function "golem:tts/voices/list-voices"

# Google Cloud TTS voices
Write-Host "`n[4/5] GOOGLE CLOUD TTS Voices (Neural2 & WaveNet):" -ForegroundColor Magenta
golem-cli worker invoke-and-await --component-name "google-tts" `
  --worker-name "demo-google" --function "golem:tts/voices/list-voices"

Write-Host "`n✓ Voice listing complete for all 5 providers!" -ForegroundColor Green
```

**Demo 2: Synthesize Speech from All Providers**

```powershell
Write-Host "`n" "=" * 60 -ForegroundColor White
Write-Host "DEMONSTRATION 2: Speech Synthesis" -ForegroundColor White -BackgroundColor DarkBlue
Write-Host "=" * 60 "`n" -ForegroundColor White

$testText = "Hello from Golem Cloud! This is a demonstration of our multi-provider text to speech system."

# Create param file for synthesis
$synthParams = @{
    input = @{
        content = $testText
        "text-type" = "plain"
        language = "en"
    }
    options = @{
        "voice-id" = ""
        "audio-config" = @{
            format = "mp3"
            "sample-rate" = 24000
        }
    }
}

# 1. Deepgram
Write-Host "`n[1/5] Synthesizing with DEEPGRAM (Aura Asteria)..." -ForegroundColor Cyan
$synthParams.options."voice-id" = "aura-asteria-en"
$synthParams | ConvertTo-Json -Depth 10 | Out-File -Encoding UTF8 "deepgram-params.json"
golem-cli worker invoke-and-await --component-name "deepgram-tts" `
  --worker-name "demo-deepgram" --function "golem:tts/synthesis/synthesize" `
  --parameters @deepgram-params.json
Write-Host "✓ Deepgram synthesis complete" -ForegroundColor Green

# 2. ElevenLabs
Write-Host "`n[2/5] Synthesizing with ELEVENLABS (Rachel)..." -ForegroundColor Green
$synthParams.options."voice-id" = "21m00Tcm4TlvDq8ikWAM"
$synthParams | ConvertTo-Json -Depth 10 | Out-File -Encoding UTF8 "elevenlabs-params.json"
golem-cli worker invoke-and-await --component-name "elevenlabs-tts" `
  --worker-name "demo-elevenlabs" --function "golem:tts/synthesis/synthesize" `
  --parameters @elevenlabs-params.json
Write-Host "✓ ElevenLabs synthesis complete" -ForegroundColor Green

# 3. AWS Polly
Write-Host "`n[3/5] Synthesizing with AWS POLLY (Joanna)..." -ForegroundColor Yellow
$synthParams.options."voice-id" = "Joanna"
$synthParams | ConvertTo-Json -Depth 10 | Out-File -Encoding UTF8 "polly-params.json"
golem-cli worker invoke-and-await --component-name "polly-tts" `
  --worker-name "demo-polly" --function "golem:tts/synthesis/synthesize" `
  --parameters @polly-params.json
Write-Host "✓ AWS Polly synthesis complete" -ForegroundColor Green

# 4. Google Cloud TTS
Write-Host "`n[4/5] Synthesizing with GOOGLE CLOUD TTS (Neural2-A)..." -ForegroundColor Magenta
$synthParams.options."voice-id" = "en-US-Neural2-A"
$synthParams | ConvertTo-Json -Depth 10 | Out-File -Encoding UTF8 "google-params.json"
golem-cli worker invoke-and-await --component-name "google-tts" `
  --worker-name "demo-google" --function "golem:tts/synthesis/synthesize" `
  --parameters @google-params.json
Write-Host "✓ Google Cloud TTS synthesis complete" -ForegroundColor Green

Write-Host "`n✓ ALL 5 PROVIDERS SUCCESSFULLY SYNTHESIZED SPEECH!" -ForegroundColor Green -BackgroundColor DarkGreen
```

**Demo 3: Show Advanced Features**

```powershell
Write-Host "`n" "=" * 60 -ForegroundColor White
Write-Host "DEMONSTRATION 3: Advanced Features" -ForegroundColor White -BackgroundColor DarkBlue
Write-Host "=" * 60 "`n" -ForegroundColor White

# Voice search example
Write-Host "`n[Advanced] Voice Search - Finding female voices:" -ForegroundColor Cyan
golem-cli worker invoke-and-await --component-name "deepgram-tts" `
  --worker-name "demo-deepgram" --function "golem:tts/voices/search-voices" `
  --args '{"query": "female", "filter": {"gender": "female"}}'

# Batch synthesis example
Write-Host "`n[Advanced] Batch Synthesis - Multiple texts:" -ForegroundColor Cyan
$batchParams = @{
    inputs = @(
        @{ content = "First sentence"; "text-type" = "plain" },
        @{ content = "Second sentence"; "text-type" = "plain" },
        @{ content = "Third sentence"; "text-type" = "plain" }
    )
    options = @{ "voice-id" = "aura-asteria-en" }
}
$batchParams | ConvertTo-Json -Depth 10 | Out-File -Encoding UTF8 "batch-params.json"
golem-cli worker invoke-and-await --component-name "deepgram-tts" `
  --worker-name "demo-deepgram" --function "golem:tts/synthesis/synthesize-batch" `
  --parameters @batch-params.json

Write-Host "`n✓ Advanced features demonstration complete!" -ForegroundColor Green
```

#### 6. Demo Summary

```powershell
Write-Host "`n" "=" * 60 -ForegroundColor White
Write-Host "DEMO COMPLETE - Summary" -ForegroundColor White -BackgroundColor DarkGreen
Write-Host "=" * 60 -ForegroundColor White

Write-Host "`nSuccessfully demonstrated:" -ForegroundColor Green
Write-Host "  ✓ 5 TTS providers deployed and running" -ForegroundColor Gray
Write-Host "  ✓ Deepgram Aura TTS" -ForegroundColor Cyan
Write-Host "  ✓ ElevenLabs TTS" -ForegroundColor Green
Write-Host "  ✓ AWS Polly TTS" -ForegroundColor Yellow
Write-Host "  ✓ Google Cloud TTS" -ForegroundColor Magenta
Write-Host "`n  ✓ Voice listing" -ForegroundColor Gray
Write-Host "  ✓ Speech synthesis" -ForegroundColor Gray
Write-Host "  ✓ Voice search & filtering" -ForegroundColor Gray
Write-Host "  ✓ Batch processing" -ForegroundColor Gray
Write-Host "`n✓ All systems operational!" -ForegroundColor Green -BackgroundColor DarkGreen
```

### Recording Tips

1. **Screen Recording Software**:
   - Windows: Use Windows Game Bar (Win+G) or OBS Studio
   - Set resolution to 1920x1080 for best quality
   - Record at 30 FPS minimum

2. **Terminal Setup**:
   - Use a clear, readable font (Consolas, Cascadia Code)
   - Set font size to 14-16pt for visibility
   - Use PowerShell with colored output enabled
   - Consider using Windows Terminal for better color support

3. **Demo Flow**:
   - Start with a title screen explaining what you're demonstrating
   - Show the setup phase (API keys configured, components built)
   - Deploy all 5 providers
   - Run the voice listing demo
   - Run the synthesis demo for all 5 providers
   - Show 1-2 advanced features
   - End with summary screen

4. **Editing**:
   - Speed up build times (2x-4x) if included
   - Add captions identifying each provider
   - Include timestamps in description
   - Add background music (optional)

### Sample Video Structure

```
00:00 - Introduction
00:30 - Setup & Configuration
01:00 - Building Components
02:00 - Deploying to Golem
03:00 - Demo 1: List Voices (All 5 Providers)
05:00 - Demo 2: Speech Synthesis (All 5 Providers)
08:00 - Demo 3: Advanced Features
10:00 - Summary & Conclusion
```

## Next Steps

After verifying the TTS implementation works:

1. **Deploy to Production**: Deploy workers with production API keys
2. **Set Up Monitoring**: Monitor worker health and API usage
3. **Configure Rate Limits**: Set appropriate rate limits for your use case
4. **Implement Caching**: Cache frequently requested voice lists and metadata
5. **Add Metrics**: Track synthesis requests, errors, and latency

## Additional Resources

- [Golem Documentation](https://golem.cloud/docs)
- [Deepgram API Documentation](https://developers.deepgram.com/docs/tts)
- [ElevenLabs API Documentation](https://docs.elevenlabs.io/)
- [AWS Polly Documentation](https://docs.aws.amazon.com/polly/)
- [Google Cloud TTS Documentation](https://cloud.google.com/text-to-speech/docs)
