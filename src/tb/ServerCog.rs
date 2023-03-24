#[cfg(feature = "TbServerCog")]
use leptos::*;
#[cfg(feature = "TbServerCog")]
///This icon requires the feature `TbServerCog` to be enabled.
#[component]
pub fn ServerCog(
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
        "icon icon-tabler icon-tabler-server-cog" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 4m0 3a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 20h-6a3 3 0 0 1 -3 -3v-2a3 3 0 0 1 3 -3h10.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 18m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M18 14.5v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 20v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21.032 16.25l-1.299 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.27 19l-1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.97 16.25l1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.733 19l1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 8v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 16v.01" /> < title > { title } < / title > <
        / svg >
    }
}
