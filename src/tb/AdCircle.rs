#[cfg(feature = "TbAdCircle")]
use leptos::*;
#[cfg(feature = "TbAdCircle")]
///This icon requires the feature `TbAdCircle` to be enabled.
#[component]
pub fn AdCircle(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-ad-circle"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-10 0a10 10 0 1 0 20 0a10 10 0 1 0 -20 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 15v-4.5a1.5 1.5 0 0 1 3 0v4.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M7 13h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 9v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1z" /> < title > { title } < /
        title > < / svg >
    }
}
