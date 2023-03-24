#[cfg(feature = "SiBigcartel")]
use leptos::*;
#[cfg(feature = "SiBigcartel")]
///This icon requires the feature `SiBigcartel` to be enabled.
#[component]
pub fn Bigcartel(
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
        "M12 13.068v-1.006c0-.63.252-1.256.88-1.508l7.79-4.9c.503-.252.755-.88.755-1.51V0L12 6.03 2.575 0v12.69c0 3.394 1.51 6.284 4.02 7.917L11.875 24l5.28-3.393c2.513-1.51 4.02-4.398 4.02-7.916V7.036L12 13.068z"
        /> < title > { title } < / title > < / svg >
    }
}
