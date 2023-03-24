#[cfg(feature = "SiUbiquiti")]
use leptos::*;
#[cfg(feature = "SiUbiquiti")]
///This icon requires the feature `SiUbiquiti` to be enabled.
#[component]
pub fn Ubiquiti(
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
        "M5.331 3.987H4.012v-1.32h1.32zm7.605 16.001c-1.78-.08-3.15-.532-4.21-1.185.718 3.118 3.405 4.65 3.535 4.723l.792.437c6.063-.405 9.611-4.318 9.611-9.436v-1.109c-1.441 4.7-4.795 6.793-9.728 6.57M4.006 9.605h1.332v2.94h1.336V7.627H8.01v9.612C8.009 21.8 12 24 12 24c-6.705 0-10.664-4.065-10.664-9.473V3.65H2.67v7.274h1.336zM2.67 1.334H1.336V0H2.67zm2.661 6.953h-1.32v-1.32h1.32zm1.334-1.98h-1.32v-1.32h1.32zm1.343-1.66H6.674v-1.32h1.335zM6.674 2.654H5.338v-1.32h1.336zM22.147 13.26l.517-1.688V.015c-6.045 0-6.674 2.317-6.674 4.531V17.24c0 .657-.064 1.354-.197 2.037 3.205-.583 5.296-2.565 6.354-6.016Z"
        /> < title > { title } < / title > < / svg >
    }
}
