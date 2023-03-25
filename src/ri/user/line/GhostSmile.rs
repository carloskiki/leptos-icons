#[cfg(feature = "RiUserLineGhostSmile")]
use leptos::*;
#[cfg(feature = "RiUserLineGhostSmile")]
///This icon requires the feature `RiUserLineGhostSmile` to be enabled.
#[component]
pub fn GhostSmile(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 2a9 9 0 0 1 9 9v7.5a3.5 3.5 0 0 1-6.39 1.976 2.999 2.999 0 0 1-5.223 0 3.5 3.5 0 0 1-6.382-1.783L3 18.499V11a9 9 0 0 1 9-9zm0 2a7 7 0 0 0-6.996 6.76L5 11v7.446l.002.138a1.5 1.5 0 0 0 2.645.88l.088-.116a2 2 0 0 1 3.393.142.999.999 0 0 0 1.74.003 2 2 0 0 1 3.296-.278l.097.13a1.5 1.5 0 0 0 2.733-.701L19 18.5V11a7 7 0 0 0-7-7zm4 9a4 4 0 0 1-7.995.2L8 13h2a2 2 0 1 0 4 0h2zm-4-6a2 2 0 1 1 0 4 2 2 0 0 1 0-4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
