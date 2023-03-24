#[cfg(feature = "SiProtractor")]
use leptos::*;
#[cfg(feature = "SiProtractor")]
///This icon requires the feature `SiProtractor` to be enabled.
#[component]
pub fn Protractor(
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
        "M12 0C5.37 0 0 5.372 0 12s5.371 12 12 12c6.628 0 12-5.372 12-12S18.627 0 12 0zm-.273 3.789v1.71h.545v-1.71a9.055 9.055 0 015.961 2.468l-1.277 1.278.386.386 1.277-1.278a9.057 9.057 0 012.469 5.96h-1.71v.546h1.717v2.001H2.905v-2H4.62v-.546h-1.71a9.058 9.058 0 012.469-5.96L6.658 7.92l.386-.386-1.278-1.278a9.056 9.056 0 015.96-2.468zM12 6.965a5.912 5.912 0 00-5.913 5.912h11.824A5.91 5.91 0 0012 6.965z"
        /> < title > { title } < / title > < / svg >
    }
}
