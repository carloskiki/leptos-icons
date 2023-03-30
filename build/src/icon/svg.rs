use anyhow::{anyhow, Result};
use std::str::from_utf8;
use std::{borrow::Cow, collections::HashMap};
use xml::{attribute::OwnedAttribute, EmitterConfig, ParserConfig};

#[derive(Debug)]
pub(crate) struct ParsedSvg {
    pub content: String,
    pub attributes: Vec<OwnedAttribute>,
}

impl ParsedSvg {
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

        let emitter_config = EmitterConfig {
            line_separator: "\n".into(),
            indent_string: "  ".into(),
            perform_indent: true,
            perform_escaping: true,
            write_document_declaration: false,
            normalize_empty_elements: true,
            cdata_to_characters: false,
            keep_element_names_stack: true,
            autopad_comments: true,
            pad_self_closing: true,
        };

        let mut reader = parser_config.create_reader(icon_content);
        let mut writer = emitter_config.create_writer(Vec::new());

        // skip XML document declaration
        reader.next()?;

        let mut is_title = false;
        let mut svg_attributes: Vec<OwnedAttribute> = Vec::new();
        let mut xmlns = None;
        let mut xmlns_xlink = None;
        let mut in_content = false;

        for event in reader.into_iter() {
            let event = event.map_err(|err| anyhow!("Error reading XML event: {err}"))?;
            match event {
                xml::reader::XmlEvent::EndDocument => (),
                xml::reader::XmlEvent::StartElement { name, .. } if name.local_name == "title" => {
                    is_title = true;
                }
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == "svg" => {
                    // TODO: parse attributes
                    xmlns = attributes
                        .iter()
                        .find(|it| {
                            println!("Attr, {it:?}");
                            it.name.prefix == Some("xmlns".to_owned())
                        })
                        .cloned();
                    println!("Found, {xmlns:?}");
                    xmlns_xlink = attributes
                        .iter()
                        .find(|it| it.name.local_name == "xmlns:xlink")
                        .cloned();
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
                    let event = event.as_writer_event().unwrap();
                    match event {
                        xml::writer::XmlEvent::StartDocument {
                            version,
                            encoding,
                            standalone,
                        } => todo!(),
                        xml::writer::XmlEvent::ProcessingInstruction { name, data } => todo!(),
                        xml::writer::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        } => writer
                            .write(xml::writer::XmlEvent::StartElement {
                                name,
                                attributes: Cow::Owned(
                                    attributes
                                        .into_owned()
                                        .into_iter()
                                        .filter(|it| it.name.prefix != Some("xmlns"))
                                        .filter(|it| it.name.local_name != "xlink")
                                        .collect::<Vec<_>>(),
                                ),
                                namespace,
                            })
                            .map_err(|_| anyhow!(" optimization writing error"))?,
                        xml::writer::XmlEvent::EndElement { name } => writer
                            .write(event)
                            .map_err(|_| anyhow!(" optimization writing error"))?,
                        xml::writer::XmlEvent::CData(_) => todo!(),
                        xml::writer::XmlEvent::Comment(_) => todo!(),
                        xml::writer::XmlEvent::Characters(_) => todo!(),
                    }
                }
            }
        }

        assert!(xmlns.is_some());

        Ok(ParsedSvg {
            /// On Windows systems, a small percentage of icons might be rendered with "&#xD;&#xA;" instead of "&#xA;".
            /// This happens when the svg file contained windows-style line breaks.
            /// TODO: Find a better way of ensuring consistent output across different system architectures.
            /// TODO: We are using `EmitterConfig::default().line_separator("\n")`, which does not help on its own. Why?
            content: from_utf8(writer.inner_mut())?
                .to_owned()
                .replace("&#xD;&#xA;", "&#xA;"),
            attributes: svg_attributes,
        })
    }
}

#[cfg(test)]
mod test {
    use super::ParsedSvg;

    #[test]
    fn parse() {
        // Let's assume we have an icon file which is using windows-style line breaks (`/r/n`, `crlf`)!
        // NOTE: This is the icon "ColorWand" icon from the "ionicons" package, which has `/r/n` newlines.
        // We have to enforce this here with an explicit .replace, as all .rs files are git-committed with an enforced `lf`-style!
        let original = indoc::indoc! { r#"
            <?xml version="1.0" encoding="utf-8"?>
            <!-- Generator: Adobe Illustrator 24.3.0, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
            <svg version="1.1" id="icons" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
                viewBox="0 0 512 512" style="enable-background:new 0 0 512 512;" xml:space="preserve">
            <path d="M96,208H48c-8.8,0-16-7.2-16-16s7.2-16,16-16h48c8.8,0,16,7.2,16,16S104.8,208,96,208z"/>
            <path d="M124.1,140.1c-4.2,0-8.3-1.7-11.3-4.7l-33.9-33.9c-6.2-6.2-6.2-16.4,0-22.6s16.4-6.2,22.6,0l33.9,33.9
                c6.3,6.2,6.3,16.4,0,22.6C132.4,138.4,128.4,140.1,124.1,140.1z"/>
            <path d="M192,112c-8.8,0-16-7.2-16-16V48c0-8.8,7.2-16,16-16s16,7.2,16,16v48C208,104.8,200.8,112,192,112z"/>
            <path d="M259.9,140.1c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0c6.2,6.2,6.2,16.4,0,22.6
                l-33.9,33.9C268.2,138.4,264.1,140.1,259.9,140.1z"/>
            <path d="M90.2,309.8c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0s6.2,16.4,0,22.6l-33.9,33.9
                C98.5,308.1,94.4,309.8,90.2,309.8z"/>
            <path d="M234.2,167c-18.4-18.7-48.5-19-67.2-0.7s-19,48.5-0.7,67.2c0.2,0.2,0.5,0.5,0.7,0.7l39.5,39.5c3.1,3.1,8.2,3.1,11.3,0
                l55.9-55.9c3.1-3.1,3.1-8.2,0-11.3L234.2,167z"/>
            <path d="M457,389.8L307.6,240.4c-3.1-3.1-8.2-3.1-11.3,0l-55.9,55.9c-3.1,3.1-3.1,8.2,0,11.3L389.8,457c18.4,18.7,48.5,19,67.2,0.7
                c18.7-18.4,19-48.5,0.7-67.2C457.5,390.3,457.3,390,457,389.8L457,389.8z"/>
            </svg>
        "# }.replace("\n", "\r\n");

        // When parsing such a file.
        let out = ParsedSvg::parse(original.as_bytes()).expect("no errors");

        // We expect all conversions to be made to just "&#xA;" (representing \n) instead of "&#xD;&#xA;" (representing \r\n).
        pretty_assertions::assert_eq!(
            indoc::indoc! { r#"
                <path d="M96,208H48c-8.8,0-16-7.2-16-16s7.2-16,16-16h48c8.8,0,16,7.2,16,16S104.8,208,96,208z" />
                <path d="M124.1,140.1c-4.2,0-8.3-1.7-11.3-4.7l-33.9-33.9c-6.2-6.2-6.2-16.4,0-22.6s16.4-6.2,22.6,0l33.9,33.9&#xA;    c6.3,6.2,6.3,16.4,0,22.6C132.4,138.4,128.4,140.1,124.1,140.1z" />
                <path d="M192,112c-8.8,0-16-7.2-16-16V48c0-8.8,7.2-16,16-16s16,7.2,16,16v48C208,104.8,200.8,112,192,112z" />
                <path d="M259.9,140.1c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0c6.2,6.2,6.2,16.4,0,22.6&#xA;    l-33.9,33.9C268.2,138.4,264.1,140.1,259.9,140.1z" />
                <path d="M90.2,309.8c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0s6.2,16.4,0,22.6l-33.9,33.9&#xA;    C98.5,308.1,94.4,309.8,90.2,309.8z" />
                <path d="M234.2,167c-18.4-18.7-48.5-19-67.2-0.7s-19,48.5-0.7,67.2c0.2,0.2,0.5,0.5,0.7,0.7l39.5,39.5c3.1,3.1,8.2,3.1,11.3,0&#xA;    l55.9-55.9c3.1-3.1,3.1-8.2,0-11.3L234.2,167z" />
                <path d="M457,389.8L307.6,240.4c-3.1-3.1-8.2-3.1-11.3,0l-55.9,55.9c-3.1,3.1-3.1,8.2,0,11.3L389.8,457c18.4,18.7,48.5,19,67.2,0.7&#xA;    c18.7-18.4,19-48.5,0.7-67.2C457.5,390.3,457.3,390,457,389.8L457,389.8z" />"# },
            out.content,
        )
    }
}
