#[cfg(feature = "OcLgShieldSlash")]
use leptos::*;
#[cfg(feature = "OcLgShieldSlash")]
///This icon requires the feature `OcLgShieldSlash` to be enabled.
#[component]
pub fn ShieldSlash(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12.54 1.137a1.748 1.748 0 0 0-1.08 0L6.018 2.905a.75.75 0 1 0 .464 1.427l5.441-1.768a.239.239 0 0 1 .154 0l8.25 2.675a.249.249 0 0 1 .173.237V10.5c0 1.284-.24 2.83-.696 3.971a.75.75 0 1 0 1.392.557C21.74 13.67 22 11.927 22 10.5V5.476a1.75 1.75 0 0 0-1.21-1.664l-8.25-2.675ZM2.017 4.843l-.974-.748a.751.751 0 0 1 .914-1.19l20.5 15.75a.751.751 0 0 1-.914 1.19l-2.012-1.546-.702.852-.008.009a.07.07 0 0 1-.008.01c-1.603 1.821-3.731 3.223-6.214 4.16a1.699 1.699 0 0 1-1.198-.001C5.771 21.205 2 16.689 2 10.5V5c0-.054.006-.107.017-.157ZM3.5 5.982V10.5c0 5.461 3.281 9.483 8.431 11.426a.193.193 0 0 0 .138 0c2.283-.861 4.192-2.131 5.61-3.738l.662-.803Z"
        /> < title > { title } < / title > < / svg >
    }
}
