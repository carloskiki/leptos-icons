#[cfg(feature = "BiLogosGooglePlusCircle")]
use leptos::*;
#[cfg(feature = "BiLogosGooglePlusCircle")]
///This icon requires the feature `BiLogosGooglePlusCircle` to be enabled.
#[component]
pub fn GooglePlusCircle(
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
        "M22.014 11.974C21.996 6.462 17.521 2 12.008 2 6.493 2 2.019 6.462 2.001 11.974L2 11.948v.112l.001-.023c.017 5.513 4.491 9.977 10.007 9.977 5.514 0 9.988-4.462 10.006-9.974l.001.026v-.118l-.001.026zM9.281 16.557c-2.509 0-4.548-2.039-4.548-4.549s2.039-4.549 4.548-4.549c1.23 0 2.258.451 3.046 1.188l-1.295 1.255c-.325-.309-.899-.673-1.751-.673-1.505 0-2.733 1.251-2.733 2.785 0 1.533 1.229 2.784 2.733 2.784 1.742 0 2.384-1.206 2.502-1.92H9.279V11.18h4.255c.066.286.115.554.115.932 0 2.597-1.742 4.445-4.368 4.445zm10.458-4.095H17.92v1.819h-1.364v-1.819h-1.82v-1.364h1.82v-1.82h1.364v1.82h1.819v1.364z"
        /> < title > { title } < / title > < / svg >
    }
}
