#[cfg(feature = "SiFacepunch")]
use leptos::*;
#[cfg(feature = "SiFacepunch")]
///This icon requires the feature `SiFacepunch` to be enabled.
#[component]
pub fn Facepunch(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 0C5.39 0 .004 5.388.004 12S5.39 24 12 24s11.996-5.388 11.996-12S18.627 0 12 0zm0 21.314c-5.131 0-9.294-4.164-9.294-9.297C2.706 6.884 6.869 2.72 12 2.72s9.294 4.164 9.294 9.297c0 5.133-4.163 9.297-9.294 9.297zm-.561-10.725l-1.92-1.904-1.41 1.411L6.75 8.719l-1.92 1.904L6.206 12l-1.444 1.445 1.92 1.921 1.427-1.445L9.587 15.4l1.92-1.921L10.029 12l1.41-1.411zm7.748-.051l-1.41 1.411 1.478 1.479-1.92 1.904-1.478-1.479-1.444 1.445-1.903-1.921 1.444-1.428-1.376-1.377 1.903-1.921 1.376 1.377 1.41-1.411 1.92 1.921z"
        /> < title > { title } < / title > < / svg >
    }
}
