#[cfg(feature = "IoBonfireSharp")]
use leptos::*;
#[cfg(feature = "IoBonfireSharp")]
///This icon requires the feature `IoBonfireSharp` to be enabled.
#[component]
pub fn BonfireSharp(
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
        "M199.89,336l-15.25-5.62a100.35,100.35,0,0,1-32-23.08c-13.93-14.9-29.29-40.71-23.38-79.11,5.2-33.73,44.2-74.21,69.34-97.87,27.24-25.62,66-65.85,64.15-99.15L262,16h15.18C328.53,16,384,53.62,384,114.41c0,45.57-22,77.61-68.91,106.9-8,5-16.44,9.66-25.42,14.53-30.63,16.62-75.29,49.83-85.73,85.32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M181.19,113.59C201,95,218.91,78.15,227.62,59.79q-2.76-1.68-5.7-3.09c-11.87-5.69-26.1-8.34-44.76-8.34h-.42l-16.52,0,.56,16.49c.47,14.06-6.06,22.51-14.33,33.21C137.8,109.26,128,121.94,128,141c0,10.23,1.29,18.76,4.2,26.36q3.5-5.31,7.61-10.63C152,141,166.86,127.05,181.19,113.59Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M330.34,239.85c-9.31,5.9-19,11.14-29.25,16.71C268.44,274.25,237.61,291,227.86,324l-.6,2A110.5,110.5,0,0,0,273.13,336c29.66,0,57.45-11.13,78.24-31.36A107.38,107.38,0,0,0,384,227a92.39,92.39,0,0,0-5.59-31C367.06,212.18,351.27,226.58,330.34,239.85Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M268.72,360H243.28a4,4,0,0,0-3.92,3.22L214,491.22a4,4,0,0,0,3.92,4.78h76.26a4,4,0,0,0,3.92-4.78l-25.41-128A4,4,0,0,0,268.72,360Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M325.68,354.32l-11.36,11.36a4,4,0,0,0-.5,5.05l59.47,89.21a4,4,0,0,0,6.16.61l41.1-41.1a4,4,0,0,0-.61-6.16l-89.21-59.47A4,4,0,0,0,325.68,354.32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M388,336a4,4,0,0,0-4,4v8.61a4,4,0,0,0,3.34,3.95l88,14.66a4,4,0,0,0,4.66-3.94V340a4,4,0,0,0-4-4Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M186.32,354.32l11.36,11.36a4,4,0,0,1,.5,5.05l-59.47,89.21a4,4,0,0,1-6.16.61l-41.1-41.1a4,4,0,0,1,.61-6.16l89.21-59.47A4,4,0,0,1,186.32,354.32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M124,336H36a4,4,0,0,0-4,4v23.28a4,4,0,0,0,4.66,3.94l88-14.66a4,4,0,0,0,3.34-3.95V340A4,4,0,0,0,124,336Z"
        /> < title > { title } < / title > < / svg >
    }
}
