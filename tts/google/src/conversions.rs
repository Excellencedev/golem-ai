use golem_tts::golem::tts::types::{AudioFormat, VoiceGender};

pub fn audio_format_to_google(format: AudioFormat) -> &'static str {
    match format {
        AudioFormat::Mp3 => "MP3",
        AudioFormat::Wav => "LINEAR16",
        AudioFormat::OggOpus => "OGG_OPUS",
        _ => "MP3",
    }
}

pub fn parse_gender(gender: &str) -> VoiceGender {
    match gender.to_lowercase().as_str() {
        "male" => VoiceGender::Male,
        "female" => VoiceGender::Female,
        _ => VoiceGender::Neutral,
    }
}
