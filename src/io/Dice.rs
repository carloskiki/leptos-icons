#[cfg(feature = "IoDice")]
use leptos::*;
#[cfg(feature = "IoDice")]
///This icon requires the feature `IoDice` to be enabled.
#[component]
pub fn Dice(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M440.88,129.37,288.16,40.62a64.14,64.14,0,0,0-64.33,0L71.12,129.37a4,4,0,0,0,0,6.9L254,243.85a4,4,0,0,0,4.06,0L440.9,136.27A4,4,0,0,0,440.88,129.37ZM256,152c-13.25,0-24-7.16-24-16s10.75-16,24-16,24,7.16,24,16S269.25,152,256,152Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M238,270.81,54,163.48a4,4,0,0,0-6,3.46V340.86a48,48,0,0,0,23.84,41.39L234,479.48a4,4,0,0,0,6-3.46V274.27A4,4,0,0,0,238,270.81ZM96,368c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S104.84,368,96,368Zm96-32c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S200.84,336,192,336Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M458,163.51,274,271.56a4,4,0,0,0-2,3.45V476a4,4,0,0,0,6,3.46l162.15-97.23A48,48,0,0,0,464,340.86V167A4,4,0,0,0,458,163.51ZM320,424c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S328.84,424,320,424Zm0-88c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S328.84,336,320,336Zm96,32c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S424.84,368,416,368Zm0-88c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S424.84,280,416,280Z"
        /> < title > { title } < / title > < / svg >
    }
}
