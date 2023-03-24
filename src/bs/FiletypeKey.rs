#[cfg(feature = "BsFiletypeKey")]
use leptos::*;
#[cfg(feature = "BsFiletypeKey")]
///This icon requires the feature `BsFiletypeKey` to be enabled.
#[component]
pub fn FiletypeKey(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-filetype-key" viewBox = "0 0 16 16" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" d =
        "M14 4.5V14a2 2 0 0 1-2 2h-1v-1h1a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM3.21 11.85h-.87L.83 13.64H.79v-1.79H0v3.999h.791v-1.283l.41-.466 1.12 1.749h.951l-1.488-2.276 1.427-1.723Zm2.903 3.352h-1.79v-1.073h1.685v-.606H4.323v-1.025h1.79v-.648H3.538v3.999h2.575v-.647Zm2.243-.888v1.535h-.794v-1.52L6.223 11.85H7.1l.853 1.696h.032l.855-1.696h.856l-1.339 2.464Z"
        /> < title > { title } < / title > < / svg >
    }
}
