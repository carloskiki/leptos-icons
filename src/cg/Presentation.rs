#[cfg(feature = "CgPresentation")]
use leptos::*;
#[cfg(feature = "CgPresentation")]
///This icon requires the feature `CgPresentation` to be enabled.
#[component]
pub fn Presentation(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22ZM20 12H18C18 8.68629 15.3137 6 12 6C8.68629 6 6 8.68629 6 12H4C4 7.58172 7.58172 4 12 4C16.4183 4 20 7.58172 20 12ZM4.25204 14H19.748C18.8599 17.4505 15.7277 20 12 20C8.27232 20 5.14012 17.4505 4.25204 14ZM8 12C8 9.79086 9.79086 8 12 8C14.2091 8 16 9.79086 16 12H8Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
