use std::fs;
use std::path::{Path, PathBuf};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDefaultEntry {
    pub key: String,
    pub value: Option<String>,
}

impl UserDefaultEntry {
    pub fn new(key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        Self {
            key: key.into(),
            value: value.map(Into::into),
        }
    }

    pub fn value_as_str(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct UserDefault {
    file_path: PathBuf,
}

impl UserDefault {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn read_raw(&self) -> Result<Vec<UserDefaultEntry>, Error> {
        if !self.file_path.exists() {
            return Err(Error::UserDefaultNotFound(self.file_path.clone()));
        }

        let xml = fs::read_to_string(&self.file_path)?;
        Self::parse_xml(&xml)
    }

    pub fn write_raw(&self, entries: &[UserDefaultEntry]) -> Result<(), Error> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let xml = Self::serialize_entries(entries);
        fs::write(&self.file_path, xml)?;
        Ok(())
    }

    pub fn get_normalized_entries(&self) -> Result<Vec<UserDefaultEntry>, Error> {
        self.read_raw()
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Error> {
        let entries = self.read_raw()?;
        for entry in entries {
            if entry.key == key {
                return Ok(entry.value);
            }
        }
        Ok(None)
    }

    pub fn set(&self, key: &str, value: impl AsRef<str>) -> Result<(), Error> {
        let val_str = value.as_ref().to_string();
        self.mutate_entries(|entries| {
            if let Some(existing) = entries.iter_mut().find(|e| e.key == key) {
                existing.value = Some(val_str);
            } else {
                entries.push(UserDefaultEntry {
                    key: key.to_string(),
                    value: Some(val_str),
                });
            }
        })
    }

    pub fn set_bool(&self, key: &str, value: bool) -> Result<(), Error> {
        self.set(key, if value { "true" } else { "false" })
    }

    pub fn set_number<N: ToString>(&self, key: &str, value: N) -> Result<(), Error> {
        self.set(key, value.to_string())
    }

    pub fn delete(&self, key: &str) -> Result<bool, Error> {
        let mut removed = false;
        self.mutate_entries(|entries| {
            let initial_len = entries.len();
            entries.retain(|e| e.key != key);
            removed = entries.len() != initial_len;
        })?;
        Ok(removed)
    }

    pub fn list_by_prefix(&self, prefix: &str) -> Result<Vec<UserDefaultEntry>, Error> {
        let entries = self.read_raw()?;
        Ok(entries
            .into_iter()
            .filter(|e| e.key.starts_with(prefix))
            .collect())
    }

    pub fn list_by_include(&self, substring: &str) -> Result<Vec<UserDefaultEntry>, Error> {
        let entries = self.read_raw()?;
        Ok(entries
            .into_iter()
            .filter(|e| e.key.contains(substring))
            .collect())
    }

    pub(crate) fn mutate_entries<F>(&self, mutate: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Vec<UserDefaultEntry>),
    {
        let mut entries = if self.file_path.exists() {
            self.read_raw()?
        } else {
            Vec::new()
        };

        mutate(&mut entries);
        self.write_raw(&entries)
    }

    pub fn parse_xml(xml: &str) -> Result<Vec<UserDefaultEntry>, Error> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut entries = Vec::new();
        let mut current_key: Option<String> = None;
        let mut current_text: Option<String> = None;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"entry" => {
                    current_key = None;
                    current_text = None;
                    for attr in e.attributes() {
                        let attr = attr.map_err(|err| Error::XmlParse(err.to_string()))?;
                        if attr.key.as_ref() == b"key" {
                            let val = attr
                                .normalized_value(quick_xml::XmlVersion::Explicit1_0)
                                .map_err(|err| Error::XmlParse(err.to_string()))?;
                            current_key = Some(val.into_owned());
                        }
                    }
                }
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"entry" => {
                    let mut key = None;
                    for attr in e.attributes() {
                        let attr = attr.map_err(|err| Error::XmlParse(err.to_string()))?;
                        if attr.key.as_ref() == b"key" {
                            let val = attr
                                .normalized_value(quick_xml::XmlVersion::Explicit1_0)
                                .map_err(|err| Error::XmlParse(err.to_string()))?;
                            key = Some(val.into_owned());
                        }
                    }
                    if let Some(k) = key {
                        entries.push(UserDefaultEntry {
                            key: k,
                            value: None,
                        });
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if current_key.is_some() {
                        let raw_str = std::str::from_utf8(e.as_ref())
                            .map_err(|err| Error::XmlParse(err.to_string()))?;
                        let text = quick_xml::escape::unescape(raw_str)
                            .map_err(|err| Error::XmlParse(err.to_string()))?;
                        current_text = Some(text.into_owned());
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"entry" => {
                    if let Some(k) = current_key.take() {
                        entries.push(UserDefaultEntry {
                            key: k,
                            value: current_text.take(),
                        });
                    }
                }
                Ok(Event::Eof) => break,
                Err(err) => return Err(Error::XmlParse(err.to_string())),
                _ => {}
            }
        }

        Ok(entries)
    }

    pub fn serialize_entries(entries: &[UserDefaultEntry]) -> String {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<userDefaultRoot>\n");
        for entry in entries {
            let key_escaped = quick_xml::escape::escape(&entry.key);
            match &entry.value {
                Some(val) if !val.is_empty() => {
                    let val_escaped = quick_xml::escape::escape(val);
                    out.push_str("    <entry key=\"");
                    out.push_str(&key_escaped);
                    out.push_str("\">");
                    out.push_str(&val_escaped);
                    out.push_str("</entry>\n");
                }
                _ => {
                    out.push_str("    <entry key=\"");
                    out.push_str(&key_escaped);
                    out.push_str("\"/>\n");
                }
            }
        }
        out.push_str("</userDefaultRoot>\n");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<userDefaultRoot>
    <entry key="hydra_app_last_version">1805120442</entry>
    <entry key="TAPRESEARCH_IDENTIFIER"/>
    <entry key="options_music_disabled">true</entry>
    <entry key="user_origin_id">314753482635337</entry>
</userDefaultRoot>
"#;

    #[test]
    fn test_parse_and_serialize() {
        let entries = UserDefault::parse_xml(SAMPLE_XML).unwrap();
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].key, "hydra_app_last_version");
        assert_eq!(entries[0].value.as_deref(), Some("1805120442"));

        assert_eq!(entries[1].key, "TAPRESEARCH_IDENTIFIER");
        assert_eq!(entries[1].value, None);

        assert_eq!(entries[2].key, "options_music_disabled");
        assert_eq!(entries[2].value.as_deref(), Some("true"));

        let serialized = UserDefault::serialize_entries(&entries);
        assert!(serialized.contains(r#"<entry key="TAPRESEARCH_IDENTIFIER"/>"#));
        assert!(serialized.contains(r#"<entry key="options_music_disabled">true</entry>"#));
    }
}
