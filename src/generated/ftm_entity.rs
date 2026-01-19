#![allow(missing_docs)]
use super::entities::*;
use serde::{Deserialize, Serialize};
/// FTM Entity enum for runtime polymorphism
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "schema")]
pub enum FtmEntity {
    Address(Address),
    Airplane(Airplane),
    Article(Article),
    Asset(Asset),
    Associate(Associate),
    Audio(Audio),
    BankAccount(BankAccount),
    Call(Call),
    CallForTenders(CallForTenders),
    Company(Company),
    Contract(Contract),
    ContractAward(ContractAward),
    CourtCase(CourtCase),
    CourtCaseParty(CourtCaseParty),
    CryptoWallet(CryptoWallet),
    Debt(Debt),
    Directorship(Directorship),
    Document(Document),
    EconomicActivity(EconomicActivity),
    Email(Email),
    Employment(Employment),
    Event(Event),
    Family(Family),
    Folder(Folder),
    HyperText(HyperText),
    Identification(Identification),
    Image(Image),
    LegalEntity(LegalEntity),
    License(License),
    Membership(Membership),
    Mention(Mention),
    Message(Message),
    Note(Note),
    Occupancy(Occupancy),
    Organization(Organization),
    Ownership(Ownership),
    Package(Package),
    Page(Page),
    Pages(Pages),
    Passport(Passport),
    Payment(Payment),
    Person(Person),
    PlainText(PlainText),
    Position(Position),
    Project(Project),
    ProjectParticipant(ProjectParticipant),
    PublicBody(PublicBody),
    RealEstate(RealEstate),
    Representation(Representation),
    Risk(Risk),
    Sanction(Sanction),
    Security(Security),
    Similar(Similar),
    Succession(Succession),
    Table(Table),
    TaxRoll(TaxRoll),
    Trip(Trip),
    UnknownLink(UnknownLink),
    UserAccount(UserAccount),
    Vehicle(Vehicle),
    Vessel(Vessel),
    Video(Video),
    Workbook(Workbook),
}
impl FtmEntity {
    /// Get the schema name for this entity
    pub fn schema(&self) -> &str {
        match self {
            FtmEntity::Address(_) => "Address",
            FtmEntity::Airplane(_) => "Airplane",
            FtmEntity::Article(_) => "Article",
            FtmEntity::Asset(_) => "Asset",
            FtmEntity::Associate(_) => "Associate",
            FtmEntity::Audio(_) => "Audio",
            FtmEntity::BankAccount(_) => "BankAccount",
            FtmEntity::Call(_) => "Call",
            FtmEntity::CallForTenders(_) => "CallForTenders",
            FtmEntity::Company(_) => "Company",
            FtmEntity::Contract(_) => "Contract",
            FtmEntity::ContractAward(_) => "ContractAward",
            FtmEntity::CourtCase(_) => "CourtCase",
            FtmEntity::CourtCaseParty(_) => "CourtCaseParty",
            FtmEntity::CryptoWallet(_) => "CryptoWallet",
            FtmEntity::Debt(_) => "Debt",
            FtmEntity::Directorship(_) => "Directorship",
            FtmEntity::Document(_) => "Document",
            FtmEntity::EconomicActivity(_) => "EconomicActivity",
            FtmEntity::Email(_) => "Email",
            FtmEntity::Employment(_) => "Employment",
            FtmEntity::Event(_) => "Event",
            FtmEntity::Family(_) => "Family",
            FtmEntity::Folder(_) => "Folder",
            FtmEntity::HyperText(_) => "HyperText",
            FtmEntity::Identification(_) => "Identification",
            FtmEntity::Image(_) => "Image",
            FtmEntity::LegalEntity(_) => "LegalEntity",
            FtmEntity::License(_) => "License",
            FtmEntity::Membership(_) => "Membership",
            FtmEntity::Mention(_) => "Mention",
            FtmEntity::Message(_) => "Message",
            FtmEntity::Note(_) => "Note",
            FtmEntity::Occupancy(_) => "Occupancy",
            FtmEntity::Organization(_) => "Organization",
            FtmEntity::Ownership(_) => "Ownership",
            FtmEntity::Package(_) => "Package",
            FtmEntity::Page(_) => "Page",
            FtmEntity::Pages(_) => "Pages",
            FtmEntity::Passport(_) => "Passport",
            FtmEntity::Payment(_) => "Payment",
            FtmEntity::Person(_) => "Person",
            FtmEntity::PlainText(_) => "PlainText",
            FtmEntity::Position(_) => "Position",
            FtmEntity::Project(_) => "Project",
            FtmEntity::ProjectParticipant(_) => "ProjectParticipant",
            FtmEntity::PublicBody(_) => "PublicBody",
            FtmEntity::RealEstate(_) => "RealEstate",
            FtmEntity::Representation(_) => "Representation",
            FtmEntity::Risk(_) => "Risk",
            FtmEntity::Sanction(_) => "Sanction",
            FtmEntity::Security(_) => "Security",
            FtmEntity::Similar(_) => "Similar",
            FtmEntity::Succession(_) => "Succession",
            FtmEntity::Table(_) => "Table",
            FtmEntity::TaxRoll(_) => "TaxRoll",
            FtmEntity::Trip(_) => "Trip",
            FtmEntity::UnknownLink(_) => "UnknownLink",
            FtmEntity::UserAccount(_) => "UserAccount",
            FtmEntity::Vehicle(_) => "Vehicle",
            FtmEntity::Vessel(_) => "Vessel",
            FtmEntity::Video(_) => "Video",
            FtmEntity::Workbook(_) => "Workbook",
        }
    }
    /// Get the entity ID
    pub fn id(&self) -> &str {
        match self {
            FtmEntity::Address(entity) => &entity.id,
            FtmEntity::Airplane(entity) => &entity.id,
            FtmEntity::Article(entity) => &entity.id,
            FtmEntity::Asset(entity) => &entity.id,
            FtmEntity::Associate(entity) => &entity.id,
            FtmEntity::Audio(entity) => &entity.id,
            FtmEntity::BankAccount(entity) => &entity.id,
            FtmEntity::Call(entity) => &entity.id,
            FtmEntity::CallForTenders(entity) => &entity.id,
            FtmEntity::Company(entity) => &entity.id,
            FtmEntity::Contract(entity) => &entity.id,
            FtmEntity::ContractAward(entity) => &entity.id,
            FtmEntity::CourtCase(entity) => &entity.id,
            FtmEntity::CourtCaseParty(entity) => &entity.id,
            FtmEntity::CryptoWallet(entity) => &entity.id,
            FtmEntity::Debt(entity) => &entity.id,
            FtmEntity::Directorship(entity) => &entity.id,
            FtmEntity::Document(entity) => &entity.id,
            FtmEntity::EconomicActivity(entity) => &entity.id,
            FtmEntity::Email(entity) => &entity.id,
            FtmEntity::Employment(entity) => &entity.id,
            FtmEntity::Event(entity) => &entity.id,
            FtmEntity::Family(entity) => &entity.id,
            FtmEntity::Folder(entity) => &entity.id,
            FtmEntity::HyperText(entity) => &entity.id,
            FtmEntity::Identification(entity) => &entity.id,
            FtmEntity::Image(entity) => &entity.id,
            FtmEntity::LegalEntity(entity) => &entity.id,
            FtmEntity::License(entity) => &entity.id,
            FtmEntity::Membership(entity) => &entity.id,
            FtmEntity::Mention(entity) => &entity.id,
            FtmEntity::Message(entity) => &entity.id,
            FtmEntity::Note(entity) => &entity.id,
            FtmEntity::Occupancy(entity) => &entity.id,
            FtmEntity::Organization(entity) => &entity.id,
            FtmEntity::Ownership(entity) => &entity.id,
            FtmEntity::Package(entity) => &entity.id,
            FtmEntity::Page(entity) => &entity.id,
            FtmEntity::Pages(entity) => &entity.id,
            FtmEntity::Passport(entity) => &entity.id,
            FtmEntity::Payment(entity) => &entity.id,
            FtmEntity::Person(entity) => &entity.id,
            FtmEntity::PlainText(entity) => &entity.id,
            FtmEntity::Position(entity) => &entity.id,
            FtmEntity::Project(entity) => &entity.id,
            FtmEntity::ProjectParticipant(entity) => &entity.id,
            FtmEntity::PublicBody(entity) => &entity.id,
            FtmEntity::RealEstate(entity) => &entity.id,
            FtmEntity::Representation(entity) => &entity.id,
            FtmEntity::Risk(entity) => &entity.id,
            FtmEntity::Sanction(entity) => &entity.id,
            FtmEntity::Security(entity) => &entity.id,
            FtmEntity::Similar(entity) => &entity.id,
            FtmEntity::Succession(entity) => &entity.id,
            FtmEntity::Table(entity) => &entity.id,
            FtmEntity::TaxRoll(entity) => &entity.id,
            FtmEntity::Trip(entity) => &entity.id,
            FtmEntity::UnknownLink(entity) => &entity.id,
            FtmEntity::UserAccount(entity) => &entity.id,
            FtmEntity::Vehicle(entity) => &entity.id,
            FtmEntity::Vessel(entity) => &entity.id,
            FtmEntity::Video(entity) => &entity.id,
            FtmEntity::Workbook(entity) => &entity.id,
        }
    }
}
impl From<Address> for FtmEntity {
    fn from(entity: Address) -> Self {
        FtmEntity::Address(entity)
    }
}
impl From<Airplane> for FtmEntity {
    fn from(entity: Airplane) -> Self {
        FtmEntity::Airplane(entity)
    }
}
impl From<Article> for FtmEntity {
    fn from(entity: Article) -> Self {
        FtmEntity::Article(entity)
    }
}
impl From<Asset> for FtmEntity {
    fn from(entity: Asset) -> Self {
        FtmEntity::Asset(entity)
    }
}
impl From<Associate> for FtmEntity {
    fn from(entity: Associate) -> Self {
        FtmEntity::Associate(entity)
    }
}
impl From<Audio> for FtmEntity {
    fn from(entity: Audio) -> Self {
        FtmEntity::Audio(entity)
    }
}
impl From<BankAccount> for FtmEntity {
    fn from(entity: BankAccount) -> Self {
        FtmEntity::BankAccount(entity)
    }
}
impl From<Call> for FtmEntity {
    fn from(entity: Call) -> Self {
        FtmEntity::Call(entity)
    }
}
impl From<CallForTenders> for FtmEntity {
    fn from(entity: CallForTenders) -> Self {
        FtmEntity::CallForTenders(entity)
    }
}
impl From<Company> for FtmEntity {
    fn from(entity: Company) -> Self {
        FtmEntity::Company(entity)
    }
}
impl From<Contract> for FtmEntity {
    fn from(entity: Contract) -> Self {
        FtmEntity::Contract(entity)
    }
}
impl From<ContractAward> for FtmEntity {
    fn from(entity: ContractAward) -> Self {
        FtmEntity::ContractAward(entity)
    }
}
impl From<CourtCase> for FtmEntity {
    fn from(entity: CourtCase) -> Self {
        FtmEntity::CourtCase(entity)
    }
}
impl From<CourtCaseParty> for FtmEntity {
    fn from(entity: CourtCaseParty) -> Self {
        FtmEntity::CourtCaseParty(entity)
    }
}
impl From<CryptoWallet> for FtmEntity {
    fn from(entity: CryptoWallet) -> Self {
        FtmEntity::CryptoWallet(entity)
    }
}
impl From<Debt> for FtmEntity {
    fn from(entity: Debt) -> Self {
        FtmEntity::Debt(entity)
    }
}
impl From<Directorship> for FtmEntity {
    fn from(entity: Directorship) -> Self {
        FtmEntity::Directorship(entity)
    }
}
impl From<Document> for FtmEntity {
    fn from(entity: Document) -> Self {
        FtmEntity::Document(entity)
    }
}
impl From<EconomicActivity> for FtmEntity {
    fn from(entity: EconomicActivity) -> Self {
        FtmEntity::EconomicActivity(entity)
    }
}
impl From<Email> for FtmEntity {
    fn from(entity: Email) -> Self {
        FtmEntity::Email(entity)
    }
}
impl From<Employment> for FtmEntity {
    fn from(entity: Employment) -> Self {
        FtmEntity::Employment(entity)
    }
}
impl From<Event> for FtmEntity {
    fn from(entity: Event) -> Self {
        FtmEntity::Event(entity)
    }
}
impl From<Family> for FtmEntity {
    fn from(entity: Family) -> Self {
        FtmEntity::Family(entity)
    }
}
impl From<Folder> for FtmEntity {
    fn from(entity: Folder) -> Self {
        FtmEntity::Folder(entity)
    }
}
impl From<HyperText> for FtmEntity {
    fn from(entity: HyperText) -> Self {
        FtmEntity::HyperText(entity)
    }
}
impl From<Identification> for FtmEntity {
    fn from(entity: Identification) -> Self {
        FtmEntity::Identification(entity)
    }
}
impl From<Image> for FtmEntity {
    fn from(entity: Image) -> Self {
        FtmEntity::Image(entity)
    }
}
impl From<LegalEntity> for FtmEntity {
    fn from(entity: LegalEntity) -> Self {
        FtmEntity::LegalEntity(entity)
    }
}
impl From<License> for FtmEntity {
    fn from(entity: License) -> Self {
        FtmEntity::License(entity)
    }
}
impl From<Membership> for FtmEntity {
    fn from(entity: Membership) -> Self {
        FtmEntity::Membership(entity)
    }
}
impl From<Mention> for FtmEntity {
    fn from(entity: Mention) -> Self {
        FtmEntity::Mention(entity)
    }
}
impl From<Message> for FtmEntity {
    fn from(entity: Message) -> Self {
        FtmEntity::Message(entity)
    }
}
impl From<Note> for FtmEntity {
    fn from(entity: Note) -> Self {
        FtmEntity::Note(entity)
    }
}
impl From<Occupancy> for FtmEntity {
    fn from(entity: Occupancy) -> Self {
        FtmEntity::Occupancy(entity)
    }
}
impl From<Organization> for FtmEntity {
    fn from(entity: Organization) -> Self {
        FtmEntity::Organization(entity)
    }
}
impl From<Ownership> for FtmEntity {
    fn from(entity: Ownership) -> Self {
        FtmEntity::Ownership(entity)
    }
}
impl From<Package> for FtmEntity {
    fn from(entity: Package) -> Self {
        FtmEntity::Package(entity)
    }
}
impl From<Page> for FtmEntity {
    fn from(entity: Page) -> Self {
        FtmEntity::Page(entity)
    }
}
impl From<Pages> for FtmEntity {
    fn from(entity: Pages) -> Self {
        FtmEntity::Pages(entity)
    }
}
impl From<Passport> for FtmEntity {
    fn from(entity: Passport) -> Self {
        FtmEntity::Passport(entity)
    }
}
impl From<Payment> for FtmEntity {
    fn from(entity: Payment) -> Self {
        FtmEntity::Payment(entity)
    }
}
impl From<Person> for FtmEntity {
    fn from(entity: Person) -> Self {
        FtmEntity::Person(entity)
    }
}
impl From<PlainText> for FtmEntity {
    fn from(entity: PlainText) -> Self {
        FtmEntity::PlainText(entity)
    }
}
impl From<Position> for FtmEntity {
    fn from(entity: Position) -> Self {
        FtmEntity::Position(entity)
    }
}
impl From<Project> for FtmEntity {
    fn from(entity: Project) -> Self {
        FtmEntity::Project(entity)
    }
}
impl From<ProjectParticipant> for FtmEntity {
    fn from(entity: ProjectParticipant) -> Self {
        FtmEntity::ProjectParticipant(entity)
    }
}
impl From<PublicBody> for FtmEntity {
    fn from(entity: PublicBody) -> Self {
        FtmEntity::PublicBody(entity)
    }
}
impl From<RealEstate> for FtmEntity {
    fn from(entity: RealEstate) -> Self {
        FtmEntity::RealEstate(entity)
    }
}
impl From<Representation> for FtmEntity {
    fn from(entity: Representation) -> Self {
        FtmEntity::Representation(entity)
    }
}
impl From<Risk> for FtmEntity {
    fn from(entity: Risk) -> Self {
        FtmEntity::Risk(entity)
    }
}
impl From<Sanction> for FtmEntity {
    fn from(entity: Sanction) -> Self {
        FtmEntity::Sanction(entity)
    }
}
impl From<Security> for FtmEntity {
    fn from(entity: Security) -> Self {
        FtmEntity::Security(entity)
    }
}
impl From<Similar> for FtmEntity {
    fn from(entity: Similar) -> Self {
        FtmEntity::Similar(entity)
    }
}
impl From<Succession> for FtmEntity {
    fn from(entity: Succession) -> Self {
        FtmEntity::Succession(entity)
    }
}
impl From<Table> for FtmEntity {
    fn from(entity: Table) -> Self {
        FtmEntity::Table(entity)
    }
}
impl From<TaxRoll> for FtmEntity {
    fn from(entity: TaxRoll) -> Self {
        FtmEntity::TaxRoll(entity)
    }
}
impl From<Trip> for FtmEntity {
    fn from(entity: Trip) -> Self {
        FtmEntity::Trip(entity)
    }
}
impl From<UnknownLink> for FtmEntity {
    fn from(entity: UnknownLink) -> Self {
        FtmEntity::UnknownLink(entity)
    }
}
impl From<UserAccount> for FtmEntity {
    fn from(entity: UserAccount) -> Self {
        FtmEntity::UserAccount(entity)
    }
}
impl From<Vehicle> for FtmEntity {
    fn from(entity: Vehicle) -> Self {
        FtmEntity::Vehicle(entity)
    }
}
impl From<Vessel> for FtmEntity {
    fn from(entity: Vessel) -> Self {
        FtmEntity::Vessel(entity)
    }
}
impl From<Video> for FtmEntity {
    fn from(entity: Video) -> Self {
        FtmEntity::Video(entity)
    }
}
impl From<Workbook> for FtmEntity {
    fn from(entity: Workbook) -> Self {
        FtmEntity::Workbook(entity)
    }
}
