#[cfg(feature = "VsEditorLayout")]
use leptos::*;
#[cfg(feature = "VsEditorLayout")]
///This icon requires the feature `VsEditorLayout` to be enabled.
#[component]
pub fn EditorLayout(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 6.5l-.47-.5H7V1.47L6.53 1H1.47L1 1.47v8.06l.47.47H4v4.53l.47.47h10.06l.47-.47V6.5zM2 9V3h4v6H2zm12 5H5v-4h1.53L7 9.53V8.013h7V14z"
        /> < title > { title } < / title > < / svg >
    }
}
