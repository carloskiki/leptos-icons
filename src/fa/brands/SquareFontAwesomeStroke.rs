#[cfg(feature = "FaBrandsSquareFontAwesomeStroke")]
use leptos::*;
#[cfg(feature = "FaBrandsSquareFontAwesomeStroke")]
///This icon requires the feature `FaBrandsSquareFontAwesomeStroke` to be enabled.
#[component]
pub fn SquareFontAwesomeStroke(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M201.6,152c-25.4,0-37.4,10.4-57.6,14.4V160c0-8.8-7.2-16-16-16s-16,7.2-16,16v192c0,0.8,0.1,1.6,0.2,2.4 c0.1,0.4,0.1,0.8,0.2,1.2c1.6,7.1,8,12.4,15.6,12.4s14-5.3,15.6-12.4c0.1-0.4,0.2-0.8,0.2-1.2c0.1-0.8,0.2-1.6,0.2-2.4V198.4 c4-0.8,7.7-1.8,11.2-3c14.3-4.7,26-11.4,46.4-11.4c31.4,0,43.2,16,74.6,16c8.9,0,15.9-1.1,24.2-3.5c1.2-0.3,2.4-0.7,3.6-1.1v96 c-10,3.2-17.6,4.6-27.8,4.6c-31.4,0-43.4-16-74.6-16c-10.2,0-18.2,1.8-25.6,4v32c7.4-2.4,15.4-4,25.6-4c31.4,0,43.2,16,74.6,16 c18.6,0,28.2-4.8,59.8-16V152c-31.6,11.2-41.2,16-59.8,16C244.8,168,232.8,152,201.6,152z M384,32H64C28.7,32,0,60.7,0,96v320 c0,35.3,28.7,64,64,64h320c35.3,0,64-28.7,64-64V96C448,60.7,419.3,32,384,32z M416,416c0,17.6-14.4,32-32,32H64 c-17.6,0-32-14.4-32-32V96c0-17.6,14.4-32,32-32h320c17.6,0,32,14.4,32,32V416z"
        /> < title > { title } < / title > < / svg >
    }
}
