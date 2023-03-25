#[cfg(feature = "BiSolidShower")]
use leptos::*;
#[cfg(feature = "BiSolidShower")]
///This icon requires the feature `BiSolidShower` to be enabled.
#[component]
pub fn Shower(
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
        "M21 18.33A6.78 6.78 0 0 0 19.5 15a6.73 6.73 0 0 0-1.5 3.33 1.51 1.51 0 1 0 3 0zM11 20.33A6.78 6.78 0 0 0 9.5 17 6.73 6.73 0 0 0 8 20.33 1.59 1.59 0 0 0 9.5 22a1.59 1.59 0 0 0 1.5-1.67zM16 20.33A6.78 6.78 0 0 0 14.5 17a6.73 6.73 0 0 0-1.5 3.33A1.59 1.59 0 0 0 14.5 22a1.59 1.59 0 0 0 1.5-1.67zM6 18.33A6.78 6.78 0 0 0 4.5 15 6.73 6.73 0 0 0 3 18.33 1.59 1.59 0 0 0 4.5 20 1.59 1.59 0 0 0 6 18.33zM2 12h20v2H2zM13 4.07V2h-2v2.07A8 8 0 0 0 4.07 11h15.86A8 8 0 0 0 13 4.07z"
        /> < title > { title } < / title > < / svg >
    }
}
