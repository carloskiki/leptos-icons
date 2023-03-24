#[cfg(feature = "SiNeo4j")]
use leptos::*;
#[cfg(feature = "SiNeo4j")]
///This icon requires the feature `SiNeo4j` to be enabled.
#[component]
pub fn Neo4j(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M9.629 13.227c-.593 0-1.139.2-1.58.533l-2.892-1.976a2.61 2.61 0 0 0 .101-.711 2.633 2.633 0 0 0-2.629-2.629A2.632 2.632 0 0 0 0 11.073a2.632 2.632 0 0 0 2.629 2.629c.593 0 1.139-.2 1.579-.533L7.1 15.145c-.063.226-.1.465-.1.711 0 .247.037.484.1.711l-2.892 1.976a2.608 2.608 0 0 0-1.579-.533A2.632 2.632 0 0 0 0 20.639a2.632 2.632 0 0 0 2.629 2.629 2.632 2.632 0 0 0 2.629-2.629c0-.247-.037-.485-.101-.711l2.892-1.976c.441.333.987.533 1.58.533a2.633 2.633 0 0 0 2.629-2.629c0-1.45-1.18-2.629-2.629-2.629ZM16.112.732c-4.72 0-7.888 2.748-7.888 8.082v3.802a3.525 3.525 0 0 1 3.071.008v-3.81c0-3.459 1.907-5.237 4.817-5.237s4.817 1.778 4.817 5.237v8.309H24V8.814C24 3.448 20.832.732 16.112.732Z"
        /> < title > { title } < / title > < / svg >
    }
}
