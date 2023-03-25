#[cfg(feature = "TbBrightnessUp")]
use leptos::*;
#[cfg(feature = "TbBrightnessUp")]
///This icon requires the feature `TbBrightnessUp` to be enabled.
#[component]
pub fn BrightnessUp(
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
        "icon icon-tabler icon-tabler-brightness-up" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 5l0 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7l1.4 -1.4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 12l2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 17l1.4 1.4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 19l0 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 17l-1.4 1.4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 12l-2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 7l-1.4 -1.4" /> < title > { title } < /
        title > < / svg >
    }
}
