#[cfg(feature = "TbWorldWww")]
use leptos::*;
#[cfg(feature = "TbWorldWww")]
///This icon requires the feature `TbWorldWww` to be enabled.
#[component]
pub fn WorldWww(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-world-www"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.5 7a9 9 0 0 0 -7.5 -4a8.991 8.991 0 0 0 -7.484 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11.5 3a16.989 16.989 0 0 0 -1.826 4" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M12.5 3a16.989 16.989 0 0 1 1.828 4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.5 17a9 9 0 0 1 -7.5 4a8.991 8.991 0 0 1 -7.484 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11.5 21a16.989 16.989 0 0 1 -1.826 -4" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.5 21a16.989 16.989 0 0 0 1.828 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M2 10l1 4l1.5 -4l1.5 4l1 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 10l1 4l1.5 -4l1.5 4l1 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.5 10l1 4l1.5 -4l1.5 4l1 -4" /> < title > {
        title } < / title > < / svg >
    }
}
