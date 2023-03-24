#[cfg(feature = "BiRegularDish")]
use leptos::*;
#[cfg(feature = "BiRegularDish")]
///This icon requires the feature `BiRegularDish` to be enabled.
#[component]
pub fn Dish(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21 15c0-4.625-3.507-8.441-8-8.941V4h-2v2.059c-4.493.5-8 4.316-8 8.941v2h18v-2zM5 15c0-3.859 3.141-7 7-7s7 3.141 7 7H5zm-3 3h20v2H2z"
        /> < title > { title } < / title > < / svg >
    }
}
