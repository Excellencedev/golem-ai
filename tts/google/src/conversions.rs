// Type conversions for Google Cloud TTS
// Most type mapping happens inline in client.rs

// Google uses voice IDs like "en-US-Neural2-A" and returns base64-encoded audio
// The client handles the conversion between Google's API format and our WIT types
