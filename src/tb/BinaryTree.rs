#[cfg(feature = "TbBinaryTree")]
use leptos::*;
#[cfg(feature = "TbBinaryTree")]
///This icon requires the feature `TbBinaryTree` to be enabled.
#[component]
pub fn BinaryTree(
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
        "icon icon-tabler icon-tabler-binary-tree" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 4a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M16 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M5.058 18.306l2.88 -4.606" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10.061 10.303l2.877 -4.604" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10.065 13.705l2.876 4.6" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M15.063 5.7l2.881 4.61" /> < title > {
        title } < / title > < / svg >
    }
}
