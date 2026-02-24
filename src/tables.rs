// Internal code must reference deprecated variants for backward compatibility.
// Users of the crate will still see deprecation warnings when they use these variants.
#![allow(deprecated)]

use core::cmp::Ordering;
use core::{fmt, slice::Iter};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{SeqAccess, Visitor},
    ser::SerializeTuple,
};

macro_rules! lookup {
    (@gen [$doc:expr, $name:ident, $enum:ident, $len:expr, $($aliases:expr => $loweralias:expr),+]) => {
        #[doc = $doc]
        #[derive(Copy, Clone, Eq, Ord)]
        pub struct $name(pub [&'static str; $len]);

        impl $name {
            pub(crate) const fn const_default() -> Self {
                Self([$($aliases,)*])
            }

            pub(crate) const fn into_country_table(self) -> CountryTable {
                CountryTable::$enum(self)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::const_default()
            }
        }

        impl LookupTable for $name {
            fn contains(&self, alias: &str) -> bool {
                let mut buf = [0u8; 64];
                if alias.len() <= buf.len() && alias.is_ascii() {
                    let buf = &mut buf[..alias.len()];
                    buf.copy_from_slice(alias.as_bytes());
                    buf.make_ascii_lowercase();
                    // SAFETY: input was ASCII, ASCII lowercase is still valid UTF-8
                    let lowered = unsafe { core::str::from_utf8_unchecked(buf) };
                    matches!(lowered, $($loweralias)|*)
                } else {
                    matches!(alias.to_lowercase().as_str(), $($loweralias)|*)
                }
            }

            fn len(&self) -> usize {
                self.0.len()
            }

            fn iter(&self) -> Iter<'_, &'static str> {
                self.0.iter()
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                let mut seq = s.serialize_tuple(self.0.len())?;
                for e in &self.0 {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
                where D: Deserializer<'de>
            {
                struct TableVisitor;
                impl<'de> Visitor<'de> for TableVisitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(f, "an array of strings")
                    }

                    fn visit_seq<A>(self, _seq: A) -> Result<$name, A::Error>
                        where A: SeqAccess<'de>
                    {
                        // // for i in 0..$len {
                        //     let _ = seq.next_element()?.ok_or_else(|| DError::invalid_length(i, &self))?;
                        // }
                        Ok($name::default())
                    }
                }

                deserializer.deserialize_tuple($len, TableVisitor)
            }
        }

        impl From<$name> for CountryTable {
            fn from(n: $name) -> Self {
                n.into_country_table()
            }
        }

        impl<L: LookupTable> PartialOrd<L> for $name {
            fn partial_cmp(&self, other: &L) -> Option<Ordering> {
                if self.len() == other.len() {
                    let mut res = None;
                    for (l, r) in self.iter().zip(other.iter()) {
                        res = l.partial_cmp(r);
                        match res {
                            Some(Ordering::Equal) | None => {},
                            _ => break,
                        }
                    }
                    res
                } else {
                    self.len().partial_cmp(&other.len())
                }
            }
        }

        impl<L: LookupTable> PartialEq<L> for $name {
            fn eq(&self, other: &L) -> bool {
                self.len() == other.len() &&
                self.iter().zip(other.iter()).all(|(l, r)| *l == *r)
            }
        }

        impl core::hash::Hash for $name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                for s in self.iter() {
                    s.hash(state);
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}]", self.0.join(","))
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} - [{}]", stringify!($name), self.0.join(","))
            }
        }
    };
    ($name:ident, $enum:ident, $long_name:expr, $len:expr, $($aliases:expr => $loweralias:expr),+) => {
        lookup! { @gen [concat!("Aliases for ", stringify!($long_name)), $name, $enum, $len, $( $aliases => $loweralias ),* ]}
    };
}

/// A lookup table where all elements are statically known
pub trait LookupTable {
    /// True if this lookup table contains `alias`
    fn contains(&self, alias: &str) -> bool;
    /// The number of elements in this lookup table
    fn len(&self) -> usize;
    /// True if there are no elements
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// An iterator over this lookup table
    fn iter(&self) -> Iter<'_, &'static str>;
}

/// Since reference for the `EmptyLookupTable`
pub const EMPTY_LOOKUP_TABLE: EmptyLookupTable = EmptyLookupTable([]);

/// A lookup table with zero entries
#[derive(Copy, Clone, Default, Serialize, Deserialize, Eq, Ord)]
pub struct EmptyLookupTable(pub [&'static str; 0]);

impl EmptyLookupTable {
    pub(crate) const fn into_country_table(self) -> CountryTable {
        CountryTable::Empty(self)
    }
}

impl LookupTable for EmptyLookupTable {
    fn contains(&self, _: &str) -> bool {
        false
    }

    fn len(&self) -> usize {
        0
    }

    fn iter(&self) -> Iter<'_, &'static str> {
        [].iter()
    }
}

impl<L: LookupTable> PartialEq<L> for EmptyLookupTable {
    fn eq(&self, other: &L) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(l, r)| *l == *r)
    }
}

impl<L: LookupTable> PartialOrd<L> for EmptyLookupTable {
    fn partial_cmp(&self, other: &L) -> Option<Ordering> {
        if other.len() > 0 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl From<EmptyLookupTable> for CountryTable {
    fn from(t: EmptyLookupTable) -> Self {
        t.into_country_table()
    }
}

impl fmt::Display for EmptyLookupTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[]")
    }
}

impl fmt::Debug for EmptyLookupTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyLookupTable")
    }
}

lookup!(SamoaTable, Samoa, "American Samoa", 1, "Samoa" => "samoa");
lookup!(SaintHelenaTable, SaintHelena, "Ascension And Tristan Da Cunha Saint Helena", 2, "StHelena" => "sthelena", "SaintHelena" => "sainthelena");
lookup!(VenezuelaTable, Venezuela, "Bolivarian Republic Of Venezuela", 1, "Venezuela" => "venezuela");
lookup!(BosniaTable, Bosnia, "Bosnia And Herzegovina", 2, "Bosnia" => "bosnia", "Herzegovina" => "herzegovina");
lookup!(BruneiTable, Brunei, "Brunei Darussalam", 1, "Brunei" => "brunei");
lookup!(BurkinaTable, Burkina, "Burkina Faso", 1, "Burkina" => "burkina");
lookup!(StMaartenTable, StMaarten, "Dutch Part Sint Maarten", 2, "StMaarten" => "stmaarten", "SaintMaarten" => "saintmaarten");
lookup!(MicronesiaTable, Micronesia, "Federated States Of Micronesia", 1, "Micronesia" => "micronesia");
lookup!(StMartinTable, StMartin, "French Part Saint Martin", 2, "StMartin" => "stmartin", "SaintMartin" => "saintmartin");
lookup!(HeardIslandTable, HeardIsland, "Heard Island And Mc Donald Islands", 2, "HeardIsland" => "heardisland", "McDonaldIslands" => "mcDonaldislands");
lookup!(IranTable, Iran, "Islamic Republic Of Iran", 1, "Iran" => "iran");
lookup!(MacedoniaTable, Macedonia, "Republic Of North Macedonia", 1, "Macedonia" => "macedonia");
lookup!(StBarthelemyTable, StBarthelemy, "Saint Barthelemy", 1, "StBarthelemy" => "stbarthelemy");
lookup!(StKittsTable, StKitts, "Saint Kitts And Nevis", 1, "StKitts" => "stkitts");
lookup!(StLuciaTable, StLucia, "Saint Lucia", 1, "StLucia" => "stlucia");
lookup!(StPierreTable, StPierre, "Saint Pierre And Miquelon", 2, "StPierre" => "stpierre", "SaintPierre" => "saintpierre");
lookup!(StVincentTable, StVincent, "Saint Vincent And The Grenadines", 2, "StVincent" => "stvincent", "SaintVincent" => "saintvincent");
lookup!(SaoTomeTable, SaoTome, "Sao Tome And Principe", 1, "SaoTome" => "saotome");
lookup!(SouthGeorgiaTable, SouthGeorgia, "South Georgia And The South Sandwich Islands", 2, "SouthGeorgia" => "southgeorgia", "SouthSandwichIslands" => "southsandwichislands");
lookup!(PalestineTable, Palestine, "State Of Palestine", 1, "Palestine" => "palestine");
lookup!(TaiwanTable, Taiwan, "Taiwan Province Of China", 1, "Taiwan" => "taiwan");
lookup!(BahamasTable, Bahamas, "The Bahamas", 1, "Bahamas" => "bahamas");
lookup!(CaymanIslandsTable, CaymanIslands, "The Cayman Islands", 1, "CaymanIslands" => "caymanislands");
lookup!(CentralAfricanRepublicTable, CentralAfricanRepublic, "The Central African Republic", 1, "CentralAfricanRepublic" => "centralafricanrepublic");
lookup!(CocosIslandsTable, CocosIslands, "The Cocos Keeling Islands", 2, "CocosIslands" => "cocosislands", "KeelingIslands" => "keelingislands");
lookup!(ComorosTable, Comoros, "The Comoros", 1, "Comoros" => "comoros");
lookup!(CongoTable, Congo, "The Congo", 1, "Congo" => "congo");
lookup!(CookIslandsTable, CookIslands, "The Cook Islands", 1, "CookIslands" => "cookislands");
lookup!(NorthKoreaTable, NorthKorea, "The Democratic Peoples Republic Of Korea", 2, "NorthKorea" => "northkorea", "DemocraticPeoplesRepublicOfKorea" => "democraticpeoplesrepublicofkorea");
lookup!(DemocraticRepublicOfTheCongoTable, DemocraticRepublicOfTheCongo, "The Democratic Republic Of The Congo", 1, "DemocraticRepublicOfTheCongo" => "democraticrepublicofthecongo");
lookup!(DominicanRepublicTable, DominicanRepublic, "The Dominican Republic", 1, "DominicanRepublic" => "dominicanrepublic");
lookup!(MalvinasTable, Malvinas, "The Falkland Islands Malvinas", 2, "Malvinas" => "malvinas", "FalklandIslands" => "falklandislands");
lookup!(FaroeIslandsTable, FaroeIslands, "The Faroe Islands", 1, "FaroeIslands" => "faroeislands");
lookup!(FrenchSouthernTerritoriesTable, FrenchSouthernTerritories, "The French Southern Territories", 1, "FrenchSouthernTerritories" => "frenchsouthernterritories");
lookup!(GambiaTable, Gambia, "The Gambia", 1, "Gambia" => "gambia");
lookup!(HolySeeTable, HolySee, "The Holy See", 3, "HolySee" => "holysee", "Vatican" => "vatican", "VaticanCity" => "vaticancity");
lookup!(LaoPeoplesDemocraticRepublicTable, LaoPeoplesDemocraticRepublic, "The Lao Peoples Democratic Republic", 2, "LaoPeoplesDemocraticRepublic" => "laopeoplesdemocraticrepublic", "Laos" => "laos");
lookup!(MarshallIslandsTable, MarshallIslands, "The Marshall Islands", 1, "MarshallIslands" => "marshallislands");
lookup!(NetherlandsTable, Netherlands, "The Netherlands", 2, "Netherlands" => "netherlands", "Holland" => "holland");
lookup!(NigerTable, Niger, "The Niger", 1, "Niger" => "niger");
lookup!(NorthernMarianaIslandsTable, NorthernMarianaIslands, "The Northern Mariana Islands", 1, "NorthernMarianaIslands" => "northernmarianaislands");
lookup!(PhilippinesTable, Philippines, "The Philippines", 1, "Philippines" => "philippines");
lookup!(SouthKoreaTable, SouthKorea, "The Republic Of Korea", 2, "SouthKorea" => "southkorea", "RepublicOfKorea" => "republicofkorea");
lookup!(MoldovaTable, Moldova, "The Republic Of Moldova", 2, "Moldova" => "moldova", "RepublicOfMoldova" => "republicofmoldova");
lookup!(RussiaTable, Russia, "The Russian Federation", 2, "Russia" => "russia", "RussianFederation" => "russianfederation");
lookup!(SudanTable, Sudan, "The Sudan", 1, "Sudan" => "sudan");
lookup!(TurksAndCaicosIslandsTable, TurksAndCaicosIslands, "The Turks And Caicos Islands", 1, "TurksAndCaicosIslands" => "turksandcaicosislands");
lookup!(UnitedArabEmiratesTable, UnitedArabEmirates, "The United Arab Emirates", 1, "UnitedArabEmirates" => "unitedarabemirates");
lookup!(EnglandTable, England, "The United Kingdom Of Great Britain And Northern Ireland", 7, "England" => "england",
        "Scotland" => "scotland",
        "GreatBritain" => "greatbritain",
        "UnitedKingdom" => "unitedkingdom",
        "NorthernIreland" => "northernireland",
        "UnitedKingdomOfGreatBritain" => "unitedkingdomofgreatbritain",
        "UnitedKingdomOfGreatBritainAndNorthernIreland" => "unitedkingdomofgreatbritainandnorthernireland");
lookup!(UnitedStatesMinorOutlyingIslandsTable, UnitedStatesMinorOutlyingIslands, "The United States Minor Outlying Islands", 1, "UnitedStatesMinorOutlyingIslands" => "unitedstatesminoroutlyingislands");
lookup!(AmericaTable, America, "The United States Of America", 3, "America" => "america", "UnitedStates" => "unitedstates", "UnitedStatesOfAmerica" => "unitedstatesofamerica");
lookup!(TrinidadTable, Trinidad, "Trinidad And Tobago", 2, "Trinidad" => "trinidad", "Tobago" => "tobago");
lookup!(TanzaniaTable, Tanzania, "United Republic Of Tanzania", 1, "Tanzania" => "tanzania");
lookup!(TurkeyTable, Turkey, "Türkiye", 1, "Turkey" => "turkey");
lookup!(TimorTable, TimorLeste, "Timor-Leste", 1, "EastTimor" => "easttimor");
lookup!(CzechiaTable, Czechia, "Czechia", 1, "CzechRepublic" => "czechrepublic");
lookup!(BurmaTable, Burma, "Myanmar", 1, "Burma" => "burma");
lookup!(SwazilandTable, Swaziland, "Eswatini", 1, "Swaziland" => "swaziland");
lookup!(CapeVerdeTable, CapeVerde, "Cabo Verde", 1, "CapeVerde" => "capeverde");
lookup!(IvoryCoastTable, IvoryCoast, "Coted Ivoire", 2, "IvoryCoast" => "ivorycoast", "CoteDIvoire" => "cotedivoire");
lookup!(SyriaTable, Syria, "Syrian Arab Republic", 1, "Syria" => "syria");
lookup!(MacauTable, Macau, "Macao", 1, "Macau" => "macau");
lookup!(MyanmarTable, Myanmar, "Myanmar", 2, "Myanmar" => "myanmar", "Burma" => "burma");
lookup!(EswatiniTable, Eswatini, "Eswatini", 2, "Eswatini" => "eswatini", "Swaziland" => "swaziland");
lookup!(CaboVerdeTable, CaboVerde, "Cabo Verde", 2, "CaboVerde" => "caboverde", "CapeVerde" => "capeverde");
lookup!(CoteDIvoireTable, CoteDIvoire, "Coted Ivoire", 2, "CoteDIvoire" => "cotedivoire", "IvoryCoast" => "ivorycoast");
lookup!(NorthMacedoniaTable, NorthMacedonia, "Republic Of North Macedonia", 2, "NorthMacedonia" => "northmacedonia", "Macedonia" => "macedonia");
lookup!(TurkiyeTable, Turkiye, "Türkiye", 2, "Turkiye" => "turkiye", "Turkey" => "turkey");

/// Wrapper struct for alias tables to avoid using Box
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum CountryTable {
    /// Represents no aliases
    Empty(EmptyLookupTable),
    /// Aliases for Samoa
    Samoa(SamoaTable),
    /// Aliases for `SaintHelena`
    SaintHelena(SaintHelenaTable),
    /// Aliases for Venezuela
    Venezuela(VenezuelaTable),
    /// Aliases for Bosnia
    Bosnia(BosniaTable),
    /// Aliases for Brunei
    Brunei(BruneiTable),
    /// Aliases for Burkina
    Burkina(BurkinaTable),
    /// Aliases for `StMaarten`
    StMaarten(StMaartenTable),
    /// Aliases for Micronesia
    Micronesia(MicronesiaTable),
    /// Aliases for `StMartin`
    StMartin(StMartinTable),
    /// Aliases for `HeardIsland`
    HeardIsland(HeardIslandTable),
    /// Aliases for Iran
    Iran(IranTable),
    /// Aliases for Macedonia
    #[deprecated(
        since = "2.8.0",
        note = "Macedonia was renamed to North Macedonia in 2019. Use Country::republic_of_north_macedonia() instead."
    )]
    Macedonia(MacedoniaTable),
    /// Aliases for `StBarthelemy`
    StBarthelemy(StBarthelemyTable),
    /// Aliases for `StKitts`
    StKitts(StKittsTable),
    /// Aliases for `StLucia`
    StLucia(StLuciaTable),
    /// Aliases for `StPierre`
    StPierre(StPierreTable),
    /// Aliases for `StVincent`
    StVincent(StVincentTable),
    /// Aliases for `SaoTome`
    SaoTome(SaoTomeTable),
    /// Aliases for `SouthGeorgia`
    SouthGeorgia(SouthGeorgiaTable),
    /// Aliases for Palestine
    Palestine(PalestineTable),
    /// Aliases for Taiwan
    Taiwan(TaiwanTable),
    /// Aliases for Bahamas
    Bahamas(BahamasTable),
    /// Aliases for `CaymanIslands`
    CaymanIslands(CaymanIslandsTable),
    /// Aliases for `CentralAfricanRepublic`
    CentralAfricanRepublic(CentralAfricanRepublicTable),
    /// Aliases for `CocosIslands`
    CocosIslands(CocosIslandsTable),
    /// Aliases for Comoros
    Comoros(ComorosTable),
    /// Aliases for Congo
    Congo(CongoTable),
    /// Aliases for `CookIslands`
    CookIslands(CookIslandsTable),
    /// Aliases for `NorthKorea`
    NorthKorea(NorthKoreaTable),
    /// Aliases for `DemocraticRepublicOfTheCongo`
    DemocraticRepublicOfTheCongo(DemocraticRepublicOfTheCongoTable),
    /// Aliases for `DominicanRepublic`
    DominicanRepublic(DominicanRepublicTable),
    /// Aliases for Malvinas
    Malvinas(MalvinasTable),
    /// Aliases for `FaroeIslands`
    FaroeIslands(FaroeIslandsTable),
    /// Aliases for `FrenchSouthernTerritories`
    FrenchSouthernTerritories(FrenchSouthernTerritoriesTable),
    /// Aliases for Gambia
    Gambia(GambiaTable),
    /// Aliases for `HolySee`
    HolySee(HolySeeTable),
    /// Aliases for `LaoPeoplesDemocraticRepublic`
    LaoPeoplesDemocraticRepublic(LaoPeoplesDemocraticRepublicTable),
    /// Aliases for `MarshallIslands`
    MarshallIslands(MarshallIslandsTable),
    /// Aliases for Netherlands
    Netherlands(NetherlandsTable),
    /// Aliases for Niger
    Niger(NigerTable),
    /// Aliases for `NorthernMarianaIslands`
    NorthernMarianaIslands(NorthernMarianaIslandsTable),
    /// Aliases for Philippines
    Philippines(PhilippinesTable),
    /// Aliases for `SouthKorea`
    SouthKorea(SouthKoreaTable),
    /// Aliases for Moldova
    Moldova(MoldovaTable),
    /// Aliases for Russia
    Russia(RussiaTable),
    /// Aliases for Sudan
    Sudan(SudanTable),
    /// Aliases for `TurksAndCaicosIslands`
    TurksAndCaicosIslands(TurksAndCaicosIslandsTable),
    /// Aliases for `UnitedArabEmirates`
    UnitedArabEmirates(UnitedArabEmiratesTable),
    /// Aliases for England
    England(EnglandTable),
    /// Aliases for `UnitedStatesMinorOutlyingIslands`
    UnitedStatesMinorOutlyingIslands(UnitedStatesMinorOutlyingIslandsTable),
    /// Aliases for America
    America(AmericaTable),
    /// Aliases for Trinidad
    Trinidad(TrinidadTable),
    /// Aliases for Tanzania
    Tanzania(TanzaniaTable),
    /// Aliases for Turkey
    #[deprecated(
        since = "2.8.0",
        note = "Turkey was renamed to Türkiye in 2022. Use Country::turkiye() instead."
    )]
    Turkey(TurkeyTable),
    /// Aliases for `TimorLeste`
    TimorLeste(TimorTable),
    /// Aliases for Czechia
    Czechia(CzechiaTable),
    /// Aliases for Burma
    #[deprecated(
        since = "2.8.0",
        note = "Burma was renamed to Myanmar in 1989. Use Country::myanmar() instead."
    )]
    Burma(BurmaTable),
    /// Aliases for Swaziland
    #[deprecated(
        since = "2.8.0",
        note = "Swaziland was renamed to Eswatini in 2018. Use Country::eswatini() instead."
    )]
    Swaziland(SwazilandTable),
    /// Aliases for `CapeVerde`
    #[deprecated(
        since = "2.8.0",
        note = "Cape Verde was renamed to Cabo Verde in 2013. Use Country::cabo_verde() instead."
    )]
    CapeVerde(CapeVerdeTable),
    /// Aliases for `IvoryCoast`
    #[deprecated(
        since = "2.8.0",
        note = "Ivory Coast was renamed to Côte d'Ivoire in 1986. Use Country::coted_ivoire() instead."
    )]
    IvoryCoast(IvoryCoastTable),
    /// Aliases for Syria
    Syria(SyriaTable),
    /// Aliases for Macau
    Macau(MacauTable),
    /// Aliases for Myanmar
    Myanmar(MyanmarTable),
    /// Aliases for Eswatini
    Eswatini(EswatiniTable),
    /// Aliases for `CaboVerde`
    CaboVerde(CaboVerdeTable),
    /// Aliases for `CoteDIvoire`
    CoteDIvoire(CoteDIvoireTable),
    /// Aliases for `NorthMacedonia`
    NorthMacedonia(NorthMacedoniaTable),
    /// Aliases for Turkiye
    Turkiye(TurkiyeTable),
}

impl LookupTable for CountryTable {
    fn contains(&self, alias: &str) -> bool {
        match self {
            CountryTable::Empty(e) => e.contains(alias),
            CountryTable::Samoa(t) => t.contains(alias),
            CountryTable::SaintHelena(t) => t.contains(alias),
            CountryTable::Venezuela(t) => t.contains(alias),
            CountryTable::Bosnia(t) => t.contains(alias),
            CountryTable::Brunei(t) => t.contains(alias),
            CountryTable::Burkina(t) => t.contains(alias),
            CountryTable::StMaarten(t) => t.contains(alias),
            CountryTable::Micronesia(t) => t.contains(alias),
            CountryTable::StMartin(t) => t.contains(alias),
            CountryTable::HeardIsland(t) => t.contains(alias),
            CountryTable::Iran(t) => t.contains(alias),
            CountryTable::Macedonia(t) => t.contains(alias),
            CountryTable::StBarthelemy(t) => t.contains(alias),
            CountryTable::StKitts(t) => t.contains(alias),
            CountryTable::StLucia(t) => t.contains(alias),
            CountryTable::StPierre(t) => t.contains(alias),
            CountryTable::StVincent(t) => t.contains(alias),
            CountryTable::SaoTome(t) => t.contains(alias),
            CountryTable::SouthGeorgia(t) => t.contains(alias),
            CountryTable::Palestine(t) => t.contains(alias),
            CountryTable::Taiwan(t) => t.contains(alias),
            CountryTable::Bahamas(t) => t.contains(alias),
            CountryTable::CaymanIslands(t) => t.contains(alias),
            CountryTable::CentralAfricanRepublic(t) => t.contains(alias),
            CountryTable::CocosIslands(t) => t.contains(alias),
            CountryTable::Comoros(t) => t.contains(alias),
            CountryTable::Congo(t) => t.contains(alias),
            CountryTable::CookIslands(t) => t.contains(alias),
            CountryTable::NorthKorea(t) => t.contains(alias),
            CountryTable::DemocraticRepublicOfTheCongo(t) => t.contains(alias),
            CountryTable::DominicanRepublic(t) => t.contains(alias),
            CountryTable::Malvinas(t) => t.contains(alias),
            CountryTable::FaroeIslands(t) => t.contains(alias),
            CountryTable::FrenchSouthernTerritories(t) => t.contains(alias),
            CountryTable::Gambia(t) => t.contains(alias),
            CountryTable::HolySee(t) => t.contains(alias),
            CountryTable::LaoPeoplesDemocraticRepublic(t) => t.contains(alias),
            CountryTable::MarshallIslands(t) => t.contains(alias),
            CountryTable::Netherlands(t) => t.contains(alias),
            CountryTable::Niger(t) => t.contains(alias),
            CountryTable::NorthernMarianaIslands(t) => t.contains(alias),
            CountryTable::Philippines(t) => t.contains(alias),
            CountryTable::SouthKorea(t) => t.contains(alias),
            CountryTable::Moldova(t) => t.contains(alias),
            CountryTable::Russia(t) => t.contains(alias),
            CountryTable::Sudan(t) => t.contains(alias),
            CountryTable::TurksAndCaicosIslands(t) => t.contains(alias),
            CountryTable::UnitedArabEmirates(t) => t.contains(alias),
            CountryTable::England(t) => t.contains(alias),
            CountryTable::UnitedStatesMinorOutlyingIslands(t) => t.contains(alias),
            CountryTable::America(t) => t.contains(alias),
            CountryTable::Trinidad(t) => t.contains(alias),
            CountryTable::Tanzania(t) => t.contains(alias),
            CountryTable::Turkey(t) => t.contains(alias),
            CountryTable::TimorLeste(t) => t.contains(alias),
            CountryTable::Czechia(t) => t.contains(alias),
            CountryTable::Burma(t) => t.contains(alias),
            CountryTable::Swaziland(t) => t.contains(alias),
            CountryTable::CapeVerde(t) => t.contains(alias),
            CountryTable::IvoryCoast(t) => t.contains(alias),
            CountryTable::Syria(t) => t.contains(alias),
            CountryTable::Macau(t) => t.contains(alias),
            CountryTable::Myanmar(t) => t.contains(alias),
            CountryTable::Eswatini(t) => t.contains(alias),
            CountryTable::CaboVerde(t) => t.contains(alias),
            CountryTable::CoteDIvoire(t) => t.contains(alias),
            CountryTable::NorthMacedonia(t) => t.contains(alias),
            CountryTable::Turkiye(t) => t.contains(alias),
        }
    }

    fn len(&self) -> usize {
        match self {
            CountryTable::Empty(e) => e.len(),
            CountryTable::Samoa(t) => t.len(),
            CountryTable::SaintHelena(t) => t.len(),
            CountryTable::Venezuela(t) => t.len(),
            CountryTable::Bosnia(t) => t.len(),
            CountryTable::Brunei(t) => t.len(),
            CountryTable::Burkina(t) => t.len(),
            CountryTable::StMaarten(t) => t.len(),
            CountryTable::Micronesia(t) => t.len(),
            CountryTable::StMartin(t) => t.len(),
            CountryTable::HeardIsland(t) => t.len(),
            CountryTable::Iran(t) => t.len(),
            CountryTable::Macedonia(t) => t.len(),
            CountryTable::StBarthelemy(t) => t.len(),
            CountryTable::StKitts(t) => t.len(),
            CountryTable::StLucia(t) => t.len(),
            CountryTable::StPierre(t) => t.len(),
            CountryTable::StVincent(t) => t.len(),
            CountryTable::SaoTome(t) => t.len(),
            CountryTable::SouthGeorgia(t) => t.len(),
            CountryTable::Palestine(t) => t.len(),
            CountryTable::Taiwan(t) => t.len(),
            CountryTable::Bahamas(t) => t.len(),
            CountryTable::CaymanIslands(t) => t.len(),
            CountryTable::CentralAfricanRepublic(t) => t.len(),
            CountryTable::CocosIslands(t) => t.len(),
            CountryTable::Comoros(t) => t.len(),
            CountryTable::Congo(t) => t.len(),
            CountryTable::CookIslands(t) => t.len(),
            CountryTable::NorthKorea(t) => t.len(),
            CountryTable::DemocraticRepublicOfTheCongo(t) => t.len(),
            CountryTable::DominicanRepublic(t) => t.len(),
            CountryTable::Malvinas(t) => t.len(),
            CountryTable::FaroeIslands(t) => t.len(),
            CountryTable::FrenchSouthernTerritories(t) => t.len(),
            CountryTable::Gambia(t) => t.len(),
            CountryTable::HolySee(t) => t.len(),
            CountryTable::LaoPeoplesDemocraticRepublic(t) => t.len(),
            CountryTable::MarshallIslands(t) => t.len(),
            CountryTable::Netherlands(t) => t.len(),
            CountryTable::Niger(t) => t.len(),
            CountryTable::NorthernMarianaIslands(t) => t.len(),
            CountryTable::Philippines(t) => t.len(),
            CountryTable::SouthKorea(t) => t.len(),
            CountryTable::Moldova(t) => t.len(),
            CountryTable::Russia(t) => t.len(),
            CountryTable::Sudan(t) => t.len(),
            CountryTable::TurksAndCaicosIslands(t) => t.len(),
            CountryTable::UnitedArabEmirates(t) => t.len(),
            CountryTable::England(t) => t.len(),
            CountryTable::UnitedStatesMinorOutlyingIslands(t) => t.len(),
            CountryTable::America(t) => t.len(),
            CountryTable::Trinidad(t) => t.len(),
            CountryTable::Tanzania(t) => t.len(),
            CountryTable::Turkey(t) => t.len(),
            CountryTable::TimorLeste(t) => t.len(),
            CountryTable::Czechia(t) => t.len(),
            CountryTable::Burma(t) => t.len(),
            CountryTable::Swaziland(t) => t.len(),
            CountryTable::CapeVerde(t) => t.len(),
            CountryTable::IvoryCoast(t) => t.len(),
            CountryTable::Syria(t) => t.len(),
            CountryTable::Macau(t) => t.len(),
            CountryTable::Myanmar(t) => t.len(),
            CountryTable::Eswatini(t) => t.len(),
            CountryTable::CaboVerde(t) => t.len(),
            CountryTable::CoteDIvoire(t) => t.len(),
            CountryTable::NorthMacedonia(t) => t.len(),
            CountryTable::Turkiye(t) => t.len(),
        }
    }

    fn iter(&self) -> Iter<'_, &'static str> {
        match self {
            CountryTable::Empty(e) => e.iter(),
            CountryTable::Samoa(t) => t.iter(),
            CountryTable::SaintHelena(t) => t.iter(),
            CountryTable::Venezuela(t) => t.iter(),
            CountryTable::Bosnia(t) => t.iter(),
            CountryTable::Brunei(t) => t.iter(),
            CountryTable::Burkina(t) => t.iter(),
            CountryTable::StMaarten(t) => t.iter(),
            CountryTable::Micronesia(t) => t.iter(),
            CountryTable::StMartin(t) => t.iter(),
            CountryTable::HeardIsland(t) => t.iter(),
            CountryTable::Iran(t) => t.iter(),
            CountryTable::Macedonia(t) => t.iter(),
            CountryTable::StBarthelemy(t) => t.iter(),
            CountryTable::StKitts(t) => t.iter(),
            CountryTable::StLucia(t) => t.iter(),
            CountryTable::StPierre(t) => t.iter(),
            CountryTable::StVincent(t) => t.iter(),
            CountryTable::SaoTome(t) => t.iter(),
            CountryTable::SouthGeorgia(t) => t.iter(),
            CountryTable::Palestine(t) => t.iter(),
            CountryTable::Taiwan(t) => t.iter(),
            CountryTable::Bahamas(t) => t.iter(),
            CountryTable::CaymanIslands(t) => t.iter(),
            CountryTable::CentralAfricanRepublic(t) => t.iter(),
            CountryTable::CocosIslands(t) => t.iter(),
            CountryTable::Comoros(t) => t.iter(),
            CountryTable::Congo(t) => t.iter(),
            CountryTable::CookIslands(t) => t.iter(),
            CountryTable::NorthKorea(t) => t.iter(),
            CountryTable::DemocraticRepublicOfTheCongo(t) => t.iter(),
            CountryTable::DominicanRepublic(t) => t.iter(),
            CountryTable::Malvinas(t) => t.iter(),
            CountryTable::FaroeIslands(t) => t.iter(),
            CountryTable::FrenchSouthernTerritories(t) => t.iter(),
            CountryTable::Gambia(t) => t.iter(),
            CountryTable::HolySee(t) => t.iter(),
            CountryTable::LaoPeoplesDemocraticRepublic(t) => t.iter(),
            CountryTable::MarshallIslands(t) => t.iter(),
            CountryTable::Netherlands(t) => t.iter(),
            CountryTable::Niger(t) => t.iter(),
            CountryTable::NorthernMarianaIslands(t) => t.iter(),
            CountryTable::Philippines(t) => t.iter(),
            CountryTable::SouthKorea(t) => t.iter(),
            CountryTable::Moldova(t) => t.iter(),
            CountryTable::Russia(t) => t.iter(),
            CountryTable::Sudan(t) => t.iter(),
            CountryTable::TurksAndCaicosIslands(t) => t.iter(),
            CountryTable::UnitedArabEmirates(t) => t.iter(),
            CountryTable::England(t) => t.iter(),
            CountryTable::UnitedStatesMinorOutlyingIslands(t) => t.iter(),
            CountryTable::America(t) => t.iter(),
            CountryTable::Trinidad(t) => t.iter(),
            CountryTable::Tanzania(t) => t.iter(),
            CountryTable::Turkey(t) => t.iter(),
            CountryTable::TimorLeste(t) => t.iter(),
            CountryTable::Czechia(t) => t.iter(),
            CountryTable::Burma(t) => t.iter(),
            CountryTable::Swaziland(t) => t.iter(),
            CountryTable::CapeVerde(t) => t.iter(),
            CountryTable::IvoryCoast(t) => t.iter(),
            CountryTable::Syria(t) => t.iter(),
            CountryTable::Macau(t) => t.iter(),
            CountryTable::Myanmar(t) => t.iter(),
            CountryTable::Eswatini(t) => t.iter(),
            CountryTable::CaboVerde(t) => t.iter(),
            CountryTable::CoteDIvoire(t) => t.iter(),
            CountryTable::NorthMacedonia(t) => t.iter(),
            CountryTable::Turkiye(t) => t.iter(),
        }
    }
}

impl fmt::Display for CountryTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountryTable::Empty(e) => write!(f, "{e}"),
            CountryTable::Samoa(t) => write!(f, "{t}"),
            CountryTable::SaintHelena(t) => write!(f, "{t}"),
            CountryTable::Venezuela(t) => write!(f, "{t}"),
            CountryTable::Bosnia(t) => write!(f, "{t}"),
            CountryTable::Brunei(t) => write!(f, "{t}"),
            CountryTable::Burkina(t) => write!(f, "{t}"),
            CountryTable::StMaarten(t) => write!(f, "{t}"),
            CountryTable::Micronesia(t) => write!(f, "{t}"),
            CountryTable::StMartin(t) => write!(f, "{t}"),
            CountryTable::HeardIsland(t) => write!(f, "{t}"),
            CountryTable::Iran(t) => write!(f, "{t}"),
            CountryTable::Macedonia(t) => write!(f, "{t}"),
            CountryTable::StBarthelemy(t) => write!(f, "{t}"),
            CountryTable::StKitts(t) => write!(f, "{t}"),
            CountryTable::StLucia(t) => write!(f, "{t}"),
            CountryTable::StPierre(t) => write!(f, "{t}"),
            CountryTable::StVincent(t) => write!(f, "{t}"),
            CountryTable::SaoTome(t) => write!(f, "{t}"),
            CountryTable::SouthGeorgia(t) => write!(f, "{t}"),
            CountryTable::Palestine(t) => write!(f, "{t}"),
            CountryTable::Taiwan(t) => write!(f, "{t}"),
            CountryTable::Bahamas(t) => write!(f, "{t}"),
            CountryTable::CaymanIslands(t) => write!(f, "{t}"),
            CountryTable::CentralAfricanRepublic(t) => write!(f, "{t}"),
            CountryTable::CocosIslands(t) => write!(f, "{t}"),
            CountryTable::Comoros(t) => write!(f, "{t}"),
            CountryTable::Congo(t) => write!(f, "{t}"),
            CountryTable::CookIslands(t) => write!(f, "{t}"),
            CountryTable::NorthKorea(t) => write!(f, "{t}"),
            CountryTable::DemocraticRepublicOfTheCongo(t) => write!(f, "{t}"),
            CountryTable::DominicanRepublic(t) => write!(f, "{t}"),
            CountryTable::Malvinas(t) => write!(f, "{t}"),
            CountryTable::FaroeIslands(t) => write!(f, "{t}"),
            CountryTable::FrenchSouthernTerritories(t) => write!(f, "{t}"),
            CountryTable::Gambia(t) => write!(f, "{t}"),
            CountryTable::HolySee(t) => write!(f, "{t}"),
            CountryTable::LaoPeoplesDemocraticRepublic(t) => write!(f, "{t}"),
            CountryTable::MarshallIslands(t) => write!(f, "{t}"),
            CountryTable::Netherlands(t) => write!(f, "{t}"),
            CountryTable::Niger(t) => write!(f, "{t}"),
            CountryTable::NorthernMarianaIslands(t) => write!(f, "{t}"),
            CountryTable::Philippines(t) => write!(f, "{t}"),
            CountryTable::SouthKorea(t) => write!(f, "{t}"),
            CountryTable::Moldova(t) => write!(f, "{t}"),
            CountryTable::Russia(t) => write!(f, "{t}"),
            CountryTable::Sudan(t) => write!(f, "{t}"),
            CountryTable::TurksAndCaicosIslands(t) => write!(f, "{t}"),
            CountryTable::UnitedArabEmirates(t) => write!(f, "{t}"),
            CountryTable::England(t) => write!(f, "{t}"),
            CountryTable::UnitedStatesMinorOutlyingIslands(t) => write!(f, "{t}"),
            CountryTable::America(t) => write!(f, "{t}"),
            CountryTable::Trinidad(t) => write!(f, "{t}"),
            CountryTable::Tanzania(t) => write!(f, "{t}"),
            CountryTable::Turkey(t) => write!(f, "{t}"),
            CountryTable::TimorLeste(t) => write!(f, "{t}"),
            CountryTable::Czechia(t) => write!(f, "{t}"),
            CountryTable::Burma(t) => write!(f, "{t}"),
            CountryTable::Swaziland(t) => write!(f, "{t}"),
            CountryTable::CapeVerde(t) => write!(f, "{t}"),
            CountryTable::IvoryCoast(t) => write!(f, "{t}"),
            CountryTable::Syria(t) => write!(f, "{t}"),
            CountryTable::Macau(t) => write!(f, "{t}"),
            CountryTable::Myanmar(t) => write!(f, "{t}"),
            CountryTable::Eswatini(t) => write!(f, "{t}"),
            CountryTable::CaboVerde(t) => write!(f, "{t}"),
            CountryTable::CoteDIvoire(t) => write!(f, "{t}"),
            CountryTable::NorthMacedonia(t) => write!(f, "{t}"),
            CountryTable::Turkiye(t) => write!(f, "{t}"),
        }
    }
}
