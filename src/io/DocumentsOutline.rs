#[cfg(feature = "IoDocumentsOutline")]
use leptos::*;
#[cfg(feature = "IoDocumentsOutline")]
///This icon requires the feature `IoDocumentsOutline` to be enabled.
#[component]
pub fn DocumentsOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,264.13V436c0,24.3-19.05,44-42.95,44H107C83.05,480,64,460.3,64,436V172a44.26,44.26,0,0,1,44-44h94.12a24.55,24.55,0,0,1,17.49,7.36l109.15,111A25.4,25.4,0,0,1,336,264.13Z"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M200,128V236a28.34,28.34,0,0,0,28,28H336" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M176,128V76a44.26,44.26,0,0,1,44-44h94a24.83,24.83,0,0,1,17.61,7.36l109.15,111A25.09,25.09,0,0,1,448,168V340c0,24.3-19.05,44-42.95,44H344"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M312,32V140a28.34,28.34,0,0,0,28,28H448" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
