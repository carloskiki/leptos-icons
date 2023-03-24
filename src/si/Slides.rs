#[cfg(feature = "SiSlides")]
use leptos::*;
#[cfg(feature = "SiSlides")]
///This icon requires the feature `SiSlides` to be enabled.
#[component]
pub fn Slides(
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
        "M0 0v24h24V0H0zm14.176 15.794c-.644.584-1.41.87-2.31.87-.9 0-1.664-.137-2.324-.406-.646-.27-.976-.525-.976-.78l-.076.095c0-.13.08-.324.244-.585.163-.26.31-.39.44-.39.014 0 .12.06.314.184.195.125.472.247.837.372.364.123.804.185 1.317.185.525 0 .945-.147 1.275-.44.33-.292.495-.69.495-1.19s-.15-.89-.436-1.17c-.285-.28-.87-.544-1.74-.79-.87-.25-1.5-.564-1.89-.946-.39-.386-.586-.968-.586-1.748s.3-1.435.9-1.962c.6-.54 1.38-.795 2.34-.795.675 0 1.26.074 1.785.254.524.165.78.36.78.555 0 .164-.075.39-.226.644-.15.27-.285.39-.39.39-.017 0-.09-.045-.257-.12-.435-.24-.93-.36-1.484-.36s-.976.105-1.29.33c-.3.225-.45.525-.45.885s.12.644.344.854.615.39 1.17.57c1.23.375 2.055.78 2.49 1.23.436.45.66 1.126.66 2.025.002.9-.312 1.65-.958 2.234z"
        /> < title > { title } < / title > < / svg >
    }
}
