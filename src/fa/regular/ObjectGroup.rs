#[cfg(feature = "FaRegularObjectGroup")]
use leptos::*;
#[cfg(feature = "FaRegularObjectGroup")]
///This icon requires the feature `FaRegularObjectGroup` to be enabled.
#[component]
pub fn ObjectGroup(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 115.8C6.2 107 0 94.2 0 80C0 53.5 21.5 32 48 32c14.2 0 27 6.2 35.8 16H428.2c8.8-9.8 21.6-16 35.8-16c26.5 0 48 21.5 48 48c0 14.2-6.2 27-16 35.8V396.2c9.8 8.8 16 21.6 16 35.8c0 26.5-21.5 48-48 48c-14.2 0-27-6.2-35.8-16H83.8C75 473.8 62.2 480 48 480c-26.5 0-48-21.5-48-48c0-14.2 6.2-27 16-35.8V115.8zM93.3 96c-4.8 13.6-15.6 24.4-29.3 29.3V386.7c13.6 4.8 24.4 15.6 29.3 29.3H418.7c4.8-13.6 15.6-24.4 29.3-29.3V125.3c-13.6-4.8-24.4-15.6-29.3-29.3H93.3zM96 160c0-17.7 14.3-32 32-32H256c17.7 0 32 14.3 32 32v96c0 17.7-14.3 32-32 32H128c-17.7 0-32-14.3-32-32V160zM224 320h32c35.3 0 64-28.7 64-64V224h64c17.7 0 32 14.3 32 32v96c0 17.7-14.3 32-32 32H256c-17.7 0-32-14.3-32-32V320z"
        /> < title > { title } < / title > < / svg >
    }
}
