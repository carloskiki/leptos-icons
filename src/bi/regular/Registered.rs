#[cfg(feature = "BiRegularRegistered")]
use leptos::*;
#[cfg(feature = "BiRegularRegistered")]
///This icon requires the feature `BiRegularRegistered` to be enabled.
#[component]
pub fn Registered(
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
        "M12.14 2a10 10 0 1 0 10 10 10 10 0 0 0-10-10zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.14 10a3 3 0 0 0-3-3h-5v10h2v-4h1.46l2.67 4h2.4l-2.75-4.12A3 3 0 0 0 16.14 10zm-3 1h-3V9h3a1 1 0 0 1 0 2z"
        /> < title > { title } < / title > < / svg >
    }
}
