#[cfg(feature = "HiMdSolidPencilSquare")]
use leptos::*;
#[cfg(feature = "HiMdSolidPencilSquare")]
///This icon requires the feature `HiMdSolidPencilSquare` to be enabled.
#[component]
pub fn PencilSquare(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.43306 13.9168L6.69485 10.7623C6.89603 10.2593 7.19728 9.80249 7.58033 9.41945L14.4995 2.50071C15.3279 1.67229 16.6711 1.67229 17.4995 2.50072C18.3279 3.32914 18.3279 4.67229 17.4995 5.50072L10.5803 12.4194C10.1973 12.8025 9.74042 13.1037 9.23746 13.3049L6.08299 14.5667C5.67484 14.73 5.2698 14.3249 5.43306 13.9168Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 5.75C3.5 5.05964 4.05964 4.5 4.75 4.5H10C10.4142 4.5 10.75 4.16421 10.75 3.75C10.75 3.33579 10.4142 3 10 3H4.75C3.23122 3 2 4.23122 2 5.75V15.25C2 16.7688 3.23122 18 4.75 18H14.25C15.7688 18 17 16.7688 17 15.25V10C17 9.58579 16.6642 9.25 16.25 9.25C15.8358 9.25 15.5 9.58579 15.5 10V15.25C15.5 15.9404 14.9404 16.5 14.25 16.5H4.75C4.05964 16.5 3.5 15.9404 3.5 15.25V5.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
