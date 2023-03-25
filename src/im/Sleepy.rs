#[cfg(feature = "ImSleepy")]
use leptos::*;
#[cfg(feature = "ImSleepy")]
///This icon requires the feature `ImSleepy` to be enabled.
#[component]
pub fn Sleepy(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 16c4.418 0 8-3.582 8-8s-3.582-8-8-8-8 3.582-8 8 3.582 8 8 8zM8 1.5c3.59 0 6.5 2.91 6.5 6.5s-2.91 6.5-6.5 6.5-6.5-2.91-6.5-6.5 2.91-6.5 6.5-6.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M10 10.5c0 1.381-0.895 2.5-2 2.5s-2-1.119-2-2.5c0-1.381 0.895-2.5 2-2.5s2 1.119 2 2.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M6.5 5.313c-0.128 0-0.256-0.049-0.354-0.146-0.302-0.302-0.991-0.302-1.293 0-0.195 0.195-0.512 0.195-0.707 0s-0.195-0.512 0-0.707c0.696-0.696 2.011-0.696 2.707 0 0.195 0.195 0.195 0.512 0 0.707-0.098 0.098-0.226 0.146-0.354 0.146z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11.5 5.313c-0.128 0-0.256-0.049-0.354-0.146-0.302-0.302-0.991-0.302-1.293 0-0.195 0.195-0.512 0.195-0.707 0s-0.195-0.512 0-0.707c0.696-0.696 2.011-0.696 2.707 0 0.195 0.195 0.195 0.512 0 0.707-0.098 0.098-0.226 0.146-0.354 0.146z"
        /> < title > { title } < / title > < / svg >
    }
}
