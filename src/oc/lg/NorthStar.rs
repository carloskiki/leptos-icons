#[cfg(feature = "OcLgNorthStar")]
use leptos::*;
#[cfg(feature = "OcLgNorthStar")]
///This icon requires the feature `OcLgNorthStar` to be enabled.
#[component]
pub fn NorthStar(
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
        "M12.5 1.25a.75.75 0 0 0-1.5 0v8.69L6.447 5.385a.75.75 0 1 0-1.061 1.06L9.94 11H1.25a.75.75 0 0 0 0 1.5h8.69l-4.554 4.553a.75.75 0 0 0 1.06 1.061L11 13.561v8.689a.75.75 0 0 0 1.5 0v-8.69l4.553 4.554a.75.75 0 0 0 1.061-1.06L13.561 12.5h8.689a.75.75 0 0 0 0-1.5h-8.69l4.554-4.553a.75.75 0 1 0-1.06-1.061L12.5 9.939V1.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
