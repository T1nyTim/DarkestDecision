mod catalog;

use core::error::Error;

use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form, Router,
    extract::Query,
    routing::{get, post},
    serve,
};
use serde::{Deserialize, Deserializer, de::value::StrDeserializer};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::catalog::{Area, Difficulty, EncounterRecord, Location, encounter_id::EncounterId, enemies::EnemyKind, heroes::HeroClass};

#[derive(Template, WebTemplate)]
#[template(path = "partials/area_and_encounter_fields.html")]
struct AreaAndEncounterFieldsTemplate {
    areas: Vec<Area>,
}

impl AreaAndEncounterFieldsTemplate {
    const fn new(areas: Vec<Area>) -> Self {
        Self { areas }
    }
}

#[derive(Deserialize)]
struct AreaQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    location: Option<Location>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    difficulty: Option<Difficulty>,
}

#[derive(Template, WebTemplate)]
#[template(path = "partials/dependent_fields.html")]
struct DependentFieldsTemplate {
    difficulties: Vec<Difficulty>,
}

impl DependentFieldsTemplate {
    const fn new(difficulties: Vec<Difficulty>) -> Self {
        Self { difficulties }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "partials/encounter_field.html")]
struct EncounterFieldTemplate {
    encounters: Vec<&'static EncounterRecord>,
}

impl EncounterFieldTemplate {
    const fn new(encounters: Vec<&'static EncounterRecord>) -> Self {
        Self { encounters }
    }
}

#[derive(Deserialize)]
struct EncounterQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    location: Option<Location>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    difficulty: Option<Difficulty>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    area: Option<Area>,
}

#[derive(Deserialize)]
struct EvaluateForm {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    hero_rank_4: Option<HeroClass>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    hero_rank_3: Option<HeroClass>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    hero_rank_2: Option<HeroClass>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    hero_rank_1: Option<HeroClass>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    location: Option<Location>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    difficulty: Option<Difficulty>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    area: Option<Area>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    encounter: Option<EncounterId>,
}

#[derive(Template, WebTemplate)]
#[template(path = "partials/evaluation.html")]
struct EvaluationTemplate {
    is_error: bool,
    message: String,
    team: Vec<TeamMemberView>,
    location: String,
    difficulty: String,
    area: String,
    encounter: String,
    enemies: Vec<EnemyKind>,
}

impl EvaluationTemplate {
    const fn new(
        is_error: bool,
        message: String,
        team: Vec<TeamMemberView>,
        location: String,
        difficulty: String,
        area: String,
        encounter: String,
        enemies: Vec<EnemyKind>,
    ) -> Self {
        Self {
            is_error,
            message,
            team,
            location,
            difficulty,
            area,
            encounter,
            enemies,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct IndexTemplate {
    locations: &'static [Location],
    hero_classes: &'static [HeroClass],
}

impl IndexTemplate {
    const fn new(locations: &'static [Location], hero_classes: &'static [HeroClass]) -> Self {
        Self { locations, hero_classes }
    }
}

#[derive(Deserialize)]
struct LocationQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    location: Option<Location>,
}

struct TeamMemberView {
    rank: u8,
    hero_class: HeroClass,
}

impl TeamMemberView {
    const fn new(rank: u8, hero_class: HeroClass) -> Self {
        Self { rank, hero_class }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/encounter/difficulties", get(difficulties))
        .route("/encounter/areas", get(areas))
        .route("/encounter/options", get(encounters))
        .route("/evaluate", post(evaluate))
        .nest_service("/static", ServeDir::new("static"));
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on http://localhost:3000");
    serve(listener, app).await?;
    Ok(())
}

async fn areas(Query(query): Query<AreaQuery>) -> AreaAndEncounterFieldsTemplate {
    let areas = match (query.location, query.difficulty) {
        (Some(location), Some(difficulty)) => Area::filter(&location, &difficulty),
        _ => Vec::new(),
    };
    AreaAndEncounterFieldsTemplate::new(areas)
}

async fn difficulties(Query(query): Query<LocationQuery>) -> DependentFieldsTemplate {
    DependentFieldsTemplate::new(query.location.map(|location| Difficulty::filter(&location)).unwrap_or_default())
}

fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    match value.as_deref() {
        None | Some("") => Ok(None),
        Some(val) => {
            let value_deserializer = StrDeserializer::<D::Error>::new(val);
            T::deserialize(value_deserializer).map(Some)
        }
    }
}

async fn encounters(Query(query): Query<EncounterQuery>) -> EncounterFieldTemplate {
    let encounters = match (query.location, query.difficulty, query.area) {
        (Some(location), Some(difficulty), Some(area)) => EncounterRecord::filter(&location, &difficulty, &area),
        _ => Vec::new(),
    };
    EncounterFieldTemplate::new(encounters)
}

async fn evaluate(Form(form): Form<EvaluateForm>) -> EvaluationTemplate {
    let raw_team = [(4, form.hero_rank_4), (3, form.hero_rank_3), (2, form.hero_rank_2), (1, form.hero_rank_1)];
    let Some(location) = form.location else {
        return evaluation_error("Select a location.");
    };
    let Some(difficulty) = form.difficulty else {
        return evaluation_error("Select a difficulty.");
    };
    let Some(area) = form.area else {
        return evaluation_error("Select an area.");
    };
    let Some(encounter_id) = form.encounter else {
        return evaluation_error("Select an encounter.");
    };
    let Some(encounter) = EncounterRecord::find(&location, &difficulty, &area, &encounter_id) else {
        return evaluation_error("Select a valid location, difficulty, area, and encounter.");
    };
    let team = raw_team
        .into_iter()
        .filter_map(|(rank, hero_class)| hero_class.map(|hero_class| TeamMemberView::new(rank, hero_class)))
        .collect();
    EvaluationTemplate::new(
        false,
        "Encounter state is normalized and ready for the combat evaluator. Skill scoring is intentionally not implemented yet.".to_owned(),
        team,
        location.label().to_owned(),
        difficulty.label().to_owned(),
        area.label().to_owned(),
        encounter.label(),
        encounter.enemies.to_vec(),
    )
}

fn evaluation_error(message: &str) -> EvaluationTemplate {
    EvaluationTemplate::new(
        true,
        message.to_owned(),
        Vec::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        Vec::new(),
    )
}

async fn index() -> IndexTemplate {
    IndexTemplate::new(Location::ALL, HeroClass::ALL)
}
