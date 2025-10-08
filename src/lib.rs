use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use hex::{decode as hex_decode, encode as hex_encode};
use percent_encoding::{CONTROLS, percent_decode_str, utf8_percent_encode};
use std::io::{Read, Write};
use zed_extension_api::{
    self as zed, Result, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection,
};

struct Base64MultiExtension;

impl zed::Extension for Base64MultiExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if args.len() < 2 {
            return Err("Usage: <format> <text>".to_string());
        }

        let format = args[0].to_lowercase();
        let input = args[1..].join(" ");

        let result = match command.name.as_str() {
            "encode" => encode_text(&format, &input)?,
            "decode" => decode_text(&format, &input)?,
            cmd => return Err(format!("Unknown command: {}", cmd)),
        };

        let input_prefix = format!("Input: {}\n\n", input);
        let result_prefix = "Result: ";
        let text = format!("{}{}{}", input_prefix, result_prefix, result);
        let range_start = input_prefix.len() + result_prefix.len();

        Ok(SlashCommandOutput {
            text: text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (range_start..text.len()).into(),
                label: format!("{} ({} {})", result, format, command.name),
            }],
        })
    }

    fn complete_slash_command_argument(
        &self,
        _command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        if !args.is_empty() {
            return Ok(vec![]); // No completions after format is chosen
        }

        let formats = vec![
            ("base64", "Base64 (standard)"),
            ("base64-url", "Base64 (URL-safe)"),
            ("url", "URL encode"),
            ("hex", "Hex encode"),
            ("gzip", "Gzip compress"),
        ];

        Ok(formats
            .into_iter()
            .map(|(value, label)| {
                SlashCommandArgumentCompletion {
                    label: label.to_string(),
                    new_text: value.to_string(),
                    run_command: false, // Don't run yet; user adds text next
                }
            })
            .collect())
    }
}

fn encode_text(format: &str, input: &str) -> Result<String, String> {
    match format {
        "base64" => Ok(BASE64_STANDARD.encode(input)),
        "base64-url" => Ok(base64::engine::general_purpose::URL_SAFE.encode(input)),
        "url" => Ok(utf8_percent_encode(input, &CONTROLS).to_string()),
        "hex" => Ok(hex_encode(input.as_bytes())),
        "gzip" => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(input.as_bytes())
                .map_err(|e| e.to_string())?;
            let compressed = encoder.finish().map_err(|e| e.to_string())?;
            Ok(base64::engine::general_purpose::STANDARD.encode(compressed)) // Base64 for readability
        }
        _ => Err("Unsupported format".to_string()),
    }
}

fn decode_text(format: &str, input: &str) -> Result<String, String> {
    match format {
        "base64" => BASE64_STANDARD
            .decode(input)
            .map(|b| String::from_utf8(b).unwrap_or_default())
            .map_err(|e| e.to_string()),
        "base64-url" => base64::engine::general_purpose::URL_SAFE
            .decode(input)
            .map(|b| String::from_utf8(b).unwrap_or_default())
            .map_err(|e| e.to_string()),
        "url" => Ok(percent_decode_str(input)
            .decode_utf8()
            .unwrap_or_default()
            .to_string()),
        "hex" => hex_decode(input)
            .map(|b| String::from_utf8(b).unwrap_or_default())
            .map_err(|e| e.to_string()),
        "gzip" => {
            let compressed = base64::engine::general_purpose::STANDARD
                .decode(input)
                .map_err(|e| e.to_string())?;
            let mut decoder = GzDecoder::new(&*compressed);
            let mut decompressed = String::new();
            decoder
                .read_to_string(&mut decompressed)
                .map_err(|e| e.to_string())?;
            Ok(decompressed)
        }
        _ => Err("Unsupported format".to_string()),
    }
}

zed::register_extension!(Base64MultiExtension);
