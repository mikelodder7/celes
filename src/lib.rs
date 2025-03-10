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
use serde::{
    de::{Error as DError, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{collections::HashMap, sync::LazyLock};
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
    pub fn get_countries() -> [Self; 250] {
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
        static VALUES: LazyLock<HashMap<usize, Country>> = LazyLock::new(|| {
            let mut map = HashMap::with_capacity(250);
            map.insert(4, Country::afghanistan());
            map.insert(248, Country::aland_islands());
            map.insert(8, Country::albania());
            map.insert(12, Country::algeria());
            map.insert(16, Country::american_samoa());
            map.insert(20, Country::andorra());
            map.insert(24, Country::angola());
            map.insert(660, Country::anguilla());
            map.insert(10, Country::antarctica());
            map.insert(28, Country::antigua_and_barbuda());
            map.insert(32, Country::argentina());
            map.insert(51, Country::armenia());
            map.insert(533, Country::aruba());
            map.insert(654, Country::ascension_and_tristan_da_cunha_saint_helena());
            map.insert(36, Country::australia());
            map.insert(40, Country::austria());
            map.insert(31, Country::azerbaijan());
            map.insert(48, Country::bahrain());
            map.insert(50, Country::bangladesh());
            map.insert(52, Country::barbados());
            map.insert(112, Country::belarus());
            map.insert(56, Country::belgium());
            map.insert(84, Country::belize());
            map.insert(204, Country::benin());
            map.insert(60, Country::bermuda());
            map.insert(64, Country::bhutan());
            map.insert(862, Country::bolivarian_republic_of_venezuela());
            map.insert(68, Country::bolivia());
            map.insert(535, Country::bonaire());
            map.insert(70, Country::bosnia_and_herzegovina());
            map.insert(72, Country::botswana());
            map.insert(74, Country::bouvet_island());
            map.insert(76, Country::brazil());
            map.insert(86, Country::british_indian_ocean_territory());
            map.insert(92, Country::british_virgin_islands());
            map.insert(96, Country::brunei_darussalam());
            map.insert(100, Country::bulgaria());
            map.insert(854, Country::burkina_faso());
            map.insert(108, Country::burundi());
            map.insert(132, Country::cabo_verde());
            map.insert(116, Country::cambodia());
            map.insert(120, Country::cameroon());
            map.insert(124, Country::canada());
            map.insert(148, Country::chad());
            map.insert(152, Country::chile());
            map.insert(156, Country::china());
            map.insert(162, Country::christmas_island());
            map.insert(170, Country::colombia());
            map.insert(188, Country::costa_rica());
            map.insert(384, Country::coted_ivoire());
            map.insert(191, Country::croatia());
            map.insert(192, Country::cuba());
            map.insert(531, Country::curacao());
            map.insert(196, Country::cyprus());
            map.insert(203, Country::czechia());
            map.insert(208, Country::denmark());
            map.insert(262, Country::djibouti());
            map.insert(212, Country::dominica());
            map.insert(534, Country::dutch_part_sint_maarten());
            map.insert(218, Country::ecuador());
            map.insert(818, Country::egypt());
            map.insert(222, Country::el_salvador());
            map.insert(226, Country::equatorial_guinea());
            map.insert(232, Country::eritrea());
            map.insert(233, Country::estonia());
            map.insert(748, Country::eswatini());
            map.insert(231, Country::ethiopia());
            map.insert(583, Country::federated_states_of_micronesia());
            map.insert(242, Country::fiji());
            map.insert(246, Country::finland());
            map.insert(250, Country::france());
            map.insert(254, Country::french_guiana());
            map.insert(663, Country::french_part_saint_martin());
            map.insert(258, Country::french_polynesia());
            map.insert(266, Country::gabon());
            map.insert(268, Country::georgia());
            map.insert(276, Country::germany());
            map.insert(288, Country::ghana());
            map.insert(292, Country::gibraltar());
            map.insert(300, Country::greece());
            map.insert(304, Country::greenland());
            map.insert(308, Country::grenada());
            map.insert(312, Country::guadeloupe());
            map.insert(316, Country::guam());
            map.insert(320, Country::guatemala());
            map.insert(831, Country::guernsey());
            map.insert(324, Country::guinea());
            map.insert(624, Country::guinea_bissau());
            map.insert(328, Country::guyana());
            map.insert(332, Country::haiti());
            map.insert(334, Country::heard_island_and_mc_donald_islands());
            map.insert(340, Country::honduras());
            map.insert(344, Country::hong_kong());
            map.insert(348, Country::hungary());
            map.insert(352, Country::iceland());
            map.insert(356, Country::india());
            map.insert(360, Country::indonesia());
            map.insert(368, Country::iraq());
            map.insert(372, Country::ireland());
            map.insert(364, Country::islamic_republic_of_iran());
            map.insert(833, Country::isle_of_man());
            map.insert(376, Country::israel());
            map.insert(380, Country::italy());
            map.insert(388, Country::jamaica());
            map.insert(392, Country::japan());
            map.insert(832, Country::jersey());
            map.insert(400, Country::jordan());
            map.insert(398, Country::kazakhstan());
            map.insert(404, Country::kenya());
            map.insert(296, Country::kiribati());
            map.insert(383, Country::kosovo());
            map.insert(414, Country::kuwait());
            map.insert(417, Country::kyrgyzstan());
            map.insert(428, Country::latvia());
            map.insert(422, Country::lebanon());
            map.insert(426, Country::lesotho());
            map.insert(430, Country::liberia());
            map.insert(434, Country::libya());
            map.insert(438, Country::liechtenstein());
            map.insert(440, Country::lithuania());
            map.insert(442, Country::luxembourg());
            map.insert(446, Country::macao());
            map.insert(450, Country::madagascar());
            map.insert(454, Country::malawi());
            map.insert(458, Country::malaysia());
            map.insert(462, Country::maldives());
            map.insert(466, Country::mali());
            map.insert(470, Country::malta());
            map.insert(474, Country::martinique());
            map.insert(478, Country::mauritania());
            map.insert(480, Country::mauritius());
            map.insert(175, Country::mayotte());
            map.insert(484, Country::mexico());
            map.insert(492, Country::monaco());
            map.insert(496, Country::mongolia());
            map.insert(499, Country::montenegro());
            map.insert(500, Country::montserrat());
            map.insert(504, Country::morocco());
            map.insert(508, Country::mozambique());
            map.insert(104, Country::myanmar());
            map.insert(516, Country::namibia());
            map.insert(520, Country::nauru());
            map.insert(524, Country::nepal());
            map.insert(540, Country::new_caledonia());
            map.insert(554, Country::new_zealand());
            map.insert(558, Country::nicaragua());
            map.insert(566, Country::nigeria());
            map.insert(570, Country::niue());
            map.insert(574, Country::norfolk_island());
            map.insert(578, Country::norway());
            map.insert(512, Country::oman());
            map.insert(586, Country::pakistan());
            map.insert(585, Country::palau());
            map.insert(591, Country::panama());
            map.insert(598, Country::papua_new_guinea());
            map.insert(600, Country::paraguay());
            map.insert(604, Country::peru());
            map.insert(612, Country::pitcairn());
            map.insert(616, Country::poland());
            map.insert(620, Country::portugal());
            map.insert(630, Country::puerto_rico());
            map.insert(634, Country::qatar());
            map.insert(807, Country::republic_of_north_macedonia());
            map.insert(638, Country::reunion());
            map.insert(642, Country::romania());
            map.insert(646, Country::rwanda());
            map.insert(652, Country::saint_barthelemy());
            map.insert(659, Country::saint_kitts_and_nevis());
            map.insert(662, Country::saint_lucia());
            map.insert(666, Country::saint_pierre_and_miquelon());
            map.insert(670, Country::saint_vincent_and_the_grenadines());
            map.insert(882, Country::samoa());
            map.insert(674, Country::san_marino());
            map.insert(678, Country::sao_tome_and_principe());
            map.insert(682, Country::saudi_arabia());
            map.insert(686, Country::senegal());
            map.insert(688, Country::serbia());
            map.insert(690, Country::seychelles());
            map.insert(694, Country::sierra_leone());
            map.insert(702, Country::singapore());
            map.insert(703, Country::slovakia());
            map.insert(705, Country::slovenia());
            map.insert(90, Country::solomon_islands());
            map.insert(706, Country::somalia());
            map.insert(710, Country::south_africa());
            map.insert(239, Country::south_georgia_and_the_south_sandwich_islands());
            map.insert(728, Country::south_sudan());
            map.insert(724, Country::spain());
            map.insert(144, Country::sri_lanka());
            map.insert(275, Country::state_of_palestine());
            map.insert(740, Country::suriname());
            map.insert(744, Country::svalbard_and_jan_mayen());
            map.insert(752, Country::sweden());
            map.insert(756, Country::switzerland());
            map.insert(760, Country::syrian_arab_republic());
            map.insert(158, Country::taiwan());
            map.insert(762, Country::tajikistan());
            map.insert(764, Country::thailand());
            map.insert(44, Country::the_bahamas());
            map.insert(136, Country::the_cayman_islands());
            map.insert(140, Country::the_central_african_republic());
            map.insert(166, Country::the_cocos_keeling_islands());
            map.insert(174, Country::the_comoros());
            map.insert(178, Country::the_congo());
            map.insert(184, Country::the_cook_islands());
            map.insert(408, Country::the_democratic_peoples_republic_of_korea());
            map.insert(180, Country::the_democratic_republic_of_the_congo());
            map.insert(214, Country::the_dominican_republic());
            map.insert(238, Country::the_falkland_islands_malvinas());
            map.insert(234, Country::the_faroe_islands());
            map.insert(260, Country::the_french_southern_territories());
            map.insert(270, Country::the_gambia());
            map.insert(336, Country::the_holy_see());
            map.insert(418, Country::the_lao_peoples_democratic_republic());
            map.insert(584, Country::the_marshall_islands());
            map.insert(528, Country::the_netherlands());
            map.insert(562, Country::the_niger());
            map.insert(580, Country::the_northern_mariana_islands());
            map.insert(608, Country::the_philippines());
            map.insert(410, Country::the_republic_of_korea());
            map.insert(498, Country::the_republic_of_moldova());
            map.insert(643, Country::the_russian_federation());
            map.insert(729, Country::the_sudan());
            map.insert(796, Country::the_turks_and_caicos_islands());
            map.insert(784, Country::the_united_arab_emirates());
            map.insert(
                826,
                Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            );
            map.insert(581, Country::the_united_states_minor_outlying_islands());
            map.insert(840, Country::the_united_states_of_america());
            map.insert(626, Country::timor_leste());
            map.insert(768, Country::togo());
            map.insert(772, Country::tokelau());
            map.insert(776, Country::tonga());
            map.insert(780, Country::trinidad_and_tobago());
            map.insert(788, Country::tunisia());
            map.insert(792, Country::turkey());
            map.insert(795, Country::turkmenistan());
            map.insert(798, Country::tuvalu());
            map.insert(850, Country::us_virgin_islands());
            map.insert(800, Country::uganda());
            map.insert(804, Country::ukraine());
            map.insert(834, Country::united_republic_of_tanzania());
            map.insert(858, Country::uruguay());
            map.insert(860, Country::uzbekistan());
            map.insert(548, Country::vanuatu());
            map.insert(704, Country::vietnam());
            map.insert(876, Country::wallis_and_futuna());
            map.insert(732, Country::western_sahara());
            map.insert(887, Country::yemen());
            map.insert(894, Country::zambia());
            map.insert(716, Country::zimbabwe());
            map
        });
        (*VALUES).get(&value).copied().ok_or("invalid value")
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
        static CODES: LazyLock<HashMap<&'static str, Country>> = LazyLock::new(|| {
            let mut map = HashMap::with_capacity(250);
            map.insert("004", Country::afghanistan());
            map.insert("248", Country::aland_islands());
            map.insert("008", Country::albania());
            map.insert("012", Country::algeria());
            map.insert("016", Country::american_samoa());
            map.insert("020", Country::andorra());
            map.insert("024", Country::angola());
            map.insert("660", Country::anguilla());
            map.insert("010", Country::antarctica());
            map.insert("028", Country::antigua_and_barbuda());
            map.insert("032", Country::argentina());
            map.insert("051", Country::armenia());
            map.insert("533", Country::aruba());
            map.insert(
                "654",
                Country::ascension_and_tristan_da_cunha_saint_helena(),
            );
            map.insert("036", Country::australia());
            map.insert("040", Country::austria());
            map.insert("031", Country::azerbaijan());
            map.insert("048", Country::bahrain());
            map.insert("050", Country::bangladesh());
            map.insert("052", Country::barbados());
            map.insert("112", Country::belarus());
            map.insert("056", Country::belgium());
            map.insert("084", Country::belize());
            map.insert("204", Country::benin());
            map.insert("060", Country::bermuda());
            map.insert("064", Country::bhutan());
            map.insert("862", Country::bolivarian_republic_of_venezuela());
            map.insert("068", Country::bolivia());
            map.insert("535", Country::bonaire());
            map.insert("070", Country::bosnia_and_herzegovina());
            map.insert("072", Country::botswana());
            map.insert("074", Country::bouvet_island());
            map.insert("076", Country::brazil());
            map.insert("086", Country::british_indian_ocean_territory());
            map.insert("092", Country::british_virgin_islands());
            map.insert("096", Country::brunei_darussalam());
            map.insert("100", Country::bulgaria());
            map.insert("854", Country::burkina_faso());
            map.insert("108", Country::burundi());
            map.insert("132", Country::cabo_verde());
            map.insert("116", Country::cambodia());
            map.insert("120", Country::cameroon());
            map.insert("124", Country::canada());
            map.insert("148", Country::chad());
            map.insert("152", Country::chile());
            map.insert("156", Country::china());
            map.insert("162", Country::christmas_island());
            map.insert("170", Country::colombia());
            map.insert("188", Country::costa_rica());
            map.insert("384", Country::coted_ivoire());
            map.insert("191", Country::croatia());
            map.insert("192", Country::cuba());
            map.insert("531", Country::curacao());
            map.insert("196", Country::cyprus());
            map.insert("203", Country::czechia());
            map.insert("208", Country::denmark());
            map.insert("262", Country::djibouti());
            map.insert("212", Country::dominica());
            map.insert("534", Country::dutch_part_sint_maarten());
            map.insert("218", Country::ecuador());
            map.insert("818", Country::egypt());
            map.insert("222", Country::el_salvador());
            map.insert("226", Country::equatorial_guinea());
            map.insert("232", Country::eritrea());
            map.insert("233", Country::estonia());
            map.insert("748", Country::eswatini());
            map.insert("231", Country::ethiopia());
            map.insert("583", Country::federated_states_of_micronesia());
            map.insert("242", Country::fiji());
            map.insert("246", Country::finland());
            map.insert("250", Country::france());
            map.insert("254", Country::french_guiana());
            map.insert("663", Country::french_part_saint_martin());
            map.insert("258", Country::french_polynesia());
            map.insert("266", Country::gabon());
            map.insert("268", Country::georgia());
            map.insert("276", Country::germany());
            map.insert("288", Country::ghana());
            map.insert("292", Country::gibraltar());
            map.insert("300", Country::greece());
            map.insert("304", Country::greenland());
            map.insert("308", Country::grenada());
            map.insert("312", Country::guadeloupe());
            map.insert("316", Country::guam());
            map.insert("320", Country::guatemala());
            map.insert("831", Country::guernsey());
            map.insert("324", Country::guinea());
            map.insert("624", Country::guinea_bissau());
            map.insert("328", Country::guyana());
            map.insert("332", Country::haiti());
            map.insert("334", Country::heard_island_and_mc_donald_islands());
            map.insert("340", Country::honduras());
            map.insert("344", Country::hong_kong());
            map.insert("348", Country::hungary());
            map.insert("352", Country::iceland());
            map.insert("356", Country::india());
            map.insert("360", Country::indonesia());
            map.insert("368", Country::iraq());
            map.insert("372", Country::ireland());
            map.insert("364", Country::islamic_republic_of_iran());
            map.insert("833", Country::isle_of_man());
            map.insert("376", Country::israel());
            map.insert("380", Country::italy());
            map.insert("388", Country::jamaica());
            map.insert("392", Country::japan());
            map.insert("832", Country::jersey());
            map.insert("400", Country::jordan());
            map.insert("398", Country::kazakhstan());
            map.insert("404", Country::kenya());
            map.insert("296", Country::kiribati());
            map.insert("383", Country::kosovo());
            map.insert("414", Country::kuwait());
            map.insert("417", Country::kyrgyzstan());
            map.insert("428", Country::latvia());
            map.insert("422", Country::lebanon());
            map.insert("426", Country::lesotho());
            map.insert("430", Country::liberia());
            map.insert("434", Country::libya());
            map.insert("438", Country::liechtenstein());
            map.insert("440", Country::lithuania());
            map.insert("442", Country::luxembourg());
            map.insert("446", Country::macao());
            map.insert("450", Country::madagascar());
            map.insert("454", Country::malawi());
            map.insert("458", Country::malaysia());
            map.insert("462", Country::maldives());
            map.insert("466", Country::mali());
            map.insert("470", Country::malta());
            map.insert("474", Country::martinique());
            map.insert("478", Country::mauritania());
            map.insert("480", Country::mauritius());
            map.insert("175", Country::mayotte());
            map.insert("484", Country::mexico());
            map.insert("492", Country::monaco());
            map.insert("496", Country::mongolia());
            map.insert("499", Country::montenegro());
            map.insert("500", Country::montserrat());
            map.insert("504", Country::morocco());
            map.insert("508", Country::mozambique());
            map.insert("104", Country::myanmar());
            map.insert("516", Country::namibia());
            map.insert("520", Country::nauru());
            map.insert("524", Country::nepal());
            map.insert("540", Country::new_caledonia());
            map.insert("554", Country::new_zealand());
            map.insert("558", Country::nicaragua());
            map.insert("566", Country::nigeria());
            map.insert("570", Country::niue());
            map.insert("574", Country::norfolk_island());
            map.insert("578", Country::norway());
            map.insert("512", Country::oman());
            map.insert("586", Country::pakistan());
            map.insert("585", Country::palau());
            map.insert("591", Country::panama());
            map.insert("598", Country::papua_new_guinea());
            map.insert("600", Country::paraguay());
            map.insert("604", Country::peru());
            map.insert("612", Country::pitcairn());
            map.insert("616", Country::poland());
            map.insert("620", Country::portugal());
            map.insert("630", Country::puerto_rico());
            map.insert("634", Country::qatar());
            map.insert("807", Country::republic_of_north_macedonia());
            map.insert("638", Country::reunion());
            map.insert("642", Country::romania());
            map.insert("646", Country::rwanda());
            map.insert("652", Country::saint_barthelemy());
            map.insert("659", Country::saint_kitts_and_nevis());
            map.insert("662", Country::saint_lucia());
            map.insert("666", Country::saint_pierre_and_miquelon());
            map.insert("670", Country::saint_vincent_and_the_grenadines());
            map.insert("882", Country::samoa());
            map.insert("674", Country::san_marino());
            map.insert("678", Country::sao_tome_and_principe());
            map.insert("682", Country::saudi_arabia());
            map.insert("686", Country::senegal());
            map.insert("688", Country::serbia());
            map.insert("690", Country::seychelles());
            map.insert("694", Country::sierra_leone());
            map.insert("702", Country::singapore());
            map.insert("703", Country::slovakia());
            map.insert("705", Country::slovenia());
            map.insert("090", Country::solomon_islands());
            map.insert("706", Country::somalia());
            map.insert("710", Country::south_africa());
            map.insert(
                "239",
                Country::south_georgia_and_the_south_sandwich_islands(),
            );
            map.insert("728", Country::south_sudan());
            map.insert("724", Country::spain());
            map.insert("144", Country::sri_lanka());
            map.insert("275", Country::state_of_palestine());
            map.insert("740", Country::suriname());
            map.insert("744", Country::svalbard_and_jan_mayen());
            map.insert("752", Country::sweden());
            map.insert("756", Country::switzerland());
            map.insert("760", Country::syrian_arab_republic());
            map.insert("158", Country::taiwan());
            map.insert("762", Country::tajikistan());
            map.insert("764", Country::thailand());
            map.insert("044", Country::the_bahamas());
            map.insert("136", Country::the_cayman_islands());
            map.insert("140", Country::the_central_african_republic());
            map.insert("166", Country::the_cocos_keeling_islands());
            map.insert("174", Country::the_comoros());
            map.insert("178", Country::the_congo());
            map.insert("184", Country::the_cook_islands());
            map.insert("408", Country::the_democratic_peoples_republic_of_korea());
            map.insert("180", Country::the_democratic_republic_of_the_congo());
            map.insert("214", Country::the_dominican_republic());
            map.insert("238", Country::the_falkland_islands_malvinas());
            map.insert("234", Country::the_faroe_islands());
            map.insert("260", Country::the_french_southern_territories());
            map.insert("270", Country::the_gambia());
            map.insert("336", Country::the_holy_see());
            map.insert("418", Country::the_lao_peoples_democratic_republic());
            map.insert("584", Country::the_marshall_islands());
            map.insert("528", Country::the_netherlands());
            map.insert("562", Country::the_niger());
            map.insert("580", Country::the_northern_mariana_islands());
            map.insert("608", Country::the_philippines());
            map.insert("410", Country::the_republic_of_korea());
            map.insert("498", Country::the_republic_of_moldova());
            map.insert("643", Country::the_russian_federation());
            map.insert("729", Country::the_sudan());
            map.insert("796", Country::the_turks_and_caicos_islands());
            map.insert("784", Country::the_united_arab_emirates());
            map.insert(
                "826",
                Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            );
            map.insert("581", Country::the_united_states_minor_outlying_islands());
            map.insert("840", Country::the_united_states_of_america());
            map.insert("626", Country::timor_leste());
            map.insert("768", Country::togo());
            map.insert("772", Country::tokelau());
            map.insert("776", Country::tonga());
            map.insert("780", Country::trinidad_and_tobago());
            map.insert("788", Country::tunisia());
            map.insert("792", Country::turkey());
            map.insert("795", Country::turkmenistan());
            map.insert("798", Country::tuvalu());
            map.insert("850", Country::us_virgin_islands());
            map.insert("800", Country::uganda());
            map.insert("804", Country::ukraine());
            map.insert("834", Country::united_republic_of_tanzania());
            map.insert("858", Country::uruguay());
            map.insert("860", Country::uzbekistan());
            map.insert("548", Country::vanuatu());
            map.insert("704", Country::vietnam());
            map.insert("876", Country::wallis_and_futuna());
            map.insert("732", Country::western_sahara());
            map.insert("887", Country::yemen());
            map.insert("894", Country::zambia());
            map.insert("716", Country::zimbabwe());
            map
        });
        (*CODES)
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
        static ALPHA2: LazyLock<HashMap<&'static str, Country>> = LazyLock::new(|| {
            let mut map = HashMap::with_capacity(250);
            map.insert("af", Country::afghanistan());
            map.insert("ax", Country::aland_islands());
            map.insert("al", Country::albania());
            map.insert("dz", Country::algeria());
            map.insert("as", Country::american_samoa());
            map.insert("ad", Country::andorra());
            map.insert("ao", Country::angola());
            map.insert("ai", Country::anguilla());
            map.insert("aq", Country::antarctica());
            map.insert("ag", Country::antigua_and_barbuda());
            map.insert("ar", Country::argentina());
            map.insert("am", Country::armenia());
            map.insert("aw", Country::aruba());
            map.insert("sh", Country::ascension_and_tristan_da_cunha_saint_helena());
            map.insert("au", Country::australia());
            map.insert("at", Country::austria());
            map.insert("az", Country::azerbaijan());
            map.insert("bh", Country::bahrain());
            map.insert("bd", Country::bangladesh());
            map.insert("bb", Country::barbados());
            map.insert("by", Country::belarus());
            map.insert("be", Country::belgium());
            map.insert("bz", Country::belize());
            map.insert("bj", Country::benin());
            map.insert("bm", Country::bermuda());
            map.insert("bt", Country::bhutan());
            map.insert("ve", Country::bolivarian_republic_of_venezuela());
            map.insert("bo", Country::bolivia());
            map.insert("bq", Country::bonaire());
            map.insert("ba", Country::bosnia_and_herzegovina());
            map.insert("bw", Country::botswana());
            map.insert("bv", Country::bouvet_island());
            map.insert("br", Country::brazil());
            map.insert("io", Country::british_indian_ocean_territory());
            map.insert("vg", Country::british_virgin_islands());
            map.insert("bn", Country::brunei_darussalam());
            map.insert("bg", Country::bulgaria());
            map.insert("bf", Country::burkina_faso());
            map.insert("bi", Country::burundi());
            map.insert("cv", Country::cabo_verde());
            map.insert("kh", Country::cambodia());
            map.insert("cm", Country::cameroon());
            map.insert("ca", Country::canada());
            map.insert("td", Country::chad());
            map.insert("cl", Country::chile());
            map.insert("cn", Country::china());
            map.insert("cx", Country::christmas_island());
            map.insert("co", Country::colombia());
            map.insert("cr", Country::costa_rica());
            map.insert("ci", Country::coted_ivoire());
            map.insert("hr", Country::croatia());
            map.insert("cu", Country::cuba());
            map.insert("cw", Country::curacao());
            map.insert("cy", Country::cyprus());
            map.insert("cz", Country::czechia());
            map.insert("dk", Country::denmark());
            map.insert("dj", Country::djibouti());
            map.insert("dm", Country::dominica());
            map.insert("sx", Country::dutch_part_sint_maarten());
            map.insert("ec", Country::ecuador());
            map.insert("eg", Country::egypt());
            map.insert("sv", Country::el_salvador());
            map.insert("gq", Country::equatorial_guinea());
            map.insert("er", Country::eritrea());
            map.insert("ee", Country::estonia());
            map.insert("sz", Country::eswatini());
            map.insert("et", Country::ethiopia());
            map.insert("fm", Country::federated_states_of_micronesia());
            map.insert("fj", Country::fiji());
            map.insert("fi", Country::finland());
            map.insert("fr", Country::france());
            map.insert("gf", Country::french_guiana());
            map.insert("mf", Country::french_part_saint_martin());
            map.insert("pf", Country::french_polynesia());
            map.insert("ga", Country::gabon());
            map.insert("ge", Country::georgia());
            map.insert("de", Country::germany());
            map.insert("gh", Country::ghana());
            map.insert("gi", Country::gibraltar());
            map.insert("gr", Country::greece());
            map.insert("gl", Country::greenland());
            map.insert("gd", Country::grenada());
            map.insert("gp", Country::guadeloupe());
            map.insert("gu", Country::guam());
            map.insert("gt", Country::guatemala());
            map.insert("gg", Country::guernsey());
            map.insert("gn", Country::guinea());
            map.insert("gw", Country::guinea_bissau());
            map.insert("gy", Country::guyana());
            map.insert("ht", Country::haiti());
            map.insert("hm", Country::heard_island_and_mc_donald_islands());
            map.insert("hn", Country::honduras());
            map.insert("hk", Country::hong_kong());
            map.insert("hu", Country::hungary());
            map.insert("is", Country::iceland());
            map.insert("in", Country::india());
            map.insert("id", Country::indonesia());
            map.insert("iq", Country::iraq());
            map.insert("ie", Country::ireland());
            map.insert("ir", Country::islamic_republic_of_iran());
            map.insert("im", Country::isle_of_man());
            map.insert("il", Country::israel());
            map.insert("it", Country::italy());
            map.insert("jm", Country::jamaica());
            map.insert("jp", Country::japan());
            map.insert("je", Country::jersey());
            map.insert("jo", Country::jordan());
            map.insert("kz", Country::kazakhstan());
            map.insert("ke", Country::kenya());
            map.insert("ki", Country::kiribati());
            map.insert("xk", Country::kosovo());
            map.insert("kw", Country::kuwait());
            map.insert("kg", Country::kyrgyzstan());
            map.insert("lv", Country::latvia());
            map.insert("lb", Country::lebanon());
            map.insert("ls", Country::lesotho());
            map.insert("lr", Country::liberia());
            map.insert("ly", Country::libya());
            map.insert("li", Country::liechtenstein());
            map.insert("lt", Country::lithuania());
            map.insert("lu", Country::luxembourg());
            map.insert("mo", Country::macao());
            map.insert("mg", Country::madagascar());
            map.insert("mw", Country::malawi());
            map.insert("my", Country::malaysia());
            map.insert("mv", Country::maldives());
            map.insert("ml", Country::mali());
            map.insert("mt", Country::malta());
            map.insert("mq", Country::martinique());
            map.insert("mr", Country::mauritania());
            map.insert("mu", Country::mauritius());
            map.insert("yt", Country::mayotte());
            map.insert("mx", Country::mexico());
            map.insert("mc", Country::monaco());
            map.insert("mn", Country::mongolia());
            map.insert("me", Country::montenegro());
            map.insert("ms", Country::montserrat());
            map.insert("ma", Country::morocco());
            map.insert("mz", Country::mozambique());
            map.insert("mm", Country::myanmar());
            map.insert("na", Country::namibia());
            map.insert("nr", Country::nauru());
            map.insert("np", Country::nepal());
            map.insert("nc", Country::new_caledonia());
            map.insert("nz", Country::new_zealand());
            map.insert("ni", Country::nicaragua());
            map.insert("ng", Country::nigeria());
            map.insert("nu", Country::niue());
            map.insert("nf", Country::norfolk_island());
            map.insert("no", Country::norway());
            map.insert("om", Country::oman());
            map.insert("pk", Country::pakistan());
            map.insert("pw", Country::palau());
            map.insert("pa", Country::panama());
            map.insert("pg", Country::papua_new_guinea());
            map.insert("py", Country::paraguay());
            map.insert("pe", Country::peru());
            map.insert("pn", Country::pitcairn());
            map.insert("pl", Country::poland());
            map.insert("pt", Country::portugal());
            map.insert("pr", Country::puerto_rico());
            map.insert("qa", Country::qatar());
            map.insert("mk", Country::republic_of_north_macedonia());
            map.insert("re", Country::reunion());
            map.insert("ro", Country::romania());
            map.insert("rw", Country::rwanda());
            map.insert("bl", Country::saint_barthelemy());
            map.insert("kn", Country::saint_kitts_and_nevis());
            map.insert("lc", Country::saint_lucia());
            map.insert("pm", Country::saint_pierre_and_miquelon());
            map.insert("vc", Country::saint_vincent_and_the_grenadines());
            map.insert("ws", Country::samoa());
            map.insert("sm", Country::san_marino());
            map.insert("st", Country::sao_tome_and_principe());
            map.insert("sa", Country::saudi_arabia());
            map.insert("sn", Country::senegal());
            map.insert("rs", Country::serbia());
            map.insert("sc", Country::seychelles());
            map.insert("sl", Country::sierra_leone());
            map.insert("sg", Country::singapore());
            map.insert("sk", Country::slovakia());
            map.insert("si", Country::slovenia());
            map.insert("sb", Country::solomon_islands());
            map.insert("so", Country::somalia());
            map.insert("za", Country::south_africa());
            map.insert(
                "gs",
                Country::south_georgia_and_the_south_sandwich_islands(),
            );
            map.insert("ss", Country::south_sudan());
            map.insert("es", Country::spain());
            map.insert("lk", Country::sri_lanka());
            map.insert("ps", Country::state_of_palestine());
            map.insert("sr", Country::suriname());
            map.insert("sj", Country::svalbard_and_jan_mayen());
            map.insert("se", Country::sweden());
            map.insert("ch", Country::switzerland());
            map.insert("sy", Country::syrian_arab_republic());
            map.insert("tw", Country::taiwan());
            map.insert("tj", Country::tajikistan());
            map.insert("th", Country::thailand());
            map.insert("bs", Country::the_bahamas());
            map.insert("ky", Country::the_cayman_islands());
            map.insert("cf", Country::the_central_african_republic());
            map.insert("cc", Country::the_cocos_keeling_islands());
            map.insert("km", Country::the_comoros());
            map.insert("cg", Country::the_congo());
            map.insert("ck", Country::the_cook_islands());
            map.insert("kp", Country::the_democratic_peoples_republic_of_korea());
            map.insert("cd", Country::the_democratic_republic_of_the_congo());
            map.insert("do", Country::the_dominican_republic());
            map.insert("fk", Country::the_falkland_islands_malvinas());
            map.insert("fo", Country::the_faroe_islands());
            map.insert("tf", Country::the_french_southern_territories());
            map.insert("gm", Country::the_gambia());
            map.insert("va", Country::the_holy_see());
            map.insert("la", Country::the_lao_peoples_democratic_republic());
            map.insert("mh", Country::the_marshall_islands());
            map.insert("nl", Country::the_netherlands());
            map.insert("ne", Country::the_niger());
            map.insert("mp", Country::the_northern_mariana_islands());
            map.insert("ph", Country::the_philippines());
            map.insert("kr", Country::the_republic_of_korea());
            map.insert("md", Country::the_republic_of_moldova());
            map.insert("ru", Country::the_russian_federation());
            map.insert("sd", Country::the_sudan());
            map.insert("tc", Country::the_turks_and_caicos_islands());
            map.insert("ae", Country::the_united_arab_emirates());
            map.insert(
                "gb",
                Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            );
            map.insert("um", Country::the_united_states_minor_outlying_islands());
            map.insert("us", Country::the_united_states_of_america());
            map.insert("tl", Country::timor_leste());
            map.insert("tg", Country::togo());
            map.insert("tk", Country::tokelau());
            map.insert("to", Country::tonga());
            map.insert("tt", Country::trinidad_and_tobago());
            map.insert("tn", Country::tunisia());
            map.insert("tr", Country::turkey());
            map.insert("tm", Country::turkmenistan());
            map.insert("tv", Country::tuvalu());
            map.insert("vi", Country::us_virgin_islands());
            map.insert("ug", Country::uganda());
            map.insert("ua", Country::ukraine());
            map.insert("tz", Country::united_republic_of_tanzania());
            map.insert("uy", Country::uruguay());
            map.insert("uz", Country::uzbekistan());
            map.insert("vu", Country::vanuatu());
            map.insert("vn", Country::vietnam());
            map.insert("wf", Country::wallis_and_futuna());
            map.insert("eh", Country::western_sahara());
            map.insert("ye", Country::yemen());
            map.insert("zm", Country::zambia());
            map.insert("zw", Country::zimbabwe());
            map
        });
        (*ALPHA2)
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
        static ALPHA3: LazyLock<HashMap<&'static str, Country>> = LazyLock::new(|| {
            let mut map = HashMap::with_capacity(250);
            map.insert("afg", Country::afghanistan());
            map.insert("ala", Country::aland_islands());
            map.insert("alb", Country::albania());
            map.insert("dza", Country::algeria());
            map.insert("asm", Country::american_samoa());
            map.insert("and", Country::andorra());
            map.insert("ago", Country::angola());
            map.insert("aia", Country::anguilla());
            map.insert("ata", Country::antarctica());
            map.insert("atg", Country::antigua_and_barbuda());
            map.insert("arg", Country::argentina());
            map.insert("arm", Country::armenia());
            map.insert("abw", Country::aruba());
            map.insert(
                "shn",
                Country::ascension_and_tristan_da_cunha_saint_helena(),
            );
            map.insert("aus", Country::australia());
            map.insert("aut", Country::austria());
            map.insert("aze", Country::azerbaijan());
            map.insert("bhr", Country::bahrain());
            map.insert("bgd", Country::bangladesh());
            map.insert("brb", Country::barbados());
            map.insert("blr", Country::belarus());
            map.insert("bel", Country::belgium());
            map.insert("blz", Country::belize());
            map.insert("ben", Country::benin());
            map.insert("bmu", Country::bermuda());
            map.insert("btn", Country::bhutan());
            map.insert("ven", Country::bolivarian_republic_of_venezuela());
            map.insert("bol", Country::bolivia());
            map.insert("bes", Country::bonaire());
            map.insert("bih", Country::bosnia_and_herzegovina());
            map.insert("bwa", Country::botswana());
            map.insert("bvt", Country::bouvet_island());
            map.insert("bra", Country::brazil());
            map.insert("iot", Country::british_indian_ocean_territory());
            map.insert("vgb", Country::british_virgin_islands());
            map.insert("brn", Country::brunei_darussalam());
            map.insert("bgr", Country::bulgaria());
            map.insert("bfa", Country::burkina_faso());
            map.insert("bdi", Country::burundi());
            map.insert("cpv", Country::cabo_verde());
            map.insert("khm", Country::cambodia());
            map.insert("cmr", Country::cameroon());
            map.insert("can", Country::canada());
            map.insert("tcd", Country::chad());
            map.insert("chl", Country::chile());
            map.insert("chn", Country::china());
            map.insert("cxr", Country::christmas_island());
            map.insert("col", Country::colombia());
            map.insert("cri", Country::costa_rica());
            map.insert("civ", Country::coted_ivoire());
            map.insert("hrv", Country::croatia());
            map.insert("cub", Country::cuba());
            map.insert("cuw", Country::curacao());
            map.insert("cyp", Country::cyprus());
            map.insert("cze", Country::czechia());
            map.insert("dnk", Country::denmark());
            map.insert("dji", Country::djibouti());
            map.insert("dma", Country::dominica());
            map.insert("sxm", Country::dutch_part_sint_maarten());
            map.insert("ecu", Country::ecuador());
            map.insert("egy", Country::egypt());
            map.insert("slv", Country::el_salvador());
            map.insert("gnq", Country::equatorial_guinea());
            map.insert("eri", Country::eritrea());
            map.insert("est", Country::estonia());
            map.insert("swz", Country::eswatini());
            map.insert("eth", Country::ethiopia());
            map.insert("fsm", Country::federated_states_of_micronesia());
            map.insert("fji", Country::fiji());
            map.insert("fin", Country::finland());
            map.insert("fra", Country::france());
            map.insert("guf", Country::french_guiana());
            map.insert("maf", Country::french_part_saint_martin());
            map.insert("pyf", Country::french_polynesia());
            map.insert("gab", Country::gabon());
            map.insert("geo", Country::georgia());
            map.insert("deu", Country::germany());
            map.insert("gha", Country::ghana());
            map.insert("gib", Country::gibraltar());
            map.insert("grc", Country::greece());
            map.insert("grl", Country::greenland());
            map.insert("grd", Country::grenada());
            map.insert("glp", Country::guadeloupe());
            map.insert("gum", Country::guam());
            map.insert("gtm", Country::guatemala());
            map.insert("ggy", Country::guernsey());
            map.insert("gin", Country::guinea());
            map.insert("gnb", Country::guinea_bissau());
            map.insert("guy", Country::guyana());
            map.insert("hti", Country::haiti());
            map.insert("hmd", Country::heard_island_and_mc_donald_islands());
            map.insert("hnd", Country::honduras());
            map.insert("hkg", Country::hong_kong());
            map.insert("hun", Country::hungary());
            map.insert("isl", Country::iceland());
            map.insert("ind", Country::india());
            map.insert("idn", Country::indonesia());
            map.insert("irq", Country::iraq());
            map.insert("irl", Country::ireland());
            map.insert("irn", Country::islamic_republic_of_iran());
            map.insert("imn", Country::isle_of_man());
            map.insert("isr", Country::israel());
            map.insert("ita", Country::italy());
            map.insert("jam", Country::jamaica());
            map.insert("jpn", Country::japan());
            map.insert("jey", Country::jersey());
            map.insert("jor", Country::jordan());
            map.insert("kaz", Country::kazakhstan());
            map.insert("ken", Country::kenya());
            map.insert("xkx", Country::kosovo());
            map.insert("kir", Country::kiribati());
            map.insert("kwt", Country::kuwait());
            map.insert("kgz", Country::kyrgyzstan());
            map.insert("lva", Country::latvia());
            map.insert("lbn", Country::lebanon());
            map.insert("lso", Country::lesotho());
            map.insert("lbr", Country::liberia());
            map.insert("lby", Country::libya());
            map.insert("lie", Country::liechtenstein());
            map.insert("ltu", Country::lithuania());
            map.insert("lux", Country::luxembourg());
            map.insert("mac", Country::macao());
            map.insert("mdg", Country::madagascar());
            map.insert("mwi", Country::malawi());
            map.insert("mys", Country::malaysia());
            map.insert("mdv", Country::maldives());
            map.insert("mli", Country::mali());
            map.insert("mlt", Country::malta());
            map.insert("mtq", Country::martinique());
            map.insert("mrt", Country::mauritania());
            map.insert("mus", Country::mauritius());
            map.insert("myt", Country::mayotte());
            map.insert("mex", Country::mexico());
            map.insert("mco", Country::monaco());
            map.insert("mng", Country::mongolia());
            map.insert("mne", Country::montenegro());
            map.insert("msr", Country::montserrat());
            map.insert("mar", Country::morocco());
            map.insert("moz", Country::mozambique());
            map.insert("mmr", Country::myanmar());
            map.insert("nam", Country::namibia());
            map.insert("nru", Country::nauru());
            map.insert("npl", Country::nepal());
            map.insert("ncl", Country::new_caledonia());
            map.insert("nzl", Country::new_zealand());
            map.insert("nic", Country::nicaragua());
            map.insert("nga", Country::nigeria());
            map.insert("niu", Country::niue());
            map.insert("nfk", Country::norfolk_island());
            map.insert("nor", Country::norway());
            map.insert("omn", Country::oman());
            map.insert("pak", Country::pakistan());
            map.insert("plw", Country::palau());
            map.insert("pan", Country::panama());
            map.insert("png", Country::papua_new_guinea());
            map.insert("pry", Country::paraguay());
            map.insert("per", Country::peru());
            map.insert("pcn", Country::pitcairn());
            map.insert("pol", Country::poland());
            map.insert("prt", Country::portugal());
            map.insert("pri", Country::puerto_rico());
            map.insert("qat", Country::qatar());
            map.insert("mkd", Country::republic_of_north_macedonia());
            map.insert("reu", Country::reunion());
            map.insert("rou", Country::romania());
            map.insert("rwa", Country::rwanda());
            map.insert("blm", Country::saint_barthelemy());
            map.insert("kna", Country::saint_kitts_and_nevis());
            map.insert("lca", Country::saint_lucia());
            map.insert("spm", Country::saint_pierre_and_miquelon());
            map.insert("vct", Country::saint_vincent_and_the_grenadines());
            map.insert("wsm", Country::samoa());
            map.insert("smr", Country::san_marino());
            map.insert("stp", Country::sao_tome_and_principe());
            map.insert("sau", Country::saudi_arabia());
            map.insert("sen", Country::senegal());
            map.insert("srb", Country::serbia());
            map.insert("syc", Country::seychelles());
            map.insert("sle", Country::sierra_leone());
            map.insert("sgp", Country::singapore());
            map.insert("svk", Country::slovakia());
            map.insert("svn", Country::slovenia());
            map.insert("slb", Country::solomon_islands());
            map.insert("som", Country::somalia());
            map.insert("zaf", Country::south_africa());
            map.insert(
                "sgs",
                Country::south_georgia_and_the_south_sandwich_islands(),
            );
            map.insert("ssd", Country::south_sudan());
            map.insert("esp", Country::spain());
            map.insert("lka", Country::sri_lanka());
            map.insert("pse", Country::state_of_palestine());
            map.insert("sur", Country::suriname());
            map.insert("sjm", Country::svalbard_and_jan_mayen());
            map.insert("swe", Country::sweden());
            map.insert("che", Country::switzerland());
            map.insert("syr", Country::syrian_arab_republic());
            map.insert("twn", Country::taiwan());
            map.insert("tjk", Country::tajikistan());
            map.insert("tha", Country::thailand());
            map.insert("bhs", Country::the_bahamas());
            map.insert("cym", Country::the_cayman_islands());
            map.insert("caf", Country::the_central_african_republic());
            map.insert("cck", Country::the_cocos_keeling_islands());
            map.insert("com", Country::the_comoros());
            map.insert("cog", Country::the_congo());
            map.insert("cok", Country::the_cook_islands());
            map.insert("prk", Country::the_democratic_peoples_republic_of_korea());
            map.insert("cod", Country::the_democratic_republic_of_the_congo());
            map.insert("dom", Country::the_dominican_republic());
            map.insert("flk", Country::the_falkland_islands_malvinas());
            map.insert("fro", Country::the_faroe_islands());
            map.insert("atf", Country::the_french_southern_territories());
            map.insert("gmb", Country::the_gambia());
            map.insert("vat", Country::the_holy_see());
            map.insert("lao", Country::the_lao_peoples_democratic_republic());
            map.insert("mhl", Country::the_marshall_islands());
            map.insert("nld", Country::the_netherlands());
            map.insert("ner", Country::the_niger());
            map.insert("mnp", Country::the_northern_mariana_islands());
            map.insert("phl", Country::the_philippines());
            map.insert("kor", Country::the_republic_of_korea());
            map.insert("mda", Country::the_republic_of_moldova());
            map.insert("rus", Country::the_russian_federation());
            map.insert("sdn", Country::the_sudan());
            map.insert("tca", Country::the_turks_and_caicos_islands());
            map.insert("are", Country::the_united_arab_emirates());
            map.insert(
                "gbr",
                Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            );
            map.insert("umi", Country::the_united_states_minor_outlying_islands());
            map.insert("usa", Country::the_united_states_of_america());
            map.insert("tls", Country::timor_leste());
            map.insert("tgo", Country::togo());
            map.insert("tkl", Country::tokelau());
            map.insert("ton", Country::tonga());
            map.insert("tto", Country::trinidad_and_tobago());
            map.insert("tun", Country::tunisia());
            map.insert("tur", Country::turkey());
            map.insert("tkm", Country::turkmenistan());
            map.insert("tuv", Country::tuvalu());
            map.insert("vir", Country::us_virgin_islands());
            map.insert("uga", Country::uganda());
            map.insert("ukr", Country::ukraine());
            map.insert("tza", Country::united_republic_of_tanzania());
            map.insert("ury", Country::uruguay());
            map.insert("uzb", Country::uzbekistan());
            map.insert("vut", Country::vanuatu());
            map.insert("vnm", Country::vietnam());
            map.insert("wlf", Country::wallis_and_futuna());
            map.insert("esh", Country::western_sahara());
            map.insert("yem", Country::yemen());
            map.insert("zmb", Country::zambia());
            map.insert("zwe", Country::zimbabwe());
            map
        });
        (*ALPHA3)
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
        static ALIASES: LazyLock<HashMap<&'static str, Country>> = LazyLock::new(|| {
            let mut map = HashMap::with_capacity(256);
            map.insert("samoa", Country::american_samoa());
            for s in ["sthelena", "sainthelena"] {
                map.insert(s, Country::ascension_and_tristan_da_cunha_saint_helena());
            }
            map.insert("venezuela", Country::bolivarian_republic_of_venezuela());
            for s in ["bosnia", "herzegovina"] {
                map.insert(s, Country::bosnia_and_herzegovina());
            }
            map.insert("brunei", Country::brunei_darussalam());
            map.insert("burkina", Country::burkina_faso());
            for s in ["stmaarten", "saintmaarten"] {
                map.insert(s, Country::dutch_part_sint_maarten());
            }
            map.insert("micronesia", Country::federated_states_of_micronesia());
            for s in ["stmartin", "saintmartin"] {
                map.insert(s, Country::french_part_saint_martin());
            }
            for s in ["heardisland", "mcdonaldislands"] {
                map.insert(s, Country::heard_island_and_mc_donald_islands());
            }
            map.insert("iran", Country::islamic_republic_of_iran());
            map.insert("macedonia", Country::republic_of_north_macedonia());
            map.insert("stbarthelemy", Country::saint_barthelemy());
            map.insert("stkitts", Country::saint_kitts_and_nevis());
            map.insert("stlucia", Country::saint_lucia());
            for s in ["stpierre", "saintpierre"] {
                map.insert(s, Country::saint_pierre_and_miquelon());
            }
            for s in ["stvincent", "saintvincent"] {
                map.insert(s, Country::saint_vincent_and_the_grenadines());
            }
            map.insert("saotome", Country::sao_tome_and_principe());
            for s in ["southgeorgia", "southsandwichislands"] {
                map.insert(s, Country::south_georgia_and_the_south_sandwich_islands());
            }
            map.insert("palestine", Country::state_of_palestine());
            map.insert("taiwan", Country::taiwan());
            map.insert("bahamas", Country::the_bahamas());
            map.insert("caymanislands", Country::the_cayman_islands());
            map.insert(
                "centralafricanrepublic",
                Country::the_central_african_republic(),
            );
            for s in ["cocosislands", "keelingislands"] {
                map.insert(s, Country::the_cocos_keeling_islands());
            }
            map.insert("comoros", Country::the_comoros());
            map.insert("congo", Country::the_congo());
            map.insert("cookislands", Country::the_cook_islands());
            map.insert("czechrepublic", Country::czechia());
            for s in ["northkorea", "democraticpeoplesrepublicofkorea"] {
                map.insert(s, Country::the_democratic_peoples_republic_of_korea());
            }
            map.insert(
                "democraticrepublicofthecongo",
                Country::the_democratic_republic_of_the_congo(),
            );
            map.insert("dominicanrepublic", Country::the_dominican_republic());
            map.insert("easttimor", Country::timor_leste());
            for s in ["malvinas", "falklandislands"] {
                map.insert(s, Country::the_falkland_islands_malvinas());
            }
            map.insert("faroeislands", Country::the_faroe_islands());
            map.insert(
                "frenchsouthernterritories",
                Country::the_french_southern_territories(),
            );
            map.insert("gambia", Country::the_gambia());
            map.insert("holysee", Country::the_holy_see());
            map.insert(
                "laopeoplesdemocraticrepublic",
                Country::the_lao_peoples_democratic_republic(),
            );
            map.insert("marshallislands", Country::the_marshall_islands());
            for s in ["netherlands", "holland"] {
                map.insert(s, Country::the_netherlands());
            }
            map.insert("niger", Country::the_niger());
            map.insert(
                "northernmarianaislands",
                Country::the_northern_mariana_islands(),
            );
            map.insert("philippines", Country::the_philippines());
            for s in ["southkorea", "republicofkorea"] {
                map.insert(s, Country::the_republic_of_korea());
            }
            for s in ["moldova", "republicofmoldova"] {
                map.insert(s, Country::the_republic_of_moldova());
            }
            for s in ["russia", "russianfederation"] {
                map.insert(s, Country::the_russian_federation());
            }
            map.insert("sudan", Country::the_sudan());
            map.insert(
                "turksandcaicosislands",
                Country::the_turks_and_caicos_islands(),
            );
            map.insert("unitedarabemirates", Country::the_united_arab_emirates());
            for s in [
                "england",
                "scotland",
                "greatbritain",
                "unitedkingdom",
                "northernireland",
                "unitedkingdomofgreatbritain",
                "unitedkingdomofgreatbritainandnorthernireland",
            ] {
                map.insert(
                    s,
                    Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
                );
            }
            map.insert(
                "unitedstatesminoroutlyingislands",
                Country::the_united_states_minor_outlying_islands(),
            );
            for s in ["america", "unitedstates", "unitedstatesofamerica"] {
                map.insert(s, Country::the_united_states_of_america());
            }
            for s in ["trinidad", "tobago"] {
                map.insert(s, Country::trinidad_and_tobago());
            }
            map.insert("tanzania", Country::united_republic_of_tanzania());
            map.insert("türkiye", Country::turkey());
            map.insert("turkey", Country::turkey());
            map
        });
        (*ALIASES)
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
        static NAMES: LazyLock<HashMap<&'static str, Country>> = LazyLock::new(|| {
            let mut map = HashMap::with_capacity(250);
            map.insert("afghanistan", Country::afghanistan());
            map.insert("alandislands", Country::aland_islands());
            map.insert("albania", Country::albania());
            map.insert("algeria", Country::algeria());
            map.insert("americansamoa", Country::american_samoa());
            map.insert("andorra", Country::andorra());
            map.insert("angola", Country::angola());
            map.insert("anguilla", Country::anguilla());
            map.insert("antarctica", Country::antarctica());
            map.insert("antiguaandbarbuda", Country::antigua_and_barbuda());
            map.insert("argentina", Country::argentina());
            map.insert("armenia", Country::armenia());
            map.insert("aruba", Country::aruba());
            map.insert(
                "ascensionandtristandacunhasainthelena",
                Country::ascension_and_tristan_da_cunha_saint_helena(),
            );
            map.insert("australia", Country::australia());
            map.insert("austria", Country::austria());
            map.insert("azerbaijan", Country::azerbaijan());
            map.insert("bahrain", Country::bahrain());
            map.insert("bangladesh", Country::bangladesh());
            map.insert("barbados", Country::barbados());
            map.insert("belarus", Country::belarus());
            map.insert("belgium", Country::belgium());
            map.insert("belize", Country::belize());
            map.insert("benin", Country::benin());
            map.insert("bermuda", Country::bermuda());
            map.insert("bhutan", Country::bhutan());
            map.insert(
                "bolivarianrepublicofvenezuela",
                Country::bolivarian_republic_of_venezuela(),
            );
            map.insert("bolivia", Country::bolivia());
            map.insert("bonaire", Country::bonaire());
            map.insert("bosniaandherzegovina", Country::bosnia_and_herzegovina());
            map.insert("botswana", Country::botswana());
            map.insert("bouvetisland", Country::bouvet_island());
            map.insert("brazil", Country::brazil());
            map.insert(
                "britishindianoceanterritory",
                Country::british_indian_ocean_territory(),
            );
            map.insert("britishvirginislands", Country::british_virgin_islands());
            map.insert("bruneidarussalam", Country::brunei_darussalam());
            map.insert("bulgaria", Country::bulgaria());
            map.insert("burkinafaso", Country::burkina_faso());
            map.insert("burundi", Country::burundi());
            map.insert("caboverde", Country::cabo_verde());
            map.insert("cambodia", Country::cambodia());
            map.insert("cameroon", Country::cameroon());
            map.insert("canada", Country::canada());
            map.insert("chad", Country::chad());
            map.insert("chile", Country::chile());
            map.insert("china", Country::china());
            map.insert("christmasisland", Country::christmas_island());
            map.insert("colombia", Country::colombia());
            map.insert("costarica", Country::costa_rica());
            map.insert("cotedivoire", Country::coted_ivoire());
            map.insert("croatia", Country::croatia());
            map.insert("cuba", Country::cuba());
            map.insert("curacao", Country::curacao());
            map.insert("cyprus", Country::cyprus());
            map.insert("czechia", Country::czechia());
            map.insert("denmark", Country::denmark());
            map.insert("djibouti", Country::djibouti());
            map.insert("dominica", Country::dominica());
            map.insert("dutchpartsintmaarten", Country::dutch_part_sint_maarten());
            map.insert("ecuador", Country::ecuador());
            map.insert("egypt", Country::egypt());
            map.insert("elsalvador", Country::el_salvador());
            map.insert("equatorialguinea", Country::equatorial_guinea());
            map.insert("eritrea", Country::eritrea());
            map.insert("estonia", Country::estonia());
            map.insert("eswatini", Country::eswatini());
            map.insert("ethiopia", Country::ethiopia());
            map.insert(
                "federatedstatesofmicronesia",
                Country::federated_states_of_micronesia(),
            );
            map.insert("fiji", Country::fiji());
            map.insert("finland", Country::finland());
            map.insert("france", Country::france());
            map.insert("frenchguiana", Country::french_guiana());
            map.insert("frenchpartsaintmartin", Country::french_part_saint_martin());
            map.insert("frenchpolynesia", Country::french_polynesia());
            map.insert("gabon", Country::gabon());
            map.insert("georgia", Country::georgia());
            map.insert("germany", Country::germany());
            map.insert("ghana", Country::ghana());
            map.insert("gibraltar", Country::gibraltar());
            map.insert("greece", Country::greece());
            map.insert("greenland", Country::greenland());
            map.insert("grenada", Country::grenada());
            map.insert("guadeloupe", Country::guadeloupe());
            map.insert("guam", Country::guam());
            map.insert("guatemala", Country::guatemala());
            map.insert("guernsey", Country::guernsey());
            map.insert("guinea", Country::guinea());
            map.insert("guineabissau", Country::guinea_bissau());
            map.insert("guyana", Country::guyana());
            map.insert("haiti", Country::haiti());
            map.insert(
                "heardislandandmcdonaldislands",
                Country::heard_island_and_mc_donald_islands(),
            );
            map.insert("honduras", Country::honduras());
            map.insert("hongkong", Country::hong_kong());
            map.insert("hungary", Country::hungary());
            map.insert("iceland", Country::iceland());
            map.insert("india", Country::india());
            map.insert("indonesia", Country::indonesia());
            map.insert("iraq", Country::iraq());
            map.insert("ireland", Country::ireland());
            map.insert("islamicrepublicofiran", Country::islamic_republic_of_iran());
            map.insert("isleofman", Country::isle_of_man());
            map.insert("israel", Country::israel());
            map.insert("italy", Country::italy());
            map.insert("jamaica", Country::jamaica());
            map.insert("japan", Country::japan());
            map.insert("jersey", Country::jersey());
            map.insert("jordan", Country::jordan());
            map.insert("kazakhstan", Country::kazakhstan());
            map.insert("kenya", Country::kenya());
            map.insert("kiribati", Country::kiribati());
            map.insert("kosovo", Country::kosovo());
            map.insert("kuwait", Country::kuwait());
            map.insert("kyrgyzstan", Country::kyrgyzstan());
            map.insert("latvia", Country::latvia());
            map.insert("lebanon", Country::lebanon());
            map.insert("lesotho", Country::lesotho());
            map.insert("liberia", Country::liberia());
            map.insert("libya", Country::libya());
            map.insert("liechtenstein", Country::liechtenstein());
            map.insert("lithuania", Country::lithuania());
            map.insert("luxembourg", Country::luxembourg());
            map.insert("macao", Country::macao());
            map.insert("madagascar", Country::madagascar());
            map.insert("malawi", Country::malawi());
            map.insert("malaysia", Country::malaysia());
            map.insert("maldives", Country::maldives());
            map.insert("mali", Country::mali());
            map.insert("malta", Country::malta());
            map.insert("martinique", Country::martinique());
            map.insert("mauritania", Country::mauritania());
            map.insert("mauritius", Country::mauritius());
            map.insert("mayotte", Country::mayotte());
            map.insert("mexico", Country::mexico());
            map.insert("monaco", Country::monaco());
            map.insert("mongolia", Country::mongolia());
            map.insert("montenegro", Country::montenegro());
            map.insert("montserrat", Country::montserrat());
            map.insert("morocco", Country::morocco());
            map.insert("mozambique", Country::mozambique());
            map.insert("myanmar", Country::myanmar());
            map.insert("namibia", Country::namibia());
            map.insert("nauru", Country::nauru());
            map.insert("nepal", Country::nepal());
            map.insert("newcaledonia", Country::new_caledonia());
            map.insert("newzealand", Country::new_zealand());
            map.insert("nicaragua", Country::nicaragua());
            map.insert("nigeria", Country::nigeria());
            map.insert("niue", Country::niue());
            map.insert("norfolkisland", Country::norfolk_island());
            map.insert("norway", Country::norway());
            map.insert("oman", Country::oman());
            map.insert("pakistan", Country::pakistan());
            map.insert("palau", Country::palau());
            map.insert("panama", Country::panama());
            map.insert("papuanewguinea", Country::papua_new_guinea());
            map.insert("paraguay", Country::paraguay());
            map.insert("peru", Country::peru());
            map.insert("pitcairn", Country::pitcairn());
            map.insert("poland", Country::poland());
            map.insert("portugal", Country::portugal());
            map.insert("puertorico", Country::puerto_rico());
            map.insert("qatar", Country::qatar());
            map.insert(
                "republicofnorthmacedonia",
                Country::republic_of_north_macedonia(),
            );
            map.insert("reunion", Country::reunion());
            map.insert("romania", Country::romania());
            map.insert("rwanda", Country::rwanda());
            map.insert("saintbarthelemy", Country::saint_barthelemy());
            map.insert("saintkittsandnevis", Country::saint_kitts_and_nevis());
            map.insert("saintlucia", Country::saint_lucia());
            map.insert(
                "saintpierreandmiquelon",
                Country::saint_pierre_and_miquelon(),
            );
            map.insert(
                "saintvincentandthegrenadines",
                Country::saint_vincent_and_the_grenadines(),
            );
            map.insert("samoa", Country::samoa());
            map.insert("sanmarino", Country::san_marino());
            map.insert("saotomeandprincipe", Country::sao_tome_and_principe());
            map.insert("saudiarabia", Country::saudi_arabia());
            map.insert("senegal", Country::senegal());
            map.insert("serbia", Country::serbia());
            map.insert("seychelles", Country::seychelles());
            map.insert("sierraleone", Country::sierra_leone());
            map.insert("singapore", Country::singapore());
            map.insert("slovakia", Country::slovakia());
            map.insert("slovenia", Country::slovenia());
            map.insert("solomonislands", Country::solomon_islands());
            map.insert("somalia", Country::somalia());
            map.insert("southafrica", Country::south_africa());
            map.insert(
                "southgeorgiaandthesouthsandwichislands",
                Country::south_georgia_and_the_south_sandwich_islands(),
            );
            map.insert("southsudan", Country::south_sudan());
            map.insert("spain", Country::spain());
            map.insert("srilanka", Country::sri_lanka());
            map.insert("stateofpalestine", Country::state_of_palestine());
            map.insert("suriname", Country::suriname());
            map.insert("svalbardandjanmayen", Country::svalbard_and_jan_mayen());
            map.insert("sweden", Country::sweden());
            map.insert("switzerland", Country::switzerland());
            map.insert("syrianarabrepublic", Country::syrian_arab_republic());
            map.insert("taiwan,republicofchina", Country::taiwan());
            map.insert("tajikistan", Country::tajikistan());
            map.insert("thailand", Country::thailand());
            map.insert("thebahamas", Country::the_bahamas());
            map.insert("thecaymanislands", Country::the_cayman_islands());
            map.insert(
                "thecentralafricanrepublic",
                Country::the_central_african_republic(),
            );
            map.insert(
                "thecocoskeelingislands",
                Country::the_cocos_keeling_islands(),
            );
            map.insert("thecomoros", Country::the_comoros());
            map.insert("thecongo", Country::the_congo());
            map.insert("thecookislands", Country::the_cook_islands());
            map.insert(
                "thedemocraticpeoplesrepublicofkorea",
                Country::the_democratic_peoples_republic_of_korea(),
            );
            map.insert(
                "thedemocraticrepublicofthecongo",
                Country::the_democratic_republic_of_the_congo(),
            );
            map.insert("thedominicanrepublic", Country::the_dominican_republic());
            map.insert(
                "thefalklandislandsmalvinas",
                Country::the_falkland_islands_malvinas(),
            );
            map.insert("thefaroeislands", Country::the_faroe_islands());
            map.insert(
                "thefrenchsouthernterritories",
                Country::the_french_southern_territories(),
            );
            map.insert("thegambia", Country::the_gambia());
            map.insert("theholysee", Country::the_holy_see());
            map.insert(
                "thelaopeoplesdemocraticrepublic",
                Country::the_lao_peoples_democratic_republic(),
            );
            map.insert("themarshallislands", Country::the_marshall_islands());
            map.insert("thenetherlands", Country::the_netherlands());
            map.insert("theniger", Country::the_niger());
            map.insert(
                "thenorthernmarianaislands",
                Country::the_northern_mariana_islands(),
            );
            map.insert("thephilippines", Country::the_philippines());
            map.insert("therepublicofkorea", Country::the_republic_of_korea());
            map.insert("therepublicofmoldova", Country::the_republic_of_moldova());
            map.insert("therussianfederation", Country::the_russian_federation());
            map.insert("thesudan", Country::the_sudan());
            map.insert(
                "theturksandcaicosislands",
                Country::the_turks_and_caicos_islands(),
            );
            map.insert("theunitedarabemirates", Country::the_united_arab_emirates());
            map.insert(
                "theunitedkingdomofgreatbritainandnorthernireland",
                Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
            );
            map.insert(
                "theunitedstatesminoroutlyingislands",
                Country::the_united_states_minor_outlying_islands(),
            );
            map.insert(
                "theunitedstatesofamerica",
                Country::the_united_states_of_america(),
            );
            map.insert("timorleste", Country::timor_leste());
            map.insert("easttimor", Country::timor_leste());
            map.insert("togo", Country::togo());
            map.insert("tokelau", Country::tokelau());
            map.insert("tonga", Country::tonga());
            map.insert("trinidadandtobago", Country::trinidad_and_tobago());
            map.insert("tunisia", Country::tunisia());
            map.insert("turkey", Country::turkey());
            map.insert("türkiye", Country::turkey());
            map.insert("turkmenistan", Country::turkmenistan());
            map.insert("tuvalu", Country::tuvalu());
            map.insert("usvirginislands", Country::us_virgin_islands());
            map.insert("uganda", Country::uganda());
            map.insert("ukraine", Country::ukraine());
            map.insert(
                "unitedrepublicoftanzania",
                Country::united_republic_of_tanzania(),
            );
            map.insert("uruguay", Country::uruguay());
            map.insert("uzbekistan", Country::uzbekistan());
            map.insert("vanuatu", Country::vanuatu());
            map.insert("vietnam", Country::vietnam());
            map.insert("wallisandfutuna", Country::wallis_and_futuna());
            map.insert("westernsahara", Country::western_sahara());
            map.insert("yemen", Country::yemen());
            map.insert("zambia", Country::zambia());
            map.insert("zimbabwe", Country::zimbabwe());
            map
        });
        (*NAMES)
            .get(name.as_ref().to_lowercase().as_str())
            .copied()
            .ok_or("unknown value")
    }
}

impl FromStr for Country {
    type Err = &'static str;

    fn from_str(code: &str) -> Result<Self, &'static str> {
        static CODES: LazyLock<HashMap<&'static str, Country>> = LazyLock::new(|| {
            let mut map = HashMap::with_capacity(20000);
            for s in ["afghanistan", "004", "af", "afg"] {
                map.insert(s, Country::afghanistan());
            }
            for s in ["alandislands", "aland_islands", "248", "ax", "ala"] {
                map.insert(s, Country::aland_islands());
            }
            for s in ["albania", "008", "al", "alb"] {
                map.insert(s, Country::albania());
            }
            for s in ["algeria", "012", "dz", "dza"] {
                map.insert(s, Country::algeria());
            }
            for s in ["americansamoa", "american_samoa", "016", "as", "asm"] {
                map.insert(s, Country::american_samoa());
            }
            for s in ["andorra", "020", "ad", "and"] {
                map.insert(s, Country::andorra());
            }
            for s in ["angola", "024", "ao", "ago"] {
                map.insert(s, Country::angola());
            }
            for s in ["anguilla", "660", "ai", "aia"] {
                map.insert(s, Country::anguilla());
            }
            for s in ["antarctica", "010", "aq", "ata"] {
                map.insert(s, Country::antarctica());
            }
            for s in [
                "antiguaandbarbuda",
                "antigua_and_barbuda",
                "028",
                "ag",
                "atg",
            ] {
                map.insert(s, Country::antigua_and_barbuda());
            }
            for s in ["argentina", "032", "ar", "arg"] {
                map.insert(s, Country::argentina());
            }
            for s in ["armenia", "051", "am", "arm"] {
                map.insert(s, Country::armenia());
            }
            for s in ["aruba", "533", "aw", "abw"] {
                map.insert(s, Country::aruba());
            }
            for s in [
                "ascensionandtristandacunhasainthelena",
                "ascension_and_tristan_da_cunha_saint_helena",
                "654",
                "sh",
                "shn",
                "sthelena",
                "sainthelena",
            ] {
                map.insert(s, Country::ascension_and_tristan_da_cunha_saint_helena());
            }
            for s in ["australia", "036", "au", "aus"] {
                map.insert(s, Country::australia());
            }
            for s in ["austria", "040", "at", "aut"] {
                map.insert(s, Country::austria());
            }
            for s in ["azerbaijan", "031", "az", "aze"] {
                map.insert(s, Country::azerbaijan());
            }
            for s in ["bahrain", "048", "bh", "bhr"] {
                map.insert(s, Country::bahrain());
            }
            for s in ["bangladesh", "050", "bd", "bgd"] {
                map.insert(s, Country::bangladesh());
            }
            for s in ["barbados", "052", "bb", "brb"] {
                map.insert(s, Country::barbados());
            }
            for s in ["belarus", "112", "by", "blr"] {
                map.insert(s, Country::belarus());
            }
            for s in ["belgium", "056", "be", "bel"] {
                map.insert(s, Country::belgium());
            }
            for s in ["belize", "084", "bz", "blz"] {
                map.insert(s, Country::belize());
            }
            for s in ["benin", "204", "bj", "ben"] {
                map.insert(s, Country::benin());
            }
            for s in ["bermuda", "060", "bm", "bmu"] {
                map.insert(s, Country::bermuda());
            }
            for s in ["bhutan", "064", "bt", "btn"] {
                map.insert(s, Country::bhutan());
            }
            for s in [
                "bolivarianrepublicofvenezuela",
                "bolivarian_republic_of_venezuela",
                "862",
                "ve",
                "ven",
                "venezuela",
            ] {
                map.insert(s, Country::bolivarian_republic_of_venezuela());
            }
            for s in ["bolivia", "068", "bo", "bol"] {
                map.insert(s, Country::bolivia());
            }
            for s in ["bonaire", "535", "bq", "bes"] {
                map.insert(s, Country::bonaire());
            }
            for s in [
                "bosniaandherzegovina",
                "bosnia_and_herzegovina",
                "070",
                "ba",
                "bih",
                "bosnia",
                "herzegovina",
            ] {
                map.insert(s, Country::bosnia_and_herzegovina());
            }
            for s in ["botswana", "072", "bw", "bwa"] {
                map.insert(s, Country::botswana());
            }
            for s in ["bouvetisland", "bouvet_island", "074", "bv", "bvt"] {
                map.insert(s, Country::bouvet_island());
            }
            for s in ["brazil", "076", "br", "bra"] {
                map.insert(s, Country::brazil());
            }
            for s in [
                "britishindianoceanterritory",
                "british_indian_ocean_territory",
                "086",
                "io",
                "iot",
            ] {
                map.insert(s, Country::british_indian_ocean_territory());
            }
            for s in [
                "britishvirginislands",
                "british_virgin_islands",
                "092",
                "vg",
                "vgb",
            ] {
                map.insert(s, Country::british_virgin_islands());
            }
            for s in [
                "bruneidarussalam",
                "brunei_darussalam",
                "096",
                "bn",
                "brn",
                "brunei",
            ] {
                map.insert(s, Country::brunei_darussalam());
            }
            for s in ["bulgaria", "100", "bg", "bgr"] {
                map.insert(s, Country::bulgaria());
            }
            for s in ["burkinafaso", "burkina_faso", "854", "bf", "bfa", "burkina"] {
                map.insert(s, Country::burkina_faso());
            }
            for s in ["burundi", "108", "bi", "bdi"] {
                map.insert(s, Country::burundi());
            }
            for s in ["caboverde", "cabo_verde", "132", "cv", "cpv"] {
                map.insert(s, Country::cabo_verde());
            }
            for s in ["cambodia", "116", "kh", "khm"] {
                map.insert(s, Country::cambodia());
            }
            for s in ["cameroon", "120", "cm", "cmr"] {
                map.insert(s, Country::cameroon());
            }
            for s in ["canada", "124", "ca", "can"] {
                map.insert(s, Country::canada());
            }
            for s in ["chad", "148", "td", "tcd"] {
                map.insert(s, Country::chad());
            }
            for s in ["chile", "152", "cl", "chl"] {
                map.insert(s, Country::chile());
            }
            for s in ["china", "156", "cn", "chn"] {
                map.insert(s, Country::china());
            }
            for s in ["christmasisland", "christmas_island", "162", "cx", "cxr"] {
                map.insert(s, Country::christmas_island());
            }
            for s in ["colombia", "170", "co", "col"] {
                map.insert(s, Country::colombia());
            }
            for s in ["costarica", "costa_rica", "188", "cr", "cri"] {
                map.insert(s, Country::costa_rica());
            }
            for s in ["cotedivoire", "coted_ivoire", "384", "ci", "civ"] {
                map.insert(s, Country::coted_ivoire());
            }
            for s in ["croatia", "191", "hr", "hrv"] {
                map.insert(s, Country::croatia());
            }
            for s in ["cuba", "192", "cu", "cub"] {
                map.insert(s, Country::cuba());
            }
            for s in ["curacao", "531", "cw", "cuw"] {
                map.insert(s, Country::curacao());
            }
            for s in ["cyprus", "196", "cy", "cyp"] {
                map.insert(s, Country::cyprus());
            }
            for s in ["czechia", "czechrepublic", "203", "cz", "cze"] {
                map.insert(s, Country::czechia());
            }
            for s in ["denmark", "208", "dk", "dnk"] {
                map.insert(s, Country::denmark());
            }
            for s in ["djibouti", "262", "dj", "dji"] {
                map.insert(s, Country::djibouti());
            }
            for s in ["dominica", "212", "dm", "dma"] {
                map.insert(s, Country::dominica());
            }
            for s in [
                "dutchpartsintmaarten",
                "dutch_part_sint_maarten",
                "534",
                "sx",
                "sxm",
                "stmaarten",
                "sintmaarten",
            ] {
                map.insert(s, Country::dutch_part_sint_maarten());
            }
            for s in ["ecuador", "218", "ec", "ecu"] {
                map.insert(s, Country::ecuador());
            }
            for s in ["egypt", "818", "eg", "egy"] {
                map.insert(s, Country::egypt());
            }
            for s in ["elsalvador", "el_salvador", "222", "sv", "slv"] {
                map.insert(s, Country::el_salvador());
            }
            for s in ["equatorialguinea", "equatorial_guinea", "226", "gq", "gnq"] {
                map.insert(s, Country::equatorial_guinea());
            }
            for s in ["eritrea", "232", "er", "eri"] {
                map.insert(s, Country::eritrea());
            }
            for s in ["estonia", "233", "ee", "est"] {
                map.insert(s, Country::estonia());
            }
            for s in ["eswatini", "748", "sz", "swz"] {
                map.insert(s, Country::eswatini());
            }
            for s in ["ethiopia", "231", "et", "eth"] {
                map.insert(s, Country::ethiopia());
            }
            for s in [
                "federatedstatesofmicronesia",
                "federated_states_of_micronesia",
                "583",
                "fm",
                "fsm",
                "micronesia",
            ] {
                map.insert(s, Country::federated_states_of_micronesia());
            }
            for s in ["fiji", "242", "fj", "fji"] {
                map.insert(s, Country::fiji());
            }
            for s in ["finland", "246", "fi", "fin"] {
                map.insert(s, Country::finland());
            }
            for s in ["france", "250", "fr", "fra"] {
                map.insert(s, Country::france());
            }
            for s in ["frenchguiana", "french_guiana", "254", "gf", "guf"] {
                map.insert(s, Country::french_guiana());
            }
            for s in [
                "frenchpartsaintmartin",
                "french_part_saint_martin",
                "663",
                "mf",
                "maf",
                "stmartin",
                "saintmartin",
            ] {
                map.insert(s, Country::french_part_saint_martin());
            }
            for s in ["frenchpolynesia", "258", "pf", "pyf"] {
                map.insert(s, Country::french_polynesia());
            }
            for s in ["gabon", "266", "ga", "gab"] {
                map.insert(s, Country::gabon());
            }
            for s in ["georgia", "268", "ge", "geo"] {
                map.insert(s, Country::georgia());
            }
            for s in ["germany", "276", "de", "deu"] {
                map.insert(s, Country::germany());
            }
            for s in ["ghana", "288", "gh", "gha"] {
                map.insert(s, Country::ghana());
            }
            for s in ["gibraltar", "292", "gi", "gib"] {
                map.insert(s, Country::gibraltar());
            }
            for s in ["greece", "300", "gr", "grc"] {
                map.insert(s, Country::greece());
            }
            for s in ["greenland", "304", "gl", "grl"] {
                map.insert(s, Country::greenland());
            }
            for s in ["grenada", "308", "gd", "grd"] {
                map.insert(s, Country::grenada());
            }
            for s in ["guadeloupe", "312", "gp", "glp"] {
                map.insert(s, Country::guadeloupe());
            }
            for s in ["guam", "316", "gu", "gum"] {
                map.insert(s, Country::guam());
            }
            for s in ["guatemala", "320", "gt", "gtm"] {
                map.insert(s, Country::guatemala());
            }
            for s in ["guernsey", "831", "gg", "ggy"] {
                map.insert(s, Country::guernsey());
            }
            for s in ["guinea", "324", "gn", "gin"] {
                map.insert(s, Country::guinea());
            }
            for s in ["guineabissau", "guinea_bissau", "624", "gw", "gnb"] {
                map.insert(s, Country::guinea_bissau());
            }
            for s in ["guyana", "328", "gy", "guy"] {
                map.insert(s, Country::guyana());
            }
            for s in ["haiti", "332", "ht", "hti"] {
                map.insert(s, Country::haiti());
            }
            for s in [
                "heardislandandmcdonaldislands",
                "heard_island_and_mc_donald_islands",
                "334",
                "hm",
                "hmd",
                "heardisland",
                "mcdonaldislands",
            ] {
                map.insert(s, Country::heard_island_and_mc_donald_islands());
            }
            for s in ["honduras", "340", "hn", "hnd"] {
                map.insert(s, Country::honduras());
            }
            for s in ["hongkong", "hong_kong", "344", "hk", "hkg"] {
                map.insert(s, Country::hong_kong());
            }
            for s in ["hungary", "348", "hu", "hun"] {
                map.insert(s, Country::hungary());
            }
            for s in ["iceland", "352", "is", "isl"] {
                map.insert(s, Country::iceland());
            }
            for s in ["india", "356", "in", "ind"] {
                map.insert(s, Country::india());
            }
            for s in ["indonesia", "360", "id", "idn"] {
                map.insert(s, Country::indonesia());
            }
            for s in ["iraq", "368", "iq", "irq"] {
                map.insert(s, Country::iraq());
            }
            for s in ["ireland", "372", "ie", "irl"] {
                map.insert(s, Country::ireland());
            }
            for s in [
                "islamicrepublicofiran",
                "islamic_republic_of_iran",
                "364",
                "ir",
                "irn",
                "iran",
            ] {
                map.insert(s, Country::islamic_republic_of_iran());
            }
            for s in ["isleofman", "isle_of_man", "833", "im", "imn"] {
                map.insert(s, Country::isle_of_man());
            }
            for s in ["israel", "376", "il", "isr"] {
                map.insert(s, Country::israel());
            }
            for s in ["italy", "380", "it", "ita"] {
                map.insert(s, Country::italy());
            }
            for s in ["jamaica", "388", "jm", "jam"] {
                map.insert(s, Country::jamaica());
            }
            for s in ["japan", "392", "jp", "jpn"] {
                map.insert(s, Country::japan());
            }
            for s in ["jersey", "832", "je", "jey"] {
                map.insert(s, Country::jersey());
            }
            for s in ["jordan", "400", "jo", "jor"] {
                map.insert(s, Country::jordan());
            }
            for s in ["kazakhstan", "398", "kz", "kaz"] {
                map.insert(s, Country::kazakhstan());
            }
            for s in ["kenya", "404", "ke", "ken"] {
                map.insert(s, Country::kenya());
            }
            for s in ["kiribati", "296", "ki", "kir"] {
                map.insert(s, Country::kiribati());
            }
            for s in ["kosovo", "383", "xk", "xkx"] {
                map.insert(s, Country::kosovo());
            }
            for s in ["kuwait", "414", "kw", "kwt"] {
                map.insert(s, Country::kuwait());
            }
            for s in ["kyrgyzstan", "417", "kg", "kgz"] {
                map.insert(s, Country::kyrgyzstan());
            }
            for s in ["latvia", "428", "lv", "lva"] {
                map.insert(s, Country::latvia());
            }
            for s in ["lebanon", "422", "lb", "lbn"] {
                map.insert(s, Country::lebanon());
            }
            for s in ["lesotho", "426", "ls", "lso"] {
                map.insert(s, Country::lesotho());
            }
            for s in ["liberia", "430", "lr", "lbr"] {
                map.insert(s, Country::liberia());
            }
            for s in ["libya", "434", "ly", "lby"] {
                map.insert(s, Country::libya());
            }
            for s in ["liechtenstein", "438", "li", "lie"] {
                map.insert(s, Country::liechtenstein());
            }
            for s in ["lithuania", "440", "lt", "ltu"] {
                map.insert(s, Country::lithuania());
            }
            for s in ["luxembourg", "442", "lu", "lux"] {
                map.insert(s, Country::luxembourg());
            }
            for s in ["macao", "446", "mo", "mac"] {
                map.insert(s, Country::macao());
            }
            for s in ["madagascar", "450", "mg", "mdg"] {
                map.insert(s, Country::madagascar());
            }
            for s in ["malawi", "454", "mw", "mwi"] {
                map.insert(s, Country::malawi());
            }
            for s in ["malaysia", "458", "my", "mys"] {
                map.insert(s, Country::malaysia());
            }
            for s in ["maldives", "462", "mv", "mdv"] {
                map.insert(s, Country::maldives());
            }
            for s in ["mali", "466", "ml", "mli"] {
                map.insert(s, Country::mali());
            }
            for s in ["malta", "470", "mt", "mlt"] {
                map.insert(s, Country::malta());
            }
            for s in ["martinique", "474", "mq", "mtq"] {
                map.insert(s, Country::martinique());
            }
            for s in ["mauritania", "478", "mr", "mrt"] {
                map.insert(s, Country::mauritania());
            }
            for s in ["mauritius", "480", "mu", "mus"] {
                map.insert(s, Country::mauritius());
            }
            for s in ["mayotte", "175", "yt", "myt"] {
                map.insert(s, Country::mayotte());
            }
            for s in ["mexico", "484", "mx", "mex"] {
                map.insert(s, Country::mexico());
            }
            for s in ["monaco", "492", "mc", "mco"] {
                map.insert(s, Country::monaco());
            }
            for s in ["mongolia", "496", "mn", "mng"] {
                map.insert(s, Country::mongolia());
            }
            for s in ["montenegro", "499", "me", "mne"] {
                map.insert(s, Country::montenegro());
            }
            for s in ["montserrat", "500", "ms", "msr"] {
                map.insert(s, Country::montserrat());
            }
            for s in ["morocco", "504", "ma", "mar"] {
                map.insert(s, Country::morocco());
            }
            for s in ["mozambique", "508", "mz", "moz"] {
                map.insert(s, Country::mozambique());
            }
            for s in ["myanmar", "104", "mm", "mmr"] {
                map.insert(s, Country::myanmar());
            }
            for s in ["namibia", "516", "na", "nam"] {
                map.insert(s, Country::namibia());
            }
            for s in ["nauru", "520", "nr", "nru"] {
                map.insert(s, Country::nauru());
            }
            for s in ["nepal", "524", "np", "npl"] {
                map.insert(s, Country::nepal());
            }
            for s in ["newcaledonia", "new_caledonia", "540", "nc", "ncl"] {
                map.insert(s, Country::new_caledonia());
            }
            for s in ["newzealand", "new_zealand", "554", "nz", "nzl"] {
                map.insert(s, Country::new_zealand());
            }
            for s in ["nicaragua", "558", "ni", "nic"] {
                map.insert(s, Country::nicaragua());
            }
            for s in ["nigeria", "566", "ng", "nga"] {
                map.insert(s, Country::nigeria());
            }
            for s in ["niue", "570", "nu", "niu"] {
                map.insert(s, Country::niue());
            }
            for s in ["norfolkisland", "norfolk_island", "574", "nf", "nfk"] {
                map.insert(s, Country::norfolk_island());
            }
            for s in ["norway", "578", "no", "nor"] {
                map.insert(s, Country::norway());
            }
            for s in ["oman", "512", "om", "omn"] {
                map.insert(s, Country::oman());
            }
            for s in ["pakistan", "586", "pk", "pak"] {
                map.insert(s, Country::pakistan());
            }
            for s in ["palau", "585", "pw", "plw"] {
                map.insert(s, Country::palau());
            }
            for s in ["panama", "591", "pa", "pan"] {
                map.insert(s, Country::panama());
            }
            for s in ["papuanewguinea", "papua_new_guinea", "598", "pg", "png"] {
                map.insert(s, Country::papua_new_guinea());
            }
            for s in ["paraguay", "600", "py", "pry"] {
                map.insert(s, Country::paraguay());
            }
            for s in ["peru", "604", "pe", "per"] {
                map.insert(s, Country::peru());
            }
            for s in ["pitcairn", "612", "pn", "pcn"] {
                map.insert(s, Country::pitcairn());
            }
            for s in ["poland", "616", "pl", "pol"] {
                map.insert(s, Country::poland());
            }
            for s in ["portugal", "620", "pt", "prt"] {
                map.insert(s, Country::portugal());
            }
            for s in ["puertorico", "puerto_rico", "630", "pr", "pri"] {
                map.insert(s, Country::puerto_rico());
            }
            for s in ["qatar", "634", "qa", "qat"] {
                map.insert(s, Country::qatar());
            }
            for s in [
                "republicofnorthmacedonia",
                "republic_of_north_macedonia",
                "807",
                "mk",
                "mkd",
                "macedonia",
            ] {
                map.insert(s, Country::republic_of_north_macedonia());
            }
            for s in ["reunion", "638", "re", "reu"] {
                map.insert(s, Country::reunion());
            }
            for s in ["romania", "642", "ro", "rou"] {
                map.insert(s, Country::romania());
            }
            for s in ["rwanda", "646", "rw", "rwa"] {
                map.insert(s, Country::rwanda());
            }
            for s in [
                "saintbarthelemy",
                "saint_barthelemy",
                "652",
                "bl",
                "blm",
                "stbarthelemy",
            ] {
                map.insert(s, Country::saint_barthelemy());
            }
            for s in [
                "saintkittsandnevis",
                "saint_kitts_and_nevis",
                "659",
                "kn",
                "kna",
                "stkitts",
            ] {
                map.insert(s, Country::saint_kitts_and_nevis());
            }
            for s in ["saintlucia", "saint_lucia", "662", "lc", "lca", "stlucia"] {
                map.insert(s, Country::saint_lucia());
            }
            for s in [
                "saintpierreandmiquelon",
                "saint_pierre_and_miquelon",
                "666",
                "pm",
                "spm",
                "stpierre",
                "saintpierre",
            ] {
                map.insert(s, Country::saint_pierre_and_miquelon());
            }
            for s in [
                "saintvincentandthegrenadines",
                "saint_vincent_and_the_grenadines",
                "670",
                "vc",
                "vct",
                "stvincent",
                "saintvincent",
            ] {
                map.insert(s, Country::saint_vincent_and_the_grenadines());
            }
            for s in ["samoa", "882", "ws", "wsm"] {
                map.insert(s, Country::samoa());
            }
            for s in ["sanmarino", "san_marino", "674", "sm", "smr"] {
                map.insert(s, Country::san_marino());
            }
            for s in [
                "saotomeandprincipe",
                "sao_tome_and_principe",
                "678",
                "st",
                "stp",
                "saotome",
            ] {
                map.insert(s, Country::sao_tome_and_principe());
            }
            for s in ["saudiarabia", "saudi_arabia", "682", "sa", "sau"] {
                map.insert(s, Country::saudi_arabia());
            }
            for s in ["senegal", "686", "sn", "sen"] {
                map.insert(s, Country::senegal());
            }
            for s in ["serbia", "688", "rs", "srb"] {
                map.insert(s, Country::serbia());
            }
            for s in ["seychelles", "690", "sc", "syc"] {
                map.insert(s, Country::seychelles());
            }
            for s in ["sierraleone", "sierra_leone", "694", "sl", "sle"] {
                map.insert(s, Country::sierra_leone());
            }
            for s in ["singapore", "702", "sg", "sgp"] {
                map.insert(s, Country::singapore());
            }
            for s in ["slovakia", "703", "sk", "svk"] {
                map.insert(s, Country::slovakia());
            }
            for s in ["slovenia", "705", "si", "svn"] {
                map.insert(s, Country::slovenia());
            }
            for s in ["solomonislands", "solomon_islands", "090", "sb", "slb"] {
                map.insert(s, Country::solomon_islands());
            }
            for s in ["somalia", "706", "so", "som"] {
                map.insert(s, Country::somalia());
            }
            for s in ["southafrica", "south_africa", "710", "za", "zaf"] {
                map.insert(s, Country::south_africa());
            }
            for s in [
                "southgeorgiaandthesouthsandwichislands",
                "south_georgia_and_the_south_sandwich_islands",
                "239",
                "gs",
                "sgs",
                "southgeorgia",
                "southsandwichislands",
            ] {
                map.insert(s, Country::south_georgia_and_the_south_sandwich_islands());
            }
            for s in ["southsudan", "south_sudan", "728", "ss", "ssd"] {
                map.insert(s, Country::south_sudan());
            }
            for s in ["spain", "724", "es", "esp"] {
                map.insert(s, Country::spain());
            }
            for s in ["srilanka", "sri_lanka", "144", "lk", "lka"] {
                map.insert(s, Country::sri_lanka());
            }
            for s in [
                "stateofpalestine",
                "state_of_palestine",
                "275",
                "ps",
                "pse",
                "palestine",
            ] {
                map.insert(s, Country::state_of_palestine());
            }
            for s in ["suriname", "740", "sr", "sur"] {
                map.insert(s, Country::suriname());
            }
            for s in [
                "svalbardandjanmayen",
                "svalbard_and_jan_mayen",
                "744",
                "sj",
                "sjm",
            ] {
                map.insert(s, Country::svalbard_and_jan_mayen());
            }
            for s in ["sweden", "752", "se", "swe"] {
                map.insert(s, Country::sweden());
            }
            for s in ["switzerland", "756", "ch", "che"] {
                map.insert(s, Country::switzerland());
            }
            for s in [
                "syrianarabrepublic",
                "syrian_arab_republic",
                "760",
                "sy",
                "syr",
            ] {
                map.insert(s, Country::syrian_arab_republic());
            }
            for s in ["taiwan,republicofchina", "taiwan", "158", "tw", "twn"] {
                map.insert(s, Country::taiwan());
            }
            for s in ["tajikistan", "762", "tj", "tjk"] {
                map.insert(s, Country::tajikistan());
            }
            for s in ["thailand", "764", "th", "tha"] {
                map.insert(s, Country::thailand());
            }
            for s in ["thebahamas", "the_bahamas", "044", "bs", "bhs", "bahamas"] {
                map.insert(s, Country::the_bahamas());
            }
            for s in [
                "thecaymanislands",
                "the_cayman_islands",
                "136",
                "ky",
                "cym",
                "caymanislands",
            ] {
                map.insert(s, Country::the_cayman_islands());
            }
            for s in [
                "thecentralafricanrepublic",
                "the_central_african_republic",
                "140",
                "cf",
                "caf",
                "centralafricanrepublic",
            ] {
                map.insert(s, Country::the_central_african_republic());
            }
            for s in [
                "thecocoskeelingislands",
                "the_cocos_keeling_islands",
                "166",
                "cc",
                "cck",
                "cocosislands",
                "keelingislands",
            ] {
                map.insert(s, Country::the_cocos_keeling_islands());
            }
            for s in ["thecomoros", "the_comoros", "174", "km", "com", "comoros"] {
                map.insert(s, Country::the_comoros());
            }
            for s in ["thecongo", "the_congo", "178", "cg", "cog", "congo"] {
                map.insert(s, Country::the_congo());
            }
            for s in [
                "thecookislands",
                "the_cook_islands",
                "184",
                "ck",
                "cok",
                "cookislands",
            ] {
                map.insert(s, Country::the_cook_islands());
            }
            for s in [
                "thedemocraticpeoplesrepublicofkorea",
                "the_democratic_peoples_republic_of_korea",
                "408",
                "kp",
                "prk",
                "northkorea",
                "democraticpeoplesrepublicofkorea",
            ] {
                map.insert(s, Country::the_democratic_peoples_republic_of_korea());
            }
            for s in [
                "thedemocraticrepublicofthecongo",
                "the_democratic_republic_of_the_congo",
                "180",
                "cd",
                "cod",
                "democraticrepublicofthecongo",
            ] {
                map.insert(s, Country::the_democratic_republic_of_the_congo());
            }
            for s in [
                "thedominicanrepublic",
                "the_dominican_republic",
                "214",
                "do",
                "dom",
                "dominicanrepublic",
            ] {
                map.insert(s, Country::the_dominican_republic());
            }
            for s in [
                "thefalklandislandsmalvinas",
                "the_falkland_islands_malvinas",
                "238",
                "fk",
                "flk",
                "malvinas",
                "falklandislands",
            ] {
                map.insert(s, Country::the_falkland_islands_malvinas());
            }
            for s in [
                "thefaroeislands",
                "the_faroe_islands",
                "234",
                "fo",
                "fro",
                "faroeislands",
            ] {
                map.insert(s, Country::the_faroe_islands());
            }
            for s in [
                "thefrenchsouthernterritories",
                "the_french_southern_territories",
                "260",
                "tf",
                "atf",
                "frenchsouthernterritories",
            ] {
                map.insert(s, Country::the_french_southern_territories());
            }
            for s in ["thegambia", "the_gambia", "270", "gm", "gmb", "gambia"] {
                map.insert(s, Country::the_gambia());
            }
            for s in ["theholysee", "the_holy_see", "336", "va", "vat", "holysee"] {
                map.insert(s, Country::the_holy_see());
            }
            for s in [
                "thelaopeoplesdemocraticrepublic",
                "the_lao_peoples_democratic_republic",
                "418",
                "la",
                "lao",
                "laopeoplesdemocraticrepublic",
            ] {
                map.insert(s, Country::the_lao_peoples_democratic_republic());
            }
            for s in [
                "themarshallislands",
                "the_marshall_islands",
                "584",
                "mh",
                "mhl",
                "marshallislands",
            ] {
                map.insert(s, Country::the_marshall_islands());
            }
            for s in [
                "thenetherlands",
                "the_netherlands",
                "528",
                "nl",
                "nld",
                "netherlands",
                "holland",
            ] {
                map.insert(s, Country::the_netherlands());
            }
            for s in ["theniger", "the_niger", "562", "ne", "ner", "niger"] {
                map.insert(s, Country::the_niger());
            }
            for s in [
                "thenorthernmarianaislands",
                "the_northern_mariana_islands",
                "580",
                "mp",
                "mnp",
                "northernmarianaislands",
            ] {
                map.insert(s, Country::the_northern_mariana_islands());
            }
            for s in [
                "thephilippines",
                "the_philippines",
                "608",
                "ph",
                "phl",
                "philippines",
            ] {
                map.insert(s, Country::the_philippines());
            }
            for s in [
                "therepublicofkorea",
                "the_republic_of_korea",
                "410",
                "kr",
                "kor",
                "southkorea",
                "republicofkorea",
            ] {
                map.insert(s, Country::the_republic_of_korea());
            }
            for s in [
                "therepublicofmoldova",
                "the_republic_of_moldova",
                "498",
                "md",
                "mda",
                "moldova",
                "republicofmoldova",
            ] {
                map.insert(s, Country::the_republic_of_moldova());
            }
            for s in [
                "therussianfederation",
                "the_russian_federation",
                "643",
                "ru",
                "rus",
                "russia",
                "russianfederation",
            ] {
                map.insert(s, Country::the_russian_federation());
            }
            for s in ["thesudan", "the_sudan", "729", "sd", "sdn", "sudan"] {
                map.insert(s, Country::the_sudan());
            }
            for s in [
                "theturksandcaicosislands",
                "the_turks_and_caicos_islands",
                "796",
                "tc",
                "tca",
                "turksandcaicosislands",
            ] {
                map.insert(s, Country::the_turks_and_caicos_islands());
            }
            for s in [
                "theunitedarabemirates",
                "the_united_arab_emirates",
                "784",
                "ae",
                "are",
                "unitedarabemirates",
            ] {
                map.insert(s, Country::the_united_arab_emirates());
            }
            for s in [
                "theunitedkingdomofgreatbritainandnorthernireland",
                "the_united_kingdom_of_great_britain_and_northern_ireland",
                "826",
                "gb",
                "gbr",
                "england",
                "scotland",
                "greatbritain",
                "unitedkingdom",
                "northernireland",
                "unitedkingdomofgreatbritain",
                "unitedkingdomofgreatbritainandnorthernireland",
            ] {
                map.insert(
                    s,
                    Country::the_united_kingdom_of_great_britain_and_northern_ireland(),
                );
            }
            for s in [
                "theunitedstatesminoroutlyingislands",
                "the_united_states_minor_outlying_islands",
                "581",
                "um",
                "umi",
                "unitedstatesminoroutlyingislands",
            ] {
                map.insert(s, Country::the_united_states_minor_outlying_islands());
            }
            for s in [
                "theunitedstatesofamerica",
                "the_united_states_of_america",
                "840",
                "us",
                "usa",
                "america",
                "united states",
                "unitedstates",
                "unitedstatesofamerica",
            ] {
                map.insert(s, Country::the_united_states_of_america());
            }
            for s in ["timorleste", "timor_leste", "626", "tl", "tls"] {
                map.insert(s, Country::timor_leste());
            }
            for s in ["togo", "768", "tg", "tgo"] {
                map.insert(s, Country::togo());
            }
            for s in ["tokelau", "772", "tk", "tkl"] {
                map.insert(s, Country::tokelau());
            }
            for s in ["tonga", "776", "to", "ton"] {
                map.insert(s, Country::tonga());
            }
            for s in [
                "trinidadandtobago",
                "trinidad_and_tobago",
                "780",
                "tt",
                "tto",
                "trinidad",
                "tobago",
            ] {
                map.insert(s, Country::trinidad_and_tobago());
            }
            for s in ["tunisia", "788", "tn", "tun"] {
                map.insert(s, Country::tunisia());
            }
            for s in ["turkey", "türkiye", "792", "tr", "tur"] {
                map.insert(s, Country::turkey());
            }
            for s in ["turkmenistan", "795", "tm", "tkm"] {
                map.insert(s, Country::turkmenistan());
            }
            for s in ["tuvalu", "798", "tv", "tuv"] {
                map.insert(s, Country::tuvalu());
            }
            for s in ["usvirginislands", "us_virgin_islands", "850", "vi", "vir"] {
                map.insert(s, Country::us_virgin_islands());
            }
            for s in ["uganda", "800", "ug", "uga"] {
                map.insert(s, Country::uganda());
            }
            for s in ["ukraine", "804", "ua", "ukr"] {
                map.insert(s, Country::ukraine());
            }
            for s in [
                "unitedrepublicoftanzania",
                "united_republic_of_tanzania",
                "834",
                "tz",
                "tza",
                "tanzania",
            ] {
                map.insert(s, Country::united_republic_of_tanzania());
            }
            for s in ["uruguay", "858", "uy", "ury"] {
                map.insert(s, Country::uruguay());
            }
            for s in ["uzbekistan", "860", "uz", "uzb"] {
                map.insert(s, Country::uzbekistan());
            }
            for s in ["vanuatu", "548", "vu", "vut"] {
                map.insert(s, Country::vanuatu());
            }
            for s in ["vietnam", "704", "vn", "vnm"] {
                map.insert(s, Country::vietnam());
            }
            for s in ["wallisandfutuna", "wallis_and_futuna", "876", "wf", "wlf"] {
                map.insert(s, Country::wallis_and_futuna());
            }
            for s in ["westernsahara", "western_sahara", "732", "eh", "esh"] {
                map.insert(s, Country::western_sahara());
            }
            for s in ["yemen", "887", "ye", "yem"] {
                map.insert(s, Country::yemen());
            }
            for s in ["zambia", "894", "zm", "zmb"] {
                map.insert(s, Country::zambia());
            }
            for s in ["zimbabwe", "716", "zw", "zwe"] {
                map.insert(s, Country::zimbabwe());
            }
            map
        });
        (*CODES)
            .get(code.to_lowercase().as_str())
            .copied()
            .ok_or("unknown value")
    }
}
