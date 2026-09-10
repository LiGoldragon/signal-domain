#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ArtDomain {
    Music,
    Photography,
    Film,
    Creativity,
    Fiction,
    Design,
    Storytelling,
    Publishing,
    Theater,
    Dance,
    Poetry,
    Painting,
    Sculpture,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CraftDomain {
    Construction,
    Carpentry,
    Engineering,
    Sewing,
    Manufacturing,
    Metalworking,
    Handicraft,
    Invention,
    Electronics,
    Repair,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CommunityDomain {
    Reputation,
    Service,
    Hospitality,
    Volunteering,
    Neighborliness,
    Membership,
    Solidarity,
    Institutions,
    Gatherings,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TechnologyDomain {
    Software(SoftwareDomain),
    Hardware(HardwareLeaf),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum EngineeringLeaf {
    Documentation,
    Design,
    VersionControl,
    Management,
    DevelopmentProcess,
    Architecture,
    All,
    ApplicationProgrammingInterfaces,
    Modularity,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum HealthDomain {
    Disability,
    Nutrition,
    Sexuality,
    Senses,
    Therapy,
    Addiction,
    Aging,
    Prevention,
    Reproduction,
    Body,
    Mind,
    Sleep,
    Exercise,
    Medicine,
    FirstAid,
    Medication,
    Pain,
    Dentistry,
    Disease,
    Rehabilitation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LeisureDomain {
    Sport,
    Relaxation,
    Games,
    Fandom,
    Outdoors,
    Collecting,
    Recreation,
    Hobby,
    Celebration,
    Entertainment,
    Play,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum KnowledgeDomain {
    Logic,
    Research,
    Astronomy,
    Computing,
    Economics,
    Cognition,
    Taxonomy,
    Mathematics,
    Statistics,
    Geology,
    Physics,
    Linguistics,
    Philosophy,
    History,
    Physiology,
    Chemistry,
    Biology,
}
pub type DomainScopes = std::vec::Vec<DomainScope>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LanguageDomain {
    Editing,
    Translation,
    Grammar,
    Oratory,
    Notation,
    Conversation,
    Listening,
    Correspondence,
    Terminology,
    Rhetoric,
    Writing,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum HardwareLeaf {
    All,
    Networking,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SelfhoodDomain {
    Confidence,
    Wellbeing,
    Decision,
    Purpose,
    Emotion,
    Virtue,
    Growth,
    Discipline,
    Composure,
    Introspection,
    Temperament,
    Motivation,
    Identity,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum EducationDomain {
    Credential,
    Reading,
    Mentoring,
    Studying,
    Schooling,
    Autodidacticism,
    Pedagogy,
    Memorization,
    Teaching,
    Skill,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum KinshipDomain {
    Rapport,
    Boundaries,
    Family,
    Marriage,
    Relatives,
    Friendship,
    Parenting,
    Caregiving,
    Grief,
    Intimacy,
    Romance,
    Belonging,
    Reconciliation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum NatureDomain {
    Husbandry,
    Agriculture,
    Pets,
    Resources,
    Conservation,
    Weather,
    Fishing,
    Horticulture,
    Gardening,
    Stewardship,
    Hunting,
    Forestry,
    Sustainability,
    Wilderness,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AppearanceDomain {
    Grooming,
    Clothing,
    Comportment,
    Style,
    Cosmetics,
    Etiquette,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum HomeDomain {
    Maintenance,
    Realty,
    Tidying,
    Renovation,
    Utilities,
    Locksmithing,
    Furnishing,
    Housing,
    Cleaning,
    Appliances,
    Property,
    Relocation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SafetyDomain {
    Disaster,
    Deterrence,
    Risk,
    Military,
    Cybersecurity,
    Protection,
    Preparedness,
    Privacy,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum WorkDomain {
    Scheduling,
    Compensation,
    Productivity,
    Unemployment,
    Career,
    Freelancing,
    Employment,
    Workplace,
    Vocation,
    Teamwork,
    JobSearch,
    Leadership,
    Project,
    Entrepreneurship,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SoftwareDomain {
    Intelligence(IntelligenceLeaf),
    Engineering(EngineeringLeaf),
    Systems(SystemsLeaf),
    Distributed(DistributedLeaf),
    Security(SecurityLeaf),
    Programming(ProgrammingLeaf),
    Operations(OperationsLeaf),
    Observability(ObservabilityLeaf),
    Surfaces(SurfacesLeaf),
    Quality(QualityLeaf),
    Theory,
    Data(DataLeaf),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Domain {
    Kinship(KinshipDomain),
    Work(WorkDomain),
    Selfhood(SelfhoodDomain),
    Art(ArtDomain),
    Language(LanguageDomain),
    Safety(SafetyDomain),
    Travel(TravelDomain),
    Leisure(LeisureDomain),
    Home(HomeDomain),
    All,
    Craft(CraftDomain),
    Technology(TechnologyDomain),
    Health(HealthDomain),
    Governance(GovernanceDomain),
    Education(EducationDomain),
    Law(LawDomain),
    Nature(NatureDomain),
    Commerce(CommerceDomain),
    Finance(FinanceDomain),
    Knowledge(KnowledgeDomain),
    Information(InformationDomain),
    Appearance(AppearanceDomain),
    Spirituality(SpiritualityDomain),
    Food(FoodDomain),
    Community(CommunityDomain),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum QualityLeaf {
    Testing,
    All,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DataLeaf {
    SchemaEvolution,
    Serialization,
    All,
    Formats,
    Persistence,
    Modeling,
    Migration,
}
pub type ScopeSet = std::vec::Vec<DomainScope>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SurfacesLeaf {
    All,
    CommandLineInterfaces,
    Visualization,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ProgrammingLeaf {
    Compilation,
    Grammars,
    TypeSystems,
    CodeGeneration,
    DomainSpecificLanguages,
    All,
    Parsing,
    Macros,
    Metaprogramming,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TravelDomain {
    Destination,
    Logistics,
    Transit,
    Commuting,
    Cycling,
    Migration,
    Driving,
    Navigation,
    Itinerary,
    Tourism,
    Transportation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum InformationDomain {
    Classification,
    Database,
    Curation,
    Retrieval,
    News,
    RecordKeeping,
    Broadcasting,
    Archives,
    Documentation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum IntelligenceLeaf {
    All,
    AgentSystems,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SystemsLeaf {
    SystemsProgramming,
    Concurrency,
    All,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum FoodDomain {
    Foraging,
    Fermentation,
    Entertaining,
    Diet,
    Beverage,
    Fasting,
    Baking,
    Preservation,
    Dining,
    Cooking,
    Recipe,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum FinanceDomain {
    Spending,
    Insurance,
    Saving,
    Tax,
    Charity,
    Budgeting,
    Accounting,
    Investing,
    Banking,
    Credit,
    Planning,
    Debt,
    Income,
    Retirement,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DomainScope {
    pub domain: Domain,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ObservabilityLeaf {
    Tracing,
    All,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum OperationsLeaf {
    ReleaseEngineering,
    DependencyManagement,
    BuildSystem,
    Deployment,
    All,
    ConfigurationManagement,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DistributedLeaf {
    ProtocolDesign,
    All,
    EventDrivenArchitecture,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SpiritualityDomain {
    Ritual,
    Faith,
    Ethics,
    Scripture,
    Mortality,
    Contemplation,
    Transcendence,
    Pilgrimage,
    Prayer,
    Worship,
    Asceticism,
    Theology,
    Wisdom,
    Meditation,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SecurityLeaf {
    All,
    Authorization,
    SecretsManagement,
    Authentication,
    Cryptography,
    Privacy,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum GovernanceDomain {
    Services,
    Policy,
    Politics,
    Government,
    Citizenship,
    Organizing,
    Movements,
    Diplomacy,
    Administration,
    Naturalization,
    Activism,
    Elections,
    War,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LawDomain {
    Compliance,
    Custody,
    Crime,
    Litigation,
    Contract,
    Rights,
    Arbitration,
    Procedure,
    Justice,
    Liability,
    Title,
    Policing,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CommerceDomain {
    Selling,
    Trade,
    Negotiation,
    Sourcing,
    Buying,
    Market,
    Marketing,
    Retail,
    Assets,
    Pricing,
    Support,
}
pub enum Query {}
pub enum Response {}
