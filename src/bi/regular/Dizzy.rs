#[cfg(feature = "BiRegularDizzy")]
use leptos::*;
#[cfg(feature = "BiRegularDizzy")]
///This icon requires the feature `BiRegularDizzy` to be enabled.
#[component]
pub fn Dizzy(
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
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.707 12.293 9.414 11l1.293-1.293-1.414-1.414L8 9.586 6.707 8.293 5.293 9.707 6.586 11l-1.293 1.293 1.414 1.414L8 12.414l1.293 1.293zm6.586-4L16 9.586l-1.293-1.293-1.414 1.414L14.586 11l-1.293 1.293 1.414 1.414L16 12.414l1.293 1.293 1.414-1.414L17.414 11l1.293-1.293zM10 16h4v2h-4z"
        /> < title > { title } < / title > < / svg >
    }
}
