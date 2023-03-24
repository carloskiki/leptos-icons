#[cfg(feature = "TbMistOff")]
use leptos::*;
#[cfg(feature = "TbMistOff")]
///This icon requires the feature `TbMistOff` to be enabled.
#[component]
pub fn MistOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-mist-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 5h9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 10h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 10h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 15h5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 15h1m4 0h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 20h9m4 0h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
