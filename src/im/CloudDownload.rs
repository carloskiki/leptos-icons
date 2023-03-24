#[cfg(feature = "ImCloudDownload")]
use leptos::*;
#[cfg(feature = "ImCloudDownload")]
///This icon requires the feature `ImCloudDownload` to be enabled.
#[component]
pub fn CloudDownload(
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
        "M13.922 5.626c-0.051-2.011-1.695-3.626-3.717-3.626-1.184 0-2.239 0.555-2.92 1.418-0.382-0.494-0.98-0.812-1.652-0.812-1.153 0-2.088 0.936-2.088 2.089 0 0.101 0.007 0.199 0.021 0.296-0.175-0.032-0.356-0.049-0.54-0.049-1.672-0-3.027 1.356-3.027 3.029s1.355 3.029 3.027 3.029h1.434l3.539 3.664 3.539-3.664 1.742-0c1.502-0.003 2.719-1.222 2.719-2.726 0-1.283-0.886-2.358-2.078-2.648zM8 13l-3-3h2v-3h2v3h2l-3 3z"
        /> < title > { title } < / title > < / svg >
    }
}
