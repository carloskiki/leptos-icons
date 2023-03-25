#[cfg(feature = "RiBusinessLineCopyleft")]
use leptos::*;
#[cfg(feature = "RiBusinessLineCopyleft")]
///This icon requires the feature `RiBusinessLineCopyleft` to be enabled.
#[component]
pub fn Copyleft(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M12 22C6.48 22 2 17.52 2 12S6.48 2 12 2s10 4.48 10 10-4.48 10-10 10zm0-2c4.42 0 8-3.58 8-8s-3.58-8-8-8-8 3.58-8 8 3.58 8 8 8zm0-3c-1.82 0-3.413-.973-4.288-2.428l1.714-1.029C9.951 14.416 10.907 15 12 15c1.658 0 3-1.342 3-3s-1.342-3-3-3c-1.093 0-2.048.583-2.573 1.456L7.712 9.428C8.587 7.973 10.18 7 12 7c2.76 0 5 2.24 5 5s-2.24 5-5 5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
