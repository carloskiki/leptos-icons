#[cfg(feature = "TbMoodCog")]
use leptos::*;
#[cfg(feature = "TbMoodCog")]
///This icon requires the feature `TbMoodCog` to be enabled.
#[component]
pub fn MoodCog(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-mood-cog"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 12a9 9 0 1 0 -8.983 9" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.001 18m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.001 14.5v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.001 20v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21.032 16.25l-1.299 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.27 19l-1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.97 16.25l1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.733 19l1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.5 15c.658 .64 1.56 1 2.5 1" /> < title > {
        title } < / title > < / svg >
    }
}
