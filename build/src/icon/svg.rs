use anyhow::{anyhow, Result};
use std::str::from_utf8;
use std::{borrow::Cow, collections::HashMap};
use tracing::warn;
use xml::common::XmlVersion;
use xml::namespace::Namespace;
use xml::{attribute::OwnedAttribute, EmitterConfig, ParserConfig};

#[derive(Debug)]
pub(crate) struct ParsedSvg {
    pub content: String,
    #[allow(unused)]
    pub xml_attributes: XmlAttributes,
    pub svg_attributes: SvgAttributes,
}

/// Parsed attributes of the xml root element.
#[derive(Debug)]
pub(crate) struct XmlAttributes {
    #[allow(unused)]
    pub version: XmlVersion,
    #[allow(unused)]
    pub encoding: String,
}

/// Parsed attributes of the svg element.
#[derive(Debug)]
pub(crate) struct SvgAttributes {
    #[allow(unused)]
    pub namespace: Namespace,
    #[allow(unused)]
    pub version: Option<OwnedAttribute>,
    #[allow(unused)]
    pub class: Option<OwnedAttribute>,
    pub x: Option<OwnedAttribute>,
    pub y: Option<OwnedAttribute>,
    #[allow(unused)]
    pub width: Option<OwnedAttribute>,
    #[allow(unused)]
    pub height: Option<OwnedAttribute>,
    pub view_box: Option<OwnedAttribute>,
    pub stroke_linecap: Option<OwnedAttribute>,
    pub stroke_linejoin: Option<OwnedAttribute>,
    pub stroke_width: Option<OwnedAttribute>,
    pub stroke: Option<OwnedAttribute>,
    pub fill: Option<OwnedAttribute>,
    pub style: Option<OwnedAttribute>,
    pub role: Option<OwnedAttribute>,
    #[allow(unused)]
    pub unknown_attributes: Vec<OwnedAttribute>,
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

        let reader = parser_config.create_reader(icon_content);
        let mut writer = emitter_config.create_writer(Vec::new());

        let mut xml_attributes = None;
        let mut svg_version = None;
        let mut svg_class = None;
        let mut svg_x = None;
        let mut svg_y = None;
        let mut svg_width = None;
        let mut svg_height = None;
        let mut svg_view_box = None;
        let mut svg_stroke_linecap = None;
        let mut svg_stroke_linejoin = None;
        let mut svg_stroke_width = None;
        let mut svg_stroke = None;
        let mut svg_fill = None;
        let mut svg_style = None;
        let mut svg_role = None;
        let mut svg_namespace = None;
        let mut unknown_svg_attributes: Vec<OwnedAttribute> = Vec::new();

        let mut is_title = false;
        let mut in_content = false;

        for event in reader.into_iter() {
            let event = event.map_err(|err| anyhow!("Error reading XML event: {err}"))?;
            match event {
                xml::reader::XmlEvent::StartDocument {
                    version,
                    encoding,
                    standalone: _,
                } => {
                    xml_attributes = Some(XmlAttributes { version, encoding });
                }
                xml::reader::XmlEvent::EndDocument => {}
                xml::reader::XmlEvent::StartElement { name, .. } if name.local_name == "title" => {
                    is_title = true;
                }
                xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } if name.local_name == "svg" => {
                    svg_namespace = Some(namespace);
                    for attr in attributes {
                        match attr.name.local_name.as_ref() {
                            "version" => svg_version = Some(attr),
                            // We explicitly ignore any id attribute, as ids must be unique and we cannot ensure that. Users should provide an id if required.
                            "id" => {}
                            // As to https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xml:space, we explicitly ignore any "space" attribute.
                            "space" => {}
                            // As to https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseProfile, we explicitly ignore any "baseProfile" attribute.
                            "baseProfile" => {}
                            // This is an unrecognizable attribute which we can safely ignore.
                            "t" => {}
                            // This is an unrecognizable attribute which we can safely ignore.
                            "p-id" => {}
                            "class" => svg_class = Some(attr),
                            // TODO; As to https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x, the default for x and y on svg elements is 0. We could safely ignore them then. Use regex?
                            "x" => svg_x = Some(attr),
                            "y" => svg_y = Some(attr),
                            "width" => svg_width = Some(attr),
                            "height" => svg_height = Some(attr),
                            "viewBox" => svg_view_box = Some(attr),
                            "stroke-linecap" => svg_stroke_linecap = Some(attr),
                            "stroke-linejoin" => svg_stroke_linejoin = Some(attr),
                            "stroke-width" => svg_stroke_width = Some(attr),
                            "stroke" => svg_stroke = Some(attr),
                            "fill" => svg_fill = Some(attr),
                            "style" => svg_style = Some(attr),
                            "role" => svg_role = Some(attr),
                            _ => {
                                warn!(?attr, "Encountered an unexpected svg attribute");
                                unknown_svg_attributes.push(attr)
                            }
                        };
                    }
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
                        xml::writer::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace: _,
                        } => {
                            writer
                                .write(xml::writer::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    // namespace will be non empty, when the initially read svg element contained that information.
                                    // We are only writing the inner parts of an svg (we reconstruct an <svg> element around that later)
                                    // and therefore do not want namespace information to be emitted on any child element.
                                    namespace: Cow::Owned(Namespace::empty()),
                                })
                                .map_err(|err| anyhow!("Error writing XML event: {err}"))?
                        }
                        other => writer
                            .write(other)
                            .map_err(|err| anyhow!("Error writing XML event: {err}"))?,
                    }
                }
            }
        }

        Ok(ParsedSvg {
            /// On Windows systems, a small percentage of icons might be rendered with "&#xD;&#xA;" instead of "&#xA;".
            /// This seems to happens when the svg file contained windows-style line breaks.
            /// TODO: Find a better way of ensuring consistent output across different system architectures.
            /// TODO: We are using `EmitterConfig::default().line_separator("\n")`, which does not help on its own. Why?
            content: from_utf8(writer.inner_mut())?
                .to_owned()
                .replace("&#xD;&#xA;", "\n")
                .replace("&#xA;", "\n"),
            xml_attributes: xml_attributes.expect("present"),
            svg_attributes: SvgAttributes {
                namespace: svg_namespace.expect("present"),
                version: svg_version,
                class: svg_class,
                x: svg_x,
                y: svg_y,
                width: svg_width,
                height: svg_height,
                view_box: svg_view_box,
                stroke_linecap: svg_stroke_linecap,
                stroke_linejoin: svg_stroke_linejoin,
                stroke_width: svg_stroke_width,
                stroke: svg_stroke,
                fill: svg_fill,
                style: svg_style,
                role: svg_role,
                unknown_attributes: unknown_svg_attributes,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::ParsedSvg;

    #[test]
    fn parse_content() {
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
                <path d="M124.1,140.1c-4.2,0-8.3-1.7-11.3-4.7l-33.9-33.9c-6.2-6.2-6.2-16.4,0-22.6s16.4-6.2,22.6,0l33.9,33.9
                    c6.3,6.2,6.3,16.4,0,22.6C132.4,138.4,128.4,140.1,124.1,140.1z" />
                <path d="M192,112c-8.8,0-16-7.2-16-16V48c0-8.8,7.2-16,16-16s16,7.2,16,16v48C208,104.8,200.8,112,192,112z" />
                <path d="M259.9,140.1c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0c6.2,6.2,6.2,16.4,0,22.6
                    l-33.9,33.9C268.2,138.4,264.1,140.1,259.9,140.1z" />
                <path d="M90.2,309.8c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0s6.2,16.4,0,22.6l-33.9,33.9
                    C98.5,308.1,94.4,309.8,90.2,309.8z" />
                <path d="M234.2,167c-18.4-18.7-48.5-19-67.2-0.7s-19,48.5-0.7,67.2c0.2,0.2,0.5,0.5,0.7,0.7l39.5,39.5c3.1,3.1,8.2,3.1,11.3,0
                    l55.9-55.9c3.1-3.1,3.1-8.2,0-11.3L234.2,167z" />
                <path d="M457,389.8L307.6,240.4c-3.1-3.1-8.2-3.1-11.3,0l-55.9,55.9c-3.1,3.1-3.1,8.2,0,11.3L389.8,457c18.4,18.7,48.5,19,67.2,0.7
                    c18.7-18.4,19-48.5,0.7-67.2C457.5,390.3,457.3,390,457,389.8L457,389.8z" />"# },
            out.content,
        )
    }
}
