pub type BoxStr = Box<str>;

pub type BoxSlice<T> = Box<[T]>;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Root {
    #[serde(rename = "Lexicon")]
    pub lexicons: BoxSlice<Lexicon>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Lexicon {
    #[serde(rename = "@id")]
    pub id: BoxStr,
    #[serde(rename = "@label")]
    pub label: BoxStr,
    #[serde(rename = "@language")]
    pub language: BoxStr,
    #[serde(rename = "@email")]
    pub email: BoxStr,
    #[serde(rename = "@license")]
    pub license: BoxStr,
    #[serde(rename = "@version")]
    pub version: BoxStr,
    #[serde(rename = "@url")]
    pub url: Option<BoxStr>,
    #[serde(rename = "@citation")]
    pub citation: Option<BoxStr>,
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    #[serde(rename = "LexicalEntry")]
    pub lexical_entries: BoxSlice<LexicalEntry>,
    #[serde(rename = "Synset")]
    pub synsets: BoxSlice<Synset>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub syntactic_behaviours: BoxSlice<SyntacticBehaviour>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LexicalEntry {
    #[serde(rename = "@id")]
    pub id: BoxStr,
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Form", default)]
    pub forms: BoxSlice<Form>,
    #[serde(rename = "Sense")]
    pub senses: BoxSlice<Sense>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub syntactic_behaviours: BoxSlice<SyntacticBehaviour>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Lemma {
    #[serde(rename = "@writtenForm")]
    pub written_form: BoxStr,
    pub script: Option<BoxStr>,
    #[serde(rename = "@partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: BoxSlice<Pronunciation>,
    #[serde(rename = "Tag", default)]
    pub tags: BoxSlice<Tag>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Form {
    #[serde(rename = "@id")]
    pub id: Option<BoxStr>,
    #[serde(rename = "@writtenForm")]
    pub written_form: BoxStr,
    pub script: Option<BoxStr>,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: BoxSlice<Pronunciation>,
    #[serde(rename = "Tag", default)]
    pub tags: BoxSlice<Tag>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PartOfSpeech {
    #[serde(rename = "a")]
    Adjective,
    #[serde(rename = "s")]
    AdjectiveSatellite,
    #[serde(rename = "r")]
    Adverb,
    #[serde(rename = "n")]
    Noun,
    #[serde(rename = "v")]
    Verb,
    #[serde(rename = "c")]
    Conjunction,
    #[serde(rename = "p")]
    Adposition,
    #[serde(rename = "x")]
    Other,
    #[serde(rename = "u")]
    Unknown,
}

fn default_phonemic() -> bool { true }

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Pronunciation {
    pub variety: Option<BoxStr>,
    pub notation: Option<BoxStr>,
    #[serde(default="default_phonemic")]
    pub phonemic: bool,
    pub audio: Option<BoxStr>,
    #[serde(rename = "$value")]
    pub value: BoxStr,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub category: BoxStr,
    #[serde(rename = "$value")]
    pub value: BoxStr,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Sense {
    #[serde(rename = "@id")]
    pub id: BoxStr,
    #[serde(rename = "@synset")]
    pub synset_id: BoxStr,
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    // #[serde(rename = "subcat", default)]
    // pub subcat_ids: BoxSlice<BoxStr>,
    #[serde(rename = "SenseRelation", default)]
    pub sense_relations: BoxSlice<SenseRelation>,
    #[serde(rename = "Example", default)]
    pub examples: BoxSlice<Example>,
    #[serde(rename = "Count", default)]
    pub counts: BoxSlice<Count>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SenseRelation {
    #[serde(rename = "@relType")]
    pub rel_type: SenseRelationType,
    #[serde(rename = "@target")]
    pub target_id: BoxStr,
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Count {
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    #[serde(rename = "$value")]
    pub value: BoxStr,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SenseRelationType {
    Also,
    Antonym,
    Derivation,
    DomainRegion,
    DomainTopic,
    Exemplifies,
    HasDomainRegion,
    HasDomainTopic,
    IsExemplifiedBy,
    Other,
    Participle,
    Pertainym,
    Similar,
    // =========
    SimpleAspectIp,
    SecondaryAspectIp,
    SimpleAspectPi,
    SecondaryAspectPi,
    Feminine,
    HasFeminine,
    Masculine,
    HasMasculine,
    Young,
    HasYoung,
    Diminutive,
    HasDiminutive,
    Augmentative,
    HasAugmentative,
    AntoGradable,
    AntoSimple,
    AntoConverse,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Synset {
    #[serde(rename = "@id")]
    pub id: BoxStr,
    #[serde(rename = "@partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    // #[serde(rename = "members", default)]
    // pub member_ids: BoxSlice<BoxStr>,
    #[serde(rename = "Definition")]
    pub definitions: BoxSlice<Definition>,
    #[serde(rename = "ILIDefinition")]
    pub ili_definition: Option<IliDefinition>,
    #[serde(rename = "SynsetRelation", default)]
    pub relations: BoxSlice<SynsetRelation>,
    #[serde(rename = "Example", default)]
    pub examples: BoxSlice<Example>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Definition {
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    #[serde(rename = "$value")]
    pub value: BoxStr,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IliDefinition {
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    #[serde(rename = "$value")]
    pub value: BoxStr,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Example {
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
    #[serde(rename = "$value")]
    pub value: BoxStr,
}


#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SynsetRelation {
    #[serde(rename = "@relType")]
    pub rel_type: SynsetRelationType,
    #[serde(rename = "@target")]
    pub target_id: BoxStr,
    pub status: Option<BoxStr>,
    pub note: Option<BoxStr>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SynsetRelationType {
    Also,
    Attribute,
    Causes,
    DomainRegion,
    DomainTopic,
    Entails,
    Exemplifies,
    HasDomainRegion,
    HasDomainTopic,
    HoloMember,
    HoloPart,
    HoloSubstance,
    Hypernym,
    Hyponym,
    InstanceHypernym,
    InstanceHyponym,
    IsCausedBy,
    IsEntailedBy,
    IsExemplifiedBy,
    MeroMember,
    MeroPart,
    MeroSubstance,
    Similar,
    // =========
    Agent,
    Antonym,
    BeInState,
    ClassifiedBy,
    Classifies,
    CoAgentInstrument,
    CoAgentPatient,
    CoAgentResult,
    CoInstrumentAgent,
    CoInstrumentPatient,
    CoInstrumentResult,
    CoPatientAgent,
    CoPatientInstrument,
    CoResultAgent,
    CoResultInstrument,
    CoRole,
    Direction,
    EqSynonym,
    HoloLocation,
    HoloPortion,
    Holonym,
    InManner,
    Instrument,
    Involved,
    InvolvedAgent,
    InvolvedDirection,
    InvolvedInstrument,
    InvolvedLocation,
    InvolvedPatient,
    InvolvedResult,
    InvolvedSourceDirection,
    InvolvedTargetDirection,
    Location,
    MannerOf,
    MeroLocation,
    MeroPortion,
    Meronym,
    Other,
    Patient,
    RestrictedBy,
    Restricts,
    Result,
    Role,
    SourceDirection,
    StateOf,
    TargetDirection,
    Subevent,
    IsSubeventOf,
    Feminine,
    HasFeminine,
    Masculine,
    HasMasculine,
    Young,
    HasYoung,
    Diminutive,
    HasDiminutive,
    Augmentative,
    HasAugmentative,
    AntoGradable,
    AntoSimple,
    AntoConverse,
    IrSynonym,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SyntacticBehaviour {
    #[serde(rename = "@id")]
    pub id: Option<BoxStr>,
    #[serde(rename = "@subcategorizationFrame")]
    pub subcategorization_frame: BoxStr,
    #[serde(rename = "senses", default)]
    pub sense_ids: BoxSlice<BoxStr>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("xml parse error: {0}")]
    QuickXml(#[from] quick_xml::de::DeError),
}

pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Root, Error> {
    let file = std::fs::File::open(path)?;
    from_reader(std::io::BufReader::new(file))
}

pub fn from_str(str: &str) -> Result<Root, Error> {
    Ok(quick_xml::de::from_str(str)?)
}

pub fn from_reader(reader: impl std::io::BufRead) -> Result<Root, Error> {
    Ok(quick_xml::de::from_reader(reader)?)
}
