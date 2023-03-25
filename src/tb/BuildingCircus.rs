#[cfg(feature = "TbBuildingCircus")]
use leptos::*;
#[cfg(feature = "TbBuildingCircus")]
///This icon requires the feature `TbBuildingCircus` to be enabled.
#[component]
pub fn BuildingCircus(
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
        "icon icon-tabler icon-tabler-building-circus" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 11h16" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 6.5c0 1 -5 4.5 -8 4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 6.5c0 1 5 4.5 8 4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6 11c-.333 5.333 -1 8.667 -2 10h4c1 0 4 -4 4 -9v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18 11c.333 5.333 1 8.667 2 10h-4c-1 0 -4 -4 -4 -9v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 7v-4l2 1h-2" /> < title > { title } < /
        title > < / svg >
    }
}
