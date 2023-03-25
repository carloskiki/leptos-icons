#[cfg(feature = "HiMdSolidRectangleStack")]
use leptos::*;
#[cfg(feature = "HiMdSolidRectangleStack")]
///This icon requires the feature `HiMdSolidRectangleStack` to be enabled.
#[component]
pub fn RectangleStack(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.12744 3.50197C5.16817 3.50066 5.20906 3.5 5.25011 3.5H14.7501C14.7912 3.5 14.832 3.50066 14.8728 3.50197C14.5645 2.62705 13.7305 2 12.7501 2H7.25011C6.26971 2 5.43576 2.62705 5.12744 3.50197Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1 10.25C1 9.00736 2.00736 8 3.25 8H16.75C17.9926 8 19 9.00736 19 10.25V15.75C19 16.9926 17.9926 18 16.75 18H3.25C2.00736 18 1 16.9926 1 15.75V10.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.25011 6.5C3.20906 6.5 3.16817 6.50066 3.12744 6.50197C3.43576 5.62705 4.26971 5 5.25011 5H14.7501C15.7305 5 16.5645 5.62705 16.8728 6.50197C16.832 6.50066 16.7912 6.5 16.7501 6.5H3.25011Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
