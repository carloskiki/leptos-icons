#[cfg(feature = "SiGoogleadsense")]
use leptos::*;
#[cfg(feature = "SiGoogleadsense")]
///This icon requires the feature `SiGoogleadsense` to be enabled.
#[component]
pub fn Googleadsense(
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
        "M22.056 8.447a3.894 3.894 0 0 0-5.313 1.419l-3.889 6.72a3.874 3.874 0 0 0 1.415 5.293l.01.005a3.894 3.894 0 0 0 5.312-1.42l3.889-6.718a3.875 3.875 0 0 0-1.416-5.294l-.008-.005m-14.7 12.168c-1.08 1.888-3.514 2.583-5.384 1.493-1.87-1.09-2.533-3.455-1.453-5.343s3.494-2.586 5.365-1.496c1.87 1.09 2.554 3.457 1.474 5.344m4.131-19.228a3.935 3.935 0 0 0-3.267 2.189l-3.67 6.279a4.638 4.638 0 0 0-.227.387l-2.746 4.737c1.345-.86 3.09-.993 4.55-.143a4.456 4.456 0 0 1 2.22 4.041l2.77-4.763c.082-.124.157-.252.224-.385l3.67-6.281a3.86 3.86 0 0 0-1.283-5.55 3.958 3.958 0 0 0-2.24-.511z"
        /> < title > { title } < / title > < / svg >
    }
}
