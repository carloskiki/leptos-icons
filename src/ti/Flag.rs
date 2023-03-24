#[cfg(feature = "TiFlag")]
use leptos::*;
#[cfg(feature = "TiFlag")]
///This icon requires the feature `TiFlag` to be enabled.
#[component]
pub fn Flag(
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
        "M18.383 4.318c-.374-.155-.804-.069-1.09.217-1.264 1.263-3.321 1.264-4.586 0-2.045-2.043-5.37-2.043-7.414 0-.188.187-.293.442-.293.707v13c0 .552.447 1 1 1s1-.448 1-1v-4.553c1.271-.997 3.121-.911 4.293.26 2.045 2.043 5.371 2.043 7.414 0 .188-.188.293-.442.293-.707v-8c0-.405-.244-.769-.617-.924z"
        /> < title > { title } < / title > < / svg >
    }
}
