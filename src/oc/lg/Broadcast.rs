#[cfg(feature = "OcLgBroadcast")]
use leptos::*;
#[cfg(feature = "OcLgBroadcast")]
///This icon requires the feature `OcLgBroadcast` to be enabled.
#[component]
pub fn Broadcast(
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
        "M20.485 2.515a.75.75 0 0 0-1.06 1.06A10.465 10.465 0 0 1 22.5 11c0 2.9-1.174 5.523-3.075 7.424a.75.75 0 0 0 1.06 1.061A11.965 11.965 0 0 0 24 11c0-3.314-1.344-6.315-3.515-8.485Zm-15.91 1.06a.75.75 0 0 0-1.06-1.06A11.965 11.965 0 0 0 0 11c0 3.313 1.344 6.314 3.515 8.485a.75.75 0 0 0 1.06-1.06A10.465 10.465 0 0 1 1.5 11c0-2.9 1.174-5.524 3.075-7.425ZM8.11 7.11a.75.75 0 0 0-1.06-1.06A6.98 6.98 0 0 0 5 11a6.98 6.98 0 0 0 2.05 4.95.75.75 0 0 0 1.06-1.061 5.48 5.48 0 0 1-1.61-3.89 5.48 5.48 0 0 1 1.61-3.888Zm8.84-1.06a.75.75 0 1 0-1.06 1.06A5.48 5.48 0 0 1 17.5 11a5.48 5.48 0 0 1-1.61 3.889.75.75 0 1 0 1.06 1.06A6.98 6.98 0 0 0 19 11a6.98 6.98 0 0 0-2.05-4.949ZM14 11a2 2 0 0 1-1.25 1.855v8.395a.75.75 0 0 1-1.5 0v-8.395A2 2 0 1 1 14 11Z"
        /> < title > { title } < / title > < / svg >
    }
}
