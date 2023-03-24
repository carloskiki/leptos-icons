#[cfg(feature = "RiHealthLineNurse")]
use leptos::*;
#[cfg(feature = "RiHealthLineNurse")]
///This icon requires the feature `RiHealthLineNurse` to be enabled.
#[component]
pub fn Nurse(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M12 15c4.08 0 7.446 3.054 7.938 7H4.062c.492-3.946 3.858-7 7.938-7zm-1.813 2.28C8.753 17.734 7.546 18.713 6.8 20H12l-1.813-2.72zm3.627 0L12 20h5.199c-.745-1.287-1.952-2.266-3.385-2.72zM18 2v6c0 3.314-2.686 6-6 6s-6-2.686-6-6V2h12zM8 8c0 2.21 1.79 4 4 4s4-1.79 4-4H8zm8-4H8v2h8V4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
