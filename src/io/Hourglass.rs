#[cfg(feature = "IoHourglass")]
use leptos::*;
#[cfg(feature = "IoHourglass")]
///This icon requires the feature `IoHourglass` to be enabled.
#[component]
pub fn Hourglass(
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
        "M415.7,427.13c-8.74-76.89-43.83-108.76-69.46-132C328.52,279,320,270.61,320,256c0-14.41,8.49-22.64,26.16-38.44,25.93-23.17,61.44-54.91,69.56-132.84a47,47,0,0,0-12-36.26A50.3,50.3,0,0,0,366.39,32H145.61a50.34,50.34,0,0,0-37.39,16.46A47.05,47.05,0,0,0,96.28,84.72c8.09,77.68,43.47,109.19,69.3,132.19C183.42,232.8,192,241.09,192,256c0,15.1-8.6,23.56-26.5,39.75C140,318.85,105,350.48,96.3,427.13A46.59,46.59,0,0,0,108,463.33,50.44,50.44,0,0,0,145.61,480H366.39A50.44,50.44,0,0,0,404,463.33,46.59,46.59,0,0,0,415.7,427.13ZM343.3,432H169.13c-15.6,0-20-18-9.06-29.16C186.55,376,240,356.78,240,326V224c0-19.85-38-35-61.51-67.2-3.88-5.31-3.49-12.8,6.37-12.8H327.59c8.41,0,10.22,7.43,6.4,12.75C310.82,189,272,204.05,272,224V326c0,30.53,55.71,47,80.4,76.87C362.35,414.91,358.87,432,343.3,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
