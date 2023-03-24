#[cfg(feature = "TiLocationArrowOutline")]
use leptos::*;
#[cfg(feature = "TiLocationArrowOutline")]
///This icon requires the feature `TiLocationArrowOutline` to be enabled.
#[component]
pub fn LocationArrowOutline(
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
        "M11.087 20.914c-.353 0-1.219-.146-1.668-1.496l-1.209-3.627-3.628-1.209c-1.244-.415-1.469-1.172-1.493-1.587s.114-1.193 1.302-1.747l11.375-5.309c1.031-.479 1.922-.309 2.348.362.224.351.396.97-.053 1.933l-5.309 11.375c-.529 1.135-1.272 1.305-1.665 1.305zm-5.39-8.068l4.094 1.363 1.365 4.093 4.775-10.233-10.234 4.777z"
        /> < title > { title } < / title > < / svg >
    }
}
