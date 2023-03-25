#[cfg(feature = "TbFileBroken")]
use leptos::*;
#[cfg(feature = "TbFileBroken")]
///This icon requires the feature `TbFileBroken` to be enabled.
#[component]
pub fn FileBroken(
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
        "icon icon-tabler icon-tabler-file-broken" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 3v4a1 1 0 0 0 1 1h4" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M5 7v-2a2 2 0 0 1 2 -2h7l5 5v2"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 19a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 16h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 13h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 13h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 16h.01" /> < title > { title } < / title >
        < / svg >
    }
}
