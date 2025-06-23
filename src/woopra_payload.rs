use serde::Serialize;
use std::collections::HashMap;

use crate::exports::edgee::components::data_collection::Event;

// documentation: https://docs.woopra.com/reference/track-ce
// this struct is only used with Page and Track events
#[derive(Serialize, Debug, Default)]
pub(crate) struct WoopraPayloadTrack {
    // only these 3 fields are required
    project: String,
    event: String,
    timestamp: String,

    // all properties are prefixed with "cv_" (visitor), "ce_" (event), "cs_" (session)
    // and need to be serialized as flattened maps
    #[serde(
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "serialize_cv_prefixed",
        flatten
    )]
    visitor_properties: HashMap<String, String>,
    #[serde(
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "serialize_ce_prefixed",
        flatten
    )]
    event_properties: HashMap<String, String>,
    #[serde(
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "serialize_cs_prefixed",
        flatten
    )]
    session_properties: HashMap<String, String>,

    // all the other fields are optional
    #[serde(skip_serializing_if = "Option::is_none")]
    screen: Option<String>, // e.g. "1920x1080"
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>, // e.g. "en-US"
    #[serde(skip_serializing_if = "Option::is_none")]
    referer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<String>, // won't use for now (default is 30000)

    #[serde(skip_serializing_if = "Option::is_none")]
    browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app: Option<String>,
}

impl WoopraPayloadTrack {
    pub(crate) fn new(edgee_event: &Event, project: String, event: String) -> anyhow::Result<Self> {
        let mut payload = WoopraPayloadTrack {
            event,
            project,
            app: Some("Edgee".to_string()), // custom app value (like a special SDK)
            timestamp: edgee_event.timestamp.to_string(),
            ..WoopraPayloadTrack::default()
        };

        // add properties from context.page
        payload.add_page_properties(&edgee_event.context.page);

        // language/locale
        if !edgee_event.context.client.locale.is_empty() {
            payload.language = Some(edgee_event.context.client.locale.clone());
        }

        // user agent
        if !edgee_event
            .context
            .client
            .user_agent_full_version_list
            .is_empty()
        {
            payload.browser = Some(
                edgee_event
                    .context
                    .client
                    .user_agent_full_version_list
                    .clone(),
            );
        }

        // OS name and version
        if !edgee_event.context.client.os_name.is_empty() {
            if !edgee_event.context.client.os_version.is_empty() {
                payload.os = Some(format!(
                    "{} {}",
                    edgee_event.context.client.os_name.clone(),
                    edgee_event.context.client.os_version.clone(),
                ));
            } else {
                payload.os = Some(edgee_event.context.client.os_name.clone());
            }
        }

        // screen size
        if edgee_event.context.client.screen_width.is_positive()
            && edgee_event.context.client.screen_height.is_positive()
        {
            payload.screen = Some(format!(
                "{:?}x{:?}",
                edgee_event.context.client.screen_width.clone(),
                edgee_event.context.client.screen_height.clone()
            ));
        }

        // user ids
        if !edgee_event.context.user.anonymous_id.is_empty() {
            payload.visitor_properties.insert(
                "anonymous_id".to_string(),
                edgee_event.context.user.anonymous_id.clone(),
            );
        }
        if !edgee_event.context.user.user_id.is_empty() {
            payload.visitor_properties.insert(
                "user_id".to_string(),
                edgee_event.context.user.user_id.clone(),
            );
        }

        // user properties
        if !edgee_event.context.user.properties.is_empty() {
            for (key, value) in edgee_event.context.user.properties.clone().iter() {
                payload
                    .visitor_properties
                    .insert(key.to_string(), value.clone());
            }
        }

        // country code & IP address
        if !edgee_event.context.client.country_code.is_empty() {
            payload.visitor_properties.insert(
                "country".to_string(),
                edgee_event.context.client.country_code.clone(),
            );
        }
        if !edgee_event.context.client.ip.is_empty() {
            payload.ip = Some(edgee_event.context.client.ip.clone());
        }

        // session id and count
        payload.session_properties.insert(
            "session_id".to_string(),
            edgee_event.context.session.session_id.clone(),
        );
        payload.session_properties.insert(
            "session_count".to_string(),
            edgee_event
                .context
                .session
                .session_count
                .clone()
                .to_string(),
        );

        Ok(payload)
    }

    // this method can be used to add page properties to the payload (from event.data or context.page)
    pub(crate) fn add_page_properties(
        &mut self,
        page: &crate::exports::edgee::components::data_collection::PageData,
    ) {
        if !page.title.is_empty() {
            self.event_properties
                .insert("title".to_string(), page.title.clone());
        }
        if !page.url.is_empty() {
            let uri = format!("{}{}", page.url.clone(), page.search.clone());
            self.event_properties.insert("uri".to_string(), uri);
        }
        if !page.referrer.is_empty() {
            self.referer = Some(page.referrer.clone());
        }
        for (key, value) in page.properties.iter() {
            let key = key.replace(" ", "_");
            self.event_properties
                .insert(format!("page_{}", key), value.to_string());
        }
    }

    // this method can be used to add track properties to the payload (from event.data)
    pub(crate) fn add_track_properties(
        &mut self,
        data: &crate::exports::edgee::components::data_collection::TrackData,
    ) {
        // track data properties
        if !data.properties.is_empty() {
            for (key, value) in data.properties.clone().iter() {
                self.event_properties
                    .insert(key.to_string(), value.to_string());
            }
        }
    }
}

// documentation: https://docs.woopra.com/reference/track-identify
// this struct is only used with User events
#[derive(Serialize, Debug, Default)]
pub(crate) struct WoopraPayloadIdentify {
    // this is the only required field
    project: String,

    // visitor properties are prefixed with "cv_" (visitor)
    // and need to be serialized as flattened maps
    #[serde(
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "serialize_cv_prefixed",
        flatten
    )]
    visitor_properties: HashMap<String, String>,

    // default identifier (could be cv_email too)
    #[serde(skip_serializing_if = "Option::is_none")]
    cv_id: Option<String>,

    // optional cookie id (required only if no other identifier is provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<String>,
}

impl WoopraPayloadIdentify {
    pub(crate) fn new(edgee_event: &Event, project: String) -> anyhow::Result<Self> {
        let mut payload = WoopraPayloadIdentify {
            project,
            ..WoopraPayloadIdentify::default()
        };

        // add properties from context.user
        payload.add_user_properties(&edgee_event.context.user);

        // geo ip
        if !edgee_event.context.client.country_code.is_empty() {
            payload.visitor_properties.insert(
                "country".to_string(),
                edgee_event.context.client.country_code.clone(),
            );
        }

        Ok(payload)
    }

    // this method can be used to add user properties to the payload (from event.data or context.user)
    pub(crate) fn add_user_properties(
        &mut self,
        user: &crate::exports::edgee::components::data_collection::UserData,
    ) {
        if !user.anonymous_id.is_empty() {
            self.cv_id = Some(user.anonymous_id.clone());
        }
        if !user.user_id.is_empty() {
            // overwrite the anonymous_id with user_id if available
            self.cv_id = Some(user.user_id.clone());
        }

        // user properties
        if !user.properties.is_empty() {
            for (key, value) in user.properties.clone().iter() {
                self.visitor_properties
                    .insert(key.to_string(), value.clone());
            }
        }
    }
}

// Helper function to serialize HashMap with "cv_" prefix
fn serialize_cv_prefixed<S>(map: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut prefixed_map: HashMap<String, String> = HashMap::new();
    for (key, value) in map.iter() {
        let prefixed_key = if key.starts_with("cv_") {
            key.clone()
        } else {
            format!("cv_{}", key)
        };
        prefixed_map.insert(prefixed_key, value.clone());
    }
    prefixed_map.serialize(serializer)
}

// Helper function to serialize HashMap with "ce_" prefix
fn serialize_ce_prefixed<S>(map: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut prefixed_map: HashMap<String, String> = HashMap::new();
    for (key, value) in map.iter() {
        let prefixed_key = if key.starts_with("ce_") {
            key.clone()
        } else {
            format!("ce_{}", key)
        };
        prefixed_map.insert(prefixed_key, value.clone());
    }
    prefixed_map.serialize(serializer)
}

// Helper function to serialize HashMap with "ce_" prefix
fn serialize_cs_prefixed<S>(map: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut prefixed_map: HashMap<String, String> = HashMap::new();
    for (key, value) in map.iter() {
        let prefixed_key = if key.starts_with("cs_") {
            key.clone()
        } else {
            format!("cs_{}", key)
        };
        prefixed_map.insert(prefixed_key, value.clone());
    }
    prefixed_map.serialize(serializer)
}
