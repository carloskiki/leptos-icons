#[cfg(feature = "HiLgSolidServerStack")]
use leptos::*;
#[cfg(feature = "HiLgSolidServerStack")]
///This icon requires the feature `HiLgSolidServerStack` to be enabled.
#[component]
pub fn ServerStack(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.50701 4.04763C6.07695 3.38269 6.909 3 7.78478 3H16.2151C17.0908 3 17.9229 3.38269 18.4928 4.04763L20.2147 6.05645C19.9819 6.0193 19.7432 6 19.4999 6H4.49992C4.25668 6 4.01794 6.0193 3.78516 6.05645L5.50701 4.04763Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1.5 10.5C1.5 8.84315 2.84315 7.5 4.5 7.5H19.5C21.1569 7.5 22.5 8.84315 22.5 10.5C22.5 12.1569 21.1569 13.5 19.5 13.5H4.5C2.84315 13.5 1.5 12.1569 1.5 10.5ZM16.5 10.5C16.5 10.9142 16.1642 11.25 15.75 11.25C15.3358 11.25 15 10.9142 15 10.5C15 10.0858 15.3358 9.75 15.75 9.75C16.1642 9.75 16.5 10.0858 16.5 10.5ZM18.75 11.25C19.1642 11.25 19.5 10.9142 19.5 10.5C19.5 10.0858 19.1642 9.75 18.75 9.75C18.3358 9.75 18 10.0858 18 10.5C18 10.9142 18.3358 11.25 18.75 11.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4.5 15C2.84315 15 1.5 16.3431 1.5 18C1.5 19.6569 2.84315 21 4.5 21H19.5C21.1569 21 22.5 19.6569 22.5 18C22.5 16.3431 21.1569 15 19.5 15H4.5ZM15.75 18.75C16.1642 18.75 16.5 18.4142 16.5 18C16.5 17.5858 16.1642 17.25 15.75 17.25C15.3358 17.25 15 17.5858 15 18C15 18.4142 15.3358 18.75 15.75 18.75ZM19.5 18C19.5 18.4142 19.1642 18.75 18.75 18.75C18.3358 18.75 18 18.4142 18 18C18 17.5858 18.3358 17.25 18.75 17.25C19.1642 17.25 19.5 17.5858 19.5 18Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
