#[cfg(feature = "IoColorFill")]
use leptos::*;
#[cfg(feature = "IoColorFill")]
///This icon requires the feature `IoColorFill` to be enabled.
#[component]
pub fn ColorFill(
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
        "M416,480c-35.29,0-64-29.11-64-64.88,0-33.29,28.67-65.4,44.08-82.64,1.87-2.1,3.49-3.91,4.68-5.31a19.94,19.94,0,0,1,30.55,0c1.13,1.31,2.63,3,4.36,4.93,15.5,17.3,44.33,49.51,44.33,83.05C480,450.89,451.29,480,416,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M398.23,276.64,166.89,47.22a52.1,52.1,0,0,0-73.6,0l-4.51,4.51A53.2,53.2,0,0,0,72.89,89.06,51.66,51.66,0,0,0,88.14,126l41.51,41.5L45,252a44.52,44.52,0,0,0-13,32,42.81,42.81,0,0,0,13.5,30.84l131.24,126a44,44,0,0,0,61.08-.18L361.93,320.38a15.6,15.6,0,0,1,8.23-4.29,69.21,69.21,0,0,1,11.93-.86h.3a22.53,22.53,0,0,0,15.84-38.59ZM152.29,144.85l-41.53-41.52a20,20,0,0,1,0-28.34l5.16-5.15a20.07,20.07,0,0,1,28.39,0L186,111.21Z"
        /> < title > { title } < / title > < / svg >
    }
}
