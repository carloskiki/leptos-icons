#[cfg(feature = "TbLanguageHiragana")]
use leptos::*;
#[cfg(feature = "TbLanguageHiragana")]
///This icon requires the feature `TbLanguageHiragana` to be enabled.
#[component]
pub fn LanguageHiragana(
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
        "icon icon-tabler icon-tabler-language-hiragana" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 5h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 4c0 4.846 0 7 .5 8" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10 8.5c0 2.286 -2 4.5 -3.5 4.5s-2.5 -1.135 -2.5 -2c0 -2 1 -3 3 -3s5 .57 5 2.857c0 1.524 -.667 2.571 -2 3.143"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 20l4 -9l4 9" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M19.1 18h-6.2" /> < title > { title } <
        / title > < / svg >
    }
}
