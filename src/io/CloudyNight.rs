#[cfg(feature = "IoCloudyNight")]
use leptos::*;
#[cfg(feature = "IoCloudyNight")]
///This icon requires the feature `IoCloudyNight` to be enabled.
#[component]
pub fn CloudyNight(
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
        "M340,480H106c-29.5,0-54.92-7.83-73.53-22.64C11.23,440.44,0,415.35,0,384.8c0-29.44,12.09-54.25,35-71.74,12.1-9.26,27.2-16.17,43.33-20.05A16,16,0,0,0,90.14,280.8c7.15-32.54,22.25-60.49,44.33-81.75A139.82,139.82,0,0,1,232,160c32.33,0,62.15,10.65,86.24,30.79a142.22,142.22,0,0,1,37.65,49.54,16.06,16.06,0,0,0,11.12,9c24,5.22,45.42,15.78,61.62,30.56C451.77,301,464,329.82,464,363.2c0,32.85-13.13,62.87-37,84.52C404.11,468.54,373.2,480,340,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M510.53,209.79a16.34,16.34,0,0,0-1.35-15.8,16,16,0,0,0-19.57-5.58c-10.7,4.65-24.48,7.17-39.92,7.28-55.3.4-101.38-45-101.38-100.31,0-15.75,2.48-29.84,7.18-40.76a16.3,16.3,0,0,0-1.85-16.33,16,16,0,0,0-19.1-5c-38.63,16.82-66.18,51.51-75.27,92.54a4,4,0,0,0,3.19,4.79,162.54,162.54,0,0,1,76.31,35.59,172.58,172.58,0,0,1,39.64,47.84,16.35,16.35,0,0,0,9.54,7.64c23.89,7.17,45.1,18.9,62.25,34.54q4.44,4.07,8.48,8.42a4,4,0,0,0,5.16.57A129.12,129.12,0,0,0,510.53,209.79Z"
        /> < title > { title } < / title > < / svg >
    }
}
