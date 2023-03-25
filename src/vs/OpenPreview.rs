#[cfg(feature = "VsOpenPreview")]
use leptos::*;
#[cfg(feature = "VsOpenPreview")]
///This icon requires the feature `VsOpenPreview` to be enabled.
#[component]
pub fn OpenPreview(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3 1h11l1 1v5.3a3.21 3.21 0 0 0-1-.3V2H9v10.88L7.88 14H3l-1-1V2l1-1zm0 12h5V2H3v11zm10.379-4.998a2.53 2.53 0 0 0-1.19.348h-.03a2.51 2.51 0 0 0-.799 3.53L9 14.23l.71.71 2.35-2.36c.325.22.7.358 1.09.4a2.47 2.47 0 0 0 1.14-.13 2.51 2.51 0 0 0 1-.63 2.46 2.46 0 0 0 .58-1 2.63 2.63 0 0 0 .07-1.15 2.53 2.53 0 0 0-1.35-1.81 2.53 2.53 0 0 0-1.211-.258zm.24 3.992a1.5 1.5 0 0 1-.979-.244 1.55 1.55 0 0 1-.56-.68 1.49 1.49 0 0 1-.08-.86 1.49 1.49 0 0 1 1.18-1.18 1.49 1.49 0 0 1 .86.08c.276.117.512.311.68.56a1.5 1.5 0 0 1-1.1 2.324z"
        /> < title > { title } < / title > < / svg >
    }
}
