#[cfg(feature = "SiDoubanread")]
use leptos::*;
#[cfg(feature = "SiDoubanread")]
///This icon requires the feature `SiDoubanread` to be enabled.
#[component]
pub fn Doubanread(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.328 5.553c-2.648.906-4.008 4.372-7.101 4.833C4.827 10.833.752 7.205 0 6c0 0 .526.906 1.28 2.105C5.205 14.297 7.772 18.224 12 18.75c5.28.68 8.146-4.535 8.826-6.64.607-1.732 1.733-1.66 2.494-1.433l.68.227s-2.729-7.402-8.688-5.36l.016.008z"
        /> < title > { title } < / title > < / svg >
    }
}
