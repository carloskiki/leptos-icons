#[cfg(feature = "BiLogosGooglePlus")]
use leptos::*;
#[cfg(feature = "BiLogosGooglePlus")]
///This icon requires the feature `BiLogosGooglePlus` to be enabled.
#[component]
pub fn GooglePlus(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.545 15.025v-2.421h2.422v-1.815h-2.422V8.368H17.73v2.421h-2.421v1.815h2.421v2.421zM8.052 8.302c1.133 0 1.896.484 2.33.896l1.722-1.67c-1.048-.98-2.415-1.58-4.052-1.58C4.714 5.947 2 8.659 2 12s2.714 6.053 6.052 6.053c3.493 0 5.812-2.457 5.812-5.913 0-.503-.064-.86-.153-1.24H8.049v2.256h3.332c-.158.951-1.011 2.556-3.329 2.556-2.003 0-3.637-1.665-3.637-3.704 0-2.042 1.634-3.706 3.637-3.706z"
        /> < title > { title } < / title > < / svg >
    }
}
