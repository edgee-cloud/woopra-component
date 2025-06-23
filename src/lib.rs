use crate::exports::edgee::components::data_collection::{
    Data, Dict, EdgeeRequest, Event, HttpMethod,
};
use anyhow::Context;
use exports::edgee::components::data_collection::Guest;
use std::collections::HashMap;
use woopra_payload::{WoopraPayloadIdentify, WoopraPayloadTrack};

mod woopra_payload;

wit_bindgen::generate!({world: "data-collection", path: ".edgee/wit", generate_all});
export!(Component);

struct Component;

const WOOPRA_HOST: &str = "https://www.woopra.com";
const WOOPRA_TRACK_ENDPOINT: &str = "/track/ce";
const WOOPRA_IDENTIFY_ENDPOINT: &str = "/track/identify";

impl Guest for Component {
    fn page(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        if let Data::Page(ref data) = edgee_event.data {
            let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;

            let mut payload =
                WoopraPayloadTrack::new(&edgee_event, settings.project_name, "pv".to_string())
                    .map_err(|e| e.to_string())?;

            payload.add_page_properties(data);

            let querystring = serde_qs::to_string(&payload).map_err(|e| e.to_string())?;

            Ok(
                build_edgee_request(querystring, WOOPRA_TRACK_ENDPOINT.to_string())
                    .map_err(|e| e.to_string())?,
            )
        } else {
            Err("Missing page data".to_string())
        }
    }

    fn track(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        if let Data::Track(ref data) = edgee_event.data {
            if data.name.is_empty() {
                return Err("Track is not set".to_string());
            }

            let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;

            let mut payload =
                WoopraPayloadTrack::new(&edgee_event, settings.project_name, data.name.clone())
                    .map_err(|e| e.to_string())?;

            payload.add_track_properties(data);

            let querystring = serde_qs::to_string(&payload).map_err(|e| e.to_string())?;

            Ok(
                build_edgee_request(querystring, WOOPRA_TRACK_ENDPOINT.to_string())
                    .map_err(|e| e.to_string())?,
            )
        } else {
            Err("Missing track data".to_string())
        }
    }

    fn user(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        if let Data::User(ref data) = edgee_event.data {
            let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;

            let mut payload = WoopraPayloadIdentify::new(&edgee_event, settings.project_name)
                .map_err(|e| e.to_string())?;

            payload.add_user_properties(data);

            let querystring = serde_qs::to_string(&payload).map_err(|e| e.to_string())?;

            Ok(
                build_edgee_request(querystring, WOOPRA_IDENTIFY_ENDPOINT.to_string())
                    .map_err(|e| e.to_string())?,
            )
        } else {
            Err("Missing user data".to_string())
        }
    }
}

fn build_edgee_request(querystring: String, endpoint: String) -> anyhow::Result<EdgeeRequest> {
    let headers = vec![(String::from("content-length"), String::from("0"))];

    Ok(EdgeeRequest {
        method: HttpMethod::Get,
        url: format!("{}{}?{}", WOOPRA_HOST, endpoint, querystring),
        headers,
        forward_client_headers: true,
        body: String::new(),
    })
}

pub struct Settings {
    pub project_name: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let project_name = settings_map
            .get("project_name")
            .context("Missing example setting")?
            .to_string();

        Ok(Self { project_name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exports::edgee::components::data_collection::{
        Campaign, Client, Context, Data, EventType, PageData, Session, UserData,
    };
    use exports::edgee::components::data_collection::Consent;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    fn sample_user_data(edgee_id: String) -> UserData {
        UserData {
            user_id: "123".to_string(),
            anonymous_id: "456".to_string(),
            edgee_id,
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
            ],
        }
    }

    fn sample_context(edgee_id: String, locale: String, session_start: bool) -> Context {
        Context {
            page: sample_page_data(),
            user: sample_user_data(edgee_id),
            client: Client {
                city: "Paris".to_string(),
                ip: "192.168.0.1".to_string(),
                locale,
                timezone: "CET".to_string(),
                user_agent: "Chrome".to_string(),
                user_agent_architecture: "x86".to_string(),
                user_agent_bitness: "64".to_string(),
                user_agent_full_version_list: "abc".to_string(),
                user_agent_version_list: "abc".to_string(),
                user_agent_mobile: "mobile".to_string(),
                user_agent_model: "don't know".to_string(),
                os_name: "MacOS".to_string(),
                os_version: "latest".to_string(),
                screen_width: 1024,
                screen_height: 768,
                screen_density: 2.0,
                continent: "Europe".to_string(),
                country_code: "FR".to_string(),
                country_name: "France".to_string(),
                region: "West Europe".to_string(),
            },
            campaign: Campaign {
                name: "random".to_string(),
                source: "random".to_string(),
                medium: "random".to_string(),
                term: "random".to_string(),
                content: "random".to_string(),
                creative_format: "random".to_string(),
                marketing_tactic: "random".to_string(),
            },
            session: Session {
                session_id: "random".to_string(),
                previous_session_id: "random".to_string(),
                session_count: 2,
                session_start,
                first_seen: 123,
                last_seen: 123,
            },
        }
    }

    fn sample_page_data() -> PageData {
        PageData {
            name: "page name".to_string(),
            category: "category".to_string(),
            keywords: vec!["value1".to_string(), "value2".into()],
            title: "page title".to_string(),
            url: "https://example.com/full-url?test=1".to_string(),
            path: "/full-path".to_string(),
            search: "?test=1".to_string(),
            referrer: "https://example.com/another-page".to_string(),
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ],
        }
    }

    fn sample_page_event(
        consent: Option<Consent>,
        edgee_id: String,
        locale: String,
        session_start: bool,
    ) -> Event {
        Event {
            uuid: Uuid::new_v4().to_string(),
            timestamp: 123,
            timestamp_millis: 123,
            timestamp_micros: 123,
            event_type: EventType::Page,
            data: Data::Page(sample_page_data()),
            context: sample_context(edgee_id, locale, session_start),
            consent,
        }
    }

    #[test]
    fn page_works_fine() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "abc".to_string(),
            "fr".to_string(),
            true,
        );
        let settings = vec![("project_name".to_string(), "example.com".to_string())];
        let result = Component::page(event, settings);

        assert_eq!(result.is_err(), false);
        let edgee_request = result.unwrap();
        assert_eq!(edgee_request.method, HttpMethod::Get);
        assert_eq!(edgee_request.body.is_empty(), true);
        assert_eq!(
            edgee_request.url.starts_with("https://www.woopra.com"),
            true
        );
        // add more checks (headers, querystring, etc.)
    }
}
