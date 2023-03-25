#[cfg(feature = "HiLgOutlineRss")]
use leptos::*;
#[cfg(feature = "HiLgOutlineRss")]
///This icon requires the feature `HiLgOutlineRss` to be enabled.
#[component]
pub fn Rss(
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
        "M12.75 19.5V18.75C12.75 14.6079 9.39214 11.25 5.25 11.25H4.5M4.5 4.5H5.25C13.1201 4.5 19.5 10.8799 19.5 18.75V19.5M6 18.75C6 19.1642 5.66421 19.5 5.25 19.5C4.83579 19.5 4.5 19.1642 4.5 18.75C4.5 18.3358 4.83579 18 5.25 18C5.66421 18 6 18.3358 6 18.75Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
