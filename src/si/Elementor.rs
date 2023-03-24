#[cfg(feature = "SiElementor")]
use leptos::*;
#[cfg(feature = "SiElementor")]
///This icon requires the feature `SiElementor` to be enabled.
#[component]
pub fn Elementor(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12 0C5.372 0 0 5.372 0 12c0 6.626 5.372 12 12 12s12-5.372 12-12c0-6.626-5.372-12-12-12ZM9 17H7V7H9Zm8 0H11V15h6Zm0-4H11V11h6Zm0-4H11V7h6Z"
        /> < title > { title } < / title > < / svg >
    }
}
