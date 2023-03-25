#[cfg(feature = "RiOthersLineCake3")]
use leptos::*;
#[cfg(feature = "RiOthersLineCake3")]
///This icon requires the feature `RiOthersLineCake3` to be enabled.
#[component]
pub fn Cake3(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M15.5 2a3.5 3.5 0 0 1 3.437 4.163l-.015.066a4.502 4.502 0 0 1 .303 8.428l-1.086 6.507a1 1 0 0 1-.986.836H6.847a1 1 0 0 1-.986-.836l-1.029-6.17a3 3 0 0 1-.829-5.824L4 9a6 6 0 0 1 8.574-5.421A3.496 3.496 0 0 1 15.5 2zM9 15H6.86l.834 5H9v-5zm4 0h-2v5h2v-5zm4.139 0H15v5h1.305l.834-5zM10 5C7.858 5 6.109 6.684 6.005 8.767L6 8.964l.003.17a2 2 0 0 1-1.186 1.863l-.15.059A1.001 1.001 0 0 0 5 13h12.5a2.5 2.5 0 1 0-.956-4.81l-.175.081a2 2 0 0 1-2.663-.804l-.07-.137A4 4 0 0 0 10 5zm5.5-1a1.5 1.5 0 0 0-1.287.729 6.006 6.006 0 0 1 1.24 1.764c.444-.228.93-.384 1.446-.453A1.5 1.5 0 0 0 15.5 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
