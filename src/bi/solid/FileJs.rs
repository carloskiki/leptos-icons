#[cfg(feature = "BiSolidFileJs")]
use leptos::*;
#[cfg(feature = "BiSolidFileJs")]
///This icon requires the feature `BiSolidFileJs` to be enabled.
#[component]
pub fn FileJs(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6zm-2.745 14.186c0 1.345-.644 1.814-1.681 1.814-.245 0-.567-.042-.777-.112l.119-.861c.147.049.336.084.546.084.448 0 .729-.203.729-.938v-2.97h1.064v2.983zm2.043 1.807c-.539 0-1.071-.141-1.337-.287l.217-.883c.287.147.729.294 1.184.294.491 0 .749-.203.749-.511 0-.295-.224-.463-.791-.666-.784-.272-1.295-.707-1.295-1.394 0-.806.672-1.422 1.786-1.422.533 0 .925.112 1.205.238l-.238.861c-.189-.091-.525-.224-.987-.224s-.687.21-.687.455c0 .301.267.435.875.665.834.309 1.226.742 1.226 1.408-.002.793-.61 1.466-1.907 1.466zM14 9h-1V4l5 5h-4z"
        /> < title > { title } < / title > < / svg >
    }
}
