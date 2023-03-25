#[cfg(feature = "RiUserFillCriminal")]
use leptos::*;
#[cfg(feature = "RiUserFillCriminal")]
///This icon requires the feature `RiUserFillCriminal` to be enabled.
#[component]
pub fn Criminal(
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
        "M12 2a9 9 0 0 1 6.894 14.786c1.255.83 2.033 1.89 2.101 3.049L21 20l-9 2-9-2 .005-.165c.067-1.16.846-2.22 2.1-3.05A8.965 8.965 0 0 1 3 11a9 9 0 0 1 9-9zm0 11c-1.38 0-2.5.672-2.5 1.5S10.62 16 12 16s2.5-.672 2.5-1.5S13.38 13 12 13zM9 8c-1.105 0-2 .672-2 1.5S7.895 11 9 11s2-.672 2-1.5S10.105 8 9 8zm6 0c-1.105 0-2 .672-2 1.5s.895 1.5 2 1.5 2-.672 2-1.5S16.105 8 15 8z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
