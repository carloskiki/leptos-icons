#[cfg(feature = "SiBookbub")]
use leptos::*;
#[cfg(feature = "SiBookbub")]
///This icon requires the feature `SiBookbub` to be enabled.
#[component]
pub fn Bookbub(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 20V4h5.4c1.6 0 2.8.3 3.8 1 .9.7 1.4 1.6 1.4 2.7 0 .8-.3 1.6-.8 2.3-.6.7-1.3 1.2-2.2 1.4 1.1.1 2 .6 2.7 1.3.7.7 1 1.6 1 2.6 0 1.4-.5 2.6-1.5 3.4-1 .9-2.4 1.3-4.1 1.3H0zM3 6.4v4.2h1.7c.8 0 1.5-.2 1.9-.6.4-.4.7-1 .7-1.7 0-1.3-.9-1.9-2.7-1.9H3zM3 13v4.7h2.1c.9 0 1.6-.2 2.1-.6.5-.5.8-1.1.8-1.9C8 13.7 7 13 5 13H3zm9.7 7V4h5.4c1.6 0 2.8.3 3.8 1 .9.7 1.4 1.6 1.4 2.7 0 .8-.3 1.6-.8 2.3-.6.7-1.3 1.2-2.2 1.4 1.1.1 2 .6 2.7 1.3.7.7 1 1.6 1 2.6 0 1.4-.5 2.6-1.5 3.4-1 .9-2.4 1.3-4.1 1.3h-5.7zm3-13.6v4.2h1.7c.8 0 1.5-.2 1.9-.6s.7-1 .7-1.7c0-1.3-.9-1.9-2.7-1.9h-1.6zm0 6.6v4.7h2.1c.9 0 1.6-.2 2.1-.6.5-.4.7-1 .7-1.8 0-1.5-1-2.3-3-2.3h-1.9z"
        /> < title > { title } < / title > < / svg >
    }
}
