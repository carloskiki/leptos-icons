#[cfg(feature = "SiZillow")]
use leptos::*;
#[cfg(feature = "SiZillow")]
///This icon requires the feature `SiZillow` to be enabled.
#[component]
pub fn Zillow(
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
        "M12.006 0L1.086 8.627v3.868c3.386-2.013 11.219-5.13 14.763-6.015.11-.024.16.005.227.078.372.427 1.586 1.899 1.916 2.301a.128.128 0 0 1-.03.195 43.607 43.607 0 0 0-6.67 6.527c-.03.037-.006.043.012.03 2.642-1.134 8.828-2.94 11.622-3.452V8.627zm-.48 11.177c-2.136.708-8.195 3.307-10.452 4.576V24h21.852v-7.936c-2.99.506-11.902 3.16-15.959 5.246a.183.183 0 0 1-.23-.036l-2.044-2.429c-.055-.061-.062-.098.011-.208 1.574-2.3 4.789-5.899 6.833-7.418.042-.03.031-.06-.012-.042Z"
        /> < title > { title } < / title > < / svg >
    }
}
