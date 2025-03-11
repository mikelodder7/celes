/*
    Copyright Michael Lodder. All Rights Reserved.
    SPDX-License-Identifier: Apache-2.0 OR MIT
*/
//! Rust implementation of Countries as specified by
//! [Country Codes](https://www.iban.com/country-codes)
//! and [ISO-3166-1](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
//!
//! If there are any countries missing then please let me know or submit a PR
//!
//! The main struct is `Country` which provides the following properties
//!
//! `code` - The three digit code for the country
//! `value` - The code as an integer
//! `alpha2` - The alpha2 letter set for the country
//! `alpha3` - The alpha3 letter set for the country
//! `long_name` - The official state name for the country
//! `aliases` - Other names by which the country is known. For example,
//! The Russian Federation is also called Russia or The United Kingdom of Great Britain
//! and Northern Ireland is also called England, Great Britain,
//! Northern Ireland, Scotland, and United Kingdom.
//!
//! Each country can be instantiated by using a function with the country name in snake case
//!
//! ## Usage
//!
//! ```
//! use celes::Country;
//!
//!
//!  let gb = Country::the_united_kingdom_of_great_britain_and_northern_ireland();
//!  println!("{}", gb);
//!
//!  let usa = Country::the_united_states_of_america();
//!  println!("{}", usa);
//!
//! ```
//!
//! Additionally, each country can be created from a string or its numeric code.
//! `Country` provides multiple from methods to instantiate it from a string:
//!
//! - `from_code` - create `Country` from three digit code
//! - `from_alpha2` - create `Country` from two letter code
//! - `from_alpha3` - create `Country` from three letter code
//! - `from_alias` - create `Country` from a common alias stripped of any spaces or
//!   underscores. This only works for some countries as not all countries have aliases
//! - `from_name` - create `Country` from the full state name no space or underscores
//!
//! `Country` implements the [core::str::FromStr](https://doc.rust-lang.org/core/str/trait.FromStr.html) trait that accepts any valid argument to the previously mentioned functions
//! such as:
//!
//! - The country aliases like UnitedKingdom, GreatBritain, Russia, America
//! - The full country name
//! - The alpha2 code
//! - The alpha3 code
//!
//! If you are uncertain which function to use, just use `Country::from_str` as it accepts
//! any of the valid string values. `Country::from_str` is case-insensitive
//!
//! ## From String Example
//!
//! ```rust
//! use celes::Country;
//! use core::str::FromStr;
//!
//! // All of these are equivalent
//! assert_eq!("US", Country::from_str("USA").unwrap().alpha2);
//! assert_eq!("US", Country::from_str("US").unwrap().alpha2);
//! assert_eq!("US", Country::from_str("America").unwrap().alpha2);
//! assert_eq!("US", Country::from_str("UnitedStates").unwrap().alpha2);
//! assert_eq!("US", Country::from_str("TheUnitedStatesOfAmerica").unwrap().alpha2);
//!
//! // All of these are equivalent
//! assert_eq!("GB", Country::from_str("England").unwrap().alpha2);
//! assert_eq!("GB", Country::from_str("gb").unwrap().alpha2);
//! assert_eq!("GB", Country::from_str("Scotland").unwrap().alpha2);
//! assert_eq!("GB", Country::from_str("TheUnitedKingdomOfGreatBritainAndNorthernIreland").unwrap().alpha2);
//! ```

mod tables;

use core::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    str::FromStr,
};
use phf::{Map, phf_map};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DError, Visitor},
};
pub use tables::*;

/// Creates the country function. Meant to be called inside `Country`
macro_rules! country {
    ($func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr) => {
        country!{ @gen [concat!("Creates a struct for ", $long_name), $func, $code, $value, $alpha2, $alpha3, $long_name] }
    };
    ($func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr, $table:ident, $( $alias:expr  ),*) => {
        country!{ @gen [concat!("Creates a struct for ", $long_name), $func, $code, $value, $alpha2, $alpha3, $long_name, $table, $( $alias ),* ] }
    };
    (@gen [$doc:expr, $func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr]) => {
        #[doc = $doc]
        #[inline]
        pub const fn $func() -> Self {
            Self {
                code: $code,
                value: $value,
                alpha2: $alpha2,
                alpha3: $alpha3,
                long_name: $long_name,
                aliases: EMPTY_LOOKUP_TABLE.into_country_table(),
            }
        }
    };
    (@gen [$doc:expr, $func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr, $table:ident, $( $alias:expr ),* ]) => {
        #[doc = $doc]
        #[inline]
        pub const fn $func() -> Self {
            Self {
                code: $code,
                value: $value,
                alpha2: $alpha2,
                alpha3: $alpha3,
                long_name: $long_name,
                aliases: $table::const_default().into_country_table()
            }
        }
    };
}

/// Represents a country according to ISO 3166
#[derive(Copy, Clone)]
pub struct Country {
    /// The three digit code assigned to the country
    pub code: &'static str,
    /// The integer value for `code`
    pub value: usize,
    /// The two letter country code (alpha-2) assigned to the country
    pub alpha2: &'static str,
    /// The three letter country code (alpha-3) assigned to the country
    pub alpha3: &'static str,
    /// The official state name of the country
    pub long_name: &'static str,
    /// Common aliases associated with the country
    pub aliases: CountryTable,
}

impl Debug for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Country {{ code: {}, value: {}, alpha2: {}, alpha3: {}, long_name: {}, aliases: {} }}",
            self.code, self.value, self.alpha2, self.alpha3, self.long_name, self.aliases
        )
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.long_name.replace(' ', ""))
    }
}

impl Ord for Country {
    fn cmp(&self, other: &Self) -> Ordering {
        self.long_name.cmp(other.long_name)
    }
}

impl PartialOrd for Country {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Country {}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        let one = self.code == other.code;
        let two = self.value == other.value;
        let thr = self.alpha2.eq(self.alpha2);
        let fur = self.alpha3.eq(self.alpha3);
        let fve = self.long_name.eq(self.long_name);
        let six = self.aliases.len() == other.aliases.len();
        let svn = self
            .aliases
            .iter()
            .zip(other.aliases.iter())
            .all(|(l, r)| *l == *r);

        one & two & thr & fur & fve & six & svn
    }
}

impl Serialize for Country {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(self.alpha2)
    }
}

impl<'de> Deserialize<'de> for Country {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CountryVisitor;
        impl Visitor<'_> for CountryVisitor {
            type Value = Country;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "a two letter string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: DError,
            {
                Country::from_alpha2(s)
                    .map_err(|_| DError::invalid_value(serde::de::Unexpected::Str(s), &self))
            }
        }

        deserializer.deserialize_str(CountryVisitor)
    }
}

impl Hash for Country {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.long_name.hash(state);
    }
}

impl Country {
    country!(afghanistan, "004", 4, "AF", "AFG", "Afghanistan");

    country!(aland_islands, "248", 248, "AX", "ALA", "Aland Islands");

    country!(albania, "008", 8, "AL", "ALB", "Albania");

    country!(algeria, "012", 12, "DZ", "DZA", "Algeria");

    country!(
        american_samoa,
        "016",
        16,
        "AS",
        "ASM",
        "American Samoa",
        SamoaTable,
        "Samoa"
    );

    country!(andorra, "020", 20, "AD", "AND", "Andorra");

    country!(angola, "024", 24, "AO", "AGO", "Angola");

    country!(anguilla, "660", 660, "AI", "AIA", "Anguilla");

    country!(antarctica, "010", 10, "AQ", "ATA", "Antarctica");

    country!(
        antigua_and_barbuda,
        "028",
        28,
        "AG",
        "ATG",
        "Antigua And Barbuda"
    );

    country!(argentina, "032", 32, "AR", "ARG", "Argentina");

    country!(armenia, "051", 51, "AM", "ARM", "Armenia");

    country!(aruba, "533", 533, "AW", "ABW", "Aruba");

    country!(
        ascension_and_tristan_da_cunha_saint_helena,
        "654",
        654,
        "SH",
        "SHN",
        "Ascension And Tristan Da Cunha Saint Helena",
        SaintHelenaTable,
        "StHelena",
        "SaintHelena"
    );

    country!(australia, "036", 36, "AU", "AUS", "Australia");

    country!(austria, "040", 40, "AT", "AUT", "Austria");

    country!(azerbaijan, "031", 31, "AZ", "AZE", "Azerbaijan");

    country!(bahrain, "048", 48, "BH", "BHR", "Bahrain");

    country!(bangladesh, "050", 50, "BD", "BGD", "Bangladesh");

    country!(barbados, "052", 52, "BB", "BRB", "Barbados");

    country!(belarus, "112", 112, "BY", "BLR", "Belarus");

    country!(belgium, "056", 56, "BE", "BEL", "Belgium");

    country!(belize, "084", 84, "BZ", "BLZ", "Belize");

    country!(benin, "204", 204, "BJ", "BEN", "Benin");

    country!(bermuda, "060", 60, "BM", "BMU", "Bermuda");

    country!(bhutan, "064", 64, "BT", "BTN", "Bhutan");

    country!(
        bolivarian_republic_of_venezuela,
        "862",
        862,
        "VE",
        "VEN",
        "Bolivarian Republic Of Venezuela",
        VenezuelaTable,
        "Venezuela"
    );

    country!(bolivia, "068", 68, "BO", "BOL", "Bolivia");

    country!(bonaire, "535", 535, "BQ", "BES", "Bonaire");

    country!(
        bosnia_and_herzegovina,
        "070",
        70,
        "BA",
        "BIH",
        "Bosnia And Herzegovina",
        BosniaTable,
        "Bosnia",
        "Herzegovina"
    );

    country!(botswana, "072", 72, "BW", "BWA", "Botswana");

    country!(bouvet_island, "074", 74, "BV", "BVT", "Bouvet Island");

    country!(brazil, "076", 76, "BR", "BRA", "Brazil");

    country!(
        british_indian_ocean_territory,
        "086",
        86,
        "IO",
        "IOT",
        "British Indian Ocean Territory"
    );

    country!(
        british_virgin_islands,
        "092",
        92,
        "VG",
        "VGB",
        "British Virgin Islands"
    );

    country!(
        brunei_darussalam,
        "096",
        96,
        "BN",
        "BRN",
        "Brunei Darussalam",
        BruneiTable,
        "Brunei"
    );

    country!(bulgaria, "100", 100, "BG", "BGR", "Bulgaria");

    country!(
        burkina_faso,
        "854",
        854,
        "BF",
        "BFA",
        "Burkina Faso",
        BurkinaTable,
        "Burkina"
    );

    country!(burundi, "108", 108, "BI", "BDI", "Burundi");

    country!(cabo_verde, "132", 132, "CV", "CPV", "Cabo Verde");

    country!(cambodia, "116", 116, "KH", "KHM", "Cambodia");

    country!(cameroon, "120", 120, "CM", "CMR", "Cameroon");

    country!(canada, "124", 124, "CA", "CAN", "Canada");

    country!(chad, "148", 148, "TD", "TCD", "Chad");

    country!(chile, "152", 152, "CL", "CHL", "Chile");

    country!(china, "156", 156, "CN", "CHN", "China");

    country!(
        christmas_island,
        "162",
        162,
        "CX",
        "CXR",
        "Christmas Island"
    );

    country!(colombia, "170", 170, "CO", "COL", "Colombia");

    country!(costa_rica, "188", 188, "CR", "CRI", "Costa Rica");

    country!(coted_ivoire, "384", 384, "CI", "CIV", "Coted Ivoire");

    country!(croatia, "191", 191, "HR", "HRV", "Croatia");

    country!(cuba, "192", 192, "CU", "CUB", "Cuba");

    country!(curacao, "531", 531, "CW", "CUW", "Curacao");

    country!(cyprus, "196", 196, "CY", "CYP", "Cyprus");

    country!(
        czechia,
        "203",
        203,
        "CZ",
        "CZE",
        "Czechia",
        CzechiaTable,
        "CzechRepublic"
    );

    country!(denmark, "208", 208, "DK", "DNK", "Denmark");

    country!(djibouti, "262", 262, "DJ", "DJI", "Djibouti");

    country!(dominica, "212", 212, "DM", "DMA", "Dominica");

    country!(
        dutch_part_sint_maarten,
        "534",
        534,
        "SX",
        "SXM",
        "Dutch Part Sint Maarten",
        StMaartenTable,
        "StMaarten",
        "SaintMaarten"
    );

    country!(ecuador, "218", 218, "EC", "ECU", "Ecuador");

    country!(egypt, "818", 818, "EG", "EGY", "Egypt");

    country!(el_salvador, "222", 222, "SV", "SLV", "El Salvador");

    country!(
        equatorial_guinea,
        "226",
        226,
        "GQ",
        "GNQ",
        "Equatorial Guinea"
    );

    country!(eritrea, "232", 232, "ER", "ERI", "Eritrea");

    country!(estonia, "233", 233, "EE", "EST", "Estonia");

    country!(eswatini, "748", 748, "SZ", "SWZ", "Eswatini");

    country!(ethiopia, "231", 231, "ET", "ETH", "Ethiopia");

    country!(
        federated_states_of_micronesia,
        "583",
        583,
        "FM",
        "FSM",
        "Federated States Of Micronesia",
        MicronesiaTable,
        "Micronesia"
    );

    country!(fiji, "242", 242, "FJ", "FJI", "Fiji");

    country!(finland, "246", 246, "FI", "FIN", "Finland");

    country!(france, "250", 250, "FR", "FRA", "France");

    country!(french_guiana, "254", 254, "GF", "GUF", "French Guiana");

    country!(
        french_part_saint_martin,
        "663",
        663,
        "MF",
        "MAF",
        "French Part Saint Martin",
        StMartinTable,
        "StMartin",
        "SaintMartin"
    );

    country!(
        french_polynesia,
        "258",
        258,
        "PF",
        "PYF",
        "French Polynesia"
    );

    country!(gabon, "266", 266, "GA", "GAB", "Gabon");

    country!(georgia, "268", 268, "GE", "GEO", "Georgia");

    country!(germany, "276", 276, "DE", "DEU", "Germany");

    country!(ghana, "288", 288, "GH", "GHA", "Ghana");

    country!(gibraltar, "292", 292, "GI", "GIB", "Gibraltar");

    country!(greece, "300", 300, "GR", "GRC", "Greece");

    country!(greenland, "304", 304, "GL", "GRL", "Greenland");

    country!(grenada, "308", 308, "GD", "GRD", "Grenada");

    country!(guadeloupe, "312", 312, "GP", "GLP", "Guadeloupe");

    country!(guam, "316", 316, "GU", "GUM", "Guam");

    country!(guatemala, "320", 320, "GT", "GTM", "Guatemala");

    country!(guernsey, "831", 831, "GG", "GGY", "Guernsey");

    country!(guinea, "324", 324, "GN", "GIN", "Guinea");

    country!(guinea_bissau, "624", 624, "GW", "GNB", "Guinea Bissau");

    country!(guyana, "328", 328, "GY", "GUY", "Guyana");

    country!(haiti, "332", 332, "HT", "HTI", "Haiti");

    country!(
        heard_island_and_mc_donald_islands,
        "334",
        334,
        "HM",
        "HMD",
        "Heard Island And Mc Donald Islands",
        HeardIslandTable,
        "HeardIsland",
        "McDonaldIslands"
    );

    country!(honduras, "340", 340, "HN", "HND", "Honduras");

    country!(hong_kong, "344", 344, "HK", "HKG", "Hong Kong");

    country!(hungary, "348", 348, "HU", "HUN", "Hungary");

    country!(iceland, "352", 352, "IS", "ISL", "Iceland");

    country!(india, "356", 356, "IN", "IND", "India");

    country!(indonesia, "360", 360, "ID", "IDN", "Indonesia");

    country!(iraq, "368", 368, "IQ", "IRQ", "Iraq");

    country!(ireland, "372", 372, "IE", "IRL", "Ireland");

    country!(
        islamic_republic_of_iran,
        "364",
        364,
        "IR",
        "IRN",
        "Islamic Republic Of Iran",
        IranTable,
        "Iran"
    );

    country!(isle_of_man, "833", 833, "IM", "IMN", "Isle Of Man");

    country!(israel, "376", 376, "IL", "ISR", "Israel");

    country!(italy, "380", 380, "IT", "ITA", "Italy");

    country!(jamaica, "388", 388, "JM", "JAM", "Jamaica");

    country!(japan, "392", 392, "JP", "JPN", "Japan");

    country!(jersey, "832", 832, "JE", "JEY", "Jersey");

    country!(jordan, "400", 400, "JO", "JOR", "Jordan");

    country!(kazakhstan, "398", 398, "KZ", "KAZ", "Kazakhstan");

    country!(kenya, "404", 404, "KE", "KEN", "Kenya");

    country!(kiribati, "296", 296, "KI", "KIR", "Kiribati");

    country!(kosovo, "383", 383, "XK", "XKX", "Kosovo");

    country!(kuwait, "414", 414, "KW", "KWT", "Kuwait");

    country!(kyrgyzstan, "417", 417, "KG", "KGZ", "Kyrgyzstan");

    country!(latvia, "428", 428, "LV", "LVA", "Latvia");

    country!(lebanon, "422", 422, "LB", "LBN", "Lebanon");

    country!(lesotho, "426", 426, "LS", "LSO", "Lesotho");

    country!(liberia, "430", 430, "LR", "LBR", "Liberia");

    country!(libya, "434", 434, "LY", "LBY", "Libya");

    country!(liechtenstein, "438", 438, "LI", "LIE", "Liechtenstein");

    country!(lithuania, "440", 440, "LT", "LTU", "Lithuania");

    country!(luxembourg, "442", 442, "LU", "LUX", "Luxembourg");

    country!(macao, "446", 446, "MO", "MAC", "Macao");

    country!(madagascar, "450", 450, "MG", "MDG", "Madagascar");

    country!(malawi, "454", 454, "MW", "MWI", "Malawi");

    country!(malaysia, "458", 458, "MY", "MYS", "Malaysia");

    country!(maldives, "462", 462, "MV", "MDV", "Maldives");

    country!(mali, "466", 466, "ML", "MLI", "Mali");

    country!(malta, "470", 470, "MT", "MLT", "Malta");

    country!(martinique, "474", 474, "MQ", "MTQ", "Martinique");

    country!(mauritania, "478", 478, "MR", "MRT", "Mauritania");

    country!(mauritius, "480", 480, "MU", "MUS", "Mauritius");

    country!(mayotte, "175", 175, "YT", "MYT", "Mayotte");

    country!(mexico, "484", 484, "MX", "MEX", "Mexico");

    country!(monaco, "492", 492, "MC", "MCO", "Monaco");

    country!(mongolia, "496", 496, "MN", "MNG", "Mongolia");

    country!(montenegro, "499", 499, "ME", "MNE", "Montenegro");

    country!(montserrat, "500", 500, "MS", "MSR", "Montserrat");

    country!(morocco, "504", 504, "MA", "MAR", "Morocco");

    country!(mozambique, "508", 508, "MZ", "MOZ", "Mozambique");

    country!(myanmar, "104", 104, "MM", "MMR", "Myanmar");

    country!(namibia, "516", 516, "NA", "NAM", "Namibia");

    country!(nauru, "520", 520, "NR", "NRU", "Nauru");

    country!(nepal, "524", 524, "NP", "NPL", "Nepal");

    country!(new_caledonia, "540", 540, "NC", "NCL", "New Caledonia");

    country!(new_zealand, "554", 554, "NZ", "NZL", "New Zealand");

    country!(nicaragua, "558", 558, "NI", "NIC", "Nicaragua");

    country!(nigeria, "566", 566, "NG", "NGA", "Nigeria");

    country!(niue, "570", 570, "NU", "NIU", "Niue");

    country!(norfolk_island, "574", 574, "NF", "NFK", "Norfolk Island");

    country!(norway, "578", 578, "NO", "NOR", "Norway");

    country!(oman, "512", 512, "OM", "OMN", "Oman");

    country!(pakistan, "586", 586, "PK", "PAK", "Pakistan");

    country!(palau, "585", 585, "PW", "PLW", "Palau");

    country!(panama, "591", 591, "PA", "PAN", "Panama");

    country!(
        papua_new_guinea,
        "598",
        598,
        "PG",
        "PNG",
        "Papua New Guinea"
    );

    country!(paraguay, "600", 600, "PY", "PRY", "Paraguay");

    country!(peru, "604", 604, "PE", "PER", "Peru");

    country!(pitcairn, "612", 612, "PN", "PCN", "Pitcairn");

    country!(poland, "616", 616, "PL", "POL", "Poland");

    country!(portugal, "620", 620, "PT", "PRT", "Portugal");

    country!(puerto_rico, "630", 630, "PR", "PRI", "Puerto Rico");

    country!(qatar, "634", 634, "QA", "QAT", "Qatar");

    country!(
        republic_of_north_macedonia,
        "807",
        807,
        "MK",
        "MKD",
        "Republic Of North Macedonia",
        MacedoniaTable,
        "Macedonia"
    );

    country!(reunion, "638", 638, "RE", "REU", "Reunion");

    country!(romania, "642", 642, "RO", "ROU", "Romania");

    country!(rwanda, "646", 646, "RW", "RWA", "Rwanda");

    country!(
        saint_barthelemy,
        "652",
        652,
        "BL",
        "BLM",
        "Saint Barthelemy",
        StBarthelemyTable,
        "StBarthelemy"
    );

    country!(
        saint_kitts_and_nevis,
        "659",
        659,
        "KN",
        "KNA",
        "Saint Kitts And Nevis",
        StKittsTable,
        "StKitts"
    );

    country!(
        saint_lucia,
        "662",
        662,
        "LC",
        "LCA",
        "Saint Lucia",
        StLuciaTable,
        "StLucia"
    );

    country!(
        saint_pierre_and_miquelon,
        "666",
        666,
        "PM",
        "SPM",
        "Saint Pierre And Miquelon",
        StPierreTable,
        "StPierre",
        "SaintPierre"
    );

    country!(
        saint_vincent_and_the_grenadines,
        "670",
        670,
        "VC",
        "VCT",
        "Saint Vincent And The Grenadines",
        StVincentTable,
        "StVincent",
        "SaintVincent"
    );

    country!(samoa, "882", 882, "WS", "WSM", "Samoa");

    country!(san_marino, "674", 674, "SM", "SMR", "San Marino");

    country!(
        sao_tome_and_principe,
        "678",
        678,
        "ST",
        "STP",
        "Sao Tome And Principe",
        SaoTomeTable,
        "SaoTome"
    );

    country!(saudi_arabia, "682", 682, "SA", "SAU", "Saudi Arabia");

    country!(senegal, "686", 686, "SN", "SEN", "Senegal");

    country!(serbia, "688", 688, "RS", "SRB", "Serbia");

    country!(seychelles, "690", 690, "SC", "SYC", "Seychelles");

    country!(sierra_leone, "694", 694, "SL", "SLE", "Sierra Leone");

    country!(singapore, "702", 702, "SG", "SGP", "Singapore");

    country!(slovakia, "703", 703, "SK", "SVK", "Slovakia");

    country!(slovenia, "705", 705, "SI", "SVN", "Slovenia");

    country!(solomon_islands, "090", 90, "SB", "SLB", "Solomon Islands");

    country!(somalia, "706", 706, "SO", "SOM", "Somalia");

    country!(south_africa, "710", 710, "ZA", "ZAF", "South Africa");

    country!(
        south_georgia_and_the_south_sandwich_islands,
        "239",
        239,
        "GS",
        "SGS",
        "South Georgia And The South Sandwich Islands",
        SouthGeorgiaTable,
        "SouthGeorgia",
        "SouthSandwichIslands"
    );

    country!(south_sudan, "728", 728, "SS", "SSD", "South Sudan");

    country!(spain, "724", 724, "ES", "ESP", "Spain");

    country!(sri_lanka, "144", 144, "LK", "LKA", "Sri Lanka");

    country!(
        state_of_palestine,
        "275",
        275,
        "PS",
        "PSE",
        "State Of Palestine",
        PalestineTable,
        "Palestine"
    );

    country!(suriname, "740", 740, "SR", "SUR", "Suriname");

    country!(
        svalbard_and_jan_mayen,
        "744",
        744,
        "SJ",
        "SJM",
        "Svalbard And Jan Mayen"
    );

    country!(sweden, "752", 752, "SE", "SWE", "Sweden");

    country!(switzerland, "756", 756, "CH", "CHE", "Switzerland");

    country!(
        syrian_arab_republic,
        "760",
        760,
        "SY",
        "SYR",
        "Syrian Arab Republic"
    );

    country!(
        taiwan,
        "158",
        158,
        "TW",
        "TWN",
        "Taiwan, Republic Of China",
        TaiwanTable,
        "Taiwan",
        "台灣",
        "Republic of China",
        "中華民國"
    );

    country!(tajikistan, "762", 762, "TJ", "TJK", "Tajikistan");

    country!(thailand, "764", 764, "TH", "THA", "Thailand");

    country!(
        the_bahamas,
        "044",
        44,
        "BS",
        "BHS",
        "The Bahamas",
        BahamasTable,
        "Bahamas"
    );

    country!(
        the_cayman_islands,
        "136",
        136,
        "KY",
        "CYM",
        "The Cayman Islands",
        CaymanIslandsTable,
        "CaymanIslands"
    );

    country!(
        the_central_african_republic,
        "140",
        140,
        "CF",
        "CAF",
        "The Central African Republic",
        CentralAfricanRepublicTable,
        "CentralAfricanRepublic"
    );

    country!(
        the_cocos_keeling_islands,
        "166",
        166,
        "CC",
        "CCK",
        "The Cocos Keeling Islands",
        CocosIslandsTable,
        "CocosIslands",
        "KeelingIslands"
    );

    country!(
        the_comoros,
        "174",
        174,
        "KM",
        "COM",
        "The Comoros",
        ComorosTable,
        "Comoros"
    );

    country!(
        the_congo,
        "178",
        178,
        "CG",
        "COG",
        "The Congo",
        CongoTable,
        "Congo"
    );

    country!(
        the_cook_islands,
        "184",
        184,
        "CK",
        "COK",
        "The Cook Islands",
        CookIslandsTable,
        "CookIslands"
    );

    country!(
        the_democratic_peoples_republic_of_korea,
        "408",
        408,
        "KP",
        "PRK",
        "The Democratic Peoples Republic Of Korea",
        NorthKoreaTable,
        "NorthKorea",
        "DemocraticPeoplesRepublicOfKorea"
    );

    country!(
        the_democratic_republic_of_the_congo,
        "180",
        180,
        "CD",
        "COD",
        "The Democratic Republic Of The Congo",
        DemocraticRepublicOfTheCongoTable,
        "DemocraticRepublicOfTheCongo"
    );

    country!(
        the_dominican_republic,
        "214",
        214,
        "DO",
        "DOM",
        "The Dominican Republic",
        DominicanRepublicTable,
        "DominicanRepublic"
    );

    country!(
        the_falkland_islands_malvinas,
        "238",
        238,
        "FK",
        "FLK",
        "The Falkland Islands Malvinas",
        MalvinasTable,
        "Malvinas",
        "FalklandIslands"
    );

    country!(
        the_faroe_islands,
        "234",
        234,
        "FO",
        "FRO",
        "The Faroe Islands",
        FaroeIslandsTable,
        "FaroeIslands"
    );

    country!(
        the_french_southern_territories,
        "260",
        260,
        "TF",
        "ATF",
        "The French Southern Territories",
        FrenchSouthernTerritoriesTable,
        "FrenchSouthernTerritories"
    );

    country!(
        the_gambia,
        "270",
        270,
        "GM",
        "GMB",
        "The Gambia",
        GambiaTable,
        "Gabmia"
    );

    country!(
        the_holy_see,
        "336",
        336,
        "VA",
        "VAT",
        "The Holy See",
        HolySeeTable,
        "HolySee"
    );

    country!(
        the_lao_peoples_democratic_republic,
        "418",
        418,
        "LA",
        "LAO",
        "The Lao Peoples Democratic Republic",
        LaoPeoplesDemocraticRepublicTable,
        "LaoPeoplesDemocraticRepublic"
    );

    country!(
        the_marshall_islands,
        "584",
        584,
        "MH",
        "MHL",
        "The Marshall Islands",
        MarshallIslandsTable,
        "MarshallIslands"
    );

    country!(
        the_netherlands,
        "528",
        528,
        "NL",
        "NLD",
        "The Netherlands",
        NetherlandsTable,
        "Netherlands",
        "Holland"
    );

    country!(
        the_niger,
        "562",
        562,
        "NE",
        "NER",
        "The Niger",
        NigerTable,
        "Niger"
    );

    country!(
        the_northern_mariana_islands,
        "580",
        580,
        "MP",
        "MNP",
        "The Northern Mariana Islands",
        NorthernMarianaIslandsTable,
        "NorthernMarianaIslands"
    );

    country!(
        the_philippines,
        "608",
        608,
        "PH",
        "PHL",
        "The Philippines",
        PhilippinesTable,
        "Philippines"
    );

    country!(
        the_republic_of_korea,
        "410",
        410,
        "KR",
        "KOR",
        "The Republic Of Korea",
        SouthKoreaTable,
        "SouthKorea",
        "RepublicOfKorea"
    );

    country!(
        the_republic_of_moldova,
        "498",
        498,
        "MD",
        "MDA",
        "The Republic Of Moldova",
        MoldovaTable,
        "Moldova",
        "RepublicOfMoldova"
    );

    country!(
        the_russian_federation,
        "643",
        643,
        "RU",
        "RUS",
        "The Russian Federation",
        RussiaTable,
        "Russia",
        "RussianFederation"
    );

    country!(
        the_sudan,
        "729",
        729,
        "SD",
        "SDN",
        "The Sudan",
        SudanTable,
        "Sudan"
    );

    country!(
        the_turks_and_caicos_islands,
        "796",
        796,
        "TC",
        "TCA",
        "The Turks And Caicos Islands",
        TurksAndCaicosIslandsTable,
        "TurksAndCaicosIslands"
    );

    country!(
        the_united_arab_emirates,
        "784",
        784,
        "AE",
        "ARE",
        "The United Arab Emirates",
        UnitedArabEmiratesTable,
        "UnitedArabEmirates"
    );

    country!(
        the_united_kingdom_of_great_britain_and_northern_ireland,
        "826",
        826,
        "GB",
        "GBR",
        "The United Kingdom Of Great Britain And Northern Ireland",
        EnglandTable,
        "England",
        "Scotland",
        "GreatBritain",
        "UnitedKingdom",
        "NorthernIreland",
        "UnitedKingdomOfGreatBritain",
        "UnitedKingdomOfGreatBritainAndNorthernIreland"
    );

    country!(
        the_united_states_minor_outlying_islands,
        "581",
        581,
        "UM",
        "UMI",
        "The United States Minor Outlying Islands",
        UnitedStatesMinorOutlyingIslandsTable,
        "UnitedStatesMinorOutlyingIslands"
    );

    country!(
        the_united_states_of_america,
        "840",
        840,
        "US",
        "USA",
        "The United States Of America",
        AmericaTable,
        "America",
        "UnitedStates",
        "UnitedStatesOfAmerica"
    );

    country!(
        timor_leste,
        "626",
        626,
        "TL",
        "TLS",
        "Timor Leste",
        TimorTable,
        "EastTimor"
    );

    country!(togo, "768", 768, "TG", "TGO", "Togo");

    country!(tokelau, "772", 772, "TK", "TKL", "Tokelau");

    country!(tonga, "776", 776, "TO", "TON", "Tonga");

    country!(
        trinidad_and_tobago,
        "780",
        780,
        "TT",
        "TTO",
        "Trinidad And Tobago",
        TrinidadTable,
        "Trinidad",
        "Tobago"
    );

    country!(tunisia, "788", 788, "TN", "TUN", "Tunisia");

    country!(
        turkey,
        "792",
        792,
        "TR",
        "TUR",
        "Türkiye",
        TurkeyTable,
        "Turkey"
    );

    country!(turkmenistan, "795", 795, "TM", "TKM", "Turkmenistan");

    country!(tuvalu, "798", 798, "TV", "TUV", "Tuvalu");

    country!(
        us_virgin_islands,
        "850",
        850,
        "VI",
        "VIR",
        "US Virgin Islands"
    );

    country!(uganda, "800", 800, "UG", "UGA", "Uganda");

    country!(ukraine, "804", 804, "UA", "UKR", "Ukraine");

    country!(
        united_republic_of_tanzania,
        "834",
        834,
        "TZ",
        "TZA",
        "United Republic Of Tanzania",
        TanzaniaTable,
        "Tanzania"
    );

    country!(uruguay, "858", 858, "UY", "URY", "Uruguay");

    country!(uzbekistan, "860", 860, "UZ", "UZB", "Uzbekistan");

    country!(vanuatu, "548", 548, "VU", "VUT", "Vanuatu");

    country!(vietnam, "704", 704, "VN", "VNM", "Vietnam");

    country!(
        wallis_and_futuna,
        "876",
        876,
        "WF",
        "WLF",
        "Wallis And Futuna"
    );

    country!(western_sahara, "732", 732, "EH", "ESH", "Western Sahara");

    country!(yemen, "887", 887, "YE", "YEM", "Yemen");

    country!(zambia, "894", 894, "ZM", "ZMB", "Zambia");

    country!(zimbabwe, "716", 716, "ZW", "ZWE", "Zimbabwe");

    /// Returns a vector in alphabetic order of all the countries
    ///
    /// ```
    /// use celes::Country;
    /// use std::collections::BTreeMap;
    ///
    /// let countries = Country::get_countries();
    ///
    ///
    /// for c in &countries {
    ///     println!("{}", c);
    /// }
    ///
    /// for c in &countries {
    ///     println!("{}", c.alpha2);
    /// }
    ///
    /// for c in countries.iter().filter(|cty| cty.value < 300) {
    ///     println!("{}", c.long_name)
    /// }
    ///
    /// //Convert to a map
    /// let lookup = countries.iter().map(|cty| (cty.alpha2.to_string(), cty.clone())).collect::<BTreeMap<String, Country>>();
    ///
    /// ```
    pub const fn get_countries() -> [Self; 250] {
        [
            Self::afghanistan(),
            Self::aland_islands(),
            Self::albania(),
            Self::algeria(),
            Self::american_samoa(),
            Self::andorra(),
            Self::angola(),
            Self::anguilla(),
            Self::antarctica(),
            Self::antigua_and_barbuda(),
            Self::argentina(),
            Self::armenia(),
            Self::aruba(),
            Self::ascension_and_tristan_da_cunha_saint_helena(),
            Self::australia(),
            Self::austria(),
            Self::azerbaijan(),
            Self::bahrain(),
            Self::bangladesh(),
            Self::barbados(),
            Self::belarus(),
            Self::belgium(),
            Self::belize(),
            Self::benin(),
            Self::bermuda(),
            Self::bhutan(),
            Self::bolivarian_republic_of_venezuela(),
            Self::bolivia(),
            Self::bonaire(),
            Self::bosnia_and_herzegovina(),
            Self::botswana(),
            Self::bouvet_island(),
            Self::brazil(),
            Self::british_indian_ocean_territory(),
            Self::british_virgin_islands(),
            Self::brunei_darussalam(),
            Self::bulgaria(),
            Self::burkina_faso(),
            Self::burundi(),
            Self::cabo_verde(),
            Self::cambodia(),
            Self::cameroon(),
            Self::canada(),
            Self::chad(),
            Self::chile(),
            Self::china(),
            Self::christmas_island(),
            Self::colombia(),
            Self::costa_rica(),
            Self::coted_ivoire(),
            Self::croatia(),
            Self::cuba(),
            Self::curacao(),
            Self::cyprus(),
            Self::czechia(),
            Self::denmark(),
            Self::djibouti(),
            Self::dominica(),
            Self::dutch_part_sint_maarten(),
            Self::ecuador(),
            Self::egypt(),
            Self::el_salvador(),
            Self::equatorial_guinea(),
            Self::eritrea(),
            Self::estonia(),
            Self::eswatini(),
            Self::ethiopia(),
            Self::federated_states_of_micronesia(),
            Self::fiji(),
            Self::finland(),
            Self::france(),
            Self::french_guiana(),
            Self::french_part_saint_martin(),
            Self::french_polynesia(),
            Self::gabon(),
            Self::georgia(),
            Self::germany(),
            Self::ghana(),
            Self::gibraltar(),
            Self::greece(),
            Self::greenland(),
            Self::grenada(),
            Self::guadeloupe(),
            Self::guam(),
            Self::guatemala(),
            Self::guernsey(),
            Self::guinea(),
            Self::guinea_bissau(),
            Self::guyana(),
            Self::haiti(),
            Self::heard_island_and_mc_donald_islands(),
            Self::honduras(),
            Self::hong_kong(),
            Self::hungary(),
            Self::iceland(),
            Self::india(),
            Self::indonesia(),
            Self::iraq(),
            Self::ireland(),
            Self::islamic_republic_of_iran(),
            Self::isle_of_man(),
            Self::israel(),
            Self::italy(),
            Self::jamaica(),
            Self::japan(),
            Self::jersey(),
            Self::jordan(),
            Self::kazakhstan(),
            Self::kenya(),
            Self::kiribati(),
            Self::kosovo(),
            Self::kuwait(),
            Self::kyrgyzstan(),
            Self::latvia(),
            Self::lebanon(),
            Self::lesotho(),
            Self::liberia(),
            Self::libya(),
            Self::liechtenstein(),
            Self::lithuania(),
            Self::luxembourg(),
            Self::macao(),
            Self::madagascar(),
            Self::malawi(),
            Self::malaysia(),
            Self::maldives(),
            Self::mali(),
            Self::malta(),
            Self::martinique(),
            Self::mauritania(),
            Self::mauritius(),
            Self::mayotte(),
            Self::mexico(),
            Self::monaco(),
            Self::mongolia(),
            Self::montenegro(),
            Self::montserrat(),
            Self::morocco(),
            Self::mozambique(),
            Self::myanmar(),
            Self::namibia(),
            Self::nauru(),
            Self::nepal(),
            Self::new_caledonia(),
            Self::new_zealand(),
            Self::nicaragua(),
            Self::nigeria(),
            Self::niue(),
            Self::norfolk_island(),
            Self::norway(),
            Self::oman(),
            Self::pakistan(),
            Self::palau(),
            Self::panama(),
            Self::papua_new_guinea(),
            Self::paraguay(),
            Self::peru(),
            Self::pitcairn(),
            Self::poland(),
            Self::portugal(),
            Self::puerto_rico(),
            Self::qatar(),
            Self::republic_of_north_macedonia(),
            Self::reunion(),
            Self::romania(),
            Self::rwanda(),
            Self::saint_barthelemy(),
            Self::saint_kitts_and_nevis(),
            Self::saint_lucia(),
            Self::saint_pierre_and_miquelon(),
            Self::saint_vincent_and_the_grenadines(),
            Self::samoa(),
            Self::san_marino(),
            Self::sao_tome_and_principe(),
            Self::saudi_arabia(),
            Self::senegal(),
            Self::serbia(),
            Self::seychelles(),
            Self::sierra_leone(),
            Self::singapore(),
            Self::slovakia(),
            Self::slovenia(),
            Self::solomon_islands(),
            Self::somalia(),
            Self::south_africa(),
            Self::south_georgia_and_the_south_sandwich_islands(),
            Self::south_sudan(),
            Self::spain(),
            Self::sri_lanka(),
            Self::state_of_palestine(),
            Self::suriname(),
            Self::svalbard_and_jan_mayen(),
            Self::sweden(),
            Self::switzerland(),
            Self::syrian_arab_republic(),
            Self::taiwan(),
            Self::tajikistan(),
            Self::thailand(),
            Self::the_bahamas(),
            Self::the_cayman_islands(),
            Self::the_central_african_republic(),
            Self::the_cocos_keeling_islands(),
            Self::the_comoros(),
            Self::the_congo(),
            Self::the_cook_islands(),
            Self::the_democratic_peoples_republic_of_korea(),
            Self::the_democratic_republic_of_the_congo(),
            Self::the_dominican_republic(),
            Self::the_falkland_islands_malvinas(),
            Self::the_faroe_islands(),
            Self::the_french_southern_territories(),
            Self::the_gambia(),
            Self::the_holy_see(),
            Self::the_lao_peoples_democratic_republic(),
            Self::the_marshall_islands(),
            Self::the_netherlands(),
            Self::the_niger(),
            Self::the_northern_mariana_islands(),
            Self::the_philippines(),
            Self::the_republic_of_korea(),
            Self::the_republic_of_moldova(),
            Self::the_russian_federation(),
            Self::the_sudan(),
            Self::the_turks_and_caicos_islands(),
            Self::the_united_arab_emirates(),
            Self::the_united_kingdom_of_great_britain_and_northern_ireland(),
            Self::the_united_states_minor_outlying_islands(),
            Self::the_united_states_of_america(),
            Self::timor_leste(),
            Self::togo(),
            Self::tokelau(),
            Self::tonga(),
            Self::trinidad_and_tobago(),
            Self::tunisia(),
            Self::turkey(),
            Self::turkmenistan(),
            Self::tuvalu(),
            Self::us_virgin_islands(),
            Self::uganda(),
            Self::ukraine(),
            Self::united_republic_of_tanzania(),
            Self::uruguay(),
            Self::uzbekistan(),
            Self::vanuatu(),
            Self::vietnam(),
            Self::wallis_and_futuna(),
            Self::western_sahara(),
            Self::yemen(),
            Self::zambia(),
            Self::zimbabwe(),
        ]
    }

    /// Given the numeric code, return a country or an error if
    /// the parameter doesn't match any country
    ///
    /// ```
    /// use celes::Country;
    ///
    /// let res = Country::from_value(1);
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_value(2);
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_value(4);
    /// assert!(res.is_ok());
    ///
    /// assert_eq!(Country::afghanistan(), res.unwrap());
    /// ```
    pub fn from_value(value: usize) -> Result<Self, &'static str> {
        static VALUES: Map<usize, Country> = phf_map! {
            4usize => Country::afghanistan(),
            248usize => Country::aland_islands(),
            8usize => Country::albania(),
            12usize => Country::algeria(),
            16usize => Country::american_samoa(),
            20usize => Country::andorra(),
            24usize => Country::angola(),
            660usize => Country::anguilla(),
            10usize => Country::antarctica(),
            28usize => Country::antigua_and_barbuda(),
            32usize => Country::argentina(),
            51usize => Country::armenia(),
            533usize => Country::aruba(),
            654usize => Country::ascension_and_tristan_da_cunha_saint_helena(),
            36usize => Country::australia(),
            40usize => Country::austria(),
            31usize => Country::azerbaijan(),
            48usize => Country::bahrain(),
            50usize => Country::bangladesh(),
            52usize => Country::barbados(),
            112usize => Country::belarus(),
            56usize => Country::belgium(),
            84usize => Country::belize(),
            204usize => Country::benin(),
            60usize => Country::bermuda(),
            64usize => Country::bhutan(),
            862usize => Country::bolivarian_republic_of_venezuela(),
            68usize => Country::bolivia(),
            535usize => Country::bonaire(),
            70usize => Country::bosnia_and_herzegovina(),
            72usize => Country::botswana(),
            74usize => Country::bouvet_island(),
            76usize => Country::brazil(),
            86usize => Country::british_indian_ocean_territory(),
            92usize => Country::british_virgin_islands(),
            96usize => Country::brunei_darussalam(),
            100usize => Country::bulgaria(),
            854usize => Country::burkina_faso(),
            108usize => Country::burundi(),
            132usize => Country::cabo_verde(),
            116usize => Country::cambodia(),
            120usize => Country::cameroon(),
            124usize => Country::canada(),
            148usize => Country::chad(),
            152usize => Country::chile(),
            156usize => Country::china(),
            162usize => Country::christmas_island(),
            170usize => Country::colombia(),
            188usize => Country::costa_rica(),
            384usize => Country::coted_ivoire(),
            191usize => Country::croatia(),
            192usize => Country::cuba(),
            531usize => Country::curacao(),
            196usize => Country::cyprus(),
            203usize => Country::czechia(),
            208usize => Country::denmark(),
            262usize => Country::djibouti(),
            212usize => Country::dominica(),
            534usize => Country::dutch_part_sint_maarten(),
            218usize => Country::ecuador(),
            818usize => Country::egypt(),
            222usize => Country::el_salvador(),
            226usize => Country::equatorial_guinea(),
            232usize => Country::eritrea(),
            233usize => Country::estonia(),
            748usize => Country::eswatini(),
            231usize => Country::ethiopia(),
            583usize => Country::federated_states_of_micronesia(),
            242usize => Country::fiji(),
            246usize => Country::finland(),
            250usize => Country::france(),
            254usize => Country::french_guiana(),
            663usize => Country::french_part_saint_martin(),
            258usize => Country::french_polynesia(),
            266usize => Country::gabon(),
            268usize => Country::georgia(),
            276usize => Country::germany(),
            288usize => Country::ghana(),
            292usize => Country::gibraltar(),
            300usize => Country::greece(),
            304usize => Country::greenland(),
            308usize => Country::grenada(),
            312usize => Country::guadeloupe(),
            316usize => Country::guam(),
            320usize => Country::guatemala(),
            831usize => Country::guernsey(),
            324usize => Country::guinea(),
            624usize => Country::guinea_bissau(),
            328usize => Country::guyana(),
            332usize => Country::haiti(),
            334usize => Country::heard_island_and_mc_donald_islands(),
            340usize => Country::honduras(),
            344usize => Country::hong_kong(),
            348usize => Country::hungary(),
            352usize => Country::iceland(),
            356usize => Country::india(),
            360usize => Country::indonesia(),
            368usize => Country::iraq(),
            372usize => Country::ireland(),
            364usize => Country::islamic_republic_of_iran(),
            833usize => Country::isle_of_man(),
            376usize => Country::israel(),
            380usize => Country::italy(),
            388usize => Country::jamaica(),
            392usize => Country::japan(),
            832usize => Country::jersey(),
            400usize => Country::jordan(),
            398usize => Country::kazakhstan(),
            404usize => Country::kenya(),
            296usize => Country::kiribati(),
            383usize => Country::kosovo(),
            414usize => Country::kuwait(),
            417usize => Country::kyrgyzstan(),
            428usize => Country::latvia(),
            422usize => Country::lebanon(),
            426usize => Country::lesotho(),
            430usize => Country::liberia(),
            434usize => Country::libya(),
            438usize => Country::liechtenstein(),
            440usize => Country::lithuania(),
            442usize => Country::luxembourg(),
            446usize => Country::macao(),
            450usize => Country::madagascar(),
            454usize => Country::malawi(),
            458usize => Country::malaysia(),
            462usize => Country::maldives(),
            466usize => Country::mali(),
            470usize => Country::malta(),
            474usize => Country::martinique(),
            478usize => Country::mauritania(),
            480usize => Country::mauritius(),
            175usize => Country::mayotte(),
            484usize => Country::mexico(),
            492usize => Country::monaco(),
            496usize => Country::mongolia(),
            499usize => Country::montenegro(),
            500usize => Country::montserrat(),
            504usize => Country::morocco(),
            508usize => Country::mozambique(),
            104usize => Country::myanmar(),
            516usize => Country::namibia(),
            520usize => Country::nauru(),
            524usize => Country::nepal(),
            540usize => Country::new_caledonia(),
            554usize => Country::new_zealand(),
            558usize => Country::nicaragua(),
            566usize => Country::nigeria(),
            570usize => Country::niue(),
            574usize => Country::norfolk_island(),
            578usize => Country::norway(),
            512usize => Country::oman(),
            586usize => Country::pakistan(),
            585usize => Country::palau(),
            591usize => Country::panama(),
            598usize => Country::papua_new_guinea(),
            600usize => Country::paraguay(),
            604usize => Country::peru(),
            612usize => Country::pitcairn(),
            616usize => Country::poland(),
            620usize => Country::portugal(),
            630usize => Country::puerto_rico(),
            634usize => Country::qatar(),
            807usize => Country::republic_of_north_macedonia(),
            638usize => Country::reunion(),
            642usize => Country::romania(),
            646usize => Country::rwanda(),
            652usize => Country::saint_barthelemy(),
            659usize => Country::saint_kitts_and_nevis(),
            662usize => Country::saint_lucia(),
            666usize => Country::saint_pierre_and_miquelon(),
            670usize => Country::saint_vincent_and_the_grenadines(),
            882usize => Country::samoa(),
            674usize => Country::san_marino(),
            678usize => Country::sao_tome_and_principe(),
            682usize => Country::saudi_arabia(),
            686usize => Country::senegal(),
            688usize => Country::serbia(),
            690usize => Country::seychelles(),
            694usize => Country::sierra_leone(),
            702usize => Country::singapore(),
            703usize => Country::slovakia(),
            705usize => Country::slovenia(),
            90usize => Country::solomon_islands(),
            706usize => Country::somalia(),
            710usize => Country::south_africa(),
            239usize => Country::south_georgia_and_the_south_sandwich_islands(),
            728usize => Country::south_sudan(),
            724usize => Country::spain(),
            144usize => Country::sri_lanka(),
            275usize => Country::state_of_palestine(),
            740usize => Country::suriname(),
            744usize => Country::svalbard_and_jan_mayen(),
            752usize => Country::sweden(),
            756usize => Country::switzerland(),
            760usize => Country::syrian_arab_republic(),
            158usize => Country::taiwan(),
            762usize => Country::tajikistan(),
            764usize => Country::thailand(),
            44usize => Country::the_bahamas(),
            136usize => Country::the_cayman_islands(),
            140usize => Country::the_central_african_republic(),
            166usize => Country::the_cocos_keeling_islands(),
            174usize => Country::the_comoros(),
            178usize => Country::the_congo(),
            184usize => Country::the_cook_islands(),
            408usize => Country::the_democratic_peoples_republic_of_korea(),
            180usize => Country::the_democratic_republic_of_the_congo(),
            214usize => Country::the_dominican_republic(),
            238usize => Country::the_falkland_islands_malvinas(),
            234usize => Country::the_faroe_islands(),
            260usize => Country::the_french_southern_territories(),
            270usize => Country::the_gambia(),
            336usize => Country::the_holy_see(),
            418usize => Country::the_lao_peoples_democratic_republic(),
            584usize => Country::the_marshall_islands(),
            528usize => Country::the_netherlands(),
            562usize => Country::the_niger(),
            580usize => Country::the_northern_mariana_islands(),
            608usize => Country::the_philippines(),
            410usize => Country::the_republic_of_korea(),
            498usize => Country::the_republic_of_moldova(),
            643usize => Country::the_russian_federation(),
            729usize => Country::the_sudan(),
            796usize => Country::the_turks_and_caicos_islands(),
            784usize => Country::the_united_arab_emirates(),
            826usize => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            581usize => Country::the_united_states_minor_outlying_islands(),
            840usize => Country::the_united_states_of_america(),
            626usize => Country::timor_leste(),
            768usize => Country::togo(),
            772usize => Country::tokelau(),
            776usize => Country::tonga(),
            780usize => Country::trinidad_and_tobago(),
            788usize => Country::tunisia(),
            792usize => Country::turkey(),
            795usize => Country::turkmenistan(),
            798usize => Country::tuvalu(),
            850usize => Country::us_virgin_islands(),
            800usize => Country::uganda(),
            804usize => Country::ukraine(),
            834usize => Country::united_republic_of_tanzania(),
            858usize => Country::uruguay(),
            860usize => Country::uzbekistan(),
            548usize => Country::vanuatu(),
            704usize => Country::vietnam(),
            876usize => Country::wallis_and_futuna(),
            732usize => Country::western_sahara(),
            887usize => Country::yemen(),
            894usize => Country::zambia(),
            716usize => Country::zimbabwe(),
        };
        VALUES.get(&value).copied().ok_or("invalid value")
    }

    /// Given the three digit code, return a country or an error if
    /// the parameter doesn't match any country. The value MUST be
    /// the three digit code which includes leading zeros.
    ///
    /// ```
    /// use celes::Country;
    ///
    /// let res = Country::from_code("8");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_code("08");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_code("008");
    /// assert!(res.is_ok());
    ///
    /// assert_eq!(Country::albania(), res.unwrap());
    /// ```
    pub fn from_code<A: AsRef<str>>(code: A) -> Result<Self, &'static str> {
        static CODES: Map<&'static str, Country> = phf_map! {
            "004" => Country::afghanistan(),
            "248" => Country::aland_islands(),
            "008" => Country::albania(),
            "012" => Country::algeria(),
            "016" => Country::american_samoa(),
            "020" => Country::andorra(),
            "024" => Country::angola(),
            "660" => Country::anguilla(),
            "010" => Country::antarctica(),
            "028" => Country::antigua_and_barbuda(),
            "032" => Country::argentina(),
            "051" => Country::armenia(),
            "533" => Country::aruba(),
            "654" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "036" => Country::australia(),
            "040" => Country::austria(),
            "031" => Country::azerbaijan(),
            "048" => Country::bahrain(),
            "050" => Country::bangladesh(),
            "052" => Country::barbados(),
            "112" => Country::belarus(),
            "056" => Country::belgium(),
            "084" => Country::belize(),
            "204" => Country::benin(),
            "060" => Country::bermuda(),
            "064" => Country::bhutan(),
            "862" => Country::bolivarian_republic_of_venezuela(),
            "068" => Country::bolivia(),
            "535" => Country::bonaire(),
            "070" => Country::bosnia_and_herzegovina(),
            "072" => Country::botswana(),
            "074" => Country::bouvet_island(),
            "076" => Country::brazil(),
            "086" => Country::british_indian_ocean_territory(),
            "092" => Country::british_virgin_islands(),
            "096" => Country::brunei_darussalam(),
            "100" => Country::bulgaria(),
            "854" => Country::burkina_faso(),
            "108" => Country::burundi(),
            "132" => Country::cabo_verde(),
            "116" => Country::cambodia(),
            "120" => Country::cameroon(),
            "124" => Country::canada(),
            "148" => Country::chad(),
            "152" => Country::chile(),
            "156" => Country::china(),
            "162" => Country::christmas_island(),
            "170" => Country::colombia(),
            "188" => Country::costa_rica(),
            "384" => Country::coted_ivoire(),
            "191" => Country::croatia(),
            "192" => Country::cuba(),
            "531" => Country::curacao(),
            "196" => Country::cyprus(),
            "203" => Country::czechia(),
            "208" => Country::denmark(),
            "262" => Country::djibouti(),
            "212" => Country::dominica(),
            "534" => Country::dutch_part_sint_maarten(),
            "218" => Country::ecuador(),
            "818" => Country::egypt(),
            "222" => Country::el_salvador(),
            "226" => Country::equatorial_guinea(),
            "232" => Country::eritrea(),
            "233" => Country::estonia(),
            "748" => Country::eswatini(),
            "231" => Country::ethiopia(),
            "583" => Country::federated_states_of_micronesia(),
            "242" => Country::fiji(),
            "246" => Country::finland(),
            "250" => Country::france(),
            "254" => Country::french_guiana(),
            "663" => Country::french_part_saint_martin(),
            "258" => Country::french_polynesia(),
            "266" => Country::gabon(),
            "268" => Country::georgia(),
            "276" => Country::germany(),
            "288" => Country::ghana(),
            "292" => Country::gibraltar(),
            "300" => Country::greece(),
            "304" => Country::greenland(),
            "308" => Country::grenada(),
            "312" => Country::guadeloupe(),
            "316" => Country::guam(),
            "320" => Country::guatemala(),
            "831" => Country::guernsey(),
            "324" => Country::guinea(),
            "624" => Country::guinea_bissau(),
            "328" => Country::guyana(),
            "332" => Country::haiti(),
            "334" => Country::heard_island_and_mc_donald_islands(),
            "340" => Country::honduras(),
            "344" => Country::hong_kong(),
            "348" => Country::hungary(),
            "352" => Country::iceland(),
            "356" => Country::india(),
            "360" => Country::indonesia(),
            "368" => Country::iraq(),
            "372" => Country::ireland(),
            "364" => Country::islamic_republic_of_iran(),
            "833" => Country::isle_of_man(),
            "376" => Country::israel(),
            "380" => Country::italy(),
            "388" => Country::jamaica(),
            "392" => Country::japan(),
            "832" => Country::jersey(),
            "400" => Country::jordan(),
            "398" => Country::kazakhstan(),
            "404" => Country::kenya(),
            "296" => Country::kiribati(),
            "383" => Country::kosovo(),
            "414" => Country::kuwait(),
            "417" => Country::kyrgyzstan(),
            "428" => Country::latvia(),
            "422" => Country::lebanon(),
            "426" => Country::lesotho(),
            "430" => Country::liberia(),
            "434" => Country::libya(),
            "438" => Country::liechtenstein(),
            "440" => Country::lithuania(),
            "442" => Country::luxembourg(),
            "446" => Country::macao(),
            "450" => Country::madagascar(),
            "454" => Country::malawi(),
            "458" => Country::malaysia(),
            "462" => Country::maldives(),
            "466" => Country::mali(),
            "470" => Country::malta(),
            "474" => Country::martinique(),
            "478" => Country::mauritania(),
            "480" => Country::mauritius(),
            "175" => Country::mayotte(),
            "484" => Country::mexico(),
            "492" => Country::monaco(),
            "496" => Country::mongolia(),
            "499" => Country::montenegro(),
            "500" => Country::montserrat(),
            "504" => Country::morocco(),
            "508" => Country::mozambique(),
            "104" => Country::myanmar(),
            "516" => Country::namibia(),
            "520" => Country::nauru(),
            "524" => Country::nepal(),
            "540" => Country::new_caledonia(),
            "554" => Country::new_zealand(),
            "558" => Country::nicaragua(),
            "566" => Country::nigeria(),
            "570" => Country::niue(),
            "574" => Country::norfolk_island(),
            "578" => Country::norway(),
            "512" => Country::oman(),
            "586" => Country::pakistan(),
            "585" => Country::palau(),
            "591" => Country::panama(),
            "598" => Country::papua_new_guinea(),
            "600" => Country::paraguay(),
            "604" => Country::peru(),
            "612" => Country::pitcairn(),
            "616" => Country::poland(),
            "620" => Country::portugal(),
            "630" => Country::puerto_rico(),
            "634" => Country::qatar(),
            "807" => Country::republic_of_north_macedonia(),
            "638" => Country::reunion(),
            "642" => Country::romania(),
            "646" => Country::rwanda(),
            "652" => Country::saint_barthelemy(),
            "659" => Country::saint_kitts_and_nevis(),
            "662" => Country::saint_lucia(),
            "666" => Country::saint_pierre_and_miquelon(),
            "670" => Country::saint_vincent_and_the_grenadines(),
            "882" => Country::samoa(),
            "674" => Country::san_marino(),
            "678" => Country::sao_tome_and_principe(),
            "682" => Country::saudi_arabia(),
            "686" => Country::senegal(),
            "688" => Country::serbia(),
            "690" => Country::seychelles(),
            "694" => Country::sierra_leone(),
            "702" => Country::singapore(),
            "703" => Country::slovakia(),
            "705" => Country::slovenia(),
            "090" => Country::solomon_islands(),
            "706" => Country::somalia(),
            "710" => Country::south_africa(),
            "239" => Country::south_georgia_and_the_south_sandwich_islands(),
            "728" => Country::south_sudan(),
            "724" => Country::spain(),
            "144" => Country::sri_lanka(),
            "275" => Country::state_of_palestine(),
            "740" => Country::suriname(),
            "744" => Country::svalbard_and_jan_mayen(),
            "752" => Country::sweden(),
            "756" => Country::switzerland(),
            "760" => Country::syrian_arab_republic(),
            "158" => Country::taiwan(),
            "762" => Country::tajikistan(),
            "764" => Country::thailand(),
            "044" => Country::the_bahamas(),
            "136" => Country::the_cayman_islands(),
            "140" => Country::the_central_african_republic(),
            "166" => Country::the_cocos_keeling_islands(),
            "174" => Country::the_comoros(),
            "178" => Country::the_congo(),
            "184" => Country::the_cook_islands(),
            "408" => Country::the_democratic_peoples_republic_of_korea(),
            "180" => Country::the_democratic_republic_of_the_congo(),
            "214" => Country::the_dominican_republic(),
            "238" => Country::the_falkland_islands_malvinas(),
            "234" => Country::the_faroe_islands(),
            "260" => Country::the_french_southern_territories(),
            "270" => Country::the_gambia(),
            "336" => Country::the_holy_see(),
            "418" => Country::the_lao_peoples_democratic_republic(),
            "584" => Country::the_marshall_islands(),
            "528" => Country::the_netherlands(),
            "562" => Country::the_niger(),
            "580" => Country::the_northern_mariana_islands(),
            "608" => Country::the_philippines(),
            "410" => Country::the_republic_of_korea(),
            "498" => Country::the_republic_of_moldova(),
            "643" => Country::the_russian_federation(),
            "729" => Country::the_sudan(),
            "796" => Country::the_turks_and_caicos_islands(),
            "784" => Country::the_united_arab_emirates(),
            "826" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "581" => Country::the_united_states_minor_outlying_islands(),
            "840" => Country::the_united_states_of_america(),
            "626" => Country::timor_leste(),
            "768" => Country::togo(),
            "772" => Country::tokelau(),
            "776" => Country::tonga(),
            "780" => Country::trinidad_and_tobago(),
            "788" => Country::tunisia(),
            "792" => Country::turkey(),
            "795" => Country::turkmenistan(),
            "798" => Country::tuvalu(),
            "850" => Country::us_virgin_islands(),
            "800" => Country::uganda(),
            "804" => Country::ukraine(),
            "834" => Country::united_republic_of_tanzania(),
            "858" => Country::uruguay(),
            "860" => Country::uzbekistan(),
            "548" => Country::vanuatu(),
            "704" => Country::vietnam(),
            "876" => Country::wallis_and_futuna(),
            "732" => Country::western_sahara(),
            "887" => Country::yemen(),
            "894" => Country::zambia(),
            "716" => Country::zimbabwe(),
        };
        CODES
            .get(code.as_ref().to_lowercase().as_str())
            .copied()
            .ok_or("invalid code")
    }

    /// Given the alpha2 letters, return a country or an error if
    /// the parameter doesn't match any country. This is case-insensitive.
    ///
    /// ```
    /// use celes::Country;
    ///
    /// let res = Country::from_alpha2("u");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_alpha2("us");
    /// assert!(res.is_ok());
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    ///
    /// let res = Country::from_alpha2("Us");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    ///
    /// let res = Country::from_alpha2("uS");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    ///
    /// let res = Country::from_alpha2("US");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    /// ```
    pub fn from_alpha2<A: AsRef<str>>(alpha2: A) -> Result<Self, &'static str> {
        static ALPHA2: Map<&'static str, Country> = phf_map! {
            "af" => Country::afghanistan(),
            "ax" => Country::aland_islands(),
            "al" => Country::albania(),
            "dz" => Country::algeria(),
            "as" => Country::american_samoa(),
            "ad" => Country::andorra(),
            "ao" => Country::angola(),
            "ai" => Country::anguilla(),
            "aq" => Country::antarctica(),
            "ag" => Country::antigua_and_barbuda(),
            "ar" => Country::argentina(),
            "am" => Country::armenia(),
            "aw" => Country::aruba(),
            "sh" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "au" => Country::australia(),
            "at" => Country::austria(),
            "az" => Country::azerbaijan(),
            "bh" => Country::bahrain(),
            "bd" => Country::bangladesh(),
            "bb" => Country::barbados(),
            "by" => Country::belarus(),
            "be" => Country::belgium(),
            "bz" => Country::belize(),
            "bj" => Country::benin(),
            "bm" => Country::bermuda(),
            "bt" => Country::bhutan(),
            "ve" => Country::bolivarian_republic_of_venezuela(),
            "bo" => Country::bolivia(),
            "bq" => Country::bonaire(),
            "ba" => Country::bosnia_and_herzegovina(),
            "bw" => Country::botswana(),
            "bv" => Country::bouvet_island(),
            "br" => Country::brazil(),
            "io" => Country::british_indian_ocean_territory(),
            "vg" => Country::british_virgin_islands(),
            "bn" => Country::brunei_darussalam(),
            "bg" => Country::bulgaria(),
            "bf" => Country::burkina_faso(),
            "bi" => Country::burundi(),
            "cv" => Country::cabo_verde(),
            "kh" => Country::cambodia(),
            "cm" => Country::cameroon(),
            "ca" => Country::canada(),
            "td" => Country::chad(),
            "cl" => Country::chile(),
            "cn" => Country::china(),
            "cx" => Country::christmas_island(),
            "co" => Country::colombia(),
            "cr" => Country::costa_rica(),
            "ci" => Country::coted_ivoire(),
            "hr" => Country::croatia(),
            "cu" => Country::cuba(),
            "cw" => Country::curacao(),
            "cy" => Country::cyprus(),
            "cz" => Country::czechia(),
            "dk" => Country::denmark(),
            "dj" => Country::djibouti(),
            "dm" => Country::dominica(),
            "sx" => Country::dutch_part_sint_maarten(),
            "ec" => Country::ecuador(),
            "eg" => Country::egypt(),
            "sv" => Country::el_salvador(),
            "gq" => Country::equatorial_guinea(),
            "er" => Country::eritrea(),
            "ee" => Country::estonia(),
            "sz" => Country::eswatini(),
            "et" => Country::ethiopia(),
            "fm" => Country::federated_states_of_micronesia(),
            "fj" => Country::fiji(),
            "fi" => Country::finland(),
            "fr" => Country::france(),
            "gf" => Country::french_guiana(),
            "mf" => Country::french_part_saint_martin(),
            "pf" => Country::french_polynesia(),
            "ga" => Country::gabon(),
            "ge" => Country::georgia(),
            "de" => Country::germany(),
            "gh" => Country::ghana(),
            "gi" => Country::gibraltar(),
            "gr" => Country::greece(),
            "gl" => Country::greenland(),
            "gd" => Country::grenada(),
            "gp" => Country::guadeloupe(),
            "gu" => Country::guam(),
            "gt" => Country::guatemala(),
            "gg" => Country::guernsey(),
            "gn" => Country::guinea(),
            "gw" => Country::guinea_bissau(),
            "gy" => Country::guyana(),
            "ht" => Country::haiti(),
            "hm" => Country::heard_island_and_mc_donald_islands(),
            "hn" => Country::honduras(),
            "hk" => Country::hong_kong(),
            "hu" => Country::hungary(),
            "is" => Country::iceland(),
            "in" => Country::india(),
            "id" => Country::indonesia(),
            "iq" => Country::iraq(),
            "ie" => Country::ireland(),
            "ir" => Country::islamic_republic_of_iran(),
            "im" => Country::isle_of_man(),
            "il" => Country::israel(),
            "it" => Country::italy(),
            "jm" => Country::jamaica(),
            "jp" => Country::japan(),
            "je" => Country::jersey(),
            "jo" => Country::jordan(),
            "kz" => Country::kazakhstan(),
            "ke" => Country::kenya(),
            "ki" => Country::kiribati(),
            "xk" => Country::kosovo(),
            "kw" => Country::kuwait(),
            "kg" => Country::kyrgyzstan(),
            "lv" => Country::latvia(),
            "lb" => Country::lebanon(),
            "ls" => Country::lesotho(),
            "lr" => Country::liberia(),
            "ly" => Country::libya(),
            "li" => Country::liechtenstein(),
            "lt" => Country::lithuania(),
            "lu" => Country::luxembourg(),
            "mo" => Country::macao(),
            "mg" => Country::madagascar(),
            "mw" => Country::malawi(),
            "my" => Country::malaysia(),
            "mv" => Country::maldives(),
            "ml" => Country::mali(),
            "mt" => Country::malta(),
            "mq" => Country::martinique(),
            "mr" => Country::mauritania(),
            "mu" => Country::mauritius(),
            "yt" => Country::mayotte(),
            "mx" => Country::mexico(),
            "mc" => Country::monaco(),
            "mn" => Country::mongolia(),
            "me" => Country::montenegro(),
            "ms" => Country::montserrat(),
            "ma" => Country::morocco(),
            "mz" => Country::mozambique(),
            "mm" => Country::myanmar(),
            "na" => Country::namibia(),
            "nr" => Country::nauru(),
            "np" => Country::nepal(),
            "nc" => Country::new_caledonia(),
            "nz" => Country::new_zealand(),
            "ni" => Country::nicaragua(),
            "ng" => Country::nigeria(),
            "nu" => Country::niue(),
            "nf" => Country::norfolk_island(),
            "no" => Country::norway(),
            "om" => Country::oman(),
            "pk" => Country::pakistan(),
            "pw" => Country::palau(),
            "pa" => Country::panama(),
            "pg" => Country::papua_new_guinea(),
            "py" => Country::paraguay(),
            "pe" => Country::peru(),
            "pn" => Country::pitcairn(),
            "pl" => Country::poland(),
            "pt" => Country::portugal(),
            "pr" => Country::puerto_rico(),
            "qa" => Country::qatar(),
            "mk" => Country::republic_of_north_macedonia(),
            "re" => Country::reunion(),
            "ro" => Country::romania(),
            "rw" => Country::rwanda(),
            "bl" => Country::saint_barthelemy(),
            "kn" => Country::saint_kitts_and_nevis(),
            "lc" => Country::saint_lucia(),
            "pm" => Country::saint_pierre_and_miquelon(),
            "vc" => Country::saint_vincent_and_the_grenadines(),
            "ws" => Country::samoa(),
            "sm" => Country::san_marino(),
            "st" => Country::sao_tome_and_principe(),
            "sa" => Country::saudi_arabia(),
            "sn" => Country::senegal(),
            "rs" => Country::serbia(),
            "sc" => Country::seychelles(),
            "sl" => Country::sierra_leone(),
            "sg" => Country::singapore(),
            "sk" => Country::slovakia(),
            "si" => Country::slovenia(),
            "sb" => Country::solomon_islands(),
            "so" => Country::somalia(),
            "za" => Country::south_africa(),
            "gs" => Country::south_georgia_and_the_south_sandwich_islands(),
            "ss" => Country::south_sudan(),
            "es" => Country::spain(),
            "lk" => Country::sri_lanka(),
            "ps" => Country::state_of_palestine(),
            "sr" => Country::suriname(),
            "sj" => Country::svalbard_and_jan_mayen(),
            "se" => Country::sweden(),
            "ch" => Country::switzerland(),
            "sy" => Country::syrian_arab_republic(),
            "tw" => Country::taiwan(),
            "tj" => Country::tajikistan(),
            "th" => Country::thailand(),
            "bs" => Country::the_bahamas(),
            "ky" => Country::the_cayman_islands(),
            "cf" => Country::the_central_african_republic(),
            "cc" => Country::the_cocos_keeling_islands(),
            "km" => Country::the_comoros(),
            "cg" => Country::the_congo(),
            "ck" => Country::the_cook_islands(),
            "kp" => Country::the_democratic_peoples_republic_of_korea(),
            "cd" => Country::the_democratic_republic_of_the_congo(),
            "do" => Country::the_dominican_republic(),
            "fk" => Country::the_falkland_islands_malvinas(),
            "fo" => Country::the_faroe_islands(),
            "tf" => Country::the_french_southern_territories(),
            "gm" => Country::the_gambia(),
            "va" => Country::the_holy_see(),
            "la" => Country::the_lao_peoples_democratic_republic(),
            "mh" => Country::the_marshall_islands(),
            "nl" => Country::the_netherlands(),
            "ne" => Country::the_niger(),
            "mp" => Country::the_northern_mariana_islands(),
            "ph" => Country::the_philippines(),
            "kr" => Country::the_republic_of_korea(),
            "md" => Country::the_republic_of_moldova(),
            "ru" => Country::the_russian_federation(),
            "sd" => Country::the_sudan(),
            "tc" => Country::the_turks_and_caicos_islands(),
            "ae" => Country::the_united_arab_emirates(),
            "gb" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "um" => Country::the_united_states_minor_outlying_islands(),
            "us" => Country::the_united_states_of_america(),
            "tl" => Country::timor_leste(),
            "tg" => Country::togo(),
            "tk" => Country::tokelau(),
            "to" => Country::tonga(),
            "tt" => Country::trinidad_and_tobago(),
            "tn" => Country::tunisia(),
            "tr" => Country::turkey(),
            "tm" => Country::turkmenistan(),
            "tv" => Country::tuvalu(),
            "vi" => Country::us_virgin_islands(),
            "ug" => Country::uganda(),
            "ua" => Country::ukraine(),
            "tz" => Country::united_republic_of_tanzania(),
            "uy" => Country::uruguay(),
            "uz" => Country::uzbekistan(),
            "vu" => Country::vanuatu(),
            "vn" => Country::vietnam(),
            "wf" => Country::wallis_and_futuna(),
            "eh" => Country::western_sahara(),
            "ye" => Country::yemen(),
            "zm" => Country::zambia(),
            "zw" => Country::zimbabwe(),
        };
        ALPHA2
            .get(alpha2.as_ref().to_lowercase().as_str())
            .copied()
            .ok_or("invalid alpha2")
    }

    /// Given the alpha3 letters, return a country or an error if
    /// the parameter doesn't match any country. This is case-insensitive.
    /// ```
    /// use celes::Country;
    ///
    /// let res = Country::from_alpha3("u");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_alpha3("us");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_alpha3("Usa");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    ///
    /// let res = Country::from_alpha3("uSa");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    ///
    /// let res = Country::from_alpha3("USA");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    /// ```
    pub fn from_alpha3<A: AsRef<str>>(alpha3: A) -> Result<Self, &'static str> {
        static ALPHA3: Map<&'static str, Country> = phf_map! {
            "afg" => Country::afghanistan(),
            "ala" => Country::aland_islands(),
            "alb" => Country::albania(),
            "dza" => Country::algeria(),
            "asm" => Country::american_samoa(),
            "and" => Country::andorra(),
            "ago" => Country::angola(),
            "aia" => Country::anguilla(),
            "ata" => Country::antarctica(),
            "atg" => Country::antigua_and_barbuda(),
            "arg" => Country::argentina(),
            "arm" => Country::armenia(),
            "abw" => Country::aruba(),
            "shn" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "aus" => Country::australia(),
            "aut" => Country::austria(),
            "aze" => Country::azerbaijan(),
            "bhr" => Country::bahrain(),
            "bgd" => Country::bangladesh(),
            "brb" => Country::barbados(),
            "blr" => Country::belarus(),
            "bel" => Country::belgium(),
            "blz" => Country::belize(),
            "ben" => Country::benin(),
            "bmu" => Country::bermuda(),
            "btn" => Country::bhutan(),
            "ven" => Country::bolivarian_republic_of_venezuela(),
            "bol" => Country::bolivia(),
            "bes" => Country::bonaire(),
            "bih" => Country::bosnia_and_herzegovina(),
            "bwa" => Country::botswana(),
            "bvt" => Country::bouvet_island(),
            "bra" => Country::brazil(),
            "iot" => Country::british_indian_ocean_territory(),
            "vgb" => Country::british_virgin_islands(),
            "brn" => Country::brunei_darussalam(),
            "bgr" => Country::bulgaria(),
            "bfa" => Country::burkina_faso(),
            "bdi" => Country::burundi(),
            "cpv" => Country::cabo_verde(),
            "khm" => Country::cambodia(),
            "cmr" => Country::cameroon(),
            "can" => Country::canada(),
            "tcd" => Country::chad(),
            "chl" => Country::chile(),
            "chn" => Country::china(),
            "cxr" => Country::christmas_island(),
            "col" => Country::colombia(),
            "cri" => Country::costa_rica(),
            "civ" => Country::coted_ivoire(),
            "hrv" => Country::croatia(),
            "cub" => Country::cuba(),
            "cuw" => Country::curacao(),
            "cyp" => Country::cyprus(),
            "cze" => Country::czechia(),
            "dnk" => Country::denmark(),
            "dji" => Country::djibouti(),
            "dma" => Country::dominica(),
            "sxm" => Country::dutch_part_sint_maarten(),
            "ecu" => Country::ecuador(),
            "egy" => Country::egypt(),
            "slv" => Country::el_salvador(),
            "gnq" => Country::equatorial_guinea(),
            "eri" => Country::eritrea(),
            "est" => Country::estonia(),
            "swz" => Country::eswatini(),
            "eth" => Country::ethiopia(),
            "fsm" => Country::federated_states_of_micronesia(),
            "fji" => Country::fiji(),
            "fin" => Country::finland(),
            "fra" => Country::france(),
            "guf" => Country::french_guiana(),
            "maf" => Country::french_part_saint_martin(),
            "pyf" => Country::french_polynesia(),
            "gab" => Country::gabon(),
            "geo" => Country::georgia(),
            "deu" => Country::germany(),
            "gha" => Country::ghana(),
            "gib" => Country::gibraltar(),
            "grc" => Country::greece(),
            "grl" => Country::greenland(),
            "grd" => Country::grenada(),
            "glp" => Country::guadeloupe(),
            "gum" => Country::guam(),
            "gtm" => Country::guatemala(),
            "ggy" => Country::guernsey(),
            "gin" => Country::guinea(),
            "gnb" => Country::guinea_bissau(),
            "guy" => Country::guyana(),
            "hti" => Country::haiti(),
            "hmd" => Country::heard_island_and_mc_donald_islands(),
            "hnd" => Country::honduras(),
            "hkg" => Country::hong_kong(),
            "hun" => Country::hungary(),
            "isl" => Country::iceland(),
            "ind" => Country::india(),
            "idn" => Country::indonesia(),
            "irq" => Country::iraq(),
            "irl" => Country::ireland(),
            "irn" => Country::islamic_republic_of_iran(),
            "imn" => Country::isle_of_man(),
            "isr" => Country::israel(),
            "ita" => Country::italy(),
            "jam" => Country::jamaica(),
            "jpn" => Country::japan(),
            "jey" => Country::jersey(),
            "jor" => Country::jordan(),
            "kaz" => Country::kazakhstan(),
            "ken" => Country::kenya(),
            "xkx" => Country::kosovo(),
            "kir" => Country::kiribati(),
            "kwt" => Country::kuwait(),
            "kgz" => Country::kyrgyzstan(),
            "lva" => Country::latvia(),
            "lbn" => Country::lebanon(),
            "lso" => Country::lesotho(),
            "lbr" => Country::liberia(),
            "lby" => Country::libya(),
            "lie" => Country::liechtenstein(),
            "ltu" => Country::lithuania(),
            "lux" => Country::luxembourg(),
            "mac" => Country::macao(),
            "mdg" => Country::madagascar(),
            "mwi" => Country::malawi(),
            "mys" => Country::malaysia(),
            "mdv" => Country::maldives(),
            "mli" => Country::mali(),
            "mlt" => Country::malta(),
            "mtq" => Country::martinique(),
            "mrt" => Country::mauritania(),
            "mus" => Country::mauritius(),
            "myt" => Country::mayotte(),
            "mex" => Country::mexico(),
            "mco" => Country::monaco(),
            "mng" => Country::mongolia(),
            "mne" => Country::montenegro(),
            "msr" => Country::montserrat(),
            "mar" => Country::morocco(),
            "moz" => Country::mozambique(),
            "mmr" => Country::myanmar(),
            "nam" => Country::namibia(),
            "nru" => Country::nauru(),
            "npl" => Country::nepal(),
            "ncl" => Country::new_caledonia(),
            "nzl" => Country::new_zealand(),
            "nic" => Country::nicaragua(),
            "nga" => Country::nigeria(),
            "niu" => Country::niue(),
            "nfk" => Country::norfolk_island(),
            "nor" => Country::norway(),
            "omn" => Country::oman(),
            "pak" => Country::pakistan(),
            "plw" => Country::palau(),
            "pan" => Country::panama(),
            "png" => Country::papua_new_guinea(),
            "pry" => Country::paraguay(),
            "per" => Country::peru(),
            "pcn" => Country::pitcairn(),
            "pol" => Country::poland(),
            "prt" => Country::portugal(),
            "pri" => Country::puerto_rico(),
            "qat" => Country::qatar(),
            "mkd" => Country::republic_of_north_macedonia(),
            "reu" => Country::reunion(),
            "rou" => Country::romania(),
            "rwa" => Country::rwanda(),
            "blm" => Country::saint_barthelemy(),
            "kna" => Country::saint_kitts_and_nevis(),
            "lca" => Country::saint_lucia(),
            "spm" => Country::saint_pierre_and_miquelon(),
            "vct" => Country::saint_vincent_and_the_grenadines(),
            "wsm" => Country::samoa(),
            "smr" => Country::san_marino(),
            "stp" => Country::sao_tome_and_principe(),
            "sau" => Country::saudi_arabia(),
            "sen" => Country::senegal(),
            "srb" => Country::serbia(),
            "syc" => Country::seychelles(),
            "sle" => Country::sierra_leone(),
            "sgp" => Country::singapore(),
            "svk" => Country::slovakia(),
            "svn" => Country::slovenia(),
            "slb" => Country::solomon_islands(),
            "som" => Country::somalia(),
            "zaf" => Country::south_africa(),
            "sgs" => Country::south_georgia_and_the_south_sandwich_islands(),
            "ssd" => Country::south_sudan(),
            "esp" => Country::spain(),
            "lka" => Country::sri_lanka(),
            "pse" => Country::state_of_palestine(),
            "sur" => Country::suriname(),
            "sjm" => Country::svalbard_and_jan_mayen(),
            "swe" => Country::sweden(),
            "che" => Country::switzerland(),
            "syr" => Country::syrian_arab_republic(),
            "twn" => Country::taiwan(),
            "tjk" => Country::tajikistan(),
            "tha" => Country::thailand(),
            "bhs" => Country::the_bahamas(),
            "cym" => Country::the_cayman_islands(),
            "caf" => Country::the_central_african_republic(),
            "cck" => Country::the_cocos_keeling_islands(),
            "com" => Country::the_comoros(),
            "cog" => Country::the_congo(),
            "cok" => Country::the_cook_islands(),
            "prk" => Country::the_democratic_peoples_republic_of_korea(),
            "cod" => Country::the_democratic_republic_of_the_congo(),
            "dom" => Country::the_dominican_republic(),
            "flk" => Country::the_falkland_islands_malvinas(),
            "fro" => Country::the_faroe_islands(),
            "atf" => Country::the_french_southern_territories(),
            "gmb" => Country::the_gambia(),
            "vat" => Country::the_holy_see(),
            "lao" => Country::the_lao_peoples_democratic_republic(),
            "mhl" => Country::the_marshall_islands(),
            "nld" => Country::the_netherlands(),
            "ner" => Country::the_niger(),
            "mnp" => Country::the_northern_mariana_islands(),
            "phl" => Country::the_philippines(),
            "kor" => Country::the_republic_of_korea(),
            "mda" => Country::the_republic_of_moldova(),
            "rus" => Country::the_russian_federation(),
            "sdn" => Country::the_sudan(),
            "tca" => Country::the_turks_and_caicos_islands(),
            "are" => Country::the_united_arab_emirates(),
            "gbr" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "umi" => Country::the_united_states_minor_outlying_islands(),
            "usa" => Country::the_united_states_of_america(),
            "tls" => Country::timor_leste(),
            "tgo" => Country::togo(),
            "tkl" => Country::tokelau(),
            "ton" => Country::tonga(),
            "tto" => Country::trinidad_and_tobago(),
            "tun" => Country::tunisia(),
            "tur" => Country::turkey(),
            "tkm" => Country::turkmenistan(),
            "tuv" => Country::tuvalu(),
            "vir" => Country::us_virgin_islands(),
            "uga" => Country::uganda(),
            "ukr" => Country::ukraine(),
            "tza" => Country::united_republic_of_tanzania(),
            "ury" => Country::uruguay(),
            "uzb" => Country::uzbekistan(),
            "vut" => Country::vanuatu(),
            "vnm" => Country::vietnam(),
            "wlf" => Country::wallis_and_futuna(),
            "esh" => Country::western_sahara(),
            "yem" => Country::yemen(),
            "zmb" => Country::zambia(),
            "zwe" => Country::zimbabwe(),
        };
        ALPHA3
            .get(alpha3.as_ref().to_lowercase().as_str())
            .copied()
            .ok_or("invalid alpha3")
    }

    /// Given the a country alias, return a country or an error if
    /// the parameter doesn't match any country
    ///
    /// The alias is any value in the `aliases` field for a country.
    /// For example, "america" would return `the_united_states_of_america`
    /// This is case-insensitive.
    ///
    /// ```
    /// use celes::Country;
    ///
    /// let res = Country::from_alias("u");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_alias("us");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_alias("america");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    ///
    /// let res = Country::from_alias("russia");
    /// assert_eq!(Country::the_russian_federation(), res.unwrap());
    ///
    /// let res = Country::from_alias("england");
    /// assert_eq!(Country::the_united_kingdom_of_great_britain_and_northern_ireland(), res.unwrap());
    /// ```
    pub fn from_alias<A: AsRef<str>>(alias: A) -> Result<Self, &'static str> {
        static ALIASES: Map<&'static str, Country> = phf_map! {
            "samoa" => Country::american_samoa(),
            "sthelena" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "sainthelena" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "venezuela" => Country::bolivarian_republic_of_venezuela(),
            "bosnia" => Country::bosnia_and_herzegovina(),
            "herzegovina" => Country::bosnia_and_herzegovina(),
            "brunei" => Country::brunei_darussalam(),
            "burkina" => Country::burkina_faso(),
            "stmaarten" => Country::dutch_part_sint_maarten(),
            "saintmaarten" => Country::dutch_part_sint_maarten(),
            "micronesia" => Country::federated_states_of_micronesia(),
            "stmartin" => Country::french_part_saint_martin(),
            "saintmartin" => Country::french_part_saint_martin(),
            "heardisland" => Country::heard_island_and_mc_donald_islands(),
            "mcdonaldislands" => Country::heard_island_and_mc_donald_islands(),
            "iran" => Country::islamic_republic_of_iran(),
            "macedonia" => Country::republic_of_north_macedonia(),
            "stbarthelemy" => Country::saint_barthelemy(),
            "stkitts" => Country::saint_kitts_and_nevis(),
            "stlucia" => Country::saint_lucia(),
            "stpierre" => Country::saint_pierre_and_miquelon(),
            "saintpierre" => Country::saint_pierre_and_miquelon(),
            "stvincent" => Country::saint_vincent_and_the_grenadines(),
            "saintvincent" => Country::saint_vincent_and_the_grenadines(),
            "saotome" => Country::sao_tome_and_principe(),
            "southgeorgia" => Country::south_georgia_and_the_south_sandwich_islands(),
            "southsandwichislands" => Country::south_georgia_and_the_south_sandwich_islands(),
            "palestine" => Country::state_of_palestine(),
            "taiwan" => Country::taiwan(),
            "bahamas" => Country::the_bahamas(),
            "caymanislands" => Country::the_cayman_islands(),
            "centralafricanrepublic" => Country::the_central_african_republic(),
            "cocosislands" => Country::the_cocos_keeling_islands(),
            "keelingislands" => Country::the_cocos_keeling_islands(),
            "comoros" => Country::the_comoros(),
            "congo" => Country::the_congo(),
            "cookislands" => Country::the_cook_islands(),
            "czechrepublic" => Country::czechia(),
            "northkorea" => Country::the_democratic_peoples_republic_of_korea(),
            "democraticpeoplesrepublicofkorea" => Country::the_democratic_peoples_republic_of_korea(),
            "democraticrepublicofthecongo" => Country::the_democratic_republic_of_the_congo(),
            "dominicanrepublic" => Country::the_dominican_republic(),
            "easttimor" => Country::timor_leste(),
            "malvinas" => Country::the_falkland_islands_malvinas(),
            "falklandislands" => Country::the_falkland_islands_malvinas(),
            "faroeislands" => Country::the_faroe_islands(),
            "frenchsouthernterritories" => Country::the_french_southern_territories(),
            "gambia" => Country::the_gambia(),
            "holysee" => Country::the_holy_see(),
            "laopeoplesdemocraticrepublic" => Country::the_lao_peoples_democratic_republic(),
            "marshallislands" => Country::the_marshall_islands(),
            "netherlands" => Country::the_netherlands(),
            "holland" => Country::the_netherlands(),
            "niger" => Country::the_niger(),
            "northernmarianaislands" => Country::the_northern_mariana_islands(),
            "philippines" => Country::the_philippines(),
            "southkorea" => Country::the_republic_of_korea(),
            "republicofkorea" => Country::the_republic_of_korea(),
            "moldova" => Country::the_republic_of_moldova(),
            "republicofmoldova" => Country::the_republic_of_moldova(),
            "russia" => Country::the_russian_federation(),
            "russianfederation" => Country::the_russian_federation(),
            "sudan" => Country::the_sudan(),
            "turksandcaicosislands" => Country::the_turks_and_caicos_islands(),
            "unitedarabemirates" => Country::the_united_arab_emirates(),
            "england" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "scotland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "greatbritain" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "unitedkingdom" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "northernireland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "unitedkingdomofgreatbritain" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "unitedkingdomofgreatbritainandnorthernireland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "unitedstatesminoroutlyingislands" => Country::the_united_states_minor_outlying_islands(),
            "america" => Country::the_united_states_of_america(),
            "unitedstates" => Country::the_united_states_of_america(),
            "unitedstatesofamerica" => Country::the_united_states_of_america(),
            "trinidad" => Country::trinidad_and_tobago(),
            "tobago" => Country::trinidad_and_tobago(),
            "tanzania" => Country::united_republic_of_tanzania(),
            "türkiye" => Country::turkey(),
            "turkey" => Country::turkey(),
        };
        ALIASES
            .get(alias.as_ref().to_lowercase().as_str())
            .copied()
            .ok_or("invalid alias")
    }

    /// Given the country name, return a country or an error if
    /// the parameter doesn't match any country.  This is case-insensitive.
    ///
    /// For example, Albania, Algeria, Brazil would return the country
    /// struct that represents those countries.
    ///
    /// ```
    /// use celes::Country;
    ///
    /// let res = Country::from_name("russianfederation");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_name("unitedstatesofamerica");
    /// assert!(res.is_err());
    ///
    /// let res = Country::from_name("Albania");
    /// assert_eq!(Country::albania(), res.unwrap());
    ///
    /// let res = Country::from_name("theunitedstatesofamerica");
    /// assert_eq!(Country::the_united_states_of_america(), res.unwrap());
    ///
    /// let res = Country::from_name("therussianfederation");
    /// assert_eq!(Country::the_russian_federation(), res.unwrap());
    ///
    /// let res = Country::from_name("theunitedkingdomofgreatbritainandnorthernireland");
    /// assert_eq!(Country::the_united_kingdom_of_great_britain_and_northern_ireland(), res.unwrap());
    /// ```
    pub fn from_name<A: AsRef<str>>(name: A) -> Result<Self, &'static str> {
        static NAMES: Map<&'static str, Country> = phf_map! {
            "afghanistan" => Country::afghanistan(),
            "alandislands" => Country::aland_islands(),
            "albania" => Country::albania(),
            "algeria" => Country::algeria(),
            "americansamoa" => Country::american_samoa(),
            "andorra" => Country::andorra(),
            "angola" => Country::angola(),
            "anguilla" => Country::anguilla(),
            "antarctica" => Country::antarctica(),
            "antiguaandbarbuda" => Country::antigua_and_barbuda(),
            "argentina" => Country::argentina(),
            "armenia" => Country::armenia(),
            "aruba" => Country::aruba(),
            "ascensionandtristandacunhasainthelena" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "australia" => Country::australia(),
            "austria" => Country::austria(),
            "azerbaijan" => Country::azerbaijan(),
            "bahrain" => Country::bahrain(),
            "bangladesh" => Country::bangladesh(),
            "barbados" => Country::barbados(),
            "belarus" => Country::belarus(),
            "belgium" => Country::belgium(),
            "belize" => Country::belize(),
            "benin" => Country::benin(),
            "bermuda" => Country::bermuda(),
            "bhutan" => Country::bhutan(),
            "bolivarianrepublicofvenezuela" => Country::bolivarian_republic_of_venezuela(),
            "bolivia" => Country::bolivia(),
            "bonaire" => Country::bonaire(),
            "bosniaandherzegovina" => Country::bosnia_and_herzegovina(),
            "botswana" => Country::botswana(),
            "bouvetisland" => Country::bouvet_island(),
            "brazil" => Country::brazil(),
            "britishindianoceanterritory" => Country::british_indian_ocean_territory(),
            "britishvirginislands" => Country::british_virgin_islands(),
            "bruneidarussalam" => Country::brunei_darussalam(),
            "bulgaria" => Country::bulgaria(),
            "burkinafaso" => Country::burkina_faso(),
            "burundi" => Country::burundi(),
            "caboverde" => Country::cabo_verde(),
            "cambodia" => Country::cambodia(),
            "cameroon" => Country::cameroon(),
            "canada" => Country::canada(),
            "chad" => Country::chad(),
            "chile" => Country::chile(),
            "china" => Country::china(),
            "christmasisland" => Country::christmas_island(),
            "colombia" => Country::colombia(),
            "costarica" => Country::costa_rica(),
            "cotedivoire" => Country::coted_ivoire(),
            "croatia" => Country::croatia(),
            "cuba" => Country::cuba(),
            "curacao" => Country::curacao(),
            "cyprus" => Country::cyprus(),
            "czechia" => Country::czechia(),
            "denmark" => Country::denmark(),
            "djibouti" => Country::djibouti(),
            "dominica" => Country::dominica(),
            "dutchpartsintmaarten" => Country::dutch_part_sint_maarten(),
            "ecuador" => Country::ecuador(),
            "egypt" => Country::egypt(),
            "elsalvador" => Country::el_salvador(),
            "equatorialguinea" => Country::equatorial_guinea(),
            "eritrea" => Country::eritrea(),
            "estonia" => Country::estonia(),
            "eswatini" => Country::eswatini(),
            "ethiopia" => Country::ethiopia(),
            "federatedstatesofmicronesia" => Country::federated_states_of_micronesia(),
            "fiji" => Country::fiji(),
            "finland" => Country::finland(),
            "france" => Country::france(),
            "frenchguiana" => Country::french_guiana(),
            "frenchpartsaintmartin" => Country::french_part_saint_martin(),
            "frenchpolynesia" => Country::french_polynesia(),
            "gabon" => Country::gabon(),
            "georgia" => Country::georgia(),
            "germany" => Country::germany(),
            "ghana" => Country::ghana(),
            "gibraltar" => Country::gibraltar(),
            "greece" => Country::greece(),
            "greenland" => Country::greenland(),
            "grenada" => Country::grenada(),
            "guadeloupe" => Country::guadeloupe(),
            "guam" => Country::guam(),
            "guatemala" => Country::guatemala(),
            "guernsey" => Country::guernsey(),
            "guinea" => Country::guinea(),
            "guineabissau" => Country::guinea_bissau(),
            "guyana" => Country::guyana(),
            "haiti" => Country::haiti(),
            "heardislandandmcdonaldislands" => Country::heard_island_and_mc_donald_islands(),
            "honduras" => Country::honduras(),
            "hongkong" => Country::hong_kong(),
            "hungary" => Country::hungary(),
            "iceland" => Country::iceland(),
            "india" => Country::india(),
            "indonesia" => Country::indonesia(),
            "iraq" => Country::iraq(),
            "ireland" => Country::ireland(),
            "islamicrepublicofiran" => Country::islamic_republic_of_iran(),
            "isleofman" => Country::isle_of_man(),
            "israel" => Country::israel(),
            "italy" => Country::italy(),
            "jamaica" => Country::jamaica(),
            "japan" => Country::japan(),
            "jersey" => Country::jersey(),
            "jordan" => Country::jordan(),
            "kazakhstan" => Country::kazakhstan(),
            "kenya" => Country::kenya(),
            "kiribati" => Country::kiribati(),
            "kosovo" => Country::kosovo(),
            "kuwait" => Country::kuwait(),
            "kyrgyzstan" => Country::kyrgyzstan(),
            "latvia" => Country::latvia(),
            "lebanon" => Country::lebanon(),
            "lesotho" => Country::lesotho(),
            "liberia" => Country::liberia(),
            "libya" => Country::libya(),
            "liechtenstein" => Country::liechtenstein(),
            "lithuania" => Country::lithuania(),
            "luxembourg" => Country::luxembourg(),
            "macao" => Country::macao(),
            "madagascar" => Country::madagascar(),
            "malawi" => Country::malawi(),
            "malaysia" => Country::malaysia(),
            "maldives" => Country::maldives(),
            "mali" => Country::mali(),
            "malta" => Country::malta(),
            "martinique" => Country::martinique(),
            "mauritania" => Country::mauritania(),
            "mauritius" => Country::mauritius(),
            "mayotte" => Country::mayotte(),
            "mexico" => Country::mexico(),
            "monaco" => Country::monaco(),
            "mongolia" => Country::mongolia(),
            "montenegro" => Country::montenegro(),
            "montserrat" => Country::montserrat(),
            "morocco" => Country::morocco(),
            "mozambique" => Country::mozambique(),
            "myanmar" => Country::myanmar(),
            "namibia" => Country::namibia(),
            "nauru" => Country::nauru(),
            "nepal" => Country::nepal(),
            "newcaledonia" => Country::new_caledonia(),
            "newzealand" => Country::new_zealand(),
            "nicaragua" => Country::nicaragua(),
            "nigeria" => Country::nigeria(),
            "niue" => Country::niue(),
            "norfolkisland" => Country::norfolk_island(),
            "norway" => Country::norway(),
            "oman" => Country::oman(),
            "pakistan" => Country::pakistan(),
            "palau" => Country::palau(),
            "panama" => Country::panama(),
            "papuanewguinea" => Country::papua_new_guinea(),
            "paraguay" => Country::paraguay(),
            "peru" => Country::peru(),
            "pitcairn" => Country::pitcairn(),
            "poland" => Country::poland(),
            "portugal" => Country::portugal(),
            "puertorico" => Country::puerto_rico(),
            "qatar" => Country::qatar(),
            "republicofnorthmacedonia" => Country::republic_of_north_macedonia(),
            "reunion" => Country::reunion(),
            "romania" => Country::romania(),
            "rwanda" => Country::rwanda(),
            "saintbarthelemy" => Country::saint_barthelemy(),
            "saintkittsandnevis" => Country::saint_kitts_and_nevis(),
            "saintlucia" => Country::saint_lucia(),
            "saintpierreandmiquelon" => Country::saint_pierre_and_miquelon(),
            "saintvincentandthegrenadines" => Country::saint_vincent_and_the_grenadines(),
            "samoa" => Country::samoa(),
            "sanmarino" => Country::san_marino(),
            "saotomeandprincipe" => Country::sao_tome_and_principe(),
            "saudiarabia" => Country::saudi_arabia(),
            "senegal" => Country::senegal(),
            "serbia" => Country::serbia(),
            "seychelles" => Country::seychelles(),
            "sierraleone" => Country::sierra_leone(),
            "singapore" => Country::singapore(),
            "slovakia" => Country::slovakia(),
            "slovenia" => Country::slovenia(),
            "solomonislands" => Country::solomon_islands(),
            "somalia" => Country::somalia(),
            "southafrica" => Country::south_africa(),
            "southgeorgiaandthesouthsandwichislands" => Country::south_georgia_and_the_south_sandwich_islands(),
            "southsudan" => Country::south_sudan(),
            "spain" => Country::spain(),
            "srilanka" => Country::sri_lanka(),
            "stateofpalestine" => Country::state_of_palestine(),
            "suriname" => Country::suriname(),
            "svalbardandjanmayen" => Country::svalbard_and_jan_mayen(),
            "sweden" => Country::sweden(),
            "switzerland" => Country::switzerland(),
            "syrianarabrepublic" => Country::syrian_arab_republic(),
            "taiwan,republicofchina" => Country::taiwan(),
            "tajikistan" => Country::tajikistan(),
            "thailand" => Country::thailand(),
            "thebahamas" => Country::the_bahamas(),
            "thecaymanislands" => Country::the_cayman_islands(),
            "thecentralafricanrepublic" => Country::the_central_african_republic(),
            "thecocoskeelingislands" => Country::the_cocos_keeling_islands(),
            "thecomoros" => Country::the_comoros(),
            "thecongo" => Country::the_congo(),
            "thecookislands" => Country::the_cook_islands(),
            "thedemocraticpeoplesrepublicofkorea" => Country::the_democratic_peoples_republic_of_korea(),
            "thedemocraticrepublicofthecongo" => Country::the_democratic_republic_of_the_congo(),
            "thedominicanrepublic" => Country::the_dominican_republic(),
            "thefalklandislandsmalvinas" => Country::the_falkland_islands_malvinas(),
            "thefaroeislands" => Country::the_faroe_islands(),
            "thefrenchsouthernterritories" => Country::the_french_southern_territories(),
            "thegambia" => Country::the_gambia(),
            "theholysee" => Country::the_holy_see(),
            "thelaopeoplesdemocraticrepublic" => Country::the_lao_peoples_democratic_republic(),
            "themarshallislands" => Country::the_marshall_islands(),
            "thenetherlands" => Country::the_netherlands(),
            "theniger" => Country::the_niger(),
            "thenorthernmarianaislands" => Country::the_northern_mariana_islands(),
            "thephilippines" => Country::the_philippines(),
            "therepublicofkorea" => Country::the_republic_of_korea(),
            "therepublicofmoldova" => Country::the_republic_of_moldova(),
            "therussianfederation" => Country::the_russian_federation(),
            "thesudan" => Country::the_sudan(),
            "theturksandcaicosislands" => Country::the_turks_and_caicos_islands(),
            "theunitedarabemirates" => Country::the_united_arab_emirates(),
            "theunitedkingdomofgreatbritainandnorthernireland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "theunitedstatesminoroutlyingislands" => Country::the_united_states_minor_outlying_islands(),
            "theunitedstatesofamerica" => Country::the_united_states_of_america(),
            "timorleste" => Country::timor_leste(),
            "easttimor" => Country::timor_leste(),
            "togo" => Country::togo(),
            "tokelau" => Country::tokelau(),
            "tonga" => Country::tonga(),
            "trinidadandtobago" => Country::trinidad_and_tobago(),
            "tunisia" => Country::tunisia(),
            "turkey" => Country::turkey(),
            "türkiye" => Country::turkey(),
            "turkmenistan" => Country::turkmenistan(),
            "tuvalu" => Country::tuvalu(),
            "usvirginislands" => Country::us_virgin_islands(),
            "uganda" => Country::uganda(),
            "ukraine" => Country::ukraine(),
            "unitedrepublicoftanzania" => Country::united_republic_of_tanzania(),
            "uruguay" => Country::uruguay(),
            "uzbekistan" => Country::uzbekistan(),
            "vanuatu" => Country::vanuatu(),
            "vietnam" => Country::vietnam(),
            "wallisandfutuna" => Country::wallis_and_futuna(),
            "westernsahara" => Country::western_sahara(),
            "yemen" => Country::yemen(),
            "zambia" => Country::zambia(),
            "zimbabwe" => Country::zimbabwe(),
        };
        NAMES
            .get(name.as_ref().to_lowercase().as_str())
            .copied()
            .ok_or("unknown value")
    }
}

impl FromStr for Country {
    type Err = &'static str;

    fn from_str(code: &str) -> Result<Self, &'static str> {
        static CODES: Map<&'static str, Country> = phf_map! {
            "afghanistan" => Country::afghanistan(),
            "004" => Country::afghanistan(),
            "af" => Country::afghanistan(),
            "afg" => Country::afghanistan(),
            "alandislands" => Country::aland_islands(),
            "aland_islands" => Country::aland_islands(),
            "248" => Country::aland_islands(),
            "ax" => Country::aland_islands(),
            "ala" => Country::aland_islands(),
            "albania" => Country::albania(),
            "008" => Country::albania(),
            "al" => Country::albania(),
            "alb" => Country::albania(),
            "algeria" => Country::algeria(),
            "012" => Country::algeria(),
            "dz" => Country::algeria(),
            "dza" => Country::algeria(),
            "americansamoa" => Country::american_samoa(),
            "american_samoa" => Country::american_samoa(),
            "016" => Country::american_samoa(),
            "as" => Country::american_samoa(),
            "asm" => Country::american_samoa(),
            "andorra" => Country::andorra(),
            "020" => Country::andorra(),
            "ad" => Country::andorra(),
            "and" => Country::andorra(),
            "angola" => Country::angola(),
            "024" => Country::angola(),
            "ao" => Country::angola(),
            "ago" => Country::angola(),
            "anguilla" => Country::anguilla(),
            "660" => Country::anguilla(),
            "ai" => Country::anguilla(),
            "aia" => Country::anguilla(),
            "antarctica" => Country::antarctica(),
            "010" => Country::antarctica(),
            "aq" => Country::antarctica(),
            "ata" => Country::antarctica(),
            "antiguaandbarbuda" => Country::antigua_and_barbuda(),
            "antigua_and_barbuda" => Country::antigua_and_barbuda(),
            "028" => Country::antigua_and_barbuda(),
            "ag" => Country::antigua_and_barbuda(),
            "atg" => Country::antigua_and_barbuda(),
            "argentina" => Country::argentina(),
            "032" => Country::argentina(),
            "ar" => Country::argentina(),
            "arg" => Country::argentina(),
            "armenia" => Country::armenia(),
            "051" => Country::armenia(),
            "am" => Country::armenia(),
            "arm" => Country::armenia(),
            "aruba" => Country::aruba(),
            "533" => Country::aruba(),
            "aw" => Country::aruba(),
            "abw" => Country::aruba(),
            "ascensionandtristandacunhasainthelena" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "ascension_and_tristan_da_cunha_saint_helena" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "654" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "sh" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "shn" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "sthelena" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "sainthelena" => Country::ascension_and_tristan_da_cunha_saint_helena(),
            "australia" => Country::australia(),
            "036" => Country::australia(),
            "au" => Country::australia(),
            "aus" => Country::australia(),
            "austria" => Country::austria(),
            "040" => Country::austria(),
            "at" => Country::austria(),
            "aut" => Country::austria(),
            "azerbaijan" => Country::azerbaijan(),
            "031" => Country::azerbaijan(),
            "az" => Country::azerbaijan(),
            "aze" => Country::azerbaijan(),
            "bahrain" => Country::bahrain(),
            "048" => Country::bahrain(),
            "bh" => Country::bahrain(),
            "bhr" => Country::bahrain(),
            "bangladesh" => Country::bangladesh(),
            "050" => Country::bangladesh(),
            "bd" => Country::bangladesh(),
            "bgd" => Country::bangladesh(),
            "barbados" => Country::barbados(),
            "052" => Country::barbados(),
            "bb" => Country::barbados(),
            "brb" => Country::barbados(),
            "belarus" => Country::belarus(),
            "112" => Country::belarus(),
            "by" => Country::belarus(),
            "blr" => Country::belarus(),
            "belgium" => Country::belgium(),
            "056" => Country::belgium(),
            "be" => Country::belgium(),
            "bel" => Country::belgium(),
            "belize" => Country::belize(),
            "084" => Country::belize(),
            "bz" => Country::belize(),
            "blz" => Country::belize(),
            "benin" => Country::benin(),
            "204" => Country::benin(),
            "bj" => Country::benin(),
            "ben" => Country::benin(),
            "bermuda" => Country::bermuda(),
            "060" => Country::bermuda(),
            "bm" => Country::bermuda(),
            "bmu" => Country::bermuda(),
            "bhutan" => Country::bhutan(),
            "064" => Country::bhutan(),
            "bt" => Country::bhutan(),
            "btn" => Country::bhutan(),
            "bolivarianrepublicofvenezuela" => Country::bolivarian_republic_of_venezuela(),
            "bolivarian_republic_of_venezuela" => Country::bolivarian_republic_of_venezuela(),
            "862" => Country::bolivarian_republic_of_venezuela(),
            "ve" => Country::bolivarian_republic_of_venezuela(),
            "ven" => Country::bolivarian_republic_of_venezuela(),
            "venezuela" => Country::bolivarian_republic_of_venezuela(),
            "bolivia" => Country::bolivia(),
            "068" => Country::bolivia(),
            "bo" => Country::bolivia(),
            "bol" => Country::bolivia(),
            "bonaire" => Country::bonaire(),
            "535" => Country::bonaire(),
            "bq" => Country::bonaire(),
            "bes" => Country::bonaire(),
            "bosniaandherzegovina" => Country::bosnia_and_herzegovina(),
            "bosnia_and_herzegovina" => Country::bosnia_and_herzegovina(),
            "070" => Country::bosnia_and_herzegovina(),
            "ba" => Country::bosnia_and_herzegovina(),
            "bih" => Country::bosnia_and_herzegovina(),
            "bosnia" => Country::bosnia_and_herzegovina(),
            "herzegovina" => Country::bosnia_and_herzegovina(),
            "botswana" => Country::botswana(),
            "072" => Country::botswana(),
            "bw" => Country::botswana(),
            "bwa" => Country::botswana(),
            "bouvetisland" => Country::bouvet_island(),
            "bouvet_island" => Country::bouvet_island(),
            "074" => Country::bouvet_island(),
            "bv" => Country::bouvet_island(),
            "bvt" => Country::bouvet_island(),
            "brazil" => Country::brazil(),
            "076" => Country::brazil(),
            "br" => Country::brazil(),
            "bra" => Country::brazil(),
            "britishindianoceanterritory" => Country::british_indian_ocean_territory(),
            "british_indian_ocean_territory" => Country::british_indian_ocean_territory(),
            "086" => Country::british_indian_ocean_territory(),
            "io" => Country::british_indian_ocean_territory(),
            "iot" => Country::british_indian_ocean_territory(),
            "britishvirginislands" => Country::british_virgin_islands(),
            "british_virgin_islands" => Country::british_virgin_islands(),
            "092" => Country::british_virgin_islands(),
            "vg" => Country::british_virgin_islands(),
            "vgb" => Country::british_virgin_islands(),
            "bruneidarussalam" => Country::brunei_darussalam(),
            "brunei_darussalam" => Country::brunei_darussalam(),
            "096" => Country::brunei_darussalam(),
            "bn" => Country::brunei_darussalam(),
            "brn" => Country::brunei_darussalam(),
            "brunei" => Country::brunei_darussalam(),
            "bulgaria" => Country::bulgaria(),
            "100" => Country::bulgaria(),
            "bg" => Country::bulgaria(),
            "bgr" => Country::bulgaria(),
            "burkinafaso" => Country::burkina_faso(),
            "burkina_faso" => Country::burkina_faso(),
            "854" => Country::burkina_faso(),
            "bf" => Country::burkina_faso(),
            "bfa" => Country::burkina_faso(),
            "burkina" => Country::burkina_faso(),
            "burundi" => Country::burundi(),
            "108" => Country::burundi(),
            "bi" => Country::burundi(),
            "bdi" => Country::burundi(),
            "caboverde" => Country::cabo_verde(),
            "cabo_verde" => Country::cabo_verde(),
            "132" => Country::cabo_verde(),
            "cv" => Country::cabo_verde(),
            "cpv" => Country::cabo_verde(),
            "cambodia" => Country::cambodia(),
            "116" => Country::cambodia(),
            "kh" => Country::cambodia(),
            "khm" => Country::cambodia(),
            "cameroon" => Country::cameroon(),
            "120" => Country::cameroon(),
            "cm" => Country::cameroon(),
            "cmr" => Country::cameroon(),
            "canada" => Country::canada(),
            "124" => Country::canada(),
            "ca" => Country::canada(),
            "can" => Country::canada(),
            "chad" => Country::chad(),
            "148" => Country::chad(),
            "td" => Country::chad(),
            "tcd" => Country::chad(),
            "chile" => Country::chile(),
            "152" => Country::chile(),
            "cl" => Country::chile(),
            "chl" => Country::chile(),
            "china" => Country::china(),
            "156" => Country::china(),
            "cn" => Country::china(),
            "chn" => Country::china(),
            "christmasisland" => Country::christmas_island(),
            "christmas_island" => Country::christmas_island(),
            "162" => Country::christmas_island(),
            "cx" => Country::christmas_island(),
            "cxr" => Country::christmas_island(),
            "colombia" => Country::colombia(),
            "170" => Country::colombia(),
            "co" => Country::colombia(),
            "col" => Country::colombia(),
            "costarica" => Country::costa_rica(),
            "costa_rica" => Country::costa_rica(),
            "188" => Country::costa_rica(),
            "cr" => Country::costa_rica(),
            "cri" => Country::costa_rica(),
            "cotedivoire" => Country::coted_ivoire(),
            "coted_ivoire" => Country::coted_ivoire(),
            "384" => Country::coted_ivoire(),
            "ci" => Country::coted_ivoire(),
            "civ" => Country::coted_ivoire(),
            "croatia" => Country::croatia(),
            "191" => Country::croatia(),
            "hr" => Country::croatia(),
            "hrv" => Country::croatia(),
            "cuba" => Country::cuba(),
            "192" => Country::cuba(),
            "cu" => Country::cuba(),
            "cub" => Country::cuba(),
            "curacao" => Country::curacao(),
            "531" => Country::curacao(),
            "cw" => Country::curacao(),
            "cuw" => Country::curacao(),
            "cyprus" => Country::cyprus(),
            "196" => Country::cyprus(),
            "cy" => Country::cyprus(),
            "cyp" => Country::cyprus(),
            "czechia" => Country::czechia(),
            "czechrepublic" => Country::czechia(),
            "203" => Country::czechia(),
            "cz" => Country::czechia(),
            "cze" => Country::czechia(),
            "denmark" => Country::denmark(),
            "208" => Country::denmark(),
            "dk" => Country::denmark(),
            "dnk" => Country::denmark(),
            "djibouti" => Country::djibouti(),
            "262" => Country::djibouti(),
            "dj" => Country::djibouti(),
            "dji" => Country::djibouti(),
            "dominica" => Country::dominica(),
            "212" => Country::dominica(),
            "dm" => Country::dominica(),
            "dma" => Country::dominica(),
            "dutchpartsintmaarten" => Country::dutch_part_sint_maarten(),
            "dutch_part_sint_maarten" => Country::dutch_part_sint_maarten(),
            "534" => Country::dutch_part_sint_maarten(),
            "sx" => Country::dutch_part_sint_maarten(),
            "sxm" => Country::dutch_part_sint_maarten(),
            "stmaarten" => Country::dutch_part_sint_maarten(),
            "sintmaarten" => Country::dutch_part_sint_maarten(),
            "ecuador" => Country::ecuador(),
            "218" => Country::ecuador(),
            "ec" => Country::ecuador(),
            "ecu" => Country::ecuador(),
            "egypt" => Country::egypt(),
            "818" => Country::egypt(),
            "eg" => Country::egypt(),
            "egy" => Country::egypt(),
            "elsalvador" => Country::el_salvador(),
            "el_salvador" => Country::el_salvador(),
            "222" => Country::el_salvador(),
            "sv" => Country::el_salvador(),
            "slv" => Country::el_salvador(),
            "equatorialguinea" => Country::equatorial_guinea(),
            "equatorial_guinea" => Country::equatorial_guinea(),
            "226" => Country::equatorial_guinea(),
            "gq" => Country::equatorial_guinea(),
            "gnq" => Country::equatorial_guinea(),
            "eritrea" => Country::eritrea(),
            "232" => Country::eritrea(),
            "er" => Country::eritrea(),
            "eri" => Country::eritrea(),
            "estonia" => Country::estonia(),
            "233" => Country::estonia(),
            "ee" => Country::estonia(),
            "est" => Country::estonia(),
            "eswatini" => Country::eswatini(),
            "748" => Country::eswatini(),
            "sz" => Country::eswatini(),
            "swz" => Country::eswatini(),
            "ethiopia" => Country::ethiopia(),
            "231" => Country::ethiopia(),
            "et" => Country::ethiopia(),
            "eth" => Country::ethiopia(),
            "federatedstatesofmicronesia" => Country::federated_states_of_micronesia(),
            "federated_states_of_micronesia" => Country::federated_states_of_micronesia(),
            "583" => Country::federated_states_of_micronesia(),
            "fm" => Country::federated_states_of_micronesia(),
            "fsm" => Country::federated_states_of_micronesia(),
            "micronesia" => Country::federated_states_of_micronesia(),
            "fiji" => Country::fiji(),
            "242" => Country::fiji(),
            "fj" => Country::fiji(),
            "fji" => Country::fiji(),
            "finland" => Country::finland(),
            "246" => Country::finland(),
            "fi" => Country::finland(),
            "fin" => Country::finland(),
            "france" => Country::france(),
            "250" => Country::france(),
            "fr" => Country::france(),
            "fra" => Country::france(),
            "frenchguiana" => Country::french_guiana(),
            "french_guiana" => Country::french_guiana(),
            "254" => Country::french_guiana(),
            "gf" => Country::french_guiana(),
            "guf" => Country::french_guiana(),
            "frenchpartsaintmartin" => Country::french_part_saint_martin(),
            "french_part_saint_martin" => Country::french_part_saint_martin(),
            "663" => Country::french_part_saint_martin(),
            "mf" => Country::french_part_saint_martin(),
            "maf" => Country::french_part_saint_martin(),
            "stmartin" => Country::french_part_saint_martin(),
            "saintmartin" => Country::french_part_saint_martin(),
            "frenchpolynesia" => Country::french_polynesia(),
            "258" => Country::french_polynesia(),
            "pf" => Country::french_polynesia(),
            "pyf" => Country::french_polynesia(),
            "gabon" => Country::gabon(),
            "266" => Country::gabon(),
            "ga" => Country::gabon(),
            "gab" => Country::gabon(),
            "georgia" => Country::georgia(),
            "268" => Country::georgia(),
            "ge" => Country::georgia(),
            "geo" => Country::georgia(),
            "germany" => Country::germany(),
            "276" => Country::germany(),
            "de" => Country::germany(),
            "deu" => Country::germany(),
            "ghana" => Country::ghana(),
            "288" => Country::ghana(),
            "gh" => Country::ghana(),
            "gha" => Country::ghana(),
            "gibraltar" => Country::gibraltar(),
            "292" => Country::gibraltar(),
            "gi" => Country::gibraltar(),
            "gib" => Country::gibraltar(),
            "greece" => Country::greece(),
            "300" => Country::greece(),
            "gr" => Country::greece(),
            "grc" => Country::greece(),
            "greenland" => Country::greenland(),
            "304" => Country::greenland(),
            "gl" => Country::greenland(),
            "grl" => Country::greenland(),
            "grenada" => Country::grenada(),
            "308" => Country::grenada(),
            "gd" => Country::grenada(),
            "grd" => Country::grenada(),
            "guadeloupe" => Country::guadeloupe(),
            "312" => Country::guadeloupe(),
            "gp" => Country::guadeloupe(),
            "glp" => Country::guadeloupe(),
            "guam" => Country::guam(),
            "316" => Country::guam(),
            "gu" => Country::guam(),
            "gum" => Country::guam(),
            "guatemala" => Country::guatemala(),
            "320" => Country::guatemala(),
            "gt" => Country::guatemala(),
            "gtm" => Country::guatemala(),
            "guernsey" => Country::guernsey(),
            "831" => Country::guernsey(),
            "gg" => Country::guernsey(),
            "ggy" => Country::guernsey(),
            "guinea" => Country::guinea(),
            "324" => Country::guinea(),
            "gn" => Country::guinea(),
            "gin" => Country::guinea(),
            "guineabissau" => Country::guinea_bissau(),
            "guinea_bissau" => Country::guinea_bissau(),
            "624" => Country::guinea_bissau(),
            "gw" => Country::guinea_bissau(),
            "gnb" => Country::guinea_bissau(),
            "guyana" => Country::guyana(),
            "328" => Country::guyana(),
            "gy" => Country::guyana(),
            "guy" => Country::guyana(),
            "haiti" => Country::haiti(),
            "332" => Country::haiti(),
            "ht" => Country::haiti(),
            "hti" => Country::haiti(),
            "heardislandandmcdonaldislands" => Country::heard_island_and_mc_donald_islands(),
            "heard_island_and_mc_donald_islands" => Country::heard_island_and_mc_donald_islands(),
            "334" => Country::heard_island_and_mc_donald_islands(),
            "hm" => Country::heard_island_and_mc_donald_islands(),
            "hmd" => Country::heard_island_and_mc_donald_islands(),
            "heardisland" => Country::heard_island_and_mc_donald_islands(),
            "mcdonaldislands" => Country::heard_island_and_mc_donald_islands(),
            "honduras" => Country::honduras(),
            "340" => Country::honduras(),
            "hn" => Country::honduras(),
            "hnd" => Country::honduras(),
            "hongkong" => Country::hong_kong(),
            "hong_kong" => Country::hong_kong(),
            "344" => Country::hong_kong(),
            "hk" => Country::hong_kong(),
            "hkg" => Country::hong_kong(),
            "hungary" => Country::hungary(),
            "348" => Country::hungary(),
            "hu" => Country::hungary(),
            "hun" => Country::hungary(),
            "iceland" => Country::iceland(),
            "352" => Country::iceland(),
            "is" => Country::iceland(),
            "isl" => Country::iceland(),
            "india" => Country::india(),
            "356" => Country::india(),
            "in" => Country::india(),
            "ind" => Country::india(),
            "indonesia" => Country::indonesia(),
            "360" => Country::indonesia(),
            "id" => Country::indonesia(),
            "idn" => Country::indonesia(),
            "iraq" => Country::iraq(),
            "368" => Country::iraq(),
            "iq" => Country::iraq(),
            "irq" => Country::iraq(),
            "ireland" => Country::ireland(),
            "372" => Country::ireland(),
            "ie" => Country::ireland(),
            "irl" => Country::ireland(),
            "islamicrepublicofiran" => Country::islamic_republic_of_iran(),
            "islamic_republic_of_iran" => Country::islamic_republic_of_iran(),
            "364" => Country::islamic_republic_of_iran(),
            "ir" => Country::islamic_republic_of_iran(),
            "irn" => Country::islamic_republic_of_iran(),
            "iran" => Country::islamic_republic_of_iran(),
            "isleofman" => Country::isle_of_man(),
            "isle_of_man" => Country::isle_of_man(),
            "833" => Country::isle_of_man(),
            "im" => Country::isle_of_man(),
            "imn" => Country::isle_of_man(),
            "israel" => Country::israel(),
            "376" => Country::israel(),
            "il" => Country::israel(),
            "isr" => Country::israel(),
            "italy" => Country::italy(),
            "380" => Country::italy(),
            "it" => Country::italy(),
            "ita" => Country::italy(),
            "jamaica" => Country::jamaica(),
            "388" => Country::jamaica(),
            "jm" => Country::jamaica(),
            "jam" => Country::jamaica(),
            "japan" => Country::japan(),
            "392" => Country::japan(),
            "jp" => Country::japan(),
            "jpn" => Country::japan(),
            "jersey" => Country::jersey(),
            "832" => Country::jersey(),
            "je" => Country::jersey(),
            "jey" => Country::jersey(),
            "jordan" => Country::jordan(),
            "400" => Country::jordan(),
            "jo" => Country::jordan(),
            "jor" => Country::jordan(),
            "kazakhstan" => Country::kazakhstan(),
            "398" => Country::kazakhstan(),
            "kz" => Country::kazakhstan(),
            "kaz" => Country::kazakhstan(),
            "kenya" => Country::kenya(),
            "404" => Country::kenya(),
            "ke" => Country::kenya(),
            "ken" => Country::kenya(),
            "kiribati" => Country::kiribati(),
            "296" => Country::kiribati(),
            "ki" => Country::kiribati(),
            "kir" => Country::kiribati(),
            "kosovo" => Country::kosovo(),
            "383" => Country::kosovo(),
            "xk" => Country::kosovo(),
            "xkx" => Country::kosovo(),
            "kuwait" => Country::kuwait(),
            "414" => Country::kuwait(),
            "kw" => Country::kuwait(),
            "kwt" => Country::kuwait(),
            "kyrgyzstan" => Country::kyrgyzstan(),
            "417" => Country::kyrgyzstan(),
            "kg" => Country::kyrgyzstan(),
            "kgz" => Country::kyrgyzstan(),
            "latvia" => Country::latvia(),
            "428" => Country::latvia(),
            "lv" => Country::latvia(),
            "lva" => Country::latvia(),
            "lebanon" => Country::lebanon(),
            "422" => Country::lebanon(),
            "lb" => Country::lebanon(),
            "lbn" => Country::lebanon(),
            "lesotho" => Country::lesotho(),
            "426" => Country::lesotho(),
            "ls" => Country::lesotho(),
            "lso" => Country::lesotho(),
            "liberia" => Country::liberia(),
            "430" => Country::liberia(),
            "lr" => Country::liberia(),
            "lbr" => Country::liberia(),
            "libya" => Country::libya(),
            "434" => Country::libya(),
            "ly" => Country::libya(),
            "lby" => Country::libya(),
            "liechtenstein" => Country::liechtenstein(),
            "438" => Country::liechtenstein(),
            "li" => Country::liechtenstein(),
            "lie" => Country::liechtenstein(),
            "lithuania" => Country::lithuania(),
            "440" => Country::lithuania(),
            "lt" => Country::lithuania(),
            "ltu" => Country::lithuania(),
            "luxembourg" => Country::luxembourg(),
            "442" => Country::luxembourg(),
            "lu" => Country::luxembourg(),
            "lux" => Country::luxembourg(),
            "macao" => Country::macao(),
            "446" => Country::macao(),
            "mo" => Country::macao(),
            "mac" => Country::macao(),
            "madagascar" => Country::madagascar(),
            "450" => Country::madagascar(),
            "mg" => Country::madagascar(),
            "mdg" => Country::madagascar(),
            "malawi" => Country::malawi(),
            "454" => Country::malawi(),
            "mw" => Country::malawi(),
            "mwi" => Country::malawi(),
            "malaysia" => Country::malaysia(),
            "458" => Country::malaysia(),
            "my" => Country::malaysia(),
            "mys" => Country::malaysia(),
            "maldives" => Country::maldives(),
            "462" => Country::maldives(),
            "mv" => Country::maldives(),
            "mdv" => Country::maldives(),
            "mali" => Country::mali(),
            "466" => Country::mali(),
            "ml" => Country::mali(),
            "mli" => Country::mali(),
            "malta" => Country::malta(),
            "470" => Country::malta(),
            "mt" => Country::malta(),
            "mlt" => Country::malta(),
            "martinique" => Country::martinique(),
            "474" => Country::martinique(),
            "mq" => Country::martinique(),
            "mtq" => Country::martinique(),
            "mauritania" => Country::mauritania(),
            "478" => Country::mauritania(),
            "mr" => Country::mauritania(),
            "mrt" => Country::mauritania(),
            "mauritius" => Country::mauritius(),
            "480" => Country::mauritius(),
            "mu" => Country::mauritius(),
            "mus" => Country::mauritius(),
            "mayotte" => Country::mayotte(),
            "175" => Country::mayotte(),
            "yt" => Country::mayotte(),
            "myt" => Country::mayotte(),
            "mexico" => Country::mexico(),
            "484" => Country::mexico(),
            "mx" => Country::mexico(),
            "mex" => Country::mexico(),
            "monaco" => Country::monaco(),
            "492" => Country::monaco(),
            "mc" => Country::monaco(),
            "mco" => Country::monaco(),
            "mongolia" => Country::mongolia(),
            "496" => Country::mongolia(),
            "mn" => Country::mongolia(),
            "mng" => Country::mongolia(),
            "montenegro" => Country::montenegro(),
            "499" => Country::montenegro(),
            "me" => Country::montenegro(),
            "mne" => Country::montenegro(),
            "montserrat" => Country::montserrat(),
            "500" => Country::montserrat(),
            "ms" => Country::montserrat(),
            "msr" => Country::montserrat(),
            "morocco" => Country::morocco(),
            "504" => Country::morocco(),
            "ma" => Country::morocco(),
            "mar" => Country::morocco(),
            "mozambique" => Country::mozambique(),
            "508" => Country::mozambique(),
            "mz" => Country::mozambique(),
            "moz" => Country::mozambique(),
            "myanmar" => Country::myanmar(),
            "104" => Country::myanmar(),
            "mm" => Country::myanmar(),
            "mmr" => Country::myanmar(),
            "namibia" => Country::namibia(),
            "516" => Country::namibia(),
            "na" => Country::namibia(),
            "nam" => Country::namibia(),
            "nauru" => Country::nauru(),
            "520" => Country::nauru(),
            "nr" => Country::nauru(),
            "nru" => Country::nauru(),
            "nepal" => Country::nepal(),
            "524" => Country::nepal(),
            "np" => Country::nepal(),
            "npl" => Country::nepal(),
            "newcaledonia" => Country::new_caledonia(),
            "new_caledonia" => Country::new_caledonia(),
            "540" => Country::new_caledonia(),
            "nc" => Country::new_caledonia(),
            "ncl" => Country::new_caledonia(),
            "newzealand" => Country::new_zealand(),
            "new_zealand" => Country::new_zealand(),
            "554" => Country::new_zealand(),
            "nz" => Country::new_zealand(),
            "nzl" => Country::new_zealand(),
            "nicaragua" => Country::nicaragua(),
            "558" => Country::nicaragua(),
            "ni" => Country::nicaragua(),
            "nic" => Country::nicaragua(),
            "nigeria" => Country::nigeria(),
            "566" => Country::nigeria(),
            "ng" => Country::nigeria(),
            "nga" => Country::nigeria(),
            "niue" => Country::niue(),
            "570" => Country::niue(),
            "nu" => Country::niue(),
            "niu" => Country::niue(),
            "norfolkisland" => Country::norfolk_island(),
            "norfolk_island" => Country::norfolk_island(),
            "574" => Country::norfolk_island(),
            "nf" => Country::norfolk_island(),
            "nfk" => Country::norfolk_island(),
            "norway" => Country::norway(),
            "578" => Country::norway(),
            "no" => Country::norway(),
            "nor" => Country::norway(),
            "oman" => Country::oman(),
            "512" => Country::oman(),
            "om" => Country::oman(),
            "omn" => Country::oman(),
            "pakistan" => Country::pakistan(),
            "586" => Country::pakistan(),
            "pk" => Country::pakistan(),
            "pak" => Country::pakistan(),
            "palau" => Country::palau(),
            "585" => Country::palau(),
            "pw" => Country::palau(),
            "plw" => Country::palau(),
            "panama" => Country::panama(),
            "591" => Country::panama(),
            "pa" => Country::panama(),
            "pan" => Country::panama(),
            "papuanewguinea" => Country::papua_new_guinea(),
            "papua_new_guinea" => Country::papua_new_guinea(),
            "598" => Country::papua_new_guinea(),
            "pg" => Country::papua_new_guinea(),
            "png" => Country::papua_new_guinea(),
            "paraguay" => Country::paraguay(),
            "600" => Country::paraguay(),
            "py" => Country::paraguay(),
            "pry" => Country::paraguay(),
            "peru" => Country::peru(),
            "604" => Country::peru(),
            "pe" => Country::peru(),
            "per" => Country::peru(),
            "pitcairn" => Country::pitcairn(),
            "612" => Country::pitcairn(),
            "pn" => Country::pitcairn(),
            "pcn" => Country::pitcairn(),
            "poland" => Country::poland(),
            "616" => Country::poland(),
            "pl" => Country::poland(),
            "pol" => Country::poland(),
            "portugal" => Country::portugal(),
            "620" => Country::portugal(),
            "pt" => Country::portugal(),
            "prt" => Country::portugal(),
            "puertorico" => Country::puerto_rico(),
            "puerto_rico" => Country::puerto_rico(),
            "630" => Country::puerto_rico(),
            "pr" => Country::puerto_rico(),
            "pri" => Country::puerto_rico(),
            "qatar" => Country::qatar(),
            "634" => Country::qatar(),
            "qa" => Country::qatar(),
            "qat" => Country::qatar(),
            "republicofnorthmacedonia" => Country::republic_of_north_macedonia(),
            "republic_of_north_macedonia" => Country::republic_of_north_macedonia(),
            "807" => Country::republic_of_north_macedonia(),
            "mk" => Country::republic_of_north_macedonia(),
            "mkd" => Country::republic_of_north_macedonia(),
            "macedonia" => Country::republic_of_north_macedonia(),
            "reunion" => Country::reunion(),
            "638" => Country::reunion(),
            "re" => Country::reunion(),
            "reu" => Country::reunion(),
            "romania" => Country::romania(),
            "642" => Country::romania(),
            "ro" => Country::romania(),
            "rou" => Country::romania(),
            "rwanda" => Country::rwanda(),
            "646" => Country::rwanda(),
            "rw" => Country::rwanda(),
            "rwa" => Country::rwanda(),
            "saintbarthelemy" => Country::saint_barthelemy(),
            "saint_barthelemy" => Country::saint_barthelemy(),
            "652" => Country::saint_barthelemy(),
            "bl" => Country::saint_barthelemy(),
            "blm" => Country::saint_barthelemy(),
            "stbarthelemy" => Country::saint_barthelemy(),
            "saintkittsandnevis" => Country::saint_kitts_and_nevis(),
            "saint_kitts_and_nevis" => Country::saint_kitts_and_nevis(),
            "659" => Country::saint_kitts_and_nevis(),
            "kn" => Country::saint_kitts_and_nevis(),
            "kna" => Country::saint_kitts_and_nevis(),
            "stkitts" => Country::saint_kitts_and_nevis(),
            "saintlucia" => Country::saint_lucia(),
            "saint_lucia" => Country::saint_lucia(),
            "662" => Country::saint_lucia(),
            "lc" => Country::saint_lucia(),
            "lca" => Country::saint_lucia(),
            "stlucia" => Country::saint_lucia(),
            "saintpierreandmiquelon" => Country::saint_pierre_and_miquelon(),
            "saint_pierre_and_miquelon" => Country::saint_pierre_and_miquelon(),
            "666" => Country::saint_pierre_and_miquelon(),
            "pm" => Country::saint_pierre_and_miquelon(),
            "spm" => Country::saint_pierre_and_miquelon(),
            "stpierre" => Country::saint_pierre_and_miquelon(),
            "saintpierre" => Country::saint_pierre_and_miquelon(),
            "saintvincentandthegrenadines" => Country::saint_vincent_and_the_grenadines(),
            "saint_vincent_and_the_grenadines" => Country::saint_vincent_and_the_grenadines(),
            "670" => Country::saint_vincent_and_the_grenadines(),
            "vc" => Country::saint_vincent_and_the_grenadines(),
            "vct" => Country::saint_vincent_and_the_grenadines(),
            "stvincent" => Country::saint_vincent_and_the_grenadines(),
            "saintvincent" => Country::saint_vincent_and_the_grenadines(),
            "samoa" => Country::samoa(),
            "882" => Country::samoa(),
            "ws" => Country::samoa(),
            "wsm" => Country::samoa(),
            "sanmarino" => Country::san_marino(),
            "san_marino" => Country::san_marino(),
            "674" => Country::san_marino(),
            "sm" => Country::san_marino(),
            "smr" => Country::san_marino(),
            "saotomeandprincipe" => Country::sao_tome_and_principe(),
            "sao_tome_and_principe" => Country::sao_tome_and_principe(),
            "678" => Country::sao_tome_and_principe(),
            "st" => Country::sao_tome_and_principe(),
            "stp" => Country::sao_tome_and_principe(),
            "saotome" => Country::sao_tome_and_principe(),
            "saudiarabia" => Country::saudi_arabia(),
            "saudi_arabia" => Country::saudi_arabia(),
            "682" => Country::saudi_arabia(),
            "sa" => Country::saudi_arabia(),
            "sau" => Country::saudi_arabia(),
            "senegal" => Country::senegal(),
            "686" => Country::senegal(),
            "sn" => Country::senegal(),
            "sen" => Country::senegal(),
            "serbia" => Country::serbia(),
            "688" => Country::serbia(),
            "rs" => Country::serbia(),
            "srb" => Country::serbia(),
            "seychelles" => Country::seychelles(),
            "690" => Country::seychelles(),
            "sc" => Country::seychelles(),
            "syc" => Country::seychelles(),
            "sierraleone" => Country::sierra_leone(),
            "sierra_leone" => Country::sierra_leone(),
            "694" => Country::sierra_leone(),
            "sl" => Country::sierra_leone(),
            "sle" => Country::sierra_leone(),
            "singapore" => Country::singapore(),
            "702" => Country::singapore(),
            "sg" => Country::singapore(),
            "sgp" => Country::singapore(),
            "slovakia" => Country::slovakia(),
            "703" => Country::slovakia(),
            "sk" => Country::slovakia(),
            "svk" => Country::slovakia(),
            "slovenia" => Country::slovenia(),
            "705" => Country::slovenia(),
            "si" => Country::slovenia(),
            "svn" => Country::slovenia(),
            "solomonislands" => Country::solomon_islands(),
            "solomon_islands" => Country::solomon_islands(),
            "090" => Country::solomon_islands(),
            "sb" => Country::solomon_islands(),
            "slb" => Country::solomon_islands(),
            "somalia" => Country::somalia(),
            "706" => Country::somalia(),
            "so" => Country::somalia(),
            "som" => Country::somalia(),
            "southafrica" => Country::south_africa(),
            "south_africa" => Country::south_africa(),
            "710" => Country::south_africa(),
            "za" => Country::south_africa(),
            "zaf" => Country::south_africa(),
            "southgeorgiaandthesouthsandwichislands" => Country::south_georgia_and_the_south_sandwich_islands(),
            "south_georgia_and_the_south_sandwich_islands" => Country::south_georgia_and_the_south_sandwich_islands(),
            "239" => Country::south_georgia_and_the_south_sandwich_islands(),
            "gs" => Country::south_georgia_and_the_south_sandwich_islands(),
            "sgs" => Country::south_georgia_and_the_south_sandwich_islands(),
            "southgeorgia" => Country::south_georgia_and_the_south_sandwich_islands(),
            "southsandwichislands" => Country::south_georgia_and_the_south_sandwich_islands(),
            "southsudan" => Country::south_sudan(),
            "south_sudan" => Country::south_sudan(),
            "728" => Country::south_sudan(),
            "ss" => Country::south_sudan(),
            "ssd" => Country::south_sudan(),
            "spain" => Country::spain(),
            "724" => Country::spain(),
            "es" => Country::spain(),
            "esp" => Country::spain(),
            "srilanka" => Country::sri_lanka(),
            "sri_lanka" => Country::sri_lanka(),
            "144" => Country::sri_lanka(),
            "lk" => Country::sri_lanka(),
            "lka" => Country::sri_lanka(),
            "stateofpalestine" => Country::state_of_palestine(),
            "state_of_palestine" => Country::state_of_palestine(),
            "275" => Country::state_of_palestine(),
            "ps" => Country::state_of_palestine(),
            "pse" => Country::state_of_palestine(),
            "palestine" => Country::state_of_palestine(),
            "suriname" => Country::suriname(),
            "740" => Country::suriname(),
            "sr" => Country::suriname(),
            "sur" => Country::suriname(),
            "svalbardandjanmayen" => Country::svalbard_and_jan_mayen(),
            "svalbard_and_jan_mayen" => Country::svalbard_and_jan_mayen(),
            "744" => Country::svalbard_and_jan_mayen(),
            "sj" => Country::svalbard_and_jan_mayen(),
            "sjm" => Country::svalbard_and_jan_mayen(),
            "sweden" => Country::sweden(),
            "752" => Country::sweden(),
            "se" => Country::sweden(),
            "swe" => Country::sweden(),
            "switzerland" => Country::switzerland(),
            "756" => Country::switzerland(),
            "ch" => Country::switzerland(),
            "che" => Country::switzerland(),
            "syrianarabrepublic" => Country::syrian_arab_republic(),
            "syrian_arab_republic" => Country::syrian_arab_republic(),
            "760" => Country::syrian_arab_republic(),
            "sy" => Country::syrian_arab_republic(),
            "syr" => Country::syrian_arab_republic(),
            "taiwan,republicofchina" => Country::taiwan(),
            "taiwan" => Country::taiwan(),
            "158" => Country::taiwan(),
            "tw" => Country::taiwan(),
            "twn" => Country::taiwan(),
            "tajikistan" => Country::tajikistan(),
            "762" => Country::tajikistan(),
            "tj" => Country::tajikistan(),
            "tjk" => Country::tajikistan(),
            "thailand" => Country::thailand(),
            "764" => Country::thailand(),
            "th" => Country::thailand(),
            "tha" => Country::thailand(),
            "thebahamas" => Country::the_bahamas(),
            "the_bahamas" => Country::the_bahamas(),
            "044" => Country::the_bahamas(),
            "bs" => Country::the_bahamas(),
            "bhs" => Country::the_bahamas(),
            "bahamas" => Country::the_bahamas(),
            "thecaymanislands" => Country::the_cayman_islands(),
            "the_cayman_islands" => Country::the_cayman_islands(),
            "136" => Country::the_cayman_islands(),
            "ky" => Country::the_cayman_islands(),
            "cym" => Country::the_cayman_islands(),
            "caymanislands" => Country::the_cayman_islands(),
            "thecentralafricanrepublic" => Country::the_central_african_republic(),
            "the_central_african_republic" => Country::the_central_african_republic(),
            "140" => Country::the_central_african_republic(),
            "cf" => Country::the_central_african_republic(),
            "caf" => Country::the_central_african_republic(),
            "centralafricanrepublic" => Country::the_central_african_republic(),
            "thecocoskeelingislands" => Country::the_cocos_keeling_islands(),
            "the_cocos_keeling_islands" => Country::the_cocos_keeling_islands(),
            "166" => Country::the_cocos_keeling_islands(),
            "cc" => Country::the_cocos_keeling_islands(),
            "cck" => Country::the_cocos_keeling_islands(),
            "cocosislands" => Country::the_cocos_keeling_islands(),
            "keelingislands" => Country::the_cocos_keeling_islands(),
            "thecomoros" => Country::the_comoros(),
            "the_comoros" => Country::the_comoros(),
            "174" => Country::the_comoros(),
            "km" => Country::the_comoros(),
            "com" => Country::the_comoros(),
            "comoros" => Country::the_comoros(),
            "thecongo" => Country::the_congo(),
            "the_congo" => Country::the_congo(),
            "178" => Country::the_congo(),
            "cg" => Country::the_congo(),
            "cog" => Country::the_congo(),
            "congo" => Country::the_congo(),
            "thecookislands" => Country::the_cook_islands(),
            "the_cook_islands" => Country::the_cook_islands(),
            "184" => Country::the_cook_islands(),
            "ck" => Country::the_cook_islands(),
            "cok" => Country::the_cook_islands(),
            "cookislands" => Country::the_cook_islands(),
            "thedemocraticpeoplesrepublicofkorea" => Country::the_democratic_peoples_republic_of_korea(),
            "the_democratic_peoples_republic_of_korea" => Country::the_democratic_peoples_republic_of_korea(),
            "408" => Country::the_democratic_peoples_republic_of_korea(),
            "kp" => Country::the_democratic_peoples_republic_of_korea(),
            "prk" => Country::the_democratic_peoples_republic_of_korea(),
            "northkorea" => Country::the_democratic_peoples_republic_of_korea(),
            "democraticpeoplesrepublicofkorea" => Country::the_democratic_peoples_republic_of_korea(),
            "thedemocraticrepublicofthecongo" => Country::the_democratic_republic_of_the_congo(),
            "the_democratic_republic_of_the_congo" => Country::the_democratic_republic_of_the_congo(),
            "180" => Country::the_democratic_republic_of_the_congo(),
            "cd" => Country::the_democratic_republic_of_the_congo(),
            "cod" => Country::the_democratic_republic_of_the_congo(),
            "democraticrepublicofthecongo" => Country::the_democratic_republic_of_the_congo(),
            "thedominicanrepublic" => Country::the_dominican_republic(),
            "the_dominican_republic" => Country::the_dominican_republic(),
            "214" => Country::the_dominican_republic(),
            "do" => Country::the_dominican_republic(),
            "dom" => Country::the_dominican_republic(),
            "dominicanrepublic" => Country::the_dominican_republic(),
            "thefalklandislandsmalvinas" => Country::the_falkland_islands_malvinas(),
            "the_falkland_islands_malvinas" => Country::the_falkland_islands_malvinas(),
            "238" => Country::the_falkland_islands_malvinas(),
            "fk" => Country::the_falkland_islands_malvinas(),
            "flk" => Country::the_falkland_islands_malvinas(),
            "malvinas" => Country::the_falkland_islands_malvinas(),
            "falklandislands" => Country::the_falkland_islands_malvinas(),
            "thefaroeislands" => Country::the_faroe_islands(),
            "the_faroe_islands" => Country::the_faroe_islands(),
            "234" => Country::the_faroe_islands(),
            "fo" => Country::the_faroe_islands(),
            "fro" => Country::the_faroe_islands(),
            "faroeislands" => Country::the_faroe_islands(),
            "thefrenchsouthernterritories" => Country::the_french_southern_territories(),
            "the_french_southern_territories" => Country::the_french_southern_territories(),
            "260" => Country::the_french_southern_territories(),
            "tf" => Country::the_french_southern_territories(),
            "atf" => Country::the_french_southern_territories(),
            "frenchsouthernterritories" => Country::the_french_southern_territories(),
            "thegambia" => Country::the_gambia(),
            "the_gambia" => Country::the_gambia(),
            "270" => Country::the_gambia(),
            "gm" => Country::the_gambia(),
            "gmb" => Country::the_gambia(),
            "gambia" => Country::the_gambia(),
            "theholysee" => Country::the_holy_see(),
            "the_holy_see" => Country::the_holy_see(),
            "336" => Country::the_holy_see(),
            "va" => Country::the_holy_see(),
            "vat" => Country::the_holy_see(),
            "holysee" => Country::the_holy_see(),
            "thelaopeoplesdemocraticrepublic" => Country::the_lao_peoples_democratic_republic(),
            "the_lao_peoples_democratic_republic" => Country::the_lao_peoples_democratic_republic(),
            "418" => Country::the_lao_peoples_democratic_republic(),
            "la" => Country::the_lao_peoples_democratic_republic(),
            "lao" => Country::the_lao_peoples_democratic_republic(),
            "laopeoplesdemocraticrepublic" => Country::the_lao_peoples_democratic_republic(),
            "themarshallislands" => Country::the_marshall_islands(),
            "the_marshall_islands" => Country::the_marshall_islands(),
            "584" => Country::the_marshall_islands(),
            "mh" => Country::the_marshall_islands(),
            "mhl" => Country::the_marshall_islands(),
            "marshallislands" => Country::the_marshall_islands(),
            "thenetherlands" => Country::the_netherlands(),
            "the_netherlands" => Country::the_netherlands(),
            "528" => Country::the_netherlands(),
            "nl" => Country::the_netherlands(),
            "nld" => Country::the_netherlands(),
            "netherlands" => Country::the_netherlands(),
            "holland" => Country::the_netherlands(),
            "theniger" => Country::the_niger(),
            "the_niger" => Country::the_niger(),
            "562" => Country::the_niger(),
            "ne" => Country::the_niger(),
            "ner" => Country::the_niger(),
            "niger" => Country::the_niger(),
            "thenorthernmarianaislands" => Country::the_northern_mariana_islands(),
            "the_northern_mariana_islands" => Country::the_northern_mariana_islands(),
            "580" => Country::the_northern_mariana_islands(),
            "mp" => Country::the_northern_mariana_islands(),
            "mnp" => Country::the_northern_mariana_islands(),
            "northernmarianaislands" => Country::the_northern_mariana_islands(),
            "thephilippines" => Country::the_philippines(),
            "the_philippines" => Country::the_philippines(),
            "608" => Country::the_philippines(),
            "ph" => Country::the_philippines(),
            "phl" => Country::the_philippines(),
            "philippines" => Country::the_philippines(),
            "therepublicofkorea" => Country::the_republic_of_korea(),
            "the_republic_of_korea" => Country::the_republic_of_korea(),
            "410" => Country::the_republic_of_korea(),
            "kr" => Country::the_republic_of_korea(),
            "kor" => Country::the_republic_of_korea(),
            "southkorea" => Country::the_republic_of_korea(),
            "republicofkorea" => Country::the_republic_of_korea(),
            "therepublicofmoldova" => Country::the_republic_of_moldova(),
            "the_republic_of_moldova" => Country::the_republic_of_moldova(),
            "498" => Country::the_republic_of_moldova(),
            "md" => Country::the_republic_of_moldova(),
            "mda" => Country::the_republic_of_moldova(),
            "moldova" => Country::the_republic_of_moldova(),
            "republicofmoldova" => Country::the_republic_of_moldova(),
            "therussianfederation" => Country::the_russian_federation(),
            "the_russian_federation" => Country::the_russian_federation(),
            "643" => Country::the_russian_federation(),
            "ru" => Country::the_russian_federation(),
            "rus" => Country::the_russian_federation(),
            "russia" => Country::the_russian_federation(),
            "russianfederation" => Country::the_russian_federation(),
            "thesudan" => Country::the_sudan(),
            "the_sudan" => Country::the_sudan(),
            "729" => Country::the_sudan(),
            "sd" => Country::the_sudan(),
            "sdn" => Country::the_sudan(),
            "sudan" => Country::the_sudan(),
            "theturksandcaicosislands" => Country::the_turks_and_caicos_islands(),
            "the_turks_and_caicos_islands" => Country::the_turks_and_caicos_islands(),
            "796" => Country::the_turks_and_caicos_islands(),
            "tc" => Country::the_turks_and_caicos_islands(),
            "tca" => Country::the_turks_and_caicos_islands(),
            "turksandcaicosislands" => Country::the_turks_and_caicos_islands(),
            "theunitedarabemirates" => Country::the_united_arab_emirates(),
            "the_united_arab_emirates" => Country::the_united_arab_emirates(),
            "784" => Country::the_united_arab_emirates(),
            "ae" => Country::the_united_arab_emirates(),
            "are" => Country::the_united_arab_emirates(),
            "unitedarabemirates" => Country::the_united_arab_emirates(),
            "theunitedkingdomofgreatbritainandnorthernireland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "the_united_kingdom_of_great_britain_and_northern_ireland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "826" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "gb" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "gbr" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "england" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "scotland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "greatbritain" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "unitedkingdom" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "northernireland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "unitedkingdomofgreatbritain" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "unitedkingdomofgreatbritainandnorthernireland" => Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            "theunitedstatesminoroutlyingislands" => Country::the_united_states_minor_outlying_islands(),
            "the_united_states_minor_outlying_islands" => Country::the_united_states_minor_outlying_islands(),
            "581" => Country::the_united_states_minor_outlying_islands(),
            "um" => Country::the_united_states_minor_outlying_islands(),
            "umi" => Country::the_united_states_minor_outlying_islands(),
            "unitedstatesminoroutlyingislands" => Country::the_united_states_minor_outlying_islands(),
            "theunitedstatesofamerica" => Country::the_united_states_of_america(),
            "the_united_states_of_america" => Country::the_united_states_of_america(),
            "840" => Country::the_united_states_of_america(),
            "us" => Country::the_united_states_of_america(),
            "usa" => Country::the_united_states_of_america(),
            "america" => Country::the_united_states_of_america(),
            "united states" => Country::the_united_states_of_america(),
            "unitedstates" => Country::the_united_states_of_america(),
            "unitedstatesofamerica" => Country::the_united_states_of_america(),
            "united_states_of_america" => Country::the_united_states_of_america(),
            "timorleste" => Country::timor_leste(),
            "timor_leste" => Country::timor_leste(),
            "626" => Country::timor_leste(),
            "tl" => Country::timor_leste(),
            "tls" => Country::timor_leste(),
            "togo" => Country::togo(),
            "768" => Country::togo(),
            "tg" => Country::togo(),
            "tgo" => Country::togo(),
            "tokelau" => Country::tokelau(),
            "772" => Country::tokelau(),
            "tk" => Country::tokelau(),
            "tkl" => Country::tokelau(),
            "tonga" => Country::tonga(),
            "776" => Country::tonga(),
            "to" => Country::tonga(),
            "ton" => Country::tonga(),
            "trinidadandtobago" => Country::trinidad_and_tobago(),
            "trinidad_and_tobago" => Country::trinidad_and_tobago(),
            "780" => Country::trinidad_and_tobago(),
            "tt" => Country::trinidad_and_tobago(),
            "tto" => Country::trinidad_and_tobago(),
            "trinidad" => Country::trinidad_and_tobago(),
            "tobago" => Country::trinidad_and_tobago(),
            "tunisia" => Country::tunisia(),
            "788" => Country::tunisia(),
            "tn" => Country::tunisia(),
            "tun" => Country::tunisia(),
            "turkey" => Country::turkey(),
            "türkiye" => Country::turkey(),
            "792" => Country::turkey(),
            "tr" => Country::turkey(),
            "tur" => Country::turkey(),
            "turkmenistan" => Country::turkmenistan(),
            "795" => Country::turkmenistan(),
            "tm" => Country::turkmenistan(),
            "tkm" => Country::turkmenistan(),
            "tuvalu" => Country::tuvalu(),
            "798" => Country::tuvalu(),
            "tv" => Country::tuvalu(),
            "tuv" => Country::tuvalu(),
            "usvirginislands" => Country::us_virgin_islands(),
            "us_virgin_islands" => Country::us_virgin_islands(),
            "850" => Country::us_virgin_islands(),
            "vi" => Country::us_virgin_islands(),
            "vir" => Country::us_virgin_islands(),
            "uganda" => Country::uganda(),
            "800" => Country::uganda(),
            "ug" => Country::uganda(),
            "uga" => Country::uganda(),
            "ukraine" => Country::ukraine(),
            "804" => Country::ukraine(),
            "ua" => Country::ukraine(),
            "ukr" => Country::ukraine(),
            "unitedrepublicoftanzania" => Country::united_republic_of_tanzania(),
            "united_republic_of_tanzania" => Country::united_republic_of_tanzania(),
            "834" => Country::united_republic_of_tanzania(),
            "tz" => Country::united_republic_of_tanzania(),
            "tza" => Country::united_republic_of_tanzania(),
            "tanzania" => Country::united_republic_of_tanzania(),
            "uruguay" => Country::uruguay(),
            "858" => Country::uruguay(),
            "uy" => Country::uruguay(),
            "ury" => Country::uruguay(),
            "uzbekistan" => Country::uzbekistan(),
            "860" => Country::uzbekistan(),
            "uz" => Country::uzbekistan(),
            "uzb" => Country::uzbekistan(),
            "vanuatu" => Country::vanuatu(),
            "548" => Country::vanuatu(),
            "vu" => Country::vanuatu(),
            "vut" => Country::vanuatu(),
            "vietnam" => Country::vietnam(),
            "704" => Country::vietnam(),
            "vn" => Country::vietnam(),
            "vnm" => Country::vietnam(),
            "wallisandfutuna" => Country::wallis_and_futuna(),
            "wallis_and_futuna" => Country::wallis_and_futuna(),
            "876" => Country::wallis_and_futuna(),
            "wf" => Country::wallis_and_futuna(),
            "wlf" => Country::wallis_and_futuna(),
            "westernsahara" => Country::western_sahara(),
            "western_sahara" => Country::western_sahara(),
            "732" => Country::western_sahara(),
            "eh" => Country::western_sahara(),
            "esh" => Country::western_sahara(),
            "yemen" => Country::yemen(),
            "887" => Country::yemen(),
            "ye" => Country::yemen(),
            "yem" => Country::yemen(),
            "zambia" => Country::zambia(),
            "894" => Country::zambia(),
            "zm" => Country::zambia(),
            "zmb" => Country::zambia(),
            "zimbabwe" => Country::zimbabwe(),
            "716" => Country::zimbabwe(),
            "zw" => Country::zimbabwe(),
            "zwe" => Country::zimbabwe(),
        };
        CODES
            .get(code.to_lowercase().as_str())
            .copied()
            .ok_or("unknown value")
    }
}
