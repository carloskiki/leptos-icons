#[cfg(feature = "SiCockpit")]
use leptos::*;
#[cfg(feature = "SiCockpit")]
///This icon requires the feature `SiCockpit` to be enabled.
#[component]
pub fn Cockpit(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 0C5.383 0 0 5.382 0 12s5.383 12 12 12 12-5.383 12-12S18.617 0 12 0zm0 1.799A10.19 10.19 0 0 1 22.207 12 10.19 10.19 0 0 1 12 22.201 10.186 10.186 0 0 1 1.799 12 10.186 10.186 0 0 1 12 1.799zm4.016 5.285c-.49-.018-1.232.368-1.899 1.031l-1.44 1.43-4.31-1.447-.842.867 3.252 2.47-.728.723a4.747 4.747 0 0 0-.639.787L7.451 12.8l-.476.484 1.947 1.444 1.424 1.943.48-.48-.144-1.98c.246-.16.497-.361.74-.603l.765-.76 2.495 3.274.869-.84-1.455-4.332 1.394-1.385c.89-.885 1.298-1.92.918-2.322a.547.547 0 0 0-.392-.158z"
        /> < title > { title } < / title > < / svg >
    }
}
