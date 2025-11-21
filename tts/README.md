# golem-tts

WebAssembly Components providing a unified API for various Text-to-Speech (TTS) providers.

## Versions

Each TTS provider has two versions: **Default** (with Golem-specific durability features) and **Portable** (no Golem dependencies).

There are 8 published WASM files for each release:

| Name                                 | Description                                                                                |
|--------------------------------------|--------------------------------------------------------------------------------------------|
| `golem-tts-elevenlabs.wasm`         | TTS implementation for ElevenLabs, using custom Golem specific durability features       |
| `golem-tts-polly.wasm`              | TTS implementation for AWS Polly, using custom Golem specific durability features        |
| `golem-tts-google.wasm`             | TTS implementation for Google Cloud TTS, using custom Golem specific durability features |
| `golem-tts-deepgram.wasm`           | TTS implementation for Deepgram Aura, using custom Golem specific durability features    |
| `golem-tts-elevenlabs-portable.wasm`| TTS implementation for ElevenLabs, with no Golem specific dependencies                   |
| `golem-tts-polly-portable.wasm`     | TTS implementation for AWS Polly, with no Golem specific dependencies                    |
| `golem-tts-google-portable.wasm`    | TTS implementation for Google Cloud TTS, with no Golem specific dependencies             |
| `golem-tts-deepgram-portable.wasm`  | TTS implementation for Deepgram Aura, with no Golem specific dependencies                |

Every component **exports** the same `golem:tts` interface, [defined here](wit/golem-tts.wit).

## Usage

For general usage information, integration examples, and getting started guides, see the [main README](../README.md).

### Environment Variables

Each provider has to be configured with appropriate credentials passed as environment variables:

| Provider    | Environment Variables                                                                      |
|-------------|--------------------------------------------------------------------------------------------|
| ElevenLabs  | `ELEVENLABS_API_KEY`                                                                       |
| AWS Polly   | `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION`, `AWS_SESSION_TOKEN` (optional)|
| Google Cloud| `GOOGLE_APPLICATION_CREDENTIALS` (path to service account JSON)                            |
| Deepgram    | `DEEPGRAM_API_KEY`                                                                         |

Additionally, setting the `TTS_PROVIDER_LOG_LEVEL=trace` environment variable enables trace logging for all communication with the underlying TTS provider.

**Common Configuration:**
- `TTS_PROVIDER_TIMEOUT` - Request timeout in seconds (default: 30)
- `TTS_PROVIDER_MAX_RETRIES` - Maximum retry attempts (default: 10)

## Features

The TTS interface supports comprehensive text-to-speech functionality including:

### Core Features
- **Text-to-Speech Synthesis** - Convert text to high-quality audio
- **Voice Management** - List, search, and get voice information
- **Multiple Audio Formats** - MP3, WAV, PCM, OGG-Opus, AAC, FLAC, etc.
- **Voice Settings** - Adjust speed, pitch, volume, stability, similarity
- **SSML Support** - Enhanced control with Speech Synthesis Markup Language

### Advanced Features (provider-dependent)
- **Streaming Synthesis** - Real-time audio generation (ElevenLabs, Deepgram)
- **Voice Cloning** - Create custom voices from audio samples (ElevenLabs)
- **Sound Effects** - Generate sound effects from descriptions (ElevenLabs)
- **Voice Conversion** - Convert audio to different voices (ElevenLabs)
- **Custom Lexicons** - Custom pronunciation dictionaries (AWS Polly)
- **Speech Marks** - Timing and word boundary information (AWS Polly)

## Provider Capabilities Matrix

| Feature                 | ElevenLabs | AWS Polly | Google Cloud | Deepgram Aura |
|-------------------------|------------|-----------|--------------|---------------|
| Basic Synthesis         | ✅         | ✅        | ✅           | ✅            |
| Voice Listing           | ✅         | ✅        | ✅           | ✅            |
| Streaming               | ✅         | ❌        | Planned      | Planned       |
| Voice Cloning           | Partial    | ❌        | ❌           | ❌            |
| Sound Effects           | ✅         | ❌        | ❌           | ❌            |
| Custom Lexicons         | ❌         | Planned   | ❌           | ❌            |
| SSML Support            | ✅         | ✅        | ✅           | Partial       |

**Note:** ✅ = Implemented, Planned = Stub implementation, ❌ = Returns unsupported-operation error

## Examples

Take the [test application](../test/tts/components-rust/test-tts/src/lib.rs) as an example of using `golem-tts` from Rust. The implemented test functions demonstrate:

| Function Name | Description                                                                                |
|---------------|--------------------------------------------------------------------------------------------|
| `test1`       | Simple text-to-speech synthesis                                                            |
| `test2`       | Voice listing and discovery                                                                |
| `test3`       | Streaming synthesis (ElevenLabs)                                                           |
| `test4`       | Batch synthesis of multiple texts                                                          |
| `test5`       | Voice settings customization (speed, pitch, volume)                                        |

### Running the examples

To run the examples first you need a running Golem instance. This can be Golem Cloud or the single-executable `golem` binary started with `golem server run`.

Then build and deploy the _test application_. The following profiles are available for testing:

| Profile Name           | Description                                                                           |
|------------------------|---------------------------------------------------------------------------------------|
| `elevenlabs-debug`     | Uses the ElevenLabs TTS implementation and compiles the code in debug profile        |
| `elevenlabs-release`   | Uses the ElevenLabs TTS implementation and compiles the code in release profile      |
| `polly-debug`          | Uses the AWS Polly TTS implementation and compiles the code in debug profile         |
| `polly-release`        | Uses the AWS Polly TTS implementation and compiles the code in release profile       |
| `google-debug`         | Uses the Google Cloud TTS implementation and compiles the code in debug profile      |
| `google-release`       | Uses the Google Cloud TTS implementation and compiles the code in release profile    |
| `deepgram-debug`       | Uses the Deepgram Aura TTS implementation and compiles the code in debug profile     |
| `deepgram-release`     | Uses the Deepgram Aura TTS implementation and compiles the code in release profile   |

```bash
cd ../test/tts
golem app build -b elevenlabs-debug
golem app deploy -b elevenlabs-debug
```

Depending on the provider selected, environment variables must be set for the worker to be started:

```bash
golem agent new test:tts/debug --env ELEVENLABS_API_KEY=xxx --env TTS_PROVIDER_LOG_LEVEL=trace
```

Then you can invoke the test functions on this worker:

```bash
golem agent invoke test:tts/debug test1 --stream
```

## Development

This repository uses [cargo-make](https://github.com/sagiegurari/cargo-make) to automate build tasks.
Some of the important tasks are:

| Command                             | Description                                                                                            |
|-------------------------------------|--------------------------------------------------------------------------------------------------------|
| `cargo make build tts`              | Build all TTS components with Golem bindings in Debug                                                 |
| `cargo make release-build tts`      | Build all TTS components with Golem bindings in Release                                               |
| `cargo make build-portable tts`     | Build all TTS components with no Golem bindings in Debug                                              |
| `cargo make release-build-portable tts` | Build all TTS components with no Golem bindings in Release                                        |
| `cargo make wit tts`                | To be used after editing the `wit/golem-tts.wit` file - distributes the changes to all wit directories|

The `test` directory contains a **Golem application** for testing various features of the TTS components.
Check [the Golem documentation](https://learn.golem.cloud/quickstart) to learn how to install Golem and `golem-cli` to run these tests.

## Implementation Status

### ElevenLabs Provider ✅
- ✅ Voice management (list, get, search)
- ✅ Basic synthesis
- ✅ Streaming synthesis (simplified)
- ✅ Voice cloning API (stub)
- ✅ Sound effects generation
- ✅ Error handling and durability

### AWS Polly Provider 🔄
- ✅ Voice management (hardcoded list)
- ✅ Basic synthesis (stub)
- ❌ Speech marks (not implemented)
- ❌ Lexicon support (not implemented)
- ❌ Streaming (not supported by provider)

### Google Cloud TTS Provider 🔄
- ✅ Voice management (hardcoded list)
- ✅ Basic synthesis (stub)
- ❌ Audio profiles (not implemented)
- ❌ Streaming (not implemented)

### Deepgram Aura Provider 🔄
- ✅ Voice management (hardcoded list)
- ✅ Basic synthesis (stub)
- ❌ Real-time streaming (not implemented)

**Legend:** ✅ = Fully implemented, 🔄 = Partial/Stub, ❌ = Not implemented yet
