#[cfg(feature = "IoTicketSharp")]
use leptos::*;
#[cfg(feature = "IoTicketSharp")]
///This icon requires the feature `IoTicketSharp` to be enabled.
#[component]
pub fn TicketSharp(
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
        "M426.24,127.72,415.3,138.66a29.67,29.67,0,0,1-42-42l10.94-10.94L314.52,16l-88,88-4,12.09-12.09,4L16,314.52l69.76,69.76L96.7,373.34a29.67,29.67,0,0,1,42,42l-10.94,10.94L197.48,496l194.4-194.4,4-12.09,12.09-4,88-88Zm-208.56,5.43,21.87-21.87,33,33-21.88,21.87Zm43,43,21.88-21.88,32.52,32.52-21.88,21.88Zm42.56,42.56,21.88-21.88,32.52,32.52L335.8,251.28Zm75.57,75.56-33-33,21.87-21.88,33,33Z"
        /> < title > { title } < / title > < / svg >
    }
}
