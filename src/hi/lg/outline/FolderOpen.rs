#[cfg(feature = "HiLgOutlineFolderOpen")]
use leptos::*;
#[cfg(feature = "HiLgOutlineFolderOpen")]
///This icon requires the feature `HiLgOutlineFolderOpen` to be enabled.
#[component]
pub fn FolderOpen(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75002 9.77602C3.86206 9.7589 3.97701 9.75 4.0943 9.75H19.9058C20.023 9.75 20.138 9.7589 20.25 9.77602M3.75002 9.77602C2.55402 9.9588 1.68986 11.0788 1.86691 12.3182L2.72405 18.3182C2.8824 19.4267 3.83173 20.25 4.95144 20.25H19.0486C20.1683 20.25 21.1176 19.4267 21.276 18.3182L22.1331 12.3182C22.3102 11.0788 21.446 9.9588 20.25 9.77602M3.75002 9.77602V6C3.75002 4.75736 4.75738 3.75 6.00002 3.75H9.8787C10.2765 3.75 10.6581 3.90804 10.9394 4.18934L13.0607 6.31066C13.342 6.59197 13.7235 6.75 14.1213 6.75H18C19.2427 6.75 20.25 7.75736 20.25 9V9.77602"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
