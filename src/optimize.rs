use std::str::from_utf8;

use anyhow::{anyhow, Result};
use svgcleaner::cleaner::{clean_doc, parse_data, write_buffer};
use svgcleaner::{CleaningOptions, ParseOptions, WriteOptions};

pub(crate) fn optimize(icon_content: String) -> Result<String> {
    let mut parse_options = ParseOptions::default();
    parse_options.parse_comments = true;
    parse_options.parse_unknown_elements = false;

    let cleaning_options = CleaningOptions {
        remove_unused_defs: true,
        convert_shapes: true,
        remove_title: true,
        remove_desc: true,
        remove_metadata: true,
        remove_dupl_linear_gradients: true,
        remove_dupl_radial_gradients: true,
        remove_dupl_fe_gaussian_blur: true,
        ungroup_groups: true,
        ungroup_defs: true,
        group_by_style: false,
        merge_gradients: true,
        regroup_gradient_stops: false,
        remove_invalid_stops: true,
        remove_invisible_elements: true,
        resolve_use: true,
        remove_version: true,
        remove_unreferenced_ids: true,
        trim_ids: true,
        remove_text_attributes: true,
        remove_unused_coordinates: true,
        remove_default_attributes: true,
        remove_xmlns_xlink_attribute: true,
        remove_needless_attributes: true,
        remove_gradient_attributes: true,
        join_style_attributes: svgcleaner::StyleJoinMode::None,
        apply_transform_to_gradients: true,
        apply_transform_to_shapes: true,
        paths_to_relative: true,
        remove_unused_segments: true,
        convert_segments: true,
        apply_transform_to_paths: true,
        coordinates_precision: 6,
        properties_precision: 6,
        paths_coordinates_precision: 8,
        transforms_precision: 8,
    };

    let write_options = WriteOptions::default();

    let mut doc = parse_data(&icon_content, &parse_options)
        .map_err(|_| anyhow!("could not parse the svg data during optimization"))?;

    clean_doc(&mut doc, &cleaning_options, &write_options)
        .map_err(|_| anyhow!("could not clean the svg data during optimization"))?;

    let mut output_buffer = Vec::new();
    write_buffer(&doc, &write_options, &mut output_buffer);

    Ok(from_utf8(&output_buffer)?.to_owned())
}
