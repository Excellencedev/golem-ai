// Comprehensive conversion utilities for Deepgram - matching PR #90
use crate::client::{Model, TextToSpeechParams, TextToSpeechRequest};
use golem_tts::golem::tts::synthesis::SynthesisOptions;
use golem_tts::golem::tts::types::{
    AudioFormat, SynthesisMetadata, SynthesisResult, TtsError, VoiceGender, VoiceQuality,
};
use golem_tts::golem::tts::voices::VoiceInfo;

pub fn estimate_audio_duration(audio_data: &[u8], sample_rate: u32) -> f32 {
    if audio_data.is_empty() {
        return 0.0;
    }

    let bytes_per_second = match sample_rate {
        8000 => 16000,
        16000 => 32000,
        22050 => 44100,
        24000 => 48000,
        48000 => 96000,
        _ => 48000,
    };

    (audio_data.len() as f32) / (bytes_per_second as f32)
}

pub fn deepgram_model_to_voice_info(model: Model) -> VoiceInfo {
    let gender = parse_gender(&model.gender);
    let quality = infer_quality_from_model(&model.voice_id);
    let language = normalize_language_code(&model.language);

    VoiceInfo {
        id: model.voice_id.clone(),
        name: model.name.clone(),
        language: language.clone(),
        additional_languages: vec![],
        gender,
        quality,
        description: Some(format!(
            "{} voice with {} accent, {}. Characteristics: {}. Suitable for: {}",
            model.gender,
            model.accent,
            model.age,
            model.characteristics.join(", "),
            model.use_cases.join(", ")
        )),
        provider: "Deepgram".to_string(),
        sample_rate: 24000,
        is_custom: false,
        is_cloned: false,
        preview_url: None,
        use_cases: model.use_cases.clone(),
    }
}

pub fn parse_gender(gender_str: &str) -> VoiceGender {
    match gender_str.to_lowercase().as_str() {
        "feminine" | "female" => VoiceGender::Female,
        "masculine" | "male" => VoiceGender::Male,
        _ => VoiceGender::Neutral,
    }
}

pub fn infer_quality_from_model(voice_id: &str) -> VoiceQuality {
    if voice_id.starts_with("aura-2-") {
        VoiceQuality::Premium
    } else {
        VoiceQuality::Standard
    }
}

pub fn normalize_language_code(code: &str) -> String {
    match code.to_lowercase().as_str() {
        "en-us" | "en-gb" | "en-au" | "en-ph" | "en-ie" => "en".to_string(),
        "es-es" | "es-mx" | "es-co" | "es-419" => "es".to_string(),
        _ => code.to_lowercase().chars().take(2).collect(),
    }
}

pub fn synthesis_options_to_tts_request(
    text: String,
    options: Option<SynthesisOptions>,
) -> Result<(TextToSpeechRequest, Option<TextToSpeechParams>), TtsError> {
    let request = TextToSpeechRequest { text: text.clone() };

    let default_params = TextToSpeechParams {
        model: None,
        encoding: Some("linear16".to_string()),
        container: Some("wav".to_string()),
        sample_rate: Some(24000),
        bit_rate: None,
    };

    if let Some(opts) = options {
        let mut params = default_params;

        if let Some(audio_config) = opts.audio_config {
            let (encoding, container, default_sample_rate, default_bit_rate) =
                audio_format_to_deepgram_params(audio_config.format);

            params.encoding = Some(encoding);
            params.container = container;

            // Set sample rate based on format
            match audio_config.format {
                AudioFormat::Mp3 | AudioFormat::Aac | AudioFormat::OggOpus => {
                    params.sample_rate = None;
                }
                _ => {
                    if let Some(user_rate) = audio_config.sample_rate {
                        let supported_rates = [8000, 16000, 24000, 32000, 48000];
                        if supported_rates.contains(&user_rate) {
                            params.sample_rate = Some(user_rate);
                        } else {
                            params.sample_rate = Some(24000);
                        }
                    } else {
                        params.sample_rate = Some(default_sample_rate);
                    }
                }
            }

            // Set bit rate for compressed formats
            match audio_config.format {
                AudioFormat::Mp3 | AudioFormat::OggOpus | AudioFormat::Aac => {
                    params.bit_rate = default_bit_rate;
                }
                _ => {
                    params.bit_rate = None;
                }
            }
        }

        // Use voice_id as model if provided
        if !opts.voice_id.is_empty() {
            params.model = Some(opts.voice_id);
        }

        Ok((request, Some(params)))
    } else {
        Ok((request, Some(default_params)))
    }
}

fn audio_format_to_deepgram_params(
    format: AudioFormat,
) -> (String, Option<String>, u32, Option<u32>) {
    match format {
        AudioFormat::Mp3 => ("mp3".to_string(), None, 22050, Some(48000)),
        AudioFormat::Wav => ("linear16".to_string(), Some("wav".to_string()), 24000, None),
        AudioFormat::Pcm => ("linear16".to_string(), None, 24000, None),
        AudioFormat::OggOpus => (
            "opus".to_string(),
            Some("ogg".to_string()),
            48000,
            Some(12000),
        ),
        AudioFormat::Aac => ("aac".to_string(), None, 22050, Some(48000)),
        AudioFormat::Flac => ("flac".to_string(), None, 48000, None),
        AudioFormat::Mulaw => ("mulaw".to_string(), Some("wav".to_string()), 8000, None),
        AudioFormat::Alaw => ("alaw".to_string(), Some("wav".to_string()), 8000, None),
    }
}

pub fn audio_data_to_synthesis_result(
    audio_data: Vec<u8>,
    text: &str,
    encoding: &str,
    sample_rate: u32,
) -> SynthesisResult {
    let audio_size = audio_data.len() as u32;
    let character_count = text.chars().count() as u32;
    let word_count = text.split_whitespace().count() as u32;
    let duration_seconds = estimate_audio_duration(&audio_data, sample_rate);

    let metadata = Some(SynthesisMetadata {
        duration_seconds,
        character_count,
        word_count,
        audio_size_bytes: audio_size,
        request_id: format!("deepgram-{}", chrono::Utc::now().timestamp()),
        provider_info: Some(format!(
            "Deepgram TTS - Encoding: {}, Sample Rate: {}Hz",
            encoding, sample_rate
        )),
    });

    SynthesisResult {
        audio_data,
        metadata,
    }
}
