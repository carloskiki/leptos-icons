#[cfg(feature = "BiSolidFlorist")]
use leptos::*;
#[cfg(feature = "BiSolidFlorist")]
///This icon requires the feature `BiSolidFlorist` to be enabled.
#[component]
pub fn Florist(
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
        "M10.84 21.871 12 22a10.221 10.221 0 0 0-9.013-8.891L2 13l.021.173a10.001 10.001 0 0 0 8.819 8.698zm11.139-8.698L22 13l-.987.109c-4.7.523-8.427 4.2-9.013 8.891l1.16-.129a10.001 10.001 0 0 0 8.819-8.698zM18.063 5.5a2.5 2.5 0 0 0-3.415-.915c-.062.035-.111.081-.168.121.005-.069.02-.136.02-.206a2.5 2.5 0 1 0-5 0c0 .07.015.137.021.206-.057-.04-.107-.086-.168-.121a2.5 2.5 0 0 0-2.5 4.33c.061.035.126.056.188.085-.062.029-.127.05-.188.085a2.5 2.5 0 0 0 2.5 4.33c.062-.035.111-.081.168-.121-.006.069-.021.136-.021.206a2.5 2.5 0 1 0 5 0c0-.07-.015-.137-.021-.206.057.04.106.086.168.121a2.5 2.5 0 0 0 2.5-4.33c-.061-.035-.126-.056-.188-.085.063-.029.127-.05.188-.085a2.5 2.5 0 0 0 .916-3.415zM12 12a3 3 0 1 1 0-6 3 3 0 0 1 0 6z"
        /> < title > { title } < / title > < / svg >
    }
}
