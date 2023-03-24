#[cfg(feature = "ImHtmlFive2")]
use leptos::*;
#[cfg(feature = "ImHtmlFive2")]
///This icon requires the feature `ImHtmlFive2` to be enabled.
#[component]
pub fn HtmlFive2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M0.946 0l1.284 14.4 5.762 1.6 5.777-1.602 1.286-14.398h-14.108zM12.668 13.482l-4.644 1.287v0.007l-0.012-0.004-0.012 0.004v-0.007l-4.644-1.287-1.098-12.304h11.508l-1.098 12.304zM10.168 8.284l-0.204 2.29-1.972 0.532-1.967-0.53-0.126-1.41h-1.773l0.247 2.774 3.626 1.003 3.615-1.003 0.485-5.422h-6.437l-0.161-1.809h6.758l0.158-1.766h-8.847l0.477 5.341z"
        /> < title > { title } < / title > < / svg >
    }
}
