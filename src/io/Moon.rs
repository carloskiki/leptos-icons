#[cfg(feature = "IoMoon")]
use leptos::*;
#[cfg(feature = "IoMoon")]
///This icon requires the feature `IoMoon` to be enabled.
#[component]
pub fn Moon(
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
        "M264,480A232,232,0,0,1,32,248C32,154,86,69.72,169.61,33.33a16,16,0,0,1,21.06,21.06C181.07,76.43,176,104.66,176,136c0,110.28,89.72,200,200,200,31.34,0,59.57-5.07,81.61-14.67a16,16,0,0,1,21.06,21.06C442.28,426,358,480,264,480Z"
        /> < title > { title } < / title > < / svg >
    }
}
