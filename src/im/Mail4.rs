#[cfg(feature = "ImMail4")]
use leptos::*;
#[cfg(feature = "ImMail4")]
///This icon requires the feature `ImMail4` to be enabled.
#[component]
pub fn Mail4(
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
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM4 4h8c0.143 0 0.281 0.031 0.409 0.088l-4.409 5.143-4.409-5.143c0.127-0.058 0.266-0.088 0.409-0.088zM3 11v-6c0-0.021 0.001-0.042 0.002-0.063l2.932 3.421-2.9 2.9c-0.023-0.083-0.034-0.17-0.034-0.258zM12 12h-8c-0.088 0-0.175-0.012-0.258-0.034l2.846-2.846 1.413 1.648 1.413-1.648 2.846 2.846c-0.083 0.023-0.17 0.034-0.258 0.034zM13 11c0 0.088-0.012 0.175-0.034 0.258l-2.9-2.9 2.932-3.421c0.001 0.021 0.002 0.042 0.002 0.063v6z"
        /> < title > { title } < / title > < / svg >
    }
}
