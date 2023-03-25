#[cfg(feature = "BiLogosInvision")]
use leptos::*;
#[cfg(feature = "BiLogosInvision")]
///This icon requires the feature `BiLogosInvision` to be enabled.
#[component]
pub fn Invision(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19.361 3.008H4.638c-.901 0-1.63.729-1.63 1.63v14.724c0 .9.729 1.631 1.63 1.631h14.724c.9 0 1.631-.73 1.631-1.631V4.638a1.63 1.63 0 0 0-1.632-1.63zM9.018 6.505c.597 0 1.098.472 1.098 1.078 0 .616-.501 1.08-1.098 1.08v.022c-.604 0-1.09-.486-1.09-1.088 0-.605.486-1.092 1.09-1.092zm7.354 10.352c-1.112 0-1.651-.662-1.651-1.566 0-.26.033-.533.114-.812l.528-1.906c.068-.208.086-.405.086-.581 0-.615-.375-.984-.971-.984-.761 0-1.26.543-1.52 1.598l-1.033 4.146h-1.811l.327-1.303c-.534.873-1.271 1.412-2.183 1.412-1.102 0-1.617-.632-1.617-1.584a4.02 4.02 0 0 1 .096-.811l.826-3.366H6.285l.388-1.43h3.075l-1.216 4.804c-.08.309-.11.559-.11.738 0 .307.148.396.385.452.146.03 1.295.011 1.918-1.376l.798-3.188h-1.294l.391-1.404h2.787l-.359 1.617c.484-.899 1.452-1.762 2.406-1.762 1.012 0 1.855.728 1.855 2.111 0 .4-.063.796-.18 1.18l-.52 1.858a2.286 2.286 0 0 0-.075.492c0 .326.136.484.368.484.238 0 .553-.181.899-1.172l.708.272c-.414 1.473-1.182 2.081-2.137 2.081z"
        /> < title > { title } < / title > < / svg >
    }
}
