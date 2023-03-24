#[cfg(feature = "BsEnvelopeHeartFill")]
use leptos::*;
#[cfg(feature = "BsEnvelopeHeartFill")]
///This icon requires the feature `BsEnvelopeHeartFill` to be enabled.
#[component]
pub fn EnvelopeHeartFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-envelope-heart-fill" viewBox = "0 0 16 16" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M.05 3.555A2 2 0 0 1 2 2h12a2 2 0 0 1 1.95 1.555l-4.2 2.568a2.785 2.785 0 0 0-.051-.105c-.666-1.3-2.363-1.917-3.699-1.25-1.336-.667-3.033-.05-3.699 1.25l-.05.105L.05 3.555ZM11.584 8.91a4.694 4.694 0 0 1-.073.139L16 11.8V4.697l-4.003 2.447c.027.562-.107 1.163-.413 1.767Zm-4.135 3.05c-1.048-.693-1.84-1.39-2.398-2.082L.19 12.856A2 2 0 0 0 2 14h12a2 2 0 0 0 1.808-1.144L10.95 9.878c-.559.692-1.35 1.389-2.398 2.081L8 12.324l-.551-.365ZM4.416 8.91c-.306-.603-.44-1.204-.413-1.766L0 4.697v7.104l4.49-2.752a4.742 4.742 0 0 1-.074-.138Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 5.993c1.664-1.711 5.825 1.283 0 5.132-5.825-3.85-1.664-6.843 0-5.132Z" /> <
        title > { title } < / title > < / svg >
    }
}
