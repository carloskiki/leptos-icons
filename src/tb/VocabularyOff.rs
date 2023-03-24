#[cfg(feature = "TbVocabularyOff")]
use leptos::*;
#[cfg(feature = "TbVocabularyOff")]
///This icon requires the feature `TbVocabularyOff` to be enabled.
#[component]
pub fn VocabularyOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-vocabulary-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 3h3a2 2 0 0 1 2 2a2 2 0 0 1 2 -2h6a1 1 0 0 1 1 1v13m-2 2h-5a2 2 0 0 0 -2 2a2 2 0 0 0 -2 -2h-6a1 1 0 0 1 -1 -1v-14c0 -.279 .114 -.53 .298 -.712"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 5v3m0 4v9" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M7 11h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 7h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 11h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
