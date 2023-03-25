#[cfg(feature = "TbFileSettings")]
use leptos::*;
#[cfg(feature = "TbFileSettings")]
///This icon requires the feature `TbFileSettings` to be enabled.
#[component]
pub fn FileSettings(
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
        "icon icon-tabler icon-tabler-file-settings" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 14m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 10.5v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 16v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.031 12.25l-1.299 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.268 15l-1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 15.803l-1.285 -.773" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.285 12.97l-1.285 -.773" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 3v4a1 1 0 0 0 1 1h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z" /> <
        title > { title } < / title > < / svg >
    }
}
