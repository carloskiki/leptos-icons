#[cfg(feature = "FaSolidWineBottle")]
use leptos::*;
#[cfg(feature = "FaSolidWineBottle")]
///This icon requires the feature `FaSolidWineBottle` to be enabled.
#[component]
pub fn WineBottle(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M393.4 9.4c12.5-12.5 32.8-12.5 45.3 0l64 64c12.5 12.5 12.5 32.8 0 45.3c-11.8 11.8-30.7 12.5-43.2 1.9l-9.5 9.5-48.8 48.8c-9.2 9.2-11.5 22.9-8.6 35.6c9.4 40.9-1.9 85.6-33.8 117.5L197.3 493.3c-25 25-65.5 25-90.5 0l-88-88c-25-25-25-65.5 0-90.5L180.2 153.3c31.9-31.9 76.6-43.1 117.5-33.8c12.6 2.9 26.4 .5 35.5-8.6l48.8-48.8 9.5-9.5c-10.6-12.6-10-31.4 1.9-43.2zM99.3 347.3l65.4 65.4c6.2 6.2 16.4 6.2 22.6 0l97.4-97.4c6.2-6.2 6.2-16.4 0-22.6l-65.4-65.4c-6.2-6.2-16.4-6.2-22.6 0L99.3 324.7c-6.2 6.2-6.2 16.4 0 22.6z"
        /> < title > { title } < / title > < / svg >
    }
}
