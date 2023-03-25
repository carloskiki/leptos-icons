#[cfg(feature = "SiMax")]
use leptos::*;
#[cfg(feature = "SiMax")]
///This icon requires the feature `SiMax` to be enabled.
#[component]
pub fn Max(
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
        "M1.769 0A1.77 1.77 0 0 0 0 1.769V22.23A1.77 1.77 0 0 0 1.769 24H22.23A1.77 1.77 0 0 0 24 22.231V1.77A1.77 1.77 0 0 0 22.231 0zm12.485 3.28a4.301 4.301 0 0 1 4.3 4.302 4.301 4.301 0 0 1-1.993 3.63 6.085 6.085 0 0 1 1.054 3.422 6.085 6.085 0 0 1-6.085 6.085 6.085 6.085 0 0 1-6.085-6.085 6.085 6.085 0 0 1 4.66-5.916 4.301 4.301 0 0 1-.152-1.136 4.301 4.301 0 0 1 4.301-4.301zm0 1.849a2.453 2.453 0 0 0-2.453 2.453 2.453 2.453 0 0 0 2.453 2.453 2.453 2.453 0 0 0 2.453-2.453 2.453 2.453 0 0 0-2.453-2.453zm-2.724 5.268a4.237 4.237 0 0 0-4.237 4.237 4.237 4.237 0 0 0 4.237 4.237 4.237 4.237 0 0 0 4.237-4.237 4.237 4.237 0 0 0-4.237-4.237zm.032 2.54a1.781 1.781 0 1 1 0 3.562 1.781 1.781 0 0 1 0-3.562Z"
        /> < title > { title } < / title > < / svg >
    }
}
