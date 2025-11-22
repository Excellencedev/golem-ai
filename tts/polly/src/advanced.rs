// Speech marks and lexicon support for AWS Polly

use golem_tts::error::Error;
use golem_tts::golem::tts::types::{
    TextInput as WitTextInput, TimingInfo as WitTimingInfo, TtsError as WitTtsError,
};
use golem_tts::http::WstdHttpClient;
use serde::{Deserialize, Serialize};

use super::client::PollyClient;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpeechMark {
    pub time: u32,
    #[serde(rename = "type")]
    pub mark_type: String,
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub value: String,
}

impl PollyClient {
    pub fn get_speech_marks(
        &self,
        input: WitTextInput,
        voice_id: String,
    ) -> Result<Vec<WitTimingInfo>, WitTtsError> {
        let http = WstdHttpClient::new();

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct SpeechMarksRequest {
            text: String,
            output_format: String,
            voice_id: String,
            speech_mark_types: Vec<String>,
            engine: String,
        }

        let request_body = SpeechMarksRequest {
            text: input.content.clone(),
            output_format: "json".to_string(),
            voice_id,
            speech_mark_types: vec!["word".to_string(), "sentence".to_string()],
            engine: "neural".to_string(),
        };

        let json_payload = serde_json::to_string(&request_body).map_err(|e| Error::Json(e))?;

        let host = self
            .base_url
            .strip_prefix("https://")
            .unwrap_or(&self.base_url);
        let headers = vec![("host", host), ("content-type", "application/json")];

        let signed_headers =
            self.sign_request("POST", "/v1/speech", "", &headers, &json_payload)?;

        let url = format!("{}/v1/speech", self.base_url);
        let mut http_request = http.post(&url).header("Content-Type", "application/json");

        for (k, v) in signed_headers {
            http_request = http_request.header(k, &v);
        }

        let response = http_request
            .body(json_payload)?
            .send()?
            .error_for_status()?;

        // Parse line-delimited JSON
        let response_text = response.text()?;
        let mut timing_marks = Vec::new();

        for line in response_text.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let mark: SpeechMark = serde_json::from_str(line).map_err(|e| Error::Json(e))?;

            timing_marks.push(WitTimingInfo {
                time_ms: mark.time,
                mark_type: mark.mark_type.clone(),
                text: mark.value.clone(),
                start_offset: mark.start,
                end_offset: mark.end,
            });
        }

        Ok(timing_marks)
    }

    pub fn put_lexicon(&self, name: String, content: String) -> Result<(), WitTtsError> {
        let http = WstdHttpClient::new();

        let host = self
            .base_url
            .strip_prefix("https://")
            .unwrap_or(&self.base_url);
        let uri = format!("/v1/lexicons/{}", name);
        let headers = vec![("host", host), ("content-type", "application/x-pls+xml")];

        let signed_headers = self.sign_request("PUT", &uri, "", &headers, &content)?;

        let url = format!("{}{}", self.base_url, uri);
        let mut http_request = http
            .put(&url)
            .header("Content-Type", "application/x-pls+xml");

        for (k, v) in signed_headers {
            http_request = http_request.header(k, &v);
        }

        http_request.body(content)?.send()?.error_for_status()?;

        Ok(())
    }

    pub fn get_lexicon(&self, name: String) -> Result<String, WitTtsError> {
        let http = WstdHttpClient::new();

        let host = self
            .base_url
            .strip_prefix("https://")
            .unwrap_or(&self.base_url);
        let uri = format!("/v1/lexicons/{}", name);
        let headers = vec![("host", host)];

        let signed_headers = self.sign_request("GET", &uri, "", &headers, "")?;

        let url = format!("{}{}", self.base_url, uri);
        let mut http_request = http.get(&url);

        for (k, v) in signed_headers {
            http_request = http_request.header(k, &v);
        }

        let response = http_request.send()?.error_for_status()?;

        Ok(response.text()?)
    }
}
