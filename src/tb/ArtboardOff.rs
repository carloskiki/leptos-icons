#[cfg(feature = "TbArtboardOff")]
use leptos::*;
#[cfg(feature = "TbArtboardOff")]
///This icon requires the feature `TbArtboardOff` to be enabled.
#[component]
pub fn ArtboardOff(
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
        "icon icon-tabler icon-tabler-artboard-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 8h3a1 1 0 0 1 1 1v3" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.716 15.698a1 1 0 0 1 -.716 .302h-6a1 1 0 0 1 -1 -1v-6c0 -.273 .11 -.52 .287 -.7"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 8h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 16h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 3v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 3v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 8h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 16h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 20v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 20v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
