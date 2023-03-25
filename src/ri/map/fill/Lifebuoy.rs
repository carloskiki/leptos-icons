#[cfg(feature = "RiMapFillLifebuoy")]
use leptos::*;
#[cfg(feature = "RiMapFillLifebuoy")]
///This icon requires the feature `RiMapFillLifebuoy` to be enabled.
#[component]
pub fn Lifebuoy(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2zM7.197 14.682l-2.175 2.174a8.549 8.549 0 0 0 1.818 1.899l.305.223 2.173-2.175a5.527 5.527 0 0 1-1.98-1.883l-.14-.238zm9.606 0a5.527 5.527 0 0 1-1.883 1.98l-.238.14 2.174 2.176a8.549 8.549 0 0 0 1.899-1.818l.223-.304-2.175-2.174zM12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8zM7.145 5.022a8.549 8.549 0 0 0-1.9 1.818l-.223.305 2.175 2.173a5.527 5.527 0 0 1 1.883-1.98l.238-.14-2.173-2.176zm9.71 0l-2.173 2.175a5.527 5.527 0 0 1 1.98 1.883l.14.238 2.176-2.173a8.549 8.549 0 0 0-1.818-1.9l-.304-.223z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
