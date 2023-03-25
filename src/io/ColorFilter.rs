#[cfg(feature = "IoColorFilter")]
use leptos::*;
#[cfg(feature = "IoColorFilter")]
///This icon requires the feature `IoColorFilter` to be enabled.
#[component]
pub fn ColorFilter(
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
        "M253.72,202.53a4,4,0,0,0,4.56,0,151.88,151.88,0,0,1,128.44-20.41,4,4,0,0,0,5.15-4C388.8,105.86,329,48,256,48S123.2,105.86,120.13,178.15a4,4,0,0,0,5.15,4,151.88,151.88,0,0,1,128.44,20.41Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M405.31,212.56a152.53,152.53,0,0,1-83.08,108.23,4,4,0,0,0-2.28,3.69c0,1.17.05,2.34.05,3.52a151.58,151.58,0,0,1-47.15,109.94,4,4,0,0,0,.64,6.31A135.24,135.24,0,0,0,344,464c72.07,0,134.1-60.28,136-132.34a136.07,136.07,0,0,0-68.76-121.87A4,4,0,0,0,405.31,212.56Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M390.57,203.67a4,4,0,0,0-2.69-4.4,135.84,135.84,0,0,0-114.4,12.49,4,4,0,0,0-.64,6.29,151.92,151.92,0,0,1,44.47,81.4,4,4,0,0,0,5.94,2.72A136.29,136.29,0,0,0,390.57,203.67Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M192,328c0-1.18,0-2.35.05-3.52a4,4,0,0,0-2.28-3.69,152.53,152.53,0,0,1-83.08-108.23,4,4,0,0,0-5.88-2.77A136.07,136.07,0,0,0,32.05,331.66C34,403.72,96,464,168.05,464a135.24,135.24,0,0,0,70.46-19.75,4,4,0,0,0,.64-6.31A151.58,151.58,0,0,1,192,328Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M168,192a135.34,135.34,0,0,0-43.88,7.27,4,4,0,0,0-2.69,4.4,136.29,136.29,0,0,0,67.32,98.5,4,4,0,0,0,5.94-2.72,151.92,151.92,0,0,1,44.47-81.4,4,4,0,0,0-.64-6.29A135.18,135.18,0,0,0,168,192Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,336a151.44,151.44,0,0,1-42.72-6.12,4,4,0,0,0-5.15,4,135.69,135.69,0,0,0,45.18,95.4,4,4,0,0,0,5.38,0,135.69,135.69,0,0,0,45.18-95.4,4,4,0,0,0-5.15-4A151.44,151.44,0,0,1,256,336Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M302.57,308.33a135.94,135.94,0,0,0-43.87-81.58,4.06,4.06,0,0,0-5.4,0,135.94,135.94,0,0,0-43.87,81.58,4,4,0,0,0,2.69,4.4,136.06,136.06,0,0,0,87.76,0A4,4,0,0,0,302.57,308.33Z"
        /> < title > { title } < / title > < / svg >
    }
}
