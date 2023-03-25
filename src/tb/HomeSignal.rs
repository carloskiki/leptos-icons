#[cfg(feature = "TbHomeSignal")]
use leptos::*;
#[cfg(feature = "TbHomeSignal")]
///This icon requires the feature `TbHomeSignal` to be enabled.
#[component]
pub fn HomeSignal(
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
        "icon icon-tabler icon-tabler-home-signal" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 22v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 22v-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 22v-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 12.494v-.494h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v.5" />
        < title > { title } < / title > < / svg >
    }
}
