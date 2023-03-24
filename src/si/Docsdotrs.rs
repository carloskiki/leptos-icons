#[cfg(feature = "SiDocsdotrs")]
use leptos::*;
#[cfg(feature = "SiDocsdotrs")]
///This icon requires the feature `SiDocsdotrs` to be enabled.
#[component]
pub fn Docsdotrs(
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
        "m22.903 11.728-4.528-1.697V4.945a1.69 1.69 0 0 0-1.097-1.58l-4.687-1.757a1.668 1.668 0 0 0-1.186 0L6.717 3.366a1.687 1.687 0 0 0-1.097 1.58v5.085l-4.528 1.697A1.69 1.69 0 0 0 0 13.308v5.16c0 .638.36 1.224.933 1.51l4.687 2.344a1.68 1.68 0 0 0 1.51 0L12 19.884l4.87 2.438a1.68 1.68 0 0 0 1.51 0l4.687-2.344a1.69 1.69 0 0 0 .933-1.51v-5.16c0-.703-.436-1.331-1.097-1.58zm-6.122-1.66-3.984 1.496V8.367l3.984-1.734zM7.22 4.88 12 3.09l4.781 1.79v.028L12 6.848l-4.781-1.94Zm3.937 13.645-3.984 1.992V16.81l3.984-1.818zm0-5.25-4.781 1.94-4.781-1.94v-.028l4.781-1.79 4.781 1.79zm11.25 5.25-3.984 1.992V16.81l3.984-1.818zm0-5.25-4.781 1.94-4.781-1.94v-.028l4.781-1.79 4.781 1.79z"
        /> < title > { title } < / title > < / svg >
    }
}
