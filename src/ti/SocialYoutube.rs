#[cfg(feature = "TiSocialYoutube")]
use leptos::*;
#[cfg(feature = "TiSocialYoutube")]
///This icon requires the feature `TiSocialYoutube` to be enabled.
#[component]
pub fn SocialYoutube(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M22.8 8.6c-.2-1.5-.4-2.6-1-3-.6-.5-5.8-.6-9.8-.6s-9.2.1-9.8.6c-.6.4-.8 1.5-1 3s-.2 2.4-.2 3.4 0 1.9.2 3.4.4 2.6 1 3c.6.5 5.8.6 9.8.6 4 0 9.2-.1 9.8-.6.6-.4.8-1.5 1-3s.2-2.4.2-3.4 0-1.9-.2-3.4zm-12.8 7v-7.2l6 3.6-6 3.6z"
        /> < title > { title } < / title > < / svg >
    }
}
