#[cfg(feature = "BiRegularAlbum")]
use leptos::*;
#[cfg(feature = "BiRegularAlbum")]
///This icon requires the feature `BiRegularAlbum` to be enabled.
#[component]
pub fn Album(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "11.99" cy = "11.99" r = "2.01" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 6a6 6 0 0 0-6 6h2a4 4 0 0 1 4-4z" /> < title > { title } < / title > < / svg
        >
    }
}
