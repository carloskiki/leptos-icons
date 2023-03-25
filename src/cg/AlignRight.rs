#[cfg(feature = "CgAlignRight")]
use leptos::*;
#[cfg(feature = "CgAlignRight")]
///This icon requires the feature `CgAlignRight` to be enabled.
#[component]
pub fn AlignRight(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15.9991 13.0039L9.99907 12.9911L9.99054 16.9911L15.9905 17.0039L15.9991 13.0039Z"
        fill = "currentColor" fill - opacity = "0.5" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M19.978 18.0021L20.0036 6.00214L18.0036 5.99788L17.978 17.9979L19.978 18.0021Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M3.9964 10.9851L15.9964 11.0107L16.0049 7.0107L4.00493 6.98512L3.9964 10.9851Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
