#[cfg(feature = "TbEyeTable")]
use leptos::*;
#[cfg(feature = "TbEyeTable")]
///This icon requires the feature `TbEyeTable` to be enabled.
#[component]
pub fn EyeTable(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-eye-table"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 18h-.011" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 18h-.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 18h-.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 3h16" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 3v17a1 1 0 0 0 1 1h12a1 1 0 0 0 1 -1v-17"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 7h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 15h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 15h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 11v-4" /> < title > { title } < / title > <
        / svg >
    }
}
