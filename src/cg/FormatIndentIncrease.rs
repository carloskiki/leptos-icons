#[cfg(feature = "CgFormatIndentIncrease")]
use leptos::*;
#[cfg(feature = "CgFormatIndentIncrease")]
///This icon requires the feature `CgFormatIndentIncrease` to be enabled.
#[component]
pub fn FormatIndentIncrease(
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
        "M20 7H4V5H20V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 11H12V9H20V11Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 15H20V13H12V15Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 15L9 12L4 9V15Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 17V19H20V17H4Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
