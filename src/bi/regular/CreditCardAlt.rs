#[cfg(feature = "BiRegularCreditCardAlt")]
use leptos::*;
#[cfg(feature = "BiRegularCreditCardAlt")]
///This icon requires the feature `BiRegularCreditCardAlt` to be enabled.
#[component]
pub fn CreditCardAlt(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "15.5" cy = "13.5" r = "2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 13.5c0-.815.396-1.532 1-1.988A2.47 2.47 0 0 0 11.5 11a2.5 2.5 0 1 0 0 5 2.47 2.47 0 0 0 1.5-.512 2.486 2.486 0 0 1-1-1.988z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 4H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zM4 18V6h16l.002 12H4z"
        /> < title > { title } < / title > < / svg >
    }
}
