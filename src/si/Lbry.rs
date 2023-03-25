#[cfg(feature = "SiLbry")]
use leptos::*;
#[cfg(feature = "SiLbry")]
///This icon requires the feature `SiLbry` to be enabled.
#[component]
pub fn Lbry(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M23.3493 14.1894l.169-1.0651-1.0315-.1775.0676-.4142 1.4456.245-.2365 1.4795zm.152-5.495v1.1921l-11.7338 7.211-8.8425-4.3367.0169-.6677 8.7918 4.3282 11.1759-6.8644v-.4904L12.3592 3.9773.5917 11.2561v3.2547l11.142 5.5119 11.6322-7.135.33.5074-11.9284 7.3038L0 14.8828v-3.9563L12.3254 3.301z"
        /> < title > { title } < / title > < / svg >
    }
}
