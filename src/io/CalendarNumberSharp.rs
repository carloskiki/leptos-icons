#[cfg(feature = "IoCalendarNumberSharp")]
use leptos::*;
#[cfg(feature = "IoCalendarNumberSharp")]
///This icon requires the feature `IoCalendarNumberSharp` to be enabled.
#[component]
pub fn CalendarNumberSharp(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M32,456a24,24,0,0,0,24,24H456a24,24,0,0,0,24-24V176H32ZM342.17,212H368V412H336V256.29l-35.39,26.08-19-25.76ZM222,335.3c-8.54-8.74-22.75-12.67-30.11-12.67h-16v-32h16c4.85,0,17.41-2.6,25.28-10.65a22,22,0,0,0,6.57-16.08c0-23.23-28.63-23.9-31.89-23.9-17.34,0-23.8,10.61-24.07,11.06l-8.13,13.78-27.56-16.27,8.14-13.77c7.64-13,25.22-26.8,51.62-26.8,16.44,0,31.76,4.77,43.13,13.42,13.39,10.2,20.76,25.28,20.76,42.48A54,54,0,0,1,240,302.35c-1.15,1.18-2.36,2.28-3.59,3.35a66.18,66.18,0,0,1,8.42,7.23c10.56,10.8,16.14,25.75,16.14,43.25,0,18.06-7.61,34-21.42,44.92-12.17,9.61-28.75,14.9-46.7,14.9-27.87,0-48.48-18.16-57.66-33.7l-8.13-13.78,27.56-16.27L162.78,366c1.08,1.84,11.15,18,30.1,18,16.66,0,36.12-7.29,36.12-27.82C229,349.93,227.78,341.23,222,335.3Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M456,64H400.08V32h-48V64H159.92V32h-48V64H56A23.8,23.8,0,0,0,32,87.77V144H480V87.77A23.8,23.8,0,0,0,456,64Z"
        /> < title > { title } < / title > < / svg >
    }
}
