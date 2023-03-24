#[cfg(feature = "FaBrandsForumbee")]
use leptos::*;
#[cfg(feature = "FaBrandsForumbee")]
///This icon requires the feature `FaBrandsForumbee` to be enabled.
#[component]
pub fn Forumbee(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.8 309.7C2 292.7 0 275.5 0 258.3 0 135 99.8 35 223.1 35c16.6 0 33.3 2 49.3 5.5C149 87.5 51.9 186 5.8 309.7zm392.9-189.2C385 103 369 87.8 350.9 75.2c-149.6 44.3-266.3 162.1-309.7 312 12.5 18.1 28 35.6 45.2 49 43.1-151.3 161.2-271.7 312.3-315.7zm15.8 252.7c15.2-25.1 25.4-53.7 29.5-82.8-79.4 42.9-145 110.6-187.6 190.3 30-4.4 58.9-15.3 84.6-31.3 35 13.1 70.9 24.3 107 33.6-9.3-36.5-20.4-74.5-33.5-109.8zm29.7-145.5c-2.6-19.5-7.9-38.7-15.8-56.8C290.5 216.7 182 327.5 137.1 466c18.1 7.6 37 12.5 56.6 15.2C240 367.1 330.5 274.4 444.2 227.7z"
        /> < title > { title } < / title > < / svg >
    }
}
