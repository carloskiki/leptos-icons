#[cfg(feature = "ImSvg")]
use leptos::*;
#[cfg(feature = "ImSvg")]
///This icon requires the feature `ImSvg` to be enabled.
#[component]
pub fn Svg(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M14.5 6.5c-0.444 0-0.843 0.193-1.118 0.5h-2.968l2.099-2.099c0.411 0.023 0.83-0.123 1.144-0.437 0.586-0.586 0.586-1.536 0-2.121s-1.536-0.586-2.121 0c-0.314 0.314-0.46 0.733-0.437 1.144l-2.099 2.099v-2.968c0.307-0.275 0.5-0.674 0.5-1.118 0-0.828-0.672-1.5-1.5-1.5s-1.5 0.672-1.5 1.5c0 0.444 0.193 0.843 0.5 1.118v2.968l-2.099-2.099c0.023-0.411-0.123-0.83-0.437-1.144-0.586-0.586-1.536-0.586-2.121 0s-0.586 1.536 0 2.121c0.314 0.314 0.733 0.46 1.144 0.437l2.099 2.099h-2.968c-0.275-0.307-0.674-0.5-1.118-0.5-0.828 0-1.5 0.672-1.5 1.5s0.672 1.5 1.5 1.5c0.444 0 0.843-0.193 1.118-0.5h2.968l-2.099 2.099c-0.411-0.023-0.83 0.123-1.144 0.437-0.586 0.586-0.586 1.536 0 2.121s1.536 0.586 2.121 0c0.314-0.314 0.46-0.733 0.437-1.144l2.099-2.099v2.968c-0.307 0.275-0.5 0.674-0.5 1.118 0 0.828 0.672 1.5 1.5 1.5s1.5-0.672 1.5-1.5c0-0.444-0.193-0.843-0.5-1.118v-2.968l2.099 2.099c-0.023 0.411 0.123 0.83 0.437 1.144 0.586 0.586 1.536 0.586 2.121 0s0.586-1.536 0-2.121c-0.314-0.314-0.733-0.46-1.144-0.437l-2.099-2.099h2.968c0.275 0.307 0.674 0.5 1.118 0.5 0.828 0 1.5-0.672 1.5-1.5s-0.672-1.5-1.5-1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
