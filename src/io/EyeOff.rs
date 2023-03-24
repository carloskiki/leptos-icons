#[cfg(feature = "IoEyeOff")]
use leptos::*;
#[cfg(feature = "IoEyeOff")]
///This icon requires the feature `IoEyeOff` to be enabled.
#[component]
pub fn EyeOff(
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
        "M432,448a15.92,15.92,0,0,1-11.31-4.69l-352-352A16,16,0,0,1,91.31,68.69l352,352A16,16,0,0,1,432,448Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M248,315.85l-51.79-51.79a2,2,0,0,0-3.39,1.69,64.11,64.11,0,0,0,53.49,53.49A2,2,0,0,0,248,315.85Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M264,196.15,315.87,248a2,2,0,0,0,3.4-1.69,64.13,64.13,0,0,0-53.55-53.55A2,2,0,0,0,264,196.15Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M491,273.36a32.2,32.2,0,0,0-.1-34.76c-26.46-40.92-60.79-75.68-99.27-100.53C349,110.55,302,96,255.68,96a226.54,226.54,0,0,0-71.82,11.79,4,4,0,0,0-1.56,6.63l47.24,47.24a4,4,0,0,0,3.82,1.05,96,96,0,0,1,116,116,4,4,0,0,0,1.05,3.81l67.95,68a4,4,0,0,0,5.4.24A343.81,343.81,0,0,0,491,273.36Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,352a96,96,0,0,1-93.3-118.63,4,4,0,0,0-1.05-3.81L94.81,162.69a4,4,0,0,0-5.41-.23c-24.39,20.81-47,46.13-67.67,75.72a31.92,31.92,0,0,0-.64,35.54c26.41,41.33,60.39,76.14,98.28,100.65C162.06,402,207.92,416,255.68,416a238.22,238.22,0,0,0,72.64-11.55,4,4,0,0,0,1.61-6.64l-47.47-47.46a4,4,0,0,0-3.81-1.05A96,96,0,0,1,256,352Z"
        /> < title > { title } < / title > < / svg >
    }
}
