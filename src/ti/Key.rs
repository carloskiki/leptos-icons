#[cfg(feature = "TiKey")]
use leptos::*;
#[cfg(feature = "TiKey")]
///This icon requires the feature `TiKey` to be enabled.
#[component]
pub fn Key(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.5 11c0 .732.166 1.424.449 2.051l-3.949 3.949v1.5s.896 1.5 2 1.5h2v-2h2v-2h2.5c2.762 0 5-2.238 5-5s-2.238-5-5-5-5 2.238-5 5zm5 2c-1.104 0-2-.896-2-2 0-1.105.896-2.002 2-2.002 1.105 0 2 .896 2 2.002 0 1.104-.895 2-2 2z"
        /> < title > { title } < / title > < / svg >
    }
}
