#[cfg(feature = "AiOutlinedBorderlessTable")]
use leptos::*;
#[cfg(feature = "AiOutlinedBorderlessTable")]
///This icon requires the feature `AiOutlinedBorderlessTable` to be enabled.
#[component]
pub fn BorderlessTable(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style t = "1569683537953" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "10937" width = "200" height = "200"
        width = { size.clone() } height = { size } > < defs xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" ><
        style type = "text/css" /></ defs >< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M117 368h231v64H117zM676 368h241v64H676zM412 368h200v64H412zM412 592h200v64H412zM676 592h241v64H676zM117 592h231v64H117zM412 432V179h-64v666h64V592zM676 368V179h-64v666h64V432z"
        p - id = "10938" /> < title > { title } < / title > < / svg >
    }
}
