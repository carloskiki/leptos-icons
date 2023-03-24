#[cfg(feature = "SiGuilded")]
use leptos::*;
#[cfg(feature = "SiGuilded")]
///This icon requires the feature `SiGuilded` to be enabled.
#[component]
pub fn Guilded(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.297 6.255s.02 2.846 1.481 5.79c1.502 2.834 3.572 4.654 5.28 5.38 1.765-.826 3.47-2.258 4.4-3.8h-4.845c-1.253-1.04-2.24-2.763-2.466-4.755H23.36c-.701 3.203-2.188 6.116-3.605 7.971a17.108 17.108 0 01-7.686 5.659h-.045c-5.098-2.031-7.84-5.23-9.65-8.84C1.214 11.347 0 7.147 0 1.5h24a34.23 34.23 0 01-.32 4.755z"
        /> < title > { title } < / title > < / svg >
    }
}
