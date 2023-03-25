#[cfg(feature = "ImGooglePlus")]
use leptos::*;
#[cfg(feature = "ImGooglePlus")]
///This icon requires the feature `ImGooglePlus` to be enabled.
#[component]
pub fn GooglePlus(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M5.091 7.147v1.747h2.888c-0.116 0.75-0.872 2.197-2.888 2.197-1.737 0-3.156-1.441-3.156-3.216s1.419-3.216 3.156-3.216c0.991 0 1.65 0.422 2.028 0.784l1.381-1.331c-0.888-0.828-2.037-1.331-3.409-1.331-2.816 0.003-5.091 2.278-5.091 5.094s2.275 5.091 5.091 5.091c2.937 0 4.888-2.066 4.888-4.975 0-0.334-0.037-0.591-0.081-0.844h-4.806z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M16 7h-1.5v-1.5h-1.5v1.5h-1.5v1.5h1.5v1.5h1.5v-1.5h1.5z" /> < title > { title }
        < / title > < / svg >
    }
}
