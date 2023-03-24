#[cfg(feature = "OcLgStop")]
use leptos::*;
#[cfg(feature = "OcLgStop")]
///This icon requires the feature `OcLgStop` to be enabled.
#[component]
pub fn Stop(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 7a.75.75 0 0 1 .75.75v4.5a.75.75 0 0 1-1.5 0v-4.5A.75.75 0 0 1 12 7Zm0 10a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.328 1.47a.749.749 0 0 1 .53-.22h8.284c.199 0 .389.079.53.22l5.858 5.858c.141.14.22.33.22.53v8.284a.749.749 0 0 1-.22.53l-5.858 5.858a.749.749 0 0 1-.53.22H7.858a.749.749 0 0 1-.53-.22L1.47 16.672a.749.749 0 0 1-.22-.53V7.858c0-.199.079-.389.22-.53Zm.84 1.28L2.75 8.169v7.662l5.419 5.419h7.662l5.419-5.418V8.168L15.832 2.75Z"
        /> < title > { title } < / title > < / svg >
    }
}
