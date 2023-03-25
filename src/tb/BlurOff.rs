#[cfg(feature = "TbBlurOff")]
use leptos::*;
#[cfg(feature = "TbBlurOff")]
///This icon requires the feature `TbBlurOff` to be enabled.
#[component]
pub fn BlurOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-blur-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 3v5m0 4v8"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M16 12h5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 9h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 6h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 18h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 15h3m4 0h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
