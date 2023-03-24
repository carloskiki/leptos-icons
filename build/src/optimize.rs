use std::collections::HashMap;
use std::fs::File;
use std::str::from_utf8;
use tracing::instrument;
use xml::{ParserConfig, EmitterConfig};
use anyhow::{anyhow, Result};

#[instrument(level = "info")]
pub(crate) fn optimize(icon_content: File) -> Result<String> {

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

    reader.into_iter().map(|event| {
        match event.map_err(|_| anyhow!("optimization reading error"))? {
            xml::reader::XmlEvent::EndDocument => Ok(()),
            xml::reader::XmlEvent::StartElement { name, .. } if name.local_name == "title" =>  {
                is_title = true;
                Ok(())
            }
            xml::reader::XmlEvent::EndElement { name } if name.local_name == "title" => {
                is_title = false;
                Ok(())
            }
            event => {
                if is_title {
                    return Ok(());
                }
                writer.write(event.as_writer_event().unwrap()).map_err(|_| anyhow!(" optimization writing error"))
            }
        }
    }).collect::<Result<()>>()?;


    Ok(from_utf8(writer.inner_mut())?.to_owned())

    // let optimization_output = Command::new("svgo").arg("-s").arg(icon_content).output()?;
    // Ok(from_utf8(&optimization_output.stdout)?.to_owned())
}
