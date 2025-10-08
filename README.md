# zed-base64

A Zed extension for encoding and decoding text in multiple formats (Base64, URL, Hex, and Gzip) using slash commands.

## Features

- Encode or decode text using different formats:
  - **Base64** (standard and URL-safe)
  - **URL percent encoding**
  - **Hexadecimal encoding**
  - **Gzip compression** (Base64-encoded for readability)
- Auto-completions for supported formats when typing `/encode` or `/decode`.
- Outputs formatted sections in the Assistant panel for easy navigation.

## Usage

This extension provides the `/encode` and `/decode` slash commands for use in Zed's Assistant panel.

### Invoking the Slash Command
1. In a "You" message block in the Assistant panel, type `/` to list available slash commands.
2. Select `/encode` (description: "Encode text into a chosen format") or `/decode` (description: "Decode text from a chosen format").
3. After `/encode ` or `/decode `, choose a format (`base64`, `base64-url`, `url`, `hex`, or `gzip`); auto-completions will suggest them.
4. Provide the input text to transform and press ⌘ + Enter (macOS) or Ctrl + Enter (other platforms) to submit.

The output will appear in the panel with a navigable section labeled with the result.

### Supported Formats
- **base64** – Standard Base64 encoding/decoding.
- **base64-url** – URL-safe Base64 variant.
- **url** – Percent-encode or decode text for URLs.
- **hex** – Encode to or decode from hexadecimal.
- **gzip** – Compress text with gzip (encoded as Base64) or decompress from Base64 gzip.

### Commands
- **encode <format> <text>**: Encodes the input text using the specified format.
- **decode <format> <text>**: Decodes the input text from the specified format.

## Examples

### Base64 Encoding
```
/encode base64 Hello World
```

Example Output:
- **Result**: `SGVsbG8gV29ybGQ=`

### Base64 Decoding
```
/decode base64 SGVsbG8gV29ybGQ=
```

Example Output:
- **Result**: `Hello World`

### URL Encoding
```
/encode url hello world!
```

Example Output:
- **Result**: `hello%20world%21`

### URL Decoding
```
/decode url hello%20world%21
```

Example Output:
- **Result**: `hello world!`

### Hex Encoding
```
/encode hex Rust
```

Example Output:
- **Result**: `52757374`

### Hex Decoding
```
/decode hex 52757374
```

Example Output:
- **Result**: `Rust`

### Gzip Compression
```
/encode gzip LargeTextHere
```

Example Output:
- **Result**: (Base64-encoded gzip string)

### Gzip Decompression
```
/decode gzip H4sIAAAAAAAA/8tIzcnJBwCGphA2BQAAAA==
```

Example Output:
- **Result**: `Hello`

## License

MIT License.
