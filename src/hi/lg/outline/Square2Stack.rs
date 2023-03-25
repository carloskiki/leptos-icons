#[cfg(feature = "HiLgOutlineSquare2Stack")]
use leptos::*;
#[cfg(feature = "HiLgOutlineSquare2Stack")]
///This icon requires the feature `HiLgOutlineSquare2Stack` to be enabled.
#[component]
pub fn Square2Stack(
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
        "M16.5 8.25V6C16.5 4.75736 15.4926 3.75 14.25 3.75H6C4.75736 3.75 3.75 4.75736 3.75 6V14.25C3.75 15.4926 4.75736 16.5 6 16.5H8.25M16.5 8.25H18C19.2426 8.25 20.25 9.25736 20.25 10.5V18C20.25 19.2426 19.2426 20.25 18 20.25H10.5C9.25736 20.25 8.25 19.2426 8.25 18V16.5M16.5 8.25H10.5C9.25736 8.25 8.25 9.25736 8.25 10.5V16.5"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
