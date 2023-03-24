#[cfg(feature = "TiSocialInstagramCircular")]
use leptos::*;
#[cfg(feature = "TiSocialInstagramCircular")]
///This icon requires the feature `TiSocialInstagramCircular` to be enabled.
#[component]
pub fn SocialInstagramCircular(
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
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 3c-5 0-9 4-9 9s4 9 9 9 9-4 9-9-4-9-9-9zm0 7c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm2.8-2c0-.7.6-1.2 1.2-1.2s1.2.6 1.2 1.2-.5 1.2-1.2 1.2-1.2-.5-1.2-1.2zm-2.8 11c-3.9 0-7-3.1-7-7h3c0 2.2 1.8 4 4 4s4-1.8 4-4h3c0 3.9-3.1 7-7 7z"
        /> < title > { title } < / title > < / svg >
    }
}
