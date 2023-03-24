#[cfg(feature = "AiOutlinedInsertRowAbove")]
use leptos::*;
#[cfg(feature = "AiOutlinedInsertRowAbove")]
///This icon requires the feature `AiOutlinedInsertRowAbove` to be enabled.
#[component]
pub fn InsertRowAbove(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style t = "1569683507961" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "10819" width = "200" height = "200"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < defs
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" >< style type = "text/css" /></ defs >< path xmlns
        = "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M878.7 336H145.3c-18.4 0-33.3 14.3-33.3 32v464c0 17.7 14.9 32 33.3 32h733.3c18.4 0 33.3-14.3 33.3-32V368c0.1-17.7-14.8-32-33.2-32zM360 792H184V632h176v160z m0-224H184V408h176v160z m240 224H424V632h176v160z m0-224H424V408h176v160z m240 224H664V632h176v160z m0-224H664V408h176v160zM904 160H120c-4.4 0-8 3.6-8 8v80c0 4.4 3.6 8 8 8h784c4.4 0 8-3.6 8-8v-80c0-4.4-3.6-8-8-8z"
        p - id = "10820" /> < title > { title } < / title > < / svg >
    }
}
