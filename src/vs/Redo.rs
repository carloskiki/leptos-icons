#[cfg(feature = "VsRedo")]
use leptos::*;
#[cfg(feature = "VsRedo")]
///This icon requires the feature `VsRedo` to be enabled.
#[component]
pub fn Redo(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M12.5 2v3.5L12 6H8.5V5h2.521l-.941-.941a3.552 3.552 0 1 0-5.023 5.023l5.197 5.198-.72.72-5.198-5.198A4.57 4.57 0 0 1 10.8 3.339l.7.7V2h1z"
        /> < title > { title } < / title > < / svg >
    }
}
