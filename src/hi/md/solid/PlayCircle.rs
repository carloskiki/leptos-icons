#[cfg(feature = "HiMdSolidPlayCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidPlayCircle")]
///This icon requires the feature `HiMdSolidPlayCircle` to be enabled.
#[component]
pub fn PlayCircle(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10ZM8.39061 7.09172C8.63141 6.96025 8.92479 6.97076 9.15557 7.11912L12.6556 9.36912C12.8702 9.50712 13 9.7448 13 10C13 10.2552 12.8702 10.4929 12.6556 10.6309L9.15557 12.8809C8.92479 13.0292 8.63141 13.0398 8.39061 12.9083C8.1498 12.7768 8 12.5244 8 12.25V7.75C8 7.47565 8.1498 7.22318 8.39061 7.09172Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
