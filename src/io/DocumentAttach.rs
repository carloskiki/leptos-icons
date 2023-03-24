#[cfg(feature = "IoDocumentAttach")]
use leptos::*;
#[cfg(feature = "IoDocumentAttach")]
///This icon requires the feature `IoDocumentAttach` to be enabled.
#[component]
pub fn DocumentAttach(
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
        "M460,240H320a48,48,0,0,1-48-48V52a4,4,0,0,0-4-4H214.75a65.42,65.42,0,0,0-6.5-9.81C196.72,23.88,179.59,16,160,16c-37.68,0-64,29.61-64,72V232c0,25,20.34,40,40,40a39.57,39.57,0,0,0,40-40V80a16,16,0,0,0-32,0V232a7.75,7.75,0,0,1-8,8c-2.23,0-8-1.44-8-8V88c0-19.34,8.41-40,32-40,29.69,0,32,30.15,32,39.38V226.13c0,17.45-5.47,33.23-15.41,44.46C166.5,282,152.47,288,136,288s-30.5-6-40.59-17.41C85.47,259.36,80,243.58,80,226.13V144a16,16,0,0,0-32,0v82.13c0,51.51,33.19,89.63,80,93.53V432a64,64,0,0,0,64,64H400a64,64,0,0,0,64-64V244A4,4,0,0,0,460,240Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,208H449.81a2,2,0,0,0,1.41-3.41L307.41,60.78A2,2,0,0,0,304,62.19V192A16,16,0,0,0,320,208Z"
        /> < title > { title } < / title > < / svg >
    }
}
