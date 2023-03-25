#[cfg(feature = "TbBallpenOff")]
use leptos::*;
#[cfg(feature = "TbBallpenOff")]
///This icon requires the feature `TbBallpenOff` to be enabled.
#[component]
pub fn BallpenOff(
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
        "icon icon-tabler icon-tabler-ballpen-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 6l7 7l-2 2" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M10 10l-4.172 4.172a2.828 2.828 0 1 0 4 4l4.172 -4.172" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16 12l4.414 -4.414a2 2 0 0 0 0 -2.829l-1.171 -1.171a2 2 0 0 0 -2.829 0l-4.414 4.414"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 20l1.768 -1.768" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < /
        title > < / svg >
    }
}
