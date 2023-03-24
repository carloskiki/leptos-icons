#[cfg(feature = "SiOdnoklassniki")]
use leptos::*;
#[cfg(feature = "SiOdnoklassniki")]
///This icon requires the feature `SiOdnoklassniki` to be enabled.
#[component]
pub fn Odnoklassniki(
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
        "M12 0a6.2 6.2 0 0 0-6.194 6.195 6.2 6.2 0 0 0 6.195 6.192 6.2 6.2 0 0 0 6.193-6.192A6.2 6.2 0 0 0 12.001 0zm0 3.63a2.567 2.567 0 0 1 2.565 2.565 2.568 2.568 0 0 1-2.564 2.564 2.568 2.568 0 0 1-2.565-2.564 2.567 2.567 0 0 1 2.565-2.564zM6.807 12.6a1.814 1.814 0 0 0-.91 3.35 11.611 11.611 0 0 0 3.597 1.49l-3.462 3.463a1.815 1.815 0 0 0 2.567 2.566L12 20.066l3.405 3.403a1.813 1.813 0 0 0 2.564 0c.71-.709.71-1.858 0-2.566l-3.462-3.462a11.593 11.593 0 0 0 3.596-1.49 1.814 1.814 0 1 0-1.932-3.073 7.867 7.867 0 0 1-8.34 0c-.318-.2-.674-.29-1.024-.278z"
        /> < title > { title } < / title > < / svg >
    }
}
