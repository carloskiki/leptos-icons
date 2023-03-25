#[cfg(feature = "TbWreckingBall")]
use leptos::*;
#[cfg(feature = "TbWreckingBall")]
///This icon requires the feature `TbWreckingBall` to be enabled.
#[component]
pub fn WreckingBall(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-wrecking-ball" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 13m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 19l-9 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 15l9 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12v-5h2a3 3 0 0 1 3 3v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 15v-2a1 1 0 0 1 1 -1h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 11v-7l-6 7" /> < title > { title } < /
        title > < / svg >
    }
}
