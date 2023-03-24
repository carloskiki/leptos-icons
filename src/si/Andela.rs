#[cfg(feature = "SiAndela")]
use leptos::*;
#[cfg(feature = "SiAndela")]
///This icon requires the feature `SiAndela` to be enabled.
#[component]
pub fn Andela(
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
        "M20.999 4.586 13.956.511A3.94 3.94 0 0 0 12 0c-.713 0-1.378.183-1.946.511L2.982 4.586a4.013 4.013 0 0 0-1.917 3.42v7.997a3.99 3.99 0 0 0 1.927 3.421l7.014 4.046c.587.337 1.262.53 1.994.53.723 0 1.407-.193 1.994-.53l6.937-4.008a4.008 4.008 0 0 0 2.004-3.468V7.997a4.003 4.003 0 0 0-1.936-3.411Zm.298 7.534h-.038c-5.039.02-9.143 4.143-9.143 9.182a.117.117 0 0 1-.116.116.118.118 0 0 1-.116-.116v-.038c-.019-5.039-4.143-9.144-9.181-9.144a.116.116 0 0 1-.116-.115c0-.068.058-.116.116-.116h.038c5.039-.019 9.143-4.143 9.143-9.182 0-.067.058-.115.116-.115.067 0 .116.058.116.115 0 5.059 4.114 9.182 9.181 9.182.068 0 .116.058.116.116.01.067-.048.115-.116.115Z"
        /> < title > { title } < / title > < / svg >
    }
}
