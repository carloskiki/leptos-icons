#[cfg(feature = "IoTrainOutline")]
use leptos::*;
#[cfg(feature = "IoTrainOutline")]
///This icon requires the feature `IoTrainOutline` to be enabled.
#[component]
pub fn TrainOutline(
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
        "M344,48H320a16,16,0,0,0-16-16H208a16,16,0,0,0-16,16H168a56.16,56.16,0,0,0-56,56V351c0,35.3,144,65,144,65s144-29.7,144-65V104A56,56,0,0,0,344,48ZM256,352a48,48,0,1,1,48-48A48,48,0,0,1,256,352Zm96-160a16,16,0,0,1-16,16H176a16,16,0,0,1-16-16V128a16,16,0,0,1,16-16H336a16,16,0,0,1,16,16Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "144" y1 = "464" x2 = "368" y2
        = "464" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "336" y1 = "432" x2 = "384" y2
        = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "176" y1 = "432" x2 = "128" y2
        = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
