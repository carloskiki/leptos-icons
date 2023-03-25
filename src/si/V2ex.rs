#[cfg(feature = "SiV2ex")]
use leptos::*;
#[cfg(feature = "SiV2ex")]
///This icon requires the feature `SiV2ex` to be enabled.
#[component]
pub fn V2ex(
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
        "M.671 1.933h13.821a1.342 1.342 0 0 1 .98.425l8.166 8.725a1.342 1.342 0 0 1 0 1.834l-8.166 8.724a1.342 1.342 0 0 1-.98.426H.673A.671.671 0 0 1 0 21.395v-6.878h13.19l2.276-2.28a.336.336 0 0 0 0-.474l-2.276-2.28H0V2.604a.671.671 0 0 1 .671-.671Z"
        /> < title > { title } < / title > < / svg >
    }
}
