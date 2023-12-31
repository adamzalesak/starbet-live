use yew_router::Routable;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum MainRoute {
    #[at("/live")]
    Live,
    #[at("/upcoming")]
    Upcoming,
    #[at("/results")]
    Results,
    #[at("/registration")]
    Registration,
    #[at("/about")]
    About,
    #[at("/privacy-policy")]
    PrivacyPolicy,
    #[at("/administration")]
    Administration,
    #[at("/contact")]
    Contact,
    #[at("/profile/:s")]
    Profile,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum ProfileRoute {
    #[at("/profile/tickets")]
    Tickets,
    #[at("/profile/summary")]
    Summary,
    #[at("/profile/statistics")]
    Statistics,
    #[at("/profile/administration")]
    Administration,
    #[not_found]
    #[at("/profile/404")]
    NotFound,
}
