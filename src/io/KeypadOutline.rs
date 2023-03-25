#[cfg(feature = "IoKeypadOutline")]
use leptos::*;
#[cfg(feature = "IoKeypadOutline")]
///This icon requires the feature `IoKeypadOutline` to be enabled.
#[component]
pub fn KeypadOutline(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "448" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "256" cy = "320" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M288,192a32,32,0,1,1-32-32A32,32,0,0,1,288,192Z" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "256" cy = "64" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "384" cy = "320" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "384" cy = "192" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "384" cy = "64" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "128" cy = "320" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "128" cy = "192" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "128" cy = "64" r = "32" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
