#[cfg(feature = "ImTrophy")]
use leptos::*;
#[cfg(feature = "ImTrophy")]
///This icon requires the feature `ImTrophy` to be enabled.
#[component]
pub fn Trophy(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M13 3v-2h-10v2h-3v2c0 1.657 1.343 3 3 3 0.314 0 0.616-0.048 0.9-0.138 0.721 1.031 1.822 1.778 3.1 2.037v3.1h-1c-1.105 0-2 0.895-2 2h8c0-1.105-0.895-2-2-2h-1v-3.1c1.278-0.259 2.378-1.006 3.1-2.037 0.284 0.089 0.587 0.138 0.9 0.138 1.657 0 3-1.343 3-3v-2h-3zM3 6.813c-0.999 0-1.813-0.813-1.813-1.813v-1h1.813v1c0 0.628 0.116 1.229 0.327 1.782-0.106 0.019-0.216 0.030-0.327 0.030zM14.813 5c0 0.999-0.813 1.813-1.813 1.813-0.112 0-0.221-0.011-0.327-0.030 0.211-0.554 0.327-1.154 0.327-1.782v-1h1.813v1z"
        /> < title > { title } < / title > < / svg >
    }
}
