#[cfg(feature = "BiRegularRestaurant")]
use leptos::*;
#[cfg(feature = "BiRegularRestaurant")]
///This icon requires the feature `BiRegularRestaurant` to be enabled.
#[component]
pub fn Restaurant(
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
        "M12 10h-2V3H8v7H6V3H4v8c0 1.654 1.346 3 3 3h1v7h2v-7h1c1.654 0 3-1.346 3-3V3h-2v7zm7-7h-1c-1.159 0-2 1.262-2 3v8h2v7h2V4a1 1 0 0 0-1-1z"
        /> < title > { title } < / title > < / svg >
    }
}
