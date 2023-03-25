#[cfg(feature = "IoCreateSharp")]
use leptos::*;
#[cfg(feature = "IoCreateSharp")]
///This icon requires the feature `IoCreateSharp` to be enabled.
#[component]
pub fn CreateSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M464.37,49.2a22.07,22.07,0,0,0-31.88-.76L414.18,66.69l31.18,31.1,18-17.91A22.16,22.16,0,0,0,464.37,49.2Z"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "252.76 336 239.49 336 208 336 176 336 176 304 176 272.51 176 259.24 185.4 249.86 323.54 112 48 112 48 464 400 464 400 188.46 262.14 326.6 252.76 336"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "400 143.16 432.79 110.3 401.7 79.21 368.85 112 400 112 400 143.16" />< polygon
        xmlns = "http://www.w3.org/2000/svg" points =
        "208 304 239.49 304 400 143.16 400 112 368.85 112 208 272.51 208 304" /> < title
        > { title } < / title > < / svg >
    }
}
