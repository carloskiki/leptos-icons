#[cfg(feature = "BiRegularMedal")]
use leptos::*;
#[cfg(feature = "BiRegularMedal")]
///This icon requires the feature `BiRegularMedal` to be enabled.
#[component]
pub fn Medal(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 22c3.859 0 7-3.141 7-7s-3.141-7-7-7c-3.86 0-7 3.141-7 7s3.14 7 7 7zm0-12c2.757 0 5 2.243 5 5s-2.243 5-5 5-5-2.243-5-5 2.243-5 5-5zm-1-8H7v5.518a8.957 8.957 0 0 1 4-1.459V2zm6 0h-4v4.059a8.957 8.957 0 0 1 4 1.459V2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m10.019 15.811-.468 2.726L12 17.25l2.449 1.287-.468-2.726 1.982-1.932-2.738-.398L12 11l-1.225 2.481-2.738.398z"
        /> < title > { title } < / title > < / svg >
    }
}
