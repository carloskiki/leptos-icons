#[cfg(feature = "TbCreativeCommonsOff")]
use leptos::*;
#[cfg(feature = "TbCreativeCommonsOff")]
///This icon requires the feature `TbCreativeCommonsOff` to be enabled.
#[component]
pub fn CreativeCommonsOff(
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
        "icon icon-tabler icon-tabler-creative-commons-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.638 5.634a9 9 0 1 0 12.723 12.733m1.686 -2.332a9 9 0 0 0 -12.093 -12.077" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5 10.5a2.187 2.187 0 0 0 -2.914 .116a1.928 1.928 0 0 0 0 2.768a2.188 2.188 0 0 0 2.914 .116"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5 10.5a2.194 2.194 0 0 0 -2.309 -.302" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
