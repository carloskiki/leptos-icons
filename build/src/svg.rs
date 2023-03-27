use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::str::from_utf8;
use tracing::instrument;
use xml::{attribute::OwnedAttribute, EmitterConfig, ParserConfig};

#[derive(Debug)]
pub(crate) struct ParsedSvg {
    pub content: String,
    pub attributes: Vec<OwnedAttribute>,
}

impl ParsedSvg {
    #[instrument(level = "info", skip(icon_content))]
    pub(crate) fn parse<R: std::io::Read>(icon_content: R) -> Result<ParsedSvg> {
        let parser_config = ParserConfig {
            trim_whitespace: true,
            whitespace_to_characters: false,
            cdata_to_characters: false,
            ignore_comments: true,
            coalesce_characters: false,
            extra_entities: HashMap::new(),
            ignore_end_of_stream: false,
            replace_unknown_entity_references: true,
            ignore_root_level_whitespace: true,
        };

        let emitter_config = EmitterConfig::default().write_document_declaration(false);

        let mut writer = emitter_config.create_writer(Vec::new());
        let mut reader = parser_config.create_reader(icon_content);

        // skip XML document declaration
        reader.next()?;

        let mut is_title = false;
        let mut svg_attributes: Vec<OwnedAttribute> = Vec::new();
        let mut in_content = false;

        for event in reader.into_iter() {
            let event = event.map_err(|_| anyhow!("optimization reading error"))?;
            match event {
                xml::reader::XmlEvent::EndDocument => (),
                xml::reader::XmlEvent::StartElement { name, .. } if name.local_name == "title" => {
                    is_title = true;
                }
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == "svg" => {
                    // TODO: parse attributes
                    svg_attributes.extend(attributes.into_iter());
                    in_content = true;
                }
                xml::reader::XmlEvent::EndElement { name } if name.local_name == "svg" => {
                    break;
                }
                xml::reader::XmlEvent::EndElement { name } if name.local_name == "title" => {
                    is_title = false;
                }
                event => {
                    if is_title || !in_content {
                        continue;
                    }
                    writer
                        .write(event.as_writer_event().unwrap())
                        .map_err(|_| anyhow!(" optimization writing error"))?
                }
            }
        }

        Ok(ParsedSvg {
            content: from_utf8(writer.inner_mut())?.to_owned(),
            attributes: svg_attributes,
        })
    }
}
