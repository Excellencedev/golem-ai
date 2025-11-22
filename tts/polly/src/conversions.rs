// Type conversions for AWS Polly
// Most type mapping happens inline in client.rs

// AWS Polly uses simple voice IDs like "Joanna", "Matthew" and returns audio directly
// The client handles the conversion between Polly's API format and our WIT types
