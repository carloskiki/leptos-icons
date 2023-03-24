#[cfg(feature = "TiSocialTumblerCircular")]
use leptos::*;
#[cfg(feature = "TiSocialTumblerCircular")]
///This icon requires the feature `TiSocialTumblerCircular` to be enabled.
#[component]
pub fn SocialTumblerCircular(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M14.377 15.59v-1.234c-.399.268-.788.4-1.166.4-.178 0-.377-.057-.6-.166-.134-.09-.211-.189-.234-.301-.066-.133-.1-.422-.1-.867v-1.966h1.834v-1.233h-1.834v-1.967h-1.066c-.089.467-.178.8-.267 1-.11.244-.288.467-.533.666-.245.201-.5.345-.767.434v1.101h.833v2.7c0 .311.044.576.134.799.066.178.199.355.4.533.154.156.377.289.666.4.355.09.666.133.934.133.311 0 .6-.033.866-.1.312-.067.612-.178.9-.332"
        />< path d =
        "M12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
