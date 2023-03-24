#[cfg(feature = "TbMoodHeart")]
use leptos::*;
#[cfg(feature = "TbMoodHeart")]
///This icon requires the feature `TbMoodHeart` to be enabled.
#[component]
pub fn MoodHeart(
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
        "icon icon-tabler icon-tabler-mood-heart" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M21 12a9 9 0 1 0 -8.012 8.946"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.5 15a3.59 3.59 0 0 0 2.774 .99" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M18.994 21.5l2.518 -2.58a1.74 1.74 0 0 0 .004 -2.413a1.627 1.627 0 0 0 -2.346 -.005l-.168 .172l-.168 -.172a1.627 1.627 0 0 0 -2.346 -.004a1.74 1.74 0 0 0 -.004 2.412l2.51 2.59z"
        /> < title > { title } < / title > < / svg >
    }
}
