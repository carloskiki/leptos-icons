#[cfg(feature = "TiArrowDownThick")]
use leptos::*;
#[cfg(feature = "TiArrowDownThick")]
///This icon requires the feature `TiArrowDownThick` to be enabled.
#[component]
pub fn ArrowDownThick(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.414 10.656c-.781-.781-2.047-.781-2.828 0l-1.586 1.586v-7.242c0-1.105-.896-2-2-2-1.105 0-2 .895-2 2v7.242l-1.586-1.586c-.781-.781-2.047-.781-2.828 0s-.781 2.047 0 2.828l6.414 6.414 6.414-6.414c.781-.781.781-2.046 0-2.828z"
        /> < title > { title } < / title > < / svg >
    }
}
