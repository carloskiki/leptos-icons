#[cfg(feature = "SiUptimekuma")]
use leptos::*;
#[cfg(feature = "SiUptimekuma")]
///This icon requires the feature `SiUptimekuma` to be enabled.
#[component]
pub fn Uptimekuma(
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
        "M11.759.955c-4.071 0-7.93 2.265-10.06 5.774l-.16.263-.116.284c-1.81 4.44-2.188 9.118.621 12.459 2.67 3.174 6.221 3.328 9.477 3.308 3.256-.02 6.323-.482 8.995-2.032C22.75 19.714 24 16.917 24 14.53c0-2.388-.724-4.698-1.882-7.343l-.112-.257-.148-.238C19.683 3.2 15.83.955 11.758.955Zm0 3.868c2.919 0 5.19 1.305 6.816 3.914 2.076 4.747 2.076 7.724 0 8.929-3.116 1.808-11.234 2.359-13.57-.42-1.558-1.853-1.558-4.69 0-8.51 1.584-2.608 3.835-3.913 6.754-3.913z"
        /> < title > { title } < / title > < / svg >
    }
}
