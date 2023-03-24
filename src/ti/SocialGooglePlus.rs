#[cfg(feature = "TiSocialGooglePlus")]
use leptos::*;
#[cfg(feature = "TiSocialGooglePlus")]
///This icon requires the feature `TiSocialGooglePlus` to be enabled.
#[component]
pub fn SocialGooglePlus(
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
        stroke_witdh = "0" style = style version = "1" width = "24" height = "24" viewBox
        = "0 0 24 24" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.9 13.5l-.7-.5c-.2-.2-.5-.4-.5-.8s.3-.7.6-1c.8-.6 1.7-1.3 1.7-2.8 0-1.5-.9-2.3-1.4-2.7h1.2l1.2-.7h-4.1c-1 0-2.4.2-3.5 1.1-.8.7-1.2 1.7-1.2 2.6 0 1.5 1.2 3.1 3.3 3.1h.6c-.1.2-.2.4-.2.7 0 .6.3 1 .6 1.3-.9.1-2.5.2-3.8.9-1.2.7-1.5 1.7-1.5 2.4 0 1.5 1.4 2.8 4.2 2.8 3.4 0 5.2-1.9 5.2-3.7 0-1.3-.8-1.9-1.7-2.7zm-2.5-2.2c-1.7 0-2.5-2.2-2.5-3.5 0-.5.1-1 .4-1.5.3-.4.9-.7 1.4-.7 1.6 0 2.5 2.2 2.5 3.6 0 .4 0 1-.5 1.4-.3.4-.9.7-1.3.7zm0 7.9c-2.1 0-3.5-1-3.5-2.4s1.3-1.9 1.7-2c.8-.3 1.9-.3 2.1-.3h.5c1.5 1.1 2.1 1.6 2.1 2.6 0 1.2-1 2.1-2.9 2.1zM17 12h-2v-1h2v-1.9l1-.1v2h2v1h-2v2h-1z"
        /> < title > { title } < / title > < / svg >
    }
}
