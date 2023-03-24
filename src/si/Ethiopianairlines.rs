#[cfg(feature = "SiEthiopianairlines")]
use leptos::*;
#[cfg(feature = "SiEthiopianairlines")]
///This icon requires the feature `SiEthiopianairlines` to be enabled.
#[component]
pub fn Ethiopianairlines(
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
        "M18.308 11.603c2.39-1.456 4.239-2.53 4.966-4.355 1.544-4.17.363-5.865-1.104-4.564C20.293 4.506 11.478 13.754.195 20.257c-.172.098-.2.322.558.091 4.48-1.572 14.23-6.705 17.555-8.745zm1.823-.333c.942-.586 1.976-.237.316 2.466-1.126 1.662-1.905 2.63-4.92 3.544-2.075.785-9.768 3.024-15.157 3.675-.401.033-.524-.114-.128-.246 5.135-1.306 17.984-8.21 19.889-9.44zm-8.977 10.47c2.204-.072 3.862.242 5.725-1.73 1.95-2.02 1.72-3.07.544-2.743-1.745.524-8.111 2.69-15.622 3.735-.338.046-.256.226.14.25 5.018.474 6.911.51 9.213.488Z"
        /> < title > { title } < / title > < / svg >
    }
}
