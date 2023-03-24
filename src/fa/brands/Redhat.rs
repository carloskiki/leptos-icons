#[cfg(feature = "FaBrandsRedhat")]
use leptos::*;
#[cfg(feature = "FaBrandsRedhat")]
///This icon requires the feature `FaBrandsRedhat` to be enabled.
#[component]
pub fn Redhat(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M341.52 285.56c33.65 0 82.34-6.94 82.34-47 .22-6.74.86-1.82-20.88-96.24-4.62-19.15-8.68-27.84-42.31-44.65-26.09-13.34-82.92-35.37-99.73-35.37-15.66 0-20.2 20.17-38.87 20.17-18 0-31.31-15.06-48.12-15.06-16.14 0-26.66 11-34.78 33.62-27.5 77.55-26.28 74.27-26.12 78.27 0 24.8 97.64 106.11 228.47 106.11M429 254.84c4.65 22 4.65 24.35 4.65 27.25 0 37.66-42.33 58.56-98 58.56-125.74.08-235.91-73.65-235.91-122.33a49.55 49.55 0 0 1 4.06-19.72C58.56 200.86 0 208.93 0 260.63c0 84.67 200.63 189 359.49 189 121.79 0 152.51-55.08 152.51-98.58 0-34.21-29.59-73.05-82.93-96.24"
        /> < title > { title } < / title > < / svg >
    }
}
