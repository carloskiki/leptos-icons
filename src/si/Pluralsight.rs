#[cfg(feature = "SiPluralsight")]
use leptos::*;
#[cfg(feature = "SiPluralsight")]
///This icon requires the feature `SiPluralsight` to be enabled.
#[component]
pub fn Pluralsight(
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
        "M15.72 1.755C10.08-.301 3.811 2.625 1.771 8.25c-2.071 5.699.854 11.956 6.494 14.01 5.655 2.055 11.956-.87 14.01-6.51 2.057-5.67-.87-11.939-6.524-13.995h-.031zM12 24C5.4 24 0 18.6 0 12S5.4 0 12 0s12 5.4 12 12-5.4 12-12 12M8.926 5.805v12.391L19.68 12 8.926 5.805zm1.049 1.769L17.625 12l-7.65 4.426V7.574M6.449 7.155v9.689L14.85 12 6.449 7.155zm1.051 1.8L12.811 12 7.5 15.061V8.939"
        /> < title > { title } < / title > < / svg >
    }
}
