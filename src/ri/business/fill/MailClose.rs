#[cfg(feature = "RiBusinessFillMailClose")]
use leptos::*;
#[cfg(feature = "RiBusinessFillMailClose")]
///This icon requires the feature `RiBusinessFillMailClose` to be enabled.
#[component]
pub fn MailClose(
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
        "M22 13.341A6 6 0 0 0 14.341 21H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v9.341zm-9.94-1.658L5.648 6.238 4.353 7.762l7.72 6.555 7.581-6.56-1.308-1.513-6.285 5.439zM21.415 19l2.122 2.121-1.415 1.415L20 20.414l-2.121 2.122-1.415-1.415L18.586 19l-2.122-2.121 1.415-1.415L20 17.586l2.121-2.122 1.415 1.415L21.414 19z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
