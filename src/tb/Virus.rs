#[cfg(feature = "TbVirus")]
use leptos::*;
#[cfg(feature = "TbVirus")]
///This icon requires the feature `TbVirus` to be enabled.
#[component]
pub fn Virus(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-virus"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 7v-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 3h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.536 8.464l2.828 -2.828" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.657 4.929l1.414 1.414" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 11v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.535 15.536l2.829 2.828" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.071 17.657l-1.414 1.414" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 17v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 21h-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.465 15.536l-2.829 2.828" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.343 19.071l-1.413 -1.414" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 12h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 13v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.464 8.464l-2.828 -2.828" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.929 6.343l1.414 -1.413" /> < title > { title
        } < / title > < / svg >
    }
}
