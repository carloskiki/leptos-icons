#[cfg(feature = "FaSolidCubes")]
use leptos::*;
#[cfg(feature = "FaSolidCubes")]
///This icon requires the feature `FaSolidCubes` to be enabled.
#[component]
pub fn Cubes(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M273.8 45.8l73.7 28L271.2 103 194.8 73.7l73.7-28c1.7-.7 3.6-.7 5.3 0zM128.1 87.1V192.6c-1.2 .4-2.4 .8-3.6 1.2L34.1 228.1c-20.5 7.8-34 27.4-34 49.3V389.5c0 20.9 12.4 39.8 31.5 48.3L122 477.5c13.5 5.9 28.9 5.9 42.4 0l106.8-46.9L378 477.5c13.5 5.9 28.9 5.9 42.4 0l90.4-39.7c19.1-8.4 31.5-27.3 31.5-48.3V277.4c0-21.9-13.5-41.5-34-49.3l-90.4-34.3c-1.2-.5-2.4-.9-3.6-1.2V87.1c0-21.9-13.5-41.5-34-49.3L289.9 3.5c-12-4.6-25.3-4.6-37.4 0L162.1 37.8c-20.5 7.8-34 27.4-34 49.3zM369.1 198.2l-77.5 29.4v-84l77.5-29.7v84.3zM145.8 236.1l73.7 28-76.4 29.3L66.8 264.1l73.7-28c1.7-.7 3.6-.7 5.3 0zm17.7 192.4V333.9l77.5-29.7v90.2l-77.5 34.1zm233-192.4c1.7-.7 3.6-.7 5.3 0l73.7 28-76.4 29.3-76.4-29.3 73.7-28zm96.1 160.3l-73 32.1V333.9l77.5-29.7v85.3c0 3-1.8 5.7-4.5 6.9z"
        /> < title > { title } < / title > < / svg >
    }
}
