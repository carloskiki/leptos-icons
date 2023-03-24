#[cfg(feature = "ImCool2")]
use leptos::*;
#[cfg(feature = "ImCool2")]
///This icon requires the feature `ImCool2` to be enabled.
#[component]
pub fn Cool2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8c4.418 0 8-3.582 8-8s-3.582-8-8-8zM8 13c-0.757 0-1.475-0.169-2.118-0.47l0.518-0.864c0.49 0.214 1.031 0.334 1.6 0.334 1.456 0 2.731-0.778 3.43-1.942l0.858 0.515c-0.874 1.454-2.467 2.427-4.288 2.427zM13 6c0 0.55-0.45 1-1 1h-2c-0.55 0-1-0.45-1-1h-2c0 0.55-0.45 1-1 1h-2c-0.55 0-1-0.45-1-1v-1.5c0-0.275 0.225-0.5 0.5-0.5h3c0.275 0 0.5 0.225 0.5 0.5v0.5h2v-0.5c0-0.275 0.225-0.5 0.5-0.5h3c0.275 0 0.5 0.225 0.5 0.5v1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
