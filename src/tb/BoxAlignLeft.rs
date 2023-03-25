#[cfg(feature = "TbBoxAlignLeft")]
use leptos::*;
#[cfg(feature = "TbBoxAlignLeft")]
///This icon requires the feature `TbBoxAlignLeft` to be enabled.
#[component]
pub fn BoxAlignLeft(
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
        "icon icon-tabler icon-tabler-box-align-left" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.752 19.753v-16h-5a1 1 0 0 0 -1 1v14a1 1 0 0 0 1 1h5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.752 19.753h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.753 19.753h-.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.753 14.752h-.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.753 8.752h-.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.753 3.752h-.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.752 3.752h-.01" /> < title > { title } < /
        title > < / svg >
    }
}
