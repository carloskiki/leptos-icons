#[cfg(feature = "IoHeartDislike")]
use leptos::*;
#[cfg(feature = "IoHeartDislike")]
///This icon requires the feature `IoHeartDislike` to be enabled.
#[component]
pub fn HeartDislike(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M417.84,448a16,16,0,0,1-11.35-4.72L40.65,75.28a16,16,0,1,1,22.7-22.56l365.83,368A16,16,0,0,1,417.84,448Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M364.92,80c-44.09,0-74.61,24.82-92.39,45.5a6,6,0,0,1-9.06,0C245.69,104.82,215.16,80,171.08,80a107.71,107.71,0,0,0-31,4.54l269.13,270.7c3-3.44,5.7-6.64,8.14-9.6,40-48.75,59.15-98.79,58.61-153C475.37,130.53,425.54,80,364.92,80Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M69,149.15a115.06,115.06,0,0,0-9,43.49c-.54,54.21,18.63,104.25,58.61,153,18.77,22.87,52.8,59.45,131.39,112.8a31.88,31.88,0,0,0,36,0c20.35-13.82,37.7-26.5,52.58-38.12Z"
        /> < title > { title } < / title > < / svg >
    }
}
