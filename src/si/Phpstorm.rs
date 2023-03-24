#[cfg(feature = "SiPhpstorm")]
use leptos::*;
#[cfg(feature = "SiPhpstorm")]
///This icon requires the feature `SiPhpstorm` to be enabled.
#[component]
pub fn Phpstorm(
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
        "M7.833 6.611v-.055c0-1-.667-1.5-1.778-1.5H4.389v3.056h1.722c1.111-.001 1.722-.668 1.722-1.501zM0 0v24h24V0H0zm2.167 3.111h4.056c2.389 0 3.833 1.389 3.833 3.445v.055c0 2.333-1.778 3.5-4.056 3.5H4.333v3H2.167v-10zM11.278 21h-9v-1.5h9V21zM18.5 10.222c0 2-1.5 3.111-3.667 3.111-1.5-.056-3-.611-4.222-1.667l1.278-1.556c.89.722 1.833 1.222 3 1.222.889 0 1.444-.333 1.444-.944v-.056c0-.555-.333-.833-2-1.277C12.333 8.555 11 8 11 6v-.056c0-1.833 1.444-3 3.5-3 1.444 0 2.723.444 3.723 1.278l-1.167 1.667c-.889-.611-1.777-1-2.611-1-.833 0-1.278.389-1.278.889v.056c0 .667.445.889 2.167 1.333 2 .556 3.167 1.278 3.167 3v.055z"
        /> < title > { title } < / title > < / svg >
    }
}
