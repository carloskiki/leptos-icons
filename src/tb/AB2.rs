#[cfg(feature = "TbAB2")]
use leptos::*;
#[cfg(feature = "TbAB2")]
///This icon requires the feature `TbAB2` to be enabled.
#[component]
pub fn AB2(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-a-b-2"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 21h3c.81 0 1.48 -.67 1.48 -1.48l.02 -.02c0 -.82 -.69 -1.5 -1.5 -1.5h-3v3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 15h2.5c.84 -.01 1.5 .66 1.5 1.5s-.66 1.5 -1.5 1.5h-2.5v-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 9v-4c0 -1.036 .895 -2 2 -2s2 .964 2 2v4" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.99 11.98a9 9 0 0 0 9 9m9 -9a9 9 0 0 0 -9 -9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 7h-4" /> < title > { title } < / title > < /
        svg >
    }
}
