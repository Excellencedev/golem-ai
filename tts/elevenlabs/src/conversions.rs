use super::client::ElevenLabsVoice;
use golem_tts::exports::golem::tts::voices::VoiceInfo as WitVoiceInfo;
use golem_tts::golem::tts::types::{VoiceGender, VoiceQuality};

impl From<ElevenLabsVoice> for WitVoiceInfo {
    fn from(voice: ElevenLabsVoice) -> Self {
        let gender = voice.labels.get("gender")
            .and_then(|g| match g.as_str() {
                "male" => Some(VoiceGender::Male),
"female" => Some(VoiceGender::Female),
                _ => Some(VoiceGender::Neutral),
            })
            .unwrap_or(VoiceGender::Neutral);

        let is_custom = voice.labels.get("use_case").map_or(false, |u| u.contains("custom"));

        WitVoiceInfo {
            id: voice.voice_id,
            name: voice.name.clone(),
            language: "en".to_string(),
            additional_languages: vec![],
            gender,
            quality: VoiceQuality::Neural,
            description: voice.description,
            provider: "ElevenLabs".to_string(),
            sample_rate: 44100,
            is_custom,
            is_cloned: is_custom,
            preview_url: voice.preview_url,
            use_cases: voice.labels.get("use_case")
                .map(|u| vec![u.clone()])
                .unwrap_or_default(),
        }
    }
}
