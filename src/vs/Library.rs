#[cfg(feature = "VsLibrary")]
use leptos::*;
#[cfg(feature = "VsLibrary")]
///This icon requires the feature `VsLibrary` to be enabled.
#[component]
pub fn Library(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5 2.5l.5-.5h2l.5.5v11l-.5.5h-2l-.5-.5v-11zM6 3v10h1V3H6zm3.171.345l.299-.641 1.88-.684.64.299 3.762 10.336-.299.641-1.879.684-.64-.299L9.17 3.345zm1.11.128l3.42 9.396.94-.341-3.42-9.397-.94.342zM1 2.5l.5-.5h2l.5.5v11l-.5.5h-2l-.5-.5v-11zM2 3v10h1V3H2z"
        /> < title > { title } < / title > < / svg >
    }
}
