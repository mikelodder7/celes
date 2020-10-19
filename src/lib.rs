//! Rust implementation of Countries as specified by
//! https://www.iban.com/country-codes using ISO 3166-1
//! and https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes
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
//! - `from_alias` - create `Country` from a common alias. This only works for some countries as not all countries have aliases
//! - `from_name` - create `Country` from the full state name no space or underscores
//!
//! `Country` implements the [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait that accepts any valid argument to the previously mentioned functions
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
//! ```
//! use celes::Country;
//! use std::str::FromStr;
//!
//! // All three of these are equivalent
//! let usa_1 = Country::from_str("USA").unwrap();
//! let usa_2 = Country::from_str("US").unwrap();
//! let usa_3 = Country::from_str("America").unwrap();
//!
//! // All three of these are equivalent
//! let gb_1 = Country::from_str("England").unwrap();
//! let gb_2 = Country::from_str("gb").unwrap();
//! let gb_3 = Country::from_str("Scotland").unwrap();
//! ```
/*
* Copyright 2019 Michael Lodder
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
* -----------------------------------------------------------------------------
*/
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Creates a BTreeSet with the given keys
macro_rules! btreeset {
    ($($key:expr),*) => {
        {
            let mut set = BTreeSet::new();
            $(
                set.insert($key.to_string());
            )*
            set
        }
    };
}

/// Creates the country function. Meant to be called inside `Country`
macro_rules! country {
    ($func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr) => {
        country!{ @gen [concat!("Creates a struct for ", $long_name), $func, $code, $value, $alpha2, $alpha3, $long_name] }
    };
    ($func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr, $( $alias:expr  ),*) => {
        country!{ @gen [concat!("Creates a struct for ", $long_name), $func, $code, $value, $alpha2, $alpha3, $long_name, $( $alias ),* ] }
    };
    (@gen [$doc:expr, $func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr]) => {
        #[doc = $doc]
        pub fn $func() -> Self {
            Self {
                code: $code.to_string(),
                value: $value,
                alpha2: $alpha2.to_string(),
                alpha3: $alpha3.to_string(),
                long_name: $long_name.to_string(),
                aliases: BTreeSet::new()
            }
        }
    };
    (@gen [$doc:expr, $func:ident, $code:expr, $value:expr, $alpha2:expr, $alpha3:expr, $long_name:expr, $( $alias:expr ),* ]) => {
        #[doc = $doc]
        pub fn $func() -> Self {
            Self {
                code: $code.to_string(),
                value: $value,
                alpha2: $alpha2.to_string(),
                alpha3: $alpha3.to_string(),
                long_name: $long_name.to_string(),
                aliases: btreeset![$($alias),*]
            }
        }
    };
}

/// Represents a country according to ISO 3166
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Country {
    /// The three digit code assigned to the country
    pub code: String,
    /// The integer value for `code`
    pub value: usize,
    /// The two letter country code (alpha-2) assigned to the country
    pub alpha2: String,
    /// The three letter country code (alpha-3) assigned to the country
    pub alpha3: String,
    /// The official state name of the country
    pub long_name: String,
    /// Common aliases associated with the country
    pub aliases: BTreeSet<String>,
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
    country!(czechia, "203", 203, "CZ", "CZE", "Czechia");
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
        "StMaarten",
        "SintMaarten"
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
        "StBarthelemy"
    );
    country!(
        saint_kitts_and_nevis,
        "659",
        659,
        "KN",
        "KNA",
        "Saint Kitts And Nevis",
        "StKitts"
    );
    country!(
        saint_lucia,
        "662",
        662,
        "LC",
        "LCA",
        "Saint Lucia",
        "StLucia"
    );
    country!(
        saint_pierre_and_miquelon,
        "666",
        666,
        "PM",
        "SPM",
        "Saint Pierre And Miquelon",
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
        taiwan_province_of_china,
        "158",
        158,
        "TW",
        "TWN",
        "Taiwan Province Of China",
        "Taiwan"
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
        "Bahamas"
    );
    country!(
        the_cayman_islands,
        "136",
        136,
        "KY",
        "CYM",
        "The Cayman Islands",
        "CaymanIslands"
    );
    country!(
        the_central_african_republic,
        "140",
        140,
        "CF",
        "CAF",
        "The Central African Republic",
        "CentralAfricanRepublic"
    );
    country!(
        the_cocos_keeling_islands,
        "166",
        166,
        "CC",
        "CCK",
        "The Cocos Keeling Islands",
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
        "Comoros"
    );
    country!(the_congo, "178", 178, "CG", "COG", "The Congo", "Congo");
    country!(
        the_cook_islands,
        "184",
        184,
        "CK",
        "COK",
        "The Cook Islands",
        "CookIslands"
    );
    country!(
        the_democratic_peoples_republic_of_korea,
        "408",
        408,
        "KP",
        "PRK",
        "The Democratic Peoples Republic Of Korea",
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
        "DemocraticRepublicOfTheCongo"
    );
    country!(
        the_dominican_republic,
        "214",
        214,
        "DO",
        "DOM",
        "The Dominican Republic",
        "DominicanRepublic"
    );
    country!(
        the_falkland_islands_malvinas,
        "238",
        238,
        "FK",
        "FLK",
        "The Falkland Islands Malvinas",
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
        "FaroeIslands"
    );
    country!(
        the_french_southern_territories,
        "260",
        260,
        "TF",
        "ATF",
        "The French Southern Territories",
        "FrenchSouthernTerritories"
    );
    country!(the_gambia, "270", 270, "GM", "GMB", "The Gambia", "Gabmia");
    country!(
        the_holy_see,
        "336",
        336,
        "VA",
        "VAT",
        "The Holy See",
        "HolySee"
    );
    country!(
        the_lao_peoples_democratic_republic,
        "418",
        418,
        "LA",
        "LAO",
        "The Lao Peoples Democratic Republic",
        "LaoPeoplesDemocraticRepublic"
    );
    country!(
        the_marshall_islands,
        "584",
        584,
        "MH",
        "MHL",
        "The Marshall Islands",
        "MarshallIslands"
    );
    country!(
        the_netherlands,
        "528",
        528,
        "NL",
        "NLD",
        "The Netherlands",
        "Netherlands",
        "Holland"
    );
    country!(the_niger, "562", 562, "NE", "NER", "The Niger", "Niger");
    country!(
        the_northern_mariana_islands,
        "580",
        580,
        "MP",
        "MNP",
        "The Northern Mariana Islands",
        "NorthernMarianaIslands"
    );
    country!(
        the_philippines,
        "608",
        608,
        "PH",
        "PHL",
        "The Philippines",
        "Philippines"
    );
    country!(
        the_republic_of_korea,
        "410",
        410,
        "KR",
        "KOR",
        "The Republic Of Korea",
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
        "Russia",
        "RussianFederation"
    );
    country!(the_sudan, "729", 729, "SD", "SDN", "The Sudan", "Sudan");
    country!(
        the_turks_and_caicos_islands,
        "796",
        796,
        "TC",
        "TCA",
        "The Turks And Caicos Islands",
        "TurksAndCaicosIslands"
    );
    country!(
        the_united_arab_emirates,
        "784",
        784,
        "AE",
        "ARE",
        "The United Arab Emirates",
        "UnitedArabEmirates"
    );
    country!(
        the_united_kingdom_of_great_britain_and_northern_ireland,
        "826",
        826,
        "GB",
        "GBR",
        "The United Kingdom Of Great Britain And Northern Ireland",
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
        "UnitedStatesMinorOutlyingIslands"
    );
    country!(
        the_united_states_of_america,
        "840",
        840,
        "US",
        "USA",
        "The United States Of America",
        "America",
        "UnitedStates",
        "UnitedStatesOfAmerica"
    );
    country!(timor_leste, "626", 626, "TL", "TLS", "Timor Leste");
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
        "Trinidad",
        "Tobago"
    );
    country!(tunisia, "788", 788, "TN", "TUN", "Tunisia");
    country!(turkey, "792", 792, "TR", "TUR", "Turkey");
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
    pub fn get_countries() -> Vec<Self> {
        vec![
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
            Self::taiwan_province_of_china(),
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
    pub fn from_value(value: usize) -> Result<Self, String> {
        match value {
            4 => Ok(Self::afghanistan()),
            248 => Ok(Self::aland_islands()),
            8 => Ok(Self::albania()),
            12 => Ok(Self::algeria()),
            16 => Ok(Self::american_samoa()),
            20 => Ok(Self::andorra()),
            24 => Ok(Self::angola()),
            660 => Ok(Self::anguilla()),
            10 => Ok(Self::antarctica()),
            28 => Ok(Self::antigua_and_barbuda()),
            32 => Ok(Self::argentina()),
            51 => Ok(Self::armenia()),
            533 => Ok(Self::aruba()),
            654 => Ok(Self::ascension_and_tristan_da_cunha_saint_helena()),
            36 => Ok(Self::australia()),
            40 => Ok(Self::austria()),
            31 => Ok(Self::azerbaijan()),
            48 => Ok(Self::bahrain()),
            50 => Ok(Self::bangladesh()),
            52 => Ok(Self::barbados()),
            112 => Ok(Self::belarus()),
            56 => Ok(Self::belgium()),
            84 => Ok(Self::belize()),
            204 => Ok(Self::benin()),
            60 => Ok(Self::bermuda()),
            64 => Ok(Self::bhutan()),
            862 => Ok(Self::bolivarian_republic_of_venezuela()),
            68 => Ok(Self::bolivia()),
            535 => Ok(Self::bonaire()),
            70 => Ok(Self::bosnia_and_herzegovina()),
            72 => Ok(Self::botswana()),
            74 => Ok(Self::bouvet_island()),
            76 => Ok(Self::brazil()),
            86 => Ok(Self::british_indian_ocean_territory()),
            92 => Ok(Self::british_virgin_islands()),
            96 => Ok(Self::brunei_darussalam()),
            100 => Ok(Self::bulgaria()),
            854 => Ok(Self::burkina_faso()),
            108 => Ok(Self::burundi()),
            132 => Ok(Self::cabo_verde()),
            116 => Ok(Self::cambodia()),
            120 => Ok(Self::cameroon()),
            124 => Ok(Self::canada()),
            148 => Ok(Self::chad()),
            152 => Ok(Self::chile()),
            156 => Ok(Self::china()),
            162 => Ok(Self::christmas_island()),
            170 => Ok(Self::colombia()),
            188 => Ok(Self::costa_rica()),
            384 => Ok(Self::coted_ivoire()),
            191 => Ok(Self::croatia()),
            192 => Ok(Self::cuba()),
            531 => Ok(Self::curacao()),
            196 => Ok(Self::cyprus()),
            203 => Ok(Self::czechia()),
            208 => Ok(Self::denmark()),
            262 => Ok(Self::djibouti()),
            212 => Ok(Self::dominica()),
            534 => Ok(Self::dutch_part_sint_maarten()),
            218 => Ok(Self::ecuador()),
            818 => Ok(Self::egypt()),
            222 => Ok(Self::el_salvador()),
            226 => Ok(Self::equatorial_guinea()),
            232 => Ok(Self::eritrea()),
            233 => Ok(Self::estonia()),
            748 => Ok(Self::eswatini()),
            231 => Ok(Self::ethiopia()),
            583 => Ok(Self::federated_states_of_micronesia()),
            242 => Ok(Self::fiji()),
            246 => Ok(Self::finland()),
            250 => Ok(Self::france()),
            254 => Ok(Self::french_guiana()),
            663 => Ok(Self::french_part_saint_martin()),
            258 => Ok(Self::french_polynesia()),
            266 => Ok(Self::gabon()),
            268 => Ok(Self::georgia()),
            276 => Ok(Self::germany()),
            288 => Ok(Self::ghana()),
            292 => Ok(Self::gibraltar()),
            300 => Ok(Self::greece()),
            304 => Ok(Self::greenland()),
            308 => Ok(Self::grenada()),
            312 => Ok(Self::guadeloupe()),
            316 => Ok(Self::guam()),
            320 => Ok(Self::guatemala()),
            831 => Ok(Self::guernsey()),
            324 => Ok(Self::guinea()),
            624 => Ok(Self::guinea_bissau()),
            328 => Ok(Self::guyana()),
            332 => Ok(Self::haiti()),
            334 => Ok(Self::heard_island_and_mc_donald_islands()),
            340 => Ok(Self::honduras()),
            344 => Ok(Self::hong_kong()),
            348 => Ok(Self::hungary()),
            352 => Ok(Self::iceland()),
            356 => Ok(Self::india()),
            360 => Ok(Self::indonesia()),
            368 => Ok(Self::iraq()),
            372 => Ok(Self::ireland()),
            364 => Ok(Self::islamic_republic_of_iran()),
            833 => Ok(Self::isle_of_man()),
            376 => Ok(Self::israel()),
            380 => Ok(Self::italy()),
            388 => Ok(Self::jamaica()),
            392 => Ok(Self::japan()),
            832 => Ok(Self::jersey()),
            400 => Ok(Self::jordan()),
            398 => Ok(Self::kazakhstan()),
            404 => Ok(Self::kenya()),
            296 => Ok(Self::kiribati()),
            414 => Ok(Self::kuwait()),
            417 => Ok(Self::kyrgyzstan()),
            428 => Ok(Self::latvia()),
            422 => Ok(Self::lebanon()),
            426 => Ok(Self::lesotho()),
            430 => Ok(Self::liberia()),
            434 => Ok(Self::libya()),
            438 => Ok(Self::liechtenstein()),
            440 => Ok(Self::lithuania()),
            442 => Ok(Self::luxembourg()),
            446 => Ok(Self::macao()),
            450 => Ok(Self::madagascar()),
            454 => Ok(Self::malawi()),
            458 => Ok(Self::malaysia()),
            462 => Ok(Self::maldives()),
            466 => Ok(Self::mali()),
            470 => Ok(Self::malta()),
            474 => Ok(Self::martinique()),
            478 => Ok(Self::mauritania()),
            480 => Ok(Self::mauritius()),
            175 => Ok(Self::mayotte()),
            484 => Ok(Self::mexico()),
            492 => Ok(Self::monaco()),
            496 => Ok(Self::mongolia()),
            499 => Ok(Self::montenegro()),
            500 => Ok(Self::montserrat()),
            504 => Ok(Self::morocco()),
            508 => Ok(Self::mozambique()),
            104 => Ok(Self::myanmar()),
            516 => Ok(Self::namibia()),
            520 => Ok(Self::nauru()),
            524 => Ok(Self::nepal()),
            540 => Ok(Self::new_caledonia()),
            554 => Ok(Self::new_zealand()),
            558 => Ok(Self::nicaragua()),
            566 => Ok(Self::nigeria()),
            570 => Ok(Self::niue()),
            574 => Ok(Self::norfolk_island()),
            578 => Ok(Self::norway()),
            512 => Ok(Self::oman()),
            586 => Ok(Self::pakistan()),
            585 => Ok(Self::palau()),
            591 => Ok(Self::panama()),
            598 => Ok(Self::papua_new_guinea()),
            600 => Ok(Self::paraguay()),
            604 => Ok(Self::peru()),
            612 => Ok(Self::pitcairn()),
            616 => Ok(Self::poland()),
            620 => Ok(Self::portugal()),
            630 => Ok(Self::puerto_rico()),
            634 => Ok(Self::qatar()),
            807 => Ok(Self::republic_of_north_macedonia()),
            638 => Ok(Self::reunion()),
            642 => Ok(Self::romania()),
            646 => Ok(Self::rwanda()),
            652 => Ok(Self::saint_barthelemy()),
            659 => Ok(Self::saint_kitts_and_nevis()),
            662 => Ok(Self::saint_lucia()),
            666 => Ok(Self::saint_pierre_and_miquelon()),
            670 => Ok(Self::saint_vincent_and_the_grenadines()),
            882 => Ok(Self::samoa()),
            674 => Ok(Self::san_marino()),
            678 => Ok(Self::sao_tome_and_principe()),
            682 => Ok(Self::saudi_arabia()),
            686 => Ok(Self::senegal()),
            688 => Ok(Self::serbia()),
            690 => Ok(Self::seychelles()),
            694 => Ok(Self::sierra_leone()),
            702 => Ok(Self::singapore()),
            703 => Ok(Self::slovakia()),
            705 => Ok(Self::slovenia()),
            90 => Ok(Self::solomon_islands()),
            706 => Ok(Self::somalia()),
            710 => Ok(Self::south_africa()),
            239 => Ok(Self::south_georgia_and_the_south_sandwich_islands()),
            728 => Ok(Self::south_sudan()),
            724 => Ok(Self::spain()),
            144 => Ok(Self::sri_lanka()),
            275 => Ok(Self::state_of_palestine()),
            740 => Ok(Self::suriname()),
            744 => Ok(Self::svalbard_and_jan_mayen()),
            752 => Ok(Self::sweden()),
            756 => Ok(Self::switzerland()),
            760 => Ok(Self::syrian_arab_republic()),
            158 => Ok(Self::taiwan_province_of_china()),
            762 => Ok(Self::tajikistan()),
            764 => Ok(Self::thailand()),
            44 => Ok(Self::the_bahamas()),
            136 => Ok(Self::the_cayman_islands()),
            140 => Ok(Self::the_central_african_republic()),
            166 => Ok(Self::the_cocos_keeling_islands()),
            174 => Ok(Self::the_comoros()),
            178 => Ok(Self::the_congo()),
            184 => Ok(Self::the_cook_islands()),
            408 => Ok(Self::the_democratic_peoples_republic_of_korea()),
            180 => Ok(Self::the_democratic_republic_of_the_congo()),
            214 => Ok(Self::the_dominican_republic()),
            238 => Ok(Self::the_falkland_islands_malvinas()),
            234 => Ok(Self::the_faroe_islands()),
            260 => Ok(Self::the_french_southern_territories()),
            270 => Ok(Self::the_gambia()),
            336 => Ok(Self::the_holy_see()),
            418 => Ok(Self::the_lao_peoples_democratic_republic()),
            584 => Ok(Self::the_marshall_islands()),
            528 => Ok(Self::the_netherlands()),
            562 => Ok(Self::the_niger()),
            580 => Ok(Self::the_northern_mariana_islands()),
            608 => Ok(Self::the_philippines()),
            410 => Ok(Self::the_republic_of_korea()),
            498 => Ok(Self::the_republic_of_moldova()),
            643 => Ok(Self::the_russian_federation()),
            729 => Ok(Self::the_sudan()),
            796 => Ok(Self::the_turks_and_caicos_islands()),
            784 => Ok(Self::the_united_arab_emirates()),
            826 => Ok(Self::the_united_kingdom_of_great_britain_and_northern_ireland()),
            581 => Ok(Self::the_united_states_minor_outlying_islands()),
            840 => Ok(Self::the_united_states_of_america()),
            626 => Ok(Self::timor_leste()),
            768 => Ok(Self::togo()),
            772 => Ok(Self::tokelau()),
            776 => Ok(Self::tonga()),
            780 => Ok(Self::trinidad_and_tobago()),
            788 => Ok(Self::tunisia()),
            792 => Ok(Self::turkey()),
            795 => Ok(Self::turkmenistan()),
            798 => Ok(Self::tuvalu()),
            850 => Ok(Self::us_virgin_islands()),
            800 => Ok(Self::uganda()),
            804 => Ok(Self::ukraine()),
            834 => Ok(Self::united_republic_of_tanzania()),
            858 => Ok(Self::uruguay()),
            860 => Ok(Self::uzbekistan()),
            548 => Ok(Self::vanuatu()),
            704 => Ok(Self::vietnam()),
            876 => Ok(Self::wallis_and_futuna()),
            732 => Ok(Self::western_sahara()),
            887 => Ok(Self::yemen()),
            894 => Ok(Self::zambia()),
            716 => Ok(Self::zimbabwe()),
            e => Err(format!("Unknown Value {}", e)),
        }
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
    pub fn from_code<A: AsRef<str>>(code: A) -> Result<Self, String> {
        match code.as_ref().to_lowercase().as_str() {
            "004" => Ok(Self::afghanistan()),
            "248" => Ok(Self::aland_islands()),
            "008" => Ok(Self::albania()),
            "012" => Ok(Self::algeria()),
            "016" => Ok(Self::american_samoa()),
            "020" => Ok(Self::andorra()),
            "024" => Ok(Self::angola()),
            "660" => Ok(Self::anguilla()),
            "010" => Ok(Self::antarctica()),
            "028" => Ok(Self::antigua_and_barbuda()),
            "032" => Ok(Self::argentina()),
            "051" => Ok(Self::armenia()),
            "533" => Ok(Self::aruba()),
            "654" => Ok(Self::ascension_and_tristan_da_cunha_saint_helena()),
            "036" => Ok(Self::australia()),
            "040" => Ok(Self::austria()),
            "031" => Ok(Self::azerbaijan()),
            "048" => Ok(Self::bahrain()),
            "050" => Ok(Self::bangladesh()),
            "052" => Ok(Self::barbados()),
            "112" => Ok(Self::belarus()),
            "056" => Ok(Self::belgium()),
            "084" => Ok(Self::belize()),
            "204" => Ok(Self::benin()),
            "060" => Ok(Self::bermuda()),
            "064" => Ok(Self::bhutan()),
            "862" => Ok(Self::bolivarian_republic_of_venezuela()),
            "068" => Ok(Self::bolivia()),
            "535" => Ok(Self::bonaire()),
            "070" => Ok(Self::bosnia_and_herzegovina()),
            "072" => Ok(Self::botswana()),
            "074" => Ok(Self::bouvet_island()),
            "076" => Ok(Self::brazil()),
            "086" => Ok(Self::british_indian_ocean_territory()),
            "092" => Ok(Self::british_virgin_islands()),
            "096" => Ok(Self::brunei_darussalam()),
            "100" => Ok(Self::bulgaria()),
            "854" => Ok(Self::burkina_faso()),
            "108" => Ok(Self::burundi()),
            "132" => Ok(Self::cabo_verde()),
            "116" => Ok(Self::cambodia()),
            "120" => Ok(Self::cameroon()),
            "124" => Ok(Self::canada()),
            "148" => Ok(Self::chad()),
            "152" => Ok(Self::chile()),
            "156" => Ok(Self::china()),
            "162" => Ok(Self::christmas_island()),
            "170" => Ok(Self::colombia()),
            "188" => Ok(Self::costa_rica()),
            "384" => Ok(Self::coted_ivoire()),
            "191" => Ok(Self::croatia()),
            "192" => Ok(Self::cuba()),
            "531" => Ok(Self::curacao()),
            "196" => Ok(Self::cyprus()),
            "203" => Ok(Self::czechia()),
            "208" => Ok(Self::denmark()),
            "262" => Ok(Self::djibouti()),
            "212" => Ok(Self::dominica()),
            "534" => Ok(Self::dutch_part_sint_maarten()),
            "218" => Ok(Self::ecuador()),
            "818" => Ok(Self::egypt()),
            "222" => Ok(Self::el_salvador()),
            "226" => Ok(Self::equatorial_guinea()),
            "232" => Ok(Self::eritrea()),
            "233" => Ok(Self::estonia()),
            "748" => Ok(Self::eswatini()),
            "231" => Ok(Self::ethiopia()),
            "583" => Ok(Self::federated_states_of_micronesia()),
            "242" => Ok(Self::fiji()),
            "246" => Ok(Self::finland()),
            "250" => Ok(Self::france()),
            "254" => Ok(Self::french_guiana()),
            "663" => Ok(Self::french_part_saint_martin()),
            "258" => Ok(Self::french_polynesia()),
            "266" => Ok(Self::gabon()),
            "268" => Ok(Self::georgia()),
            "276" => Ok(Self::germany()),
            "288" => Ok(Self::ghana()),
            "292" => Ok(Self::gibraltar()),
            "300" => Ok(Self::greece()),
            "304" => Ok(Self::greenland()),
            "308" => Ok(Self::grenada()),
            "312" => Ok(Self::guadeloupe()),
            "316" => Ok(Self::guam()),
            "320" => Ok(Self::guatemala()),
            "831" => Ok(Self::guernsey()),
            "324" => Ok(Self::guinea()),
            "624" => Ok(Self::guinea_bissau()),
            "328" => Ok(Self::guyana()),
            "332" => Ok(Self::haiti()),
            "334" => Ok(Self::heard_island_and_mc_donald_islands()),
            "340" => Ok(Self::honduras()),
            "344" => Ok(Self::hong_kong()),
            "348" => Ok(Self::hungary()),
            "352" => Ok(Self::iceland()),
            "356" => Ok(Self::india()),
            "360" => Ok(Self::indonesia()),
            "368" => Ok(Self::iraq()),
            "372" => Ok(Self::ireland()),
            "364" => Ok(Self::islamic_republic_of_iran()),
            "833" => Ok(Self::isle_of_man()),
            "376" => Ok(Self::israel()),
            "380" => Ok(Self::italy()),
            "388" => Ok(Self::jamaica()),
            "392" => Ok(Self::japan()),
            "832" => Ok(Self::jersey()),
            "400" => Ok(Self::jordan()),
            "398" => Ok(Self::kazakhstan()),
            "404" => Ok(Self::kenya()),
            "296" => Ok(Self::kiribati()),
            "414" => Ok(Self::kuwait()),
            "417" => Ok(Self::kyrgyzstan()),
            "428" => Ok(Self::latvia()),
            "422" => Ok(Self::lebanon()),
            "426" => Ok(Self::lesotho()),
            "430" => Ok(Self::liberia()),
            "434" => Ok(Self::libya()),
            "438" => Ok(Self::liechtenstein()),
            "440" => Ok(Self::lithuania()),
            "442" => Ok(Self::luxembourg()),
            "446" => Ok(Self::macao()),
            "450" => Ok(Self::madagascar()),
            "454" => Ok(Self::malawi()),
            "458" => Ok(Self::malaysia()),
            "462" => Ok(Self::maldives()),
            "466" => Ok(Self::mali()),
            "470" => Ok(Self::malta()),
            "474" => Ok(Self::martinique()),
            "478" => Ok(Self::mauritania()),
            "480" => Ok(Self::mauritius()),
            "175" => Ok(Self::mayotte()),
            "484" => Ok(Self::mexico()),
            "492" => Ok(Self::monaco()),
            "496" => Ok(Self::mongolia()),
            "499" => Ok(Self::montenegro()),
            "500" => Ok(Self::montserrat()),
            "504" => Ok(Self::morocco()),
            "508" => Ok(Self::mozambique()),
            "104" => Ok(Self::myanmar()),
            "516" => Ok(Self::namibia()),
            "520" => Ok(Self::nauru()),
            "524" => Ok(Self::nepal()),
            "540" => Ok(Self::new_caledonia()),
            "554" => Ok(Self::new_zealand()),
            "558" => Ok(Self::nicaragua()),
            "566" => Ok(Self::nigeria()),
            "570" => Ok(Self::niue()),
            "574" => Ok(Self::norfolk_island()),
            "578" => Ok(Self::norway()),
            "512" => Ok(Self::oman()),
            "586" => Ok(Self::pakistan()),
            "585" => Ok(Self::palau()),
            "591" => Ok(Self::panama()),
            "598" => Ok(Self::papua_new_guinea()),
            "600" => Ok(Self::paraguay()),
            "604" => Ok(Self::peru()),
            "612" => Ok(Self::pitcairn()),
            "616" => Ok(Self::poland()),
            "620" => Ok(Self::portugal()),
            "630" => Ok(Self::puerto_rico()),
            "634" => Ok(Self::qatar()),
            "807" => Ok(Self::republic_of_north_macedonia()),
            "638" => Ok(Self::reunion()),
            "642" => Ok(Self::romania()),
            "646" => Ok(Self::rwanda()),
            "652" => Ok(Self::saint_barthelemy()),
            "659" => Ok(Self::saint_kitts_and_nevis()),
            "662" => Ok(Self::saint_lucia()),
            "666" => Ok(Self::saint_pierre_and_miquelon()),
            "670" => Ok(Self::saint_vincent_and_the_grenadines()),
            "882" => Ok(Self::samoa()),
            "674" => Ok(Self::san_marino()),
            "678" => Ok(Self::sao_tome_and_principe()),
            "682" => Ok(Self::saudi_arabia()),
            "686" => Ok(Self::senegal()),
            "688" => Ok(Self::serbia()),
            "690" => Ok(Self::seychelles()),
            "694" => Ok(Self::sierra_leone()),
            "702" => Ok(Self::singapore()),
            "703" => Ok(Self::slovakia()),
            "705" => Ok(Self::slovenia()),
            "090" => Ok(Self::solomon_islands()),
            "706" => Ok(Self::somalia()),
            "710" => Ok(Self::south_africa()),
            "239" => Ok(Self::south_georgia_and_the_south_sandwich_islands()),
            "728" => Ok(Self::south_sudan()),
            "724" => Ok(Self::spain()),
            "144" => Ok(Self::sri_lanka()),
            "275" => Ok(Self::state_of_palestine()),
            "740" => Ok(Self::suriname()),
            "744" => Ok(Self::svalbard_and_jan_mayen()),
            "752" => Ok(Self::sweden()),
            "756" => Ok(Self::switzerland()),
            "760" => Ok(Self::syrian_arab_republic()),
            "158" => Ok(Self::taiwan_province_of_china()),
            "762" => Ok(Self::tajikistan()),
            "764" => Ok(Self::thailand()),
            "044" => Ok(Self::the_bahamas()),
            "136" => Ok(Self::the_cayman_islands()),
            "140" => Ok(Self::the_central_african_republic()),
            "166" => Ok(Self::the_cocos_keeling_islands()),
            "174" => Ok(Self::the_comoros()),
            "178" => Ok(Self::the_congo()),
            "184" => Ok(Self::the_cook_islands()),
            "408" => Ok(Self::the_democratic_peoples_republic_of_korea()),
            "180" => Ok(Self::the_democratic_republic_of_the_congo()),
            "214" => Ok(Self::the_dominican_republic()),
            "238" => Ok(Self::the_falkland_islands_malvinas()),
            "234" => Ok(Self::the_faroe_islands()),
            "260" => Ok(Self::the_french_southern_territories()),
            "270" => Ok(Self::the_gambia()),
            "336" => Ok(Self::the_holy_see()),
            "418" => Ok(Self::the_lao_peoples_democratic_republic()),
            "584" => Ok(Self::the_marshall_islands()),
            "528" => Ok(Self::the_netherlands()),
            "562" => Ok(Self::the_niger()),
            "580" => Ok(Self::the_northern_mariana_islands()),
            "608" => Ok(Self::the_philippines()),
            "410" => Ok(Self::the_republic_of_korea()),
            "498" => Ok(Self::the_republic_of_moldova()),
            "643" => Ok(Self::the_russian_federation()),
            "729" => Ok(Self::the_sudan()),
            "796" => Ok(Self::the_turks_and_caicos_islands()),
            "784" => Ok(Self::the_united_arab_emirates()),
            "826" => Ok(Self::the_united_kingdom_of_great_britain_and_northern_ireland()),
            "581" => Ok(Self::the_united_states_minor_outlying_islands()),
            "840" => Ok(Self::the_united_states_of_america()),
            "626" => Ok(Self::timor_leste()),
            "768" => Ok(Self::togo()),
            "772" => Ok(Self::tokelau()),
            "776" => Ok(Self::tonga()),
            "780" => Ok(Self::trinidad_and_tobago()),
            "788" => Ok(Self::tunisia()),
            "792" => Ok(Self::turkey()),
            "795" => Ok(Self::turkmenistan()),
            "798" => Ok(Self::tuvalu()),
            "850" => Ok(Self::us_virgin_islands()),
            "800" => Ok(Self::uganda()),
            "804" => Ok(Self::ukraine()),
            "834" => Ok(Self::united_republic_of_tanzania()),
            "858" => Ok(Self::uruguay()),
            "860" => Ok(Self::uzbekistan()),
            "548" => Ok(Self::vanuatu()),
            "704" => Ok(Self::vietnam()),
            "876" => Ok(Self::wallis_and_futuna()),
            "732" => Ok(Self::western_sahara()),
            "887" => Ok(Self::yemen()),
            "894" => Ok(Self::zambia()),
            "716" => Ok(Self::zimbabwe()),
            e => Err(format!("Unknown String {}", e)),
        }
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
    pub fn from_alpha2<A: AsRef<str>>(alpha2: A) -> Result<Self, String> {
        match alpha2.as_ref().to_lowercase().as_str() {
            "af" => Ok(Self::afghanistan()),
            "ax" => Ok(Self::aland_islands()),
            "al" => Ok(Self::albania()),
            "dz" => Ok(Self::algeria()),
            "as" => Ok(Self::american_samoa()),
            "ad" => Ok(Self::andorra()),
            "ao" => Ok(Self::angola()),
            "ai" => Ok(Self::anguilla()),
            "aq" => Ok(Self::antarctica()),
            "ag" => Ok(Self::antigua_and_barbuda()),
            "ar" => Ok(Self::argentina()),
            "am" => Ok(Self::armenia()),
            "aw" => Ok(Self::aruba()),
            "sh" => Ok(Self::ascension_and_tristan_da_cunha_saint_helena()),
            "au" => Ok(Self::australia()),
            "at" => Ok(Self::austria()),
            "az" => Ok(Self::azerbaijan()),
            "bh" => Ok(Self::bahrain()),
            "bd" => Ok(Self::bangladesh()),
            "bb" => Ok(Self::barbados()),
            "by" => Ok(Self::belarus()),
            "be" => Ok(Self::belgium()),
            "bz" => Ok(Self::belize()),
            "bj" => Ok(Self::benin()),
            "bm" => Ok(Self::bermuda()),
            "bt" => Ok(Self::bhutan()),
            "ve" => Ok(Self::bolivarian_republic_of_venezuela()),
            "bo" => Ok(Self::bolivia()),
            "bq" => Ok(Self::bonaire()),
            "ba" => Ok(Self::bosnia_and_herzegovina()),
            "bw" => Ok(Self::botswana()),
            "bv" => Ok(Self::bouvet_island()),
            "br" => Ok(Self::brazil()),
            "io" => Ok(Self::british_indian_ocean_territory()),
            "vg" => Ok(Self::british_virgin_islands()),
            "bn" => Ok(Self::brunei_darussalam()),
            "bg" => Ok(Self::bulgaria()),
            "bf" => Ok(Self::burkina_faso()),
            "bi" => Ok(Self::burundi()),
            "cv" => Ok(Self::cabo_verde()),
            "kh" => Ok(Self::cambodia()),
            "cm" => Ok(Self::cameroon()),
            "ca" => Ok(Self::canada()),
            "td" => Ok(Self::chad()),
            "cl" => Ok(Self::chile()),
            "cn" => Ok(Self::china()),
            "cx" => Ok(Self::christmas_island()),
            "co" => Ok(Self::colombia()),
            "cr" => Ok(Self::costa_rica()),
            "ci" => Ok(Self::coted_ivoire()),
            "hr" => Ok(Self::croatia()),
            "cu" => Ok(Self::cuba()),
            "cw" => Ok(Self::curacao()),
            "cy" => Ok(Self::cyprus()),
            "cz" => Ok(Self::czechia()),
            "dk" => Ok(Self::denmark()),
            "dj" => Ok(Self::djibouti()),
            "dm" => Ok(Self::dominica()),
            "sx" => Ok(Self::dutch_part_sint_maarten()),
            "ec" => Ok(Self::ecuador()),
            "eg" => Ok(Self::egypt()),
            "sv" => Ok(Self::el_salvador()),
            "gq" => Ok(Self::equatorial_guinea()),
            "er" => Ok(Self::eritrea()),
            "ee" => Ok(Self::estonia()),
            "sz" => Ok(Self::eswatini()),
            "et" => Ok(Self::ethiopia()),
            "fm" => Ok(Self::federated_states_of_micronesia()),
            "fj" => Ok(Self::fiji()),
            "fi" => Ok(Self::finland()),
            "fr" => Ok(Self::france()),
            "gf" => Ok(Self::french_guiana()),
            "mf" => Ok(Self::french_part_saint_martin()),
            "pf" => Ok(Self::french_polynesia()),
            "ga" => Ok(Self::gabon()),
            "ge" => Ok(Self::georgia()),
            "de" => Ok(Self::germany()),
            "gh" => Ok(Self::ghana()),
            "gi" => Ok(Self::gibraltar()),
            "gr" => Ok(Self::greece()),
            "gl" => Ok(Self::greenland()),
            "gd" => Ok(Self::grenada()),
            "gp" => Ok(Self::guadeloupe()),
            "gu" => Ok(Self::guam()),
            "gt" => Ok(Self::guatemala()),
            "gg" => Ok(Self::guernsey()),
            "gn" => Ok(Self::guinea()),
            "gw" => Ok(Self::guinea_bissau()),
            "gy" => Ok(Self::guyana()),
            "ht" => Ok(Self::haiti()),
            "hm" => Ok(Self::heard_island_and_mc_donald_islands()),
            "hn" => Ok(Self::honduras()),
            "hk" => Ok(Self::hong_kong()),
            "hu" => Ok(Self::hungary()),
            "is" => Ok(Self::iceland()),
            "in" => Ok(Self::india()),
            "id" => Ok(Self::indonesia()),
            "iq" => Ok(Self::iraq()),
            "ie" => Ok(Self::ireland()),
            "ir" => Ok(Self::islamic_republic_of_iran()),
            "im" => Ok(Self::isle_of_man()),
            "il" => Ok(Self::israel()),
            "it" => Ok(Self::italy()),
            "jm" => Ok(Self::jamaica()),
            "jp" => Ok(Self::japan()),
            "je" => Ok(Self::jersey()),
            "jo" => Ok(Self::jordan()),
            "kz" => Ok(Self::kazakhstan()),
            "ke" => Ok(Self::kenya()),
            "ki" => Ok(Self::kiribati()),
            "kw" => Ok(Self::kuwait()),
            "kg" => Ok(Self::kyrgyzstan()),
            "lv" => Ok(Self::latvia()),
            "lb" => Ok(Self::lebanon()),
            "ls" => Ok(Self::lesotho()),
            "lr" => Ok(Self::liberia()),
            "ly" => Ok(Self::libya()),
            "li" => Ok(Self::liechtenstein()),
            "lt" => Ok(Self::lithuania()),
            "lu" => Ok(Self::luxembourg()),
            "mo" => Ok(Self::macao()),
            "mg" => Ok(Self::madagascar()),
            "mw" => Ok(Self::malawi()),
            "my" => Ok(Self::malaysia()),
            "mv" => Ok(Self::maldives()),
            "ml" => Ok(Self::mali()),
            "mt" => Ok(Self::malta()),
            "mq" => Ok(Self::martinique()),
            "mr" => Ok(Self::mauritania()),
            "mu" => Ok(Self::mauritius()),
            "yt" => Ok(Self::mayotte()),
            "mx" => Ok(Self::mexico()),
            "mc" => Ok(Self::monaco()),
            "mn" => Ok(Self::mongolia()),
            "me" => Ok(Self::montenegro()),
            "ms" => Ok(Self::montserrat()),
            "ma" => Ok(Self::morocco()),
            "mz" => Ok(Self::mozambique()),
            "mm" => Ok(Self::myanmar()),
            "na" => Ok(Self::namibia()),
            "nr" => Ok(Self::nauru()),
            "np" => Ok(Self::nepal()),
            "nc" => Ok(Self::new_caledonia()),
            "nz" => Ok(Self::new_zealand()),
            "ni" => Ok(Self::nicaragua()),
            "ng" => Ok(Self::nigeria()),
            "nu" => Ok(Self::niue()),
            "nf" => Ok(Self::norfolk_island()),
            "no" => Ok(Self::norway()),
            "om" => Ok(Self::oman()),
            "pk" => Ok(Self::pakistan()),
            "pw" => Ok(Self::palau()),
            "pa" => Ok(Self::panama()),
            "pg" => Ok(Self::papua_new_guinea()),
            "py" => Ok(Self::paraguay()),
            "pe" => Ok(Self::peru()),
            "pn" => Ok(Self::pitcairn()),
            "pl" => Ok(Self::poland()),
            "pt" => Ok(Self::portugal()),
            "pr" => Ok(Self::puerto_rico()),
            "qa" => Ok(Self::qatar()),
            "mk" => Ok(Self::republic_of_north_macedonia()),
            "re" => Ok(Self::reunion()),
            "ro" => Ok(Self::romania()),
            "rw" => Ok(Self::rwanda()),
            "bl" => Ok(Self::saint_barthelemy()),
            "kn" => Ok(Self::saint_kitts_and_nevis()),
            "lc" => Ok(Self::saint_lucia()),
            "pm" => Ok(Self::saint_pierre_and_miquelon()),
            "vc" => Ok(Self::saint_vincent_and_the_grenadines()),
            "ws" => Ok(Self::samoa()),
            "sm" => Ok(Self::san_marino()),
            "st" => Ok(Self::sao_tome_and_principe()),
            "sa" => Ok(Self::saudi_arabia()),
            "sn" => Ok(Self::senegal()),
            "rs" => Ok(Self::serbia()),
            "sc" => Ok(Self::seychelles()),
            "sl" => Ok(Self::sierra_leone()),
            "sg" => Ok(Self::singapore()),
            "sk" => Ok(Self::slovakia()),
            "si" => Ok(Self::slovenia()),
            "sb" => Ok(Self::solomon_islands()),
            "so" => Ok(Self::somalia()),
            "za" => Ok(Self::south_africa()),
            "gs" => Ok(Self::south_georgia_and_the_south_sandwich_islands()),
            "ss" => Ok(Self::south_sudan()),
            "es" => Ok(Self::spain()),
            "lk" => Ok(Self::sri_lanka()),
            "ps" => Ok(Self::state_of_palestine()),
            "sr" => Ok(Self::suriname()),
            "sj" => Ok(Self::svalbard_and_jan_mayen()),
            "se" => Ok(Self::sweden()),
            "ch" => Ok(Self::switzerland()),
            "sy" => Ok(Self::syrian_arab_republic()),
            "tw" => Ok(Self::taiwan_province_of_china()),
            "tj" => Ok(Self::tajikistan()),
            "th" => Ok(Self::thailand()),
            "bs" => Ok(Self::the_bahamas()),
            "ky" => Ok(Self::the_cayman_islands()),
            "cf" => Ok(Self::the_central_african_republic()),
            "cc" => Ok(Self::the_cocos_keeling_islands()),
            "km" => Ok(Self::the_comoros()),
            "cg" => Ok(Self::the_congo()),
            "ck" => Ok(Self::the_cook_islands()),
            "kp" => Ok(Self::the_democratic_peoples_republic_of_korea()),
            "cd" => Ok(Self::the_democratic_republic_of_the_congo()),
            "do" => Ok(Self::the_dominican_republic()),
            "fk" => Ok(Self::the_falkland_islands_malvinas()),
            "fo" => Ok(Self::the_faroe_islands()),
            "tf" => Ok(Self::the_french_southern_territories()),
            "gm" => Ok(Self::the_gambia()),
            "va" => Ok(Self::the_holy_see()),
            "la" => Ok(Self::the_lao_peoples_democratic_republic()),
            "mh" => Ok(Self::the_marshall_islands()),
            "nl" => Ok(Self::the_netherlands()),
            "ne" => Ok(Self::the_niger()),
            "mp" => Ok(Self::the_northern_mariana_islands()),
            "ph" => Ok(Self::the_philippines()),
            "kr" => Ok(Self::the_republic_of_korea()),
            "md" => Ok(Self::the_republic_of_moldova()),
            "ru" => Ok(Self::the_russian_federation()),
            "sd" => Ok(Self::the_sudan()),
            "tc" => Ok(Self::the_turks_and_caicos_islands()),
            "ae" => Ok(Self::the_united_arab_emirates()),
            "gb" => Ok(Self::the_united_kingdom_of_great_britain_and_northern_ireland()),
            "um" => Ok(Self::the_united_states_minor_outlying_islands()),
            "us" => Ok(Self::the_united_states_of_america()),
            "tl" => Ok(Self::timor_leste()),
            "tg" => Ok(Self::togo()),
            "tk" => Ok(Self::tokelau()),
            "to" => Ok(Self::tonga()),
            "tt" => Ok(Self::trinidad_and_tobago()),
            "tn" => Ok(Self::tunisia()),
            "tr" => Ok(Self::turkey()),
            "tm" => Ok(Self::turkmenistan()),
            "tv" => Ok(Self::tuvalu()),
            "vi" => Ok(Self::us_virgin_islands()),
            "ug" => Ok(Self::uganda()),
            "ua" => Ok(Self::ukraine()),
            "tz" => Ok(Self::united_republic_of_tanzania()),
            "uy" => Ok(Self::uruguay()),
            "uz" => Ok(Self::uzbekistan()),
            "vu" => Ok(Self::vanuatu()),
            "vn" => Ok(Self::vietnam()),
            "wf" => Ok(Self::wallis_and_futuna()),
            "eh" => Ok(Self::western_sahara()),
            "ye" => Ok(Self::yemen()),
            "zm" => Ok(Self::zambia()),
            "zw" => Ok(Self::zimbabwe()),
            e => Err(format!("Unknown String {}", e)),
        }
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
    pub fn from_alpha3<A: AsRef<str>>(alpha3: A) -> Result<Self, String> {
        match alpha3.as_ref().to_lowercase().as_str() {
            "afg" => Ok(Self::afghanistan()),
            "ala" => Ok(Self::aland_islands()),
            "alb" => Ok(Self::albania()),
            "dza" => Ok(Self::algeria()),
            "asm" => Ok(Self::american_samoa()),
            "and" => Ok(Self::andorra()),
            "ago" => Ok(Self::angola()),
            "aia" => Ok(Self::anguilla()),
            "ata" => Ok(Self::antarctica()),
            "atg" => Ok(Self::antigua_and_barbuda()),
            "arg" => Ok(Self::argentina()),
            "arm" => Ok(Self::armenia()),
            "abw" => Ok(Self::aruba()),
            "shn" => Ok(Self::ascension_and_tristan_da_cunha_saint_helena()),
            "aus" => Ok(Self::australia()),
            "aut" => Ok(Self::austria()),
            "aze" => Ok(Self::azerbaijan()),
            "bhr" => Ok(Self::bahrain()),
            "bgd" => Ok(Self::bangladesh()),
            "brb" => Ok(Self::barbados()),
            "blr" => Ok(Self::belarus()),
            "bel" => Ok(Self::belgium()),
            "blz" => Ok(Self::belize()),
            "ben" => Ok(Self::benin()),
            "bmu" => Ok(Self::bermuda()),
            "btn" => Ok(Self::bhutan()),
            "ven" => Ok(Self::bolivarian_republic_of_venezuela()),
            "bol" => Ok(Self::bolivia()),
            "bes" => Ok(Self::bonaire()),
            "bih" => Ok(Self::bosnia_and_herzegovina()),
            "bwa" => Ok(Self::botswana()),
            "bvt" => Ok(Self::bouvet_island()),
            "bra" => Ok(Self::brazil()),
            "iot" => Ok(Self::british_indian_ocean_territory()),
            "vgb" => Ok(Self::british_virgin_islands()),
            "brn" => Ok(Self::brunei_darussalam()),
            "bgr" => Ok(Self::bulgaria()),
            "bfa" => Ok(Self::burkina_faso()),
            "bdi" => Ok(Self::burundi()),
            "cpv" => Ok(Self::cabo_verde()),
            "khm" => Ok(Self::cambodia()),
            "cmr" => Ok(Self::cameroon()),
            "can" => Ok(Self::canada()),
            "tcd" => Ok(Self::chad()),
            "chl" => Ok(Self::chile()),
            "chn" => Ok(Self::china()),
            "cxr" => Ok(Self::christmas_island()),
            "col" => Ok(Self::colombia()),
            "cri" => Ok(Self::costa_rica()),
            "civ" => Ok(Self::coted_ivoire()),
            "hrv" => Ok(Self::croatia()),
            "cub" => Ok(Self::cuba()),
            "cuw" => Ok(Self::curacao()),
            "cyp" => Ok(Self::cyprus()),
            "cze" => Ok(Self::czechia()),
            "dnk" => Ok(Self::denmark()),
            "dji" => Ok(Self::djibouti()),
            "dma" => Ok(Self::dominica()),
            "sxm" => Ok(Self::dutch_part_sint_maarten()),
            "ecu" => Ok(Self::ecuador()),
            "egy" => Ok(Self::egypt()),
            "slv" => Ok(Self::el_salvador()),
            "gnq" => Ok(Self::equatorial_guinea()),
            "eri" => Ok(Self::eritrea()),
            "est" => Ok(Self::estonia()),
            "swz" => Ok(Self::eswatini()),
            "eth" => Ok(Self::ethiopia()),
            "fsm" => Ok(Self::federated_states_of_micronesia()),
            "fji" => Ok(Self::fiji()),
            "fin" => Ok(Self::finland()),
            "fra" => Ok(Self::france()),
            "guf" => Ok(Self::french_guiana()),
            "maf" => Ok(Self::french_part_saint_martin()),
            "pyf" => Ok(Self::french_polynesia()),
            "gab" => Ok(Self::gabon()),
            "geo" => Ok(Self::georgia()),
            "deu" => Ok(Self::germany()),
            "gha" => Ok(Self::ghana()),
            "gib" => Ok(Self::gibraltar()),
            "grc" => Ok(Self::greece()),
            "grl" => Ok(Self::greenland()),
            "grd" => Ok(Self::grenada()),
            "glp" => Ok(Self::guadeloupe()),
            "gum" => Ok(Self::guam()),
            "gtm" => Ok(Self::guatemala()),
            "ggy" => Ok(Self::guernsey()),
            "gin" => Ok(Self::guinea()),
            "gnb" => Ok(Self::guinea_bissau()),
            "guy" => Ok(Self::guyana()),
            "hti" => Ok(Self::haiti()),
            "hmd" => Ok(Self::heard_island_and_mc_donald_islands()),
            "hnd" => Ok(Self::honduras()),
            "hkg" => Ok(Self::hong_kong()),
            "hun" => Ok(Self::hungary()),
            "isl" => Ok(Self::iceland()),
            "ind" => Ok(Self::india()),
            "idn" => Ok(Self::indonesia()),
            "irq" => Ok(Self::iraq()),
            "irl" => Ok(Self::ireland()),
            "irn" => Ok(Self::islamic_republic_of_iran()),
            "imn" => Ok(Self::isle_of_man()),
            "isr" => Ok(Self::israel()),
            "ita" => Ok(Self::italy()),
            "jam" => Ok(Self::jamaica()),
            "jpn" => Ok(Self::japan()),
            "jey" => Ok(Self::jersey()),
            "jor" => Ok(Self::jordan()),
            "kaz" => Ok(Self::kazakhstan()),
            "ken" => Ok(Self::kenya()),
            "kir" => Ok(Self::kiribati()),
            "kwt" => Ok(Self::kuwait()),
            "kgz" => Ok(Self::kyrgyzstan()),
            "lva" => Ok(Self::latvia()),
            "lbn" => Ok(Self::lebanon()),
            "lso" => Ok(Self::lesotho()),
            "lbr" => Ok(Self::liberia()),
            "lby" => Ok(Self::libya()),
            "lie" => Ok(Self::liechtenstein()),
            "ltu" => Ok(Self::lithuania()),
            "lux" => Ok(Self::luxembourg()),
            "mac" => Ok(Self::macao()),
            "mdg" => Ok(Self::madagascar()),
            "mwi" => Ok(Self::malawi()),
            "mys" => Ok(Self::malaysia()),
            "mdv" => Ok(Self::maldives()),
            "mli" => Ok(Self::mali()),
            "mlt" => Ok(Self::malta()),
            "mtq" => Ok(Self::martinique()),
            "mrt" => Ok(Self::mauritania()),
            "mus" => Ok(Self::mauritius()),
            "myt" => Ok(Self::mayotte()),
            "mex" => Ok(Self::mexico()),
            "mco" => Ok(Self::monaco()),
            "mng" => Ok(Self::mongolia()),
            "mne" => Ok(Self::montenegro()),
            "msr" => Ok(Self::montserrat()),
            "mar" => Ok(Self::morocco()),
            "moz" => Ok(Self::mozambique()),
            "mmr" => Ok(Self::myanmar()),
            "nam" => Ok(Self::namibia()),
            "nru" => Ok(Self::nauru()),
            "npl" => Ok(Self::nepal()),
            "ncl" => Ok(Self::new_caledonia()),
            "nzl" => Ok(Self::new_zealand()),
            "nic" => Ok(Self::nicaragua()),
            "nga" => Ok(Self::nigeria()),
            "niu" => Ok(Self::niue()),
            "nfk" => Ok(Self::norfolk_island()),
            "nor" => Ok(Self::norway()),
            "omn" => Ok(Self::oman()),
            "pak" => Ok(Self::pakistan()),
            "plw" => Ok(Self::palau()),
            "pan" => Ok(Self::panama()),
            "png" => Ok(Self::papua_new_guinea()),
            "pry" => Ok(Self::paraguay()),
            "per" => Ok(Self::peru()),
            "pcn" => Ok(Self::pitcairn()),
            "pol" => Ok(Self::poland()),
            "prt" => Ok(Self::portugal()),
            "pri" => Ok(Self::puerto_rico()),
            "qat" => Ok(Self::qatar()),
            "mkd" => Ok(Self::republic_of_north_macedonia()),
            "reu" => Ok(Self::reunion()),
            "rou" => Ok(Self::romania()),
            "rwa" => Ok(Self::rwanda()),
            "blm" => Ok(Self::saint_barthelemy()),
            "kna" => Ok(Self::saint_kitts_and_nevis()),
            "lca" => Ok(Self::saint_lucia()),
            "spm" => Ok(Self::saint_pierre_and_miquelon()),
            "vct" => Ok(Self::saint_vincent_and_the_grenadines()),
            "wsm" => Ok(Self::samoa()),
            "smr" => Ok(Self::san_marino()),
            "stp" => Ok(Self::sao_tome_and_principe()),
            "sau" => Ok(Self::saudi_arabia()),
            "sen" => Ok(Self::senegal()),
            "srb" => Ok(Self::serbia()),
            "syc" => Ok(Self::seychelles()),
            "sle" => Ok(Self::sierra_leone()),
            "sgp" => Ok(Self::singapore()),
            "svk" => Ok(Self::slovakia()),
            "svn" => Ok(Self::slovenia()),
            "slb" => Ok(Self::solomon_islands()),
            "som" => Ok(Self::somalia()),
            "zaf" => Ok(Self::south_africa()),
            "sgs" => Ok(Self::south_georgia_and_the_south_sandwich_islands()),
            "ssd" => Ok(Self::south_sudan()),
            "esp" => Ok(Self::spain()),
            "lka" => Ok(Self::sri_lanka()),
            "pse" => Ok(Self::state_of_palestine()),
            "sur" => Ok(Self::suriname()),
            "sjm" => Ok(Self::svalbard_and_jan_mayen()),
            "swe" => Ok(Self::sweden()),
            "che" => Ok(Self::switzerland()),
            "syr" => Ok(Self::syrian_arab_republic()),
            "twn" => Ok(Self::taiwan_province_of_china()),
            "tjk" => Ok(Self::tajikistan()),
            "tha" => Ok(Self::thailand()),
            "bhs" => Ok(Self::the_bahamas()),
            "cym" => Ok(Self::the_cayman_islands()),
            "caf" => Ok(Self::the_central_african_republic()),
            "cck" => Ok(Self::the_cocos_keeling_islands()),
            "com" => Ok(Self::the_comoros()),
            "cog" => Ok(Self::the_congo()),
            "cok" => Ok(Self::the_cook_islands()),
            "prk" => Ok(Self::the_democratic_peoples_republic_of_korea()),
            "cod" => Ok(Self::the_democratic_republic_of_the_congo()),
            "dom" => Ok(Self::the_dominican_republic()),
            "flk" => Ok(Self::the_falkland_islands_malvinas()),
            "fro" => Ok(Self::the_faroe_islands()),
            "atf" => Ok(Self::the_french_southern_territories()),
            "gmb" => Ok(Self::the_gambia()),
            "vat" => Ok(Self::the_holy_see()),
            "lao" => Ok(Self::the_lao_peoples_democratic_republic()),
            "mhl" => Ok(Self::the_marshall_islands()),
            "nld" => Ok(Self::the_netherlands()),
            "ner" => Ok(Self::the_niger()),
            "mnp" => Ok(Self::the_northern_mariana_islands()),
            "phl" => Ok(Self::the_philippines()),
            "kor" => Ok(Self::the_republic_of_korea()),
            "mda" => Ok(Self::the_republic_of_moldova()),
            "rus" => Ok(Self::the_russian_federation()),
            "sdn" => Ok(Self::the_sudan()),
            "tca" => Ok(Self::the_turks_and_caicos_islands()),
            "are" => Ok(Self::the_united_arab_emirates()),
            "gbr" => Ok(Self::the_united_kingdom_of_great_britain_and_northern_ireland()),
            "umi" => Ok(Self::the_united_states_minor_outlying_islands()),
            "usa" => Ok(Self::the_united_states_of_america()),
            "tls" => Ok(Self::timor_leste()),
            "tgo" => Ok(Self::togo()),
            "tkl" => Ok(Self::tokelau()),
            "ton" => Ok(Self::tonga()),
            "tto" => Ok(Self::trinidad_and_tobago()),
            "tun" => Ok(Self::tunisia()),
            "tur" => Ok(Self::turkey()),
            "tkm" => Ok(Self::turkmenistan()),
            "tuv" => Ok(Self::tuvalu()),
            "vir" => Ok(Self::us_virgin_islands()),
            "uga" => Ok(Self::uganda()),
            "ukr" => Ok(Self::ukraine()),
            "tza" => Ok(Self::united_republic_of_tanzania()),
            "ury" => Ok(Self::uruguay()),
            "uzb" => Ok(Self::uzbekistan()),
            "vut" => Ok(Self::vanuatu()),
            "vnm" => Ok(Self::vietnam()),
            "wlf" => Ok(Self::wallis_and_futuna()),
            "esh" => Ok(Self::western_sahara()),
            "yem" => Ok(Self::yemen()),
            "zmb" => Ok(Self::zambia()),
            "zwe" => Ok(Self::zimbabwe()),
            e => Err(format!("Unknown String {}", e)),
        }
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
    pub fn from_alias<A: AsRef<str>>(alias: A) -> Result<Self, String> {
        match alias.as_ref().to_lowercase().as_str() {
            "samoa" => Ok(Self::american_samoa()),
            "sthelena" | "sainthelena" => Ok(Self::ascension_and_tristan_da_cunha_saint_helena()),
            "venezuela" => Ok(Self::bolivarian_republic_of_venezuela()),
            "bosnia" | "herzegovina" => Ok(Self::bosnia_and_herzegovina()),
            "brunei" => Ok(Self::brunei_darussalam()),
            "burkina" => Ok(Self::burkina_faso()),
            "stmaarten" | "sintmaarten" => Ok(Self::dutch_part_sint_maarten()),
            "micronesia" => Ok(Self::federated_states_of_micronesia()),
            "stmartin" | "saintmartin" => Ok(Self::french_part_saint_martin()),
            "heardisland" | "mcdonaldislands" => Ok(Self::heard_island_and_mc_donald_islands()),
            "iran" => Ok(Self::islamic_republic_of_iran()),
            "macedonia" => Ok(Self::republic_of_north_macedonia()),
            "stbarthelemy" => Ok(Self::saint_barthelemy()),
            "stkitts" => Ok(Self::saint_kitts_and_nevis()),
            "stlucia" => Ok(Self::saint_lucia()),
            "stpierre" | "saintpierre" => Ok(Self::saint_pierre_and_miquelon()),
            "stvincent" | "saintvincent" => Ok(Self::saint_vincent_and_the_grenadines()),
            "saotome" => Ok(Self::sao_tome_and_principe()),
            "southgeorgia" | "southsandwichislands" => {
                Ok(Self::south_georgia_and_the_south_sandwich_islands())
            }
            "palestine" => Ok(Self::state_of_palestine()),
            "taiwan" => Ok(Self::taiwan_province_of_china()),
            "bahamas" => Ok(Self::the_bahamas()),
            "caymanislands" => Ok(Self::the_cayman_islands()),
            "centralafricanrepublic" => Ok(Self::the_central_african_republic()),
            "cocosislands" | "keelingislands" => Ok(Self::the_cocos_keeling_islands()),
            "comoros" => Ok(Self::the_comoros()),
            "congo" => Ok(Self::the_congo()),
            "cookislands" => Ok(Self::the_cook_islands()),
            "northkorea" | "democraticpeoplesrepublicofkorea" => {
                Ok(Self::the_democratic_peoples_republic_of_korea())
            }
            "democraticrepublicofthecongo" => Ok(Self::the_democratic_republic_of_the_congo()),
            "dominicanrepublic" => Ok(Self::the_dominican_republic()),
            "malvinas" | "falklandislands" => Ok(Self::the_falkland_islands_malvinas()),
            "faroeislands" => Ok(Self::the_faroe_islands()),
            "frenchsouthernterritories" => Ok(Self::the_french_southern_territories()),
            "gabmia" => Ok(Self::the_gambia()),
            "holysee" => Ok(Self::the_holy_see()),
            "laopeoplesdemocraticrepublic" => Ok(Self::the_lao_peoples_democratic_republic()),
            "marshallislands" => Ok(Self::the_marshall_islands()),
            "netherlands" | "holland" => Ok(Self::the_netherlands()),
            "niger" => Ok(Self::the_niger()),
            "northernmarianaislands" => Ok(Self::the_northern_mariana_islands()),
            "philippines" => Ok(Self::the_philippines()),
            "southkorea" | "republicofkorea" => Ok(Self::the_republic_of_korea()),
            "moldova" | "republicofmoldova" => Ok(Self::the_republic_of_moldova()),
            "russia" | "russianfederation" => Ok(Self::the_russian_federation()),
            "sudan" => Ok(Self::the_sudan()),
            "turksandcaicosislands" => Ok(Self::the_turks_and_caicos_islands()),
            "unitedarabemirates" => Ok(Self::the_united_arab_emirates()),
            "england"
            | "scotland"
            | "greatbritain"
            | "unitedkingdom"
            | "northernireland"
            | "unitedkingdomofgreatbritain"
            | "unitedkingdomofgreatbritainandnorthernireland" => {
                Ok(Self::the_united_kingdom_of_great_britain_and_northern_ireland())
            }
            "unitedstatesminoroutlyingislands" => {
                Ok(Self::the_united_states_minor_outlying_islands())
            }
            "america" | "unitedstates" | "unitedstatesofamerica" => Ok(Self::the_united_states_of_america()),
            "trinidad" | "tobago" => Ok(Self::trinidad_and_tobago()),
            "tanzania" => Ok(Self::united_republic_of_tanzania()),
            e => Err(format!("Unknown Alias {}", e)),
        }
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
    pub fn from_name<A: AsRef<str>>(name: A) -> Result<Self, String> {
        match name.as_ref().to_lowercase().as_str() {
            "afghanistan" => Ok(Self::afghanistan()),
            "alandislands" => Ok(Self::aland_islands()),
            "albania" => Ok(Self::albania()),
            "algeria" => Ok(Self::algeria()),
            "americansamoa" => Ok(Self::american_samoa()),
            "andorra" => Ok(Self::andorra()),
            "angola" => Ok(Self::angola()),
            "anguilla" => Ok(Self::anguilla()),
            "antarctica" => Ok(Self::antarctica()),
            "antiguaandbarbuda" => Ok(Self::antigua_and_barbuda()),
            "argentina" => Ok(Self::argentina()),
            "armenia" => Ok(Self::armenia()),
            "aruba" => Ok(Self::aruba()),
            "ascensionandtristandacunhasainthelena" => {
                Ok(Self::ascension_and_tristan_da_cunha_saint_helena())
            }
            "australia" => Ok(Self::australia()),
            "austria" => Ok(Self::austria()),
            "azerbaijan" => Ok(Self::azerbaijan()),
            "bahrain" => Ok(Self::bahrain()),
            "bangladesh" => Ok(Self::bangladesh()),
            "barbados" => Ok(Self::barbados()),
            "belarus" => Ok(Self::belarus()),
            "belgium" => Ok(Self::belgium()),
            "belize" => Ok(Self::belize()),
            "benin" => Ok(Self::benin()),
            "bermuda" => Ok(Self::bermuda()),
            "bhutan" => Ok(Self::bhutan()),
            "bolivarianrepublicofvenezuela" => Ok(Self::bolivarian_republic_of_venezuela()),
            "bolivia" => Ok(Self::bolivia()),
            "bonaire" => Ok(Self::bonaire()),
            "bosniaandherzegovina" => Ok(Self::bosnia_and_herzegovina()),
            "botswana" => Ok(Self::botswana()),
            "bouvetisland" => Ok(Self::bouvet_island()),
            "brazil" => Ok(Self::brazil()),
            "britishindianoceanterritory" => Ok(Self::british_indian_ocean_territory()),
            "britishvirginislands" => Ok(Self::british_virgin_islands()),
            "bruneidarussalam" => Ok(Self::brunei_darussalam()),
            "bulgaria" => Ok(Self::bulgaria()),
            "burkinafaso" => Ok(Self::burkina_faso()),
            "burundi" => Ok(Self::burundi()),
            "caboverde" => Ok(Self::cabo_verde()),
            "cambodia" => Ok(Self::cambodia()),
            "cameroon" => Ok(Self::cameroon()),
            "canada" => Ok(Self::canada()),
            "chad" => Ok(Self::chad()),
            "chile" => Ok(Self::chile()),
            "china" => Ok(Self::china()),
            "christmasisland" => Ok(Self::christmas_island()),
            "colombia" => Ok(Self::colombia()),
            "costarica" => Ok(Self::costa_rica()),
            "cotedivoire" => Ok(Self::coted_ivoire()),
            "croatia" => Ok(Self::croatia()),
            "cuba" => Ok(Self::cuba()),
            "curacao" => Ok(Self::curacao()),
            "cyprus" => Ok(Self::cyprus()),
            "czechia" => Ok(Self::czechia()),
            "denmark" => Ok(Self::denmark()),
            "djibouti" => Ok(Self::djibouti()),
            "dominica" => Ok(Self::dominica()),
            "dutchpartsintmaarten" => Ok(Self::dutch_part_sint_maarten()),
            "ecuador" => Ok(Self::ecuador()),
            "egypt" => Ok(Self::egypt()),
            "elsalvador" => Ok(Self::el_salvador()),
            "equatorialguinea" => Ok(Self::equatorial_guinea()),
            "eritrea" => Ok(Self::eritrea()),
            "estonia" => Ok(Self::estonia()),
            "eswatini" => Ok(Self::eswatini()),
            "ethiopia" => Ok(Self::ethiopia()),
            "federatedstatesofmicronesia" => Ok(Self::federated_states_of_micronesia()),
            "fiji" => Ok(Self::fiji()),
            "finland" => Ok(Self::finland()),
            "france" => Ok(Self::france()),
            "frenchguiana" => Ok(Self::french_guiana()),
            "frenchpartsaintmartin" => Ok(Self::french_part_saint_martin()),
            "frenchpolynesia" => Ok(Self::french_polynesia()),
            "gabon" => Ok(Self::gabon()),
            "georgia" => Ok(Self::georgia()),
            "germany" => Ok(Self::germany()),
            "ghana" => Ok(Self::ghana()),
            "gibraltar" => Ok(Self::gibraltar()),
            "greece" => Ok(Self::greece()),
            "greenland" => Ok(Self::greenland()),
            "grenada" => Ok(Self::grenada()),
            "guadeloupe" => Ok(Self::guadeloupe()),
            "guam" => Ok(Self::guam()),
            "guatemala" => Ok(Self::guatemala()),
            "guernsey" => Ok(Self::guernsey()),
            "guinea" => Ok(Self::guinea()),
            "guineabissau" => Ok(Self::guinea_bissau()),
            "guyana" => Ok(Self::guyana()),
            "haiti" => Ok(Self::haiti()),
            "heardislandandmcdonaldislands" => Ok(Self::heard_island_and_mc_donald_islands()),
            "honduras" => Ok(Self::honduras()),
            "hongkong" => Ok(Self::hong_kong()),
            "hungary" => Ok(Self::hungary()),
            "iceland" => Ok(Self::iceland()),
            "india" => Ok(Self::india()),
            "indonesia" => Ok(Self::indonesia()),
            "iraq" => Ok(Self::iraq()),
            "ireland" => Ok(Self::ireland()),
            "islamicrepublicofiran" => Ok(Self::islamic_republic_of_iran()),
            "isleofman" => Ok(Self::isle_of_man()),
            "israel" => Ok(Self::israel()),
            "italy" => Ok(Self::italy()),
            "jamaica" => Ok(Self::jamaica()),
            "japan" => Ok(Self::japan()),
            "jersey" => Ok(Self::jersey()),
            "jordan" => Ok(Self::jordan()),
            "kazakhstan" => Ok(Self::kazakhstan()),
            "kenya" => Ok(Self::kenya()),
            "kiribati" => Ok(Self::kiribati()),
            "kuwait" => Ok(Self::kuwait()),
            "kyrgyzstan" => Ok(Self::kyrgyzstan()),
            "latvia" => Ok(Self::latvia()),
            "lebanon" => Ok(Self::lebanon()),
            "lesotho" => Ok(Self::lesotho()),
            "liberia" => Ok(Self::liberia()),
            "libya" => Ok(Self::libya()),
            "liechtenstein" => Ok(Self::liechtenstein()),
            "lithuania" => Ok(Self::lithuania()),
            "luxembourg" => Ok(Self::luxembourg()),
            "macao" => Ok(Self::macao()),
            "madagascar" => Ok(Self::madagascar()),
            "malawi" => Ok(Self::malawi()),
            "malaysia" => Ok(Self::malaysia()),
            "maldives" => Ok(Self::maldives()),
            "mali" => Ok(Self::mali()),
            "malta" => Ok(Self::malta()),
            "martinique" => Ok(Self::martinique()),
            "mauritania" => Ok(Self::mauritania()),
            "mauritius" => Ok(Self::mauritius()),
            "mayotte" => Ok(Self::mayotte()),
            "mexico" => Ok(Self::mexico()),
            "monaco" => Ok(Self::monaco()),
            "mongolia" => Ok(Self::mongolia()),
            "montenegro" => Ok(Self::montenegro()),
            "montserrat" => Ok(Self::montserrat()),
            "morocco" => Ok(Self::morocco()),
            "mozambique" => Ok(Self::mozambique()),
            "myanmar" => Ok(Self::myanmar()),
            "namibia" => Ok(Self::namibia()),
            "nauru" => Ok(Self::nauru()),
            "nepal" => Ok(Self::nepal()),
            "newcaledonia" => Ok(Self::new_caledonia()),
            "newzealand" => Ok(Self::new_zealand()),
            "nicaragua" => Ok(Self::nicaragua()),
            "nigeria" => Ok(Self::nigeria()),
            "niue" => Ok(Self::niue()),
            "norfolkisland" => Ok(Self::norfolk_island()),
            "norway" => Ok(Self::norway()),
            "oman" => Ok(Self::oman()),
            "pakistan" => Ok(Self::pakistan()),
            "palau" => Ok(Self::palau()),
            "panama" => Ok(Self::panama()),
            "papuanewguinea" => Ok(Self::papua_new_guinea()),
            "paraguay" => Ok(Self::paraguay()),
            "peru" => Ok(Self::peru()),
            "pitcairn" => Ok(Self::pitcairn()),
            "poland" => Ok(Self::poland()),
            "portugal" => Ok(Self::portugal()),
            "puertorico" => Ok(Self::puerto_rico()),
            "qatar" => Ok(Self::qatar()),
            "republicofnorthmacedonia" => Ok(Self::republic_of_north_macedonia()),
            "reunion" => Ok(Self::reunion()),
            "romania" => Ok(Self::romania()),
            "rwanda" => Ok(Self::rwanda()),
            "saintbarthelemy" => Ok(Self::saint_barthelemy()),
            "saintkittsandnevis" => Ok(Self::saint_kitts_and_nevis()),
            "saintlucia" => Ok(Self::saint_lucia()),
            "saintpierreandmiquelon" => Ok(Self::saint_pierre_and_miquelon()),
            "saintvincentandthegrenadines" => Ok(Self::saint_vincent_and_the_grenadines()),
            "samoa" => Ok(Self::samoa()),
            "sanmarino" => Ok(Self::san_marino()),
            "saotomeandprincipe" => Ok(Self::sao_tome_and_principe()),
            "saudiarabia" => Ok(Self::saudi_arabia()),
            "senegal" => Ok(Self::senegal()),
            "serbia" => Ok(Self::serbia()),
            "seychelles" => Ok(Self::seychelles()),
            "sierraleone" => Ok(Self::sierra_leone()),
            "singapore" => Ok(Self::singapore()),
            "slovakia" => Ok(Self::slovakia()),
            "slovenia" => Ok(Self::slovenia()),
            "solomonislands" => Ok(Self::solomon_islands()),
            "somalia" => Ok(Self::somalia()),
            "southafrica" => Ok(Self::south_africa()),
            "southgeorgiaandthesouthsandwichislands" => {
                Ok(Self::south_georgia_and_the_south_sandwich_islands())
            }
            "southsudan" => Ok(Self::south_sudan()),
            "spain" => Ok(Self::spain()),
            "srilanka" => Ok(Self::sri_lanka()),
            "stateofpalestine" => Ok(Self::state_of_palestine()),
            "suriname" => Ok(Self::suriname()),
            "svalbardandjanmayen" => Ok(Self::svalbard_and_jan_mayen()),
            "sweden" => Ok(Self::sweden()),
            "switzerland" => Ok(Self::switzerland()),
            "syrianarabrepublic" => Ok(Self::syrian_arab_republic()),
            "taiwanprovinceofchina" => Ok(Self::taiwan_province_of_china()),
            "tajikistan" => Ok(Self::tajikistan()),
            "thailand" => Ok(Self::thailand()),
            "thebahamas" => Ok(Self::the_bahamas()),
            "thecaymanislands" => Ok(Self::the_cayman_islands()),
            "thecentralafricanrepublic" => Ok(Self::the_central_african_republic()),
            "thecocoskeelingislands" => Ok(Self::the_cocos_keeling_islands()),
            "thecomoros" => Ok(Self::the_comoros()),
            "thecongo" => Ok(Self::the_congo()),
            "thecookislands" => Ok(Self::the_cook_islands()),
            "thedemocraticpeoplesrepublicofkorea" => {
                Ok(Self::the_democratic_peoples_republic_of_korea())
            }
            "thedemocraticrepublicofthecongo" => Ok(Self::the_democratic_republic_of_the_congo()),
            "thedominicanrepublic" => Ok(Self::the_dominican_republic()),
            "thefalklandislandsmalvinas" => Ok(Self::the_falkland_islands_malvinas()),
            "thefaroeislands" => Ok(Self::the_faroe_islands()),
            "thefrenchsouthernterritories" => Ok(Self::the_french_southern_territories()),
            "thegambia" => Ok(Self::the_gambia()),
            "theholysee" => Ok(Self::the_holy_see()),
            "thelaopeoplesdemocraticrepublic" => Ok(Self::the_lao_peoples_democratic_republic()),
            "themarshallislands" => Ok(Self::the_marshall_islands()),
            "thenetherlands" => Ok(Self::the_netherlands()),
            "theniger" => Ok(Self::the_niger()),
            "thenorthernmarianaislands" => Ok(Self::the_northern_mariana_islands()),
            "thephilippines" => Ok(Self::the_philippines()),
            "therepublicofkorea" => Ok(Self::the_republic_of_korea()),
            "therepublicofmoldova" => Ok(Self::the_republic_of_moldova()),
            "therussianfederation" => Ok(Self::the_russian_federation()),
            "thesudan" => Ok(Self::the_sudan()),
            "theturksandcaicosislands" => Ok(Self::the_turks_and_caicos_islands()),
            "theunitedarabemirates" => Ok(Self::the_united_arab_emirates()),
            "theunitedkingdomofgreatbritainandnorthernireland" => {
                Ok(Self::the_united_kingdom_of_great_britain_and_northern_ireland())
            }
            "theunitedstatesminoroutlyingislands" => {
                Ok(Self::the_united_states_minor_outlying_islands())
            }
            "theunitedstatesofamerica" => Ok(Self::the_united_states_of_america()),
            "timorleste" => Ok(Self::timor_leste()),
            "togo" => Ok(Self::togo()),
            "tokelau" => Ok(Self::tokelau()),
            "tonga" => Ok(Self::tonga()),
            "trinidadandtobago" => Ok(Self::trinidad_and_tobago()),
            "tunisia" => Ok(Self::tunisia()),
            "turkey" => Ok(Self::turkey()),
            "turkmenistan" => Ok(Self::turkmenistan()),
            "tuvalu" => Ok(Self::tuvalu()),
            "usvirginislands" => Ok(Self::us_virgin_islands()),
            "uganda" => Ok(Self::uganda()),
            "ukraine" => Ok(Self::ukraine()),
            "unitedrepublicoftanzania" => Ok(Self::united_republic_of_tanzania()),
            "uruguay" => Ok(Self::uruguay()),
            "uzbekistan" => Ok(Self::uzbekistan()),
            "vanuatu" => Ok(Self::vanuatu()),
            "vietnam" => Ok(Self::vietnam()),
            "wallisandfutuna" => Ok(Self::wallis_and_futuna()),
            "westernsahara" => Ok(Self::western_sahara()),
            "yemen" => Ok(Self::yemen()),
            "zambia" => Ok(Self::zambia()),
            "zimbabwe" => Ok(Self::zimbabwe()),
            e => Err(format!("Unknown String {}", e)),
        }
    }
}

impl FromStr for Country {
    type Err = String;

    fn from_str(code: &str) -> Result<Self, String> {
        match code.to_lowercase().as_str() {
            "afghanistan" | "004" | "af" | "afg" => Ok(Self::afghanistan()),
            "alandislands" | "aland_islands" | "248" | "ax" | "ala" => Ok(Self::aland_islands()),
            "albania" | "008" | "al" | "alb" => Ok(Self::albania()),
            "algeria" | "012" | "dz" | "dza" => Ok(Self::algeria()),
            "americansamoa" | "american_samoa" | "016" | "as" | "asm" => Ok(Self::american_samoa()),
            "andorra" | "020" | "ad" | "and" => Ok(Self::andorra()),
            "angola" | "024" | "ao" | "ago" => Ok(Self::angola()),
            "anguilla" | "660" | "ai" | "aia" => Ok(Self::anguilla()),
            "antarctica" | "010" | "aq" | "ata" => Ok(Self::antarctica()),
            "antiguaandbarbuda" | "antigua_and_barbuda" | "028" | "ag" | "atg" => {
                Ok(Self::antigua_and_barbuda())
            }
            "argentina" | "032" | "ar" | "arg" => Ok(Self::argentina()),
            "armenia" | "051" | "am" | "arm" => Ok(Self::armenia()),
            "aruba" | "533" | "aw" | "abw" => Ok(Self::aruba()),
            "ascensionandtristandacunhasainthelena"
            | "ascension_and_tristan_da_cunha_saint_helena"
            | "654"
            | "sh"
            | "shn"
            | "sthelena"
            | "sainthelena" => Ok(Self::ascension_and_tristan_da_cunha_saint_helena()),
            "australia" | "036" | "au" | "aus" => Ok(Self::australia()),
            "austria" | "040" | "at" | "aut" => Ok(Self::austria()),
            "azerbaijan" | "031" | "az" | "aze" => Ok(Self::azerbaijan()),
            "bahrain" | "048" | "bh" | "bhr" => Ok(Self::bahrain()),
            "bangladesh" | "050" | "bd" | "bgd" => Ok(Self::bangladesh()),
            "barbados" | "052" | "bb" | "brb" => Ok(Self::barbados()),
            "belarus" | "112" | "by" | "blr" => Ok(Self::belarus()),
            "belgium" | "056" | "be" | "bel" => Ok(Self::belgium()),
            "belize" | "084" | "bz" | "blz" => Ok(Self::belize()),
            "benin" | "204" | "bj" | "ben" => Ok(Self::benin()),
            "bermuda" | "060" | "bm" | "bmu" => Ok(Self::bermuda()),
            "bhutan" | "064" | "bt" | "btn" => Ok(Self::bhutan()),
            "bolivarianrepublicofvenezuela"
            | "bolivarian_republic_of_venezuela"
            | "862"
            | "ve"
            | "ven"
            | "venezuela" => Ok(Self::bolivarian_republic_of_venezuela()),
            "bolivia" | "068" | "bo" | "bol" => Ok(Self::bolivia()),
            "bonaire" | "535" | "bq" | "bes" => Ok(Self::bonaire()),
            "bosniaandherzegovina"
            | "bosnia_and_herzegovina"
            | "070"
            | "ba"
            | "bih"
            | "bosnia"
            | "herzegovina" => Ok(Self::bosnia_and_herzegovina()),
            "botswana" | "072" | "bw" | "bwa" => Ok(Self::botswana()),
            "bouvetisland" | "bouvet_island" | "074" | "bv" | "bvt" => Ok(Self::bouvet_island()),
            "brazil" | "076" | "br" | "bra" => Ok(Self::brazil()),
            "britishindianoceanterritory"
            | "british_indian_ocean_territory"
            | "086"
            | "io"
            | "iot" => Ok(Self::british_indian_ocean_territory()),
            "britishvirginislands" | "british_virgin_islands" | "092" | "vg" | "vgb" => {
                Ok(Self::british_virgin_islands())
            }
            "bruneidarussalam" | "brunei_darussalam" | "096" | "bn" | "brn" | "brunei" => {
                Ok(Self::brunei_darussalam())
            }
            "bulgaria" | "100" | "bg" | "bgr" => Ok(Self::bulgaria()),
            "burkinafaso" | "burkina_faso" | "854" | "bf" | "bfa" | "burkina" => {
                Ok(Self::burkina_faso())
            }
            "burundi" | "108" | "bi" | "bdi" => Ok(Self::burundi()),
            "caboverde" | "cabo_verde" | "132" | "cv" | "cpv" => Ok(Self::cabo_verde()),
            "cambodia" | "116" | "kh" | "khm" => Ok(Self::cambodia()),
            "cameroon" | "120" | "cm" | "cmr" => Ok(Self::cameroon()),
            "canada" | "124" | "ca" | "can" => Ok(Self::canada()),
            "chad" | "148" | "td" | "tcd" => Ok(Self::chad()),
            "chile" | "152" | "cl" | "chl" => Ok(Self::chile()),
            "china" | "156" | "cn" | "chn" => Ok(Self::china()),
            "christmasisland" | "christmas_island" | "162" | "cx" | "cxr" => {
                Ok(Self::christmas_island())
            }
            "colombia" | "170" | "co" | "col" => Ok(Self::colombia()),
            "costarica" | "costa_rica" | "188" | "cr" | "cri" => Ok(Self::costa_rica()),
            "cotedivoire" | "coted_ivoire" | "384" | "ci" | "civ" => Ok(Self::coted_ivoire()),
            "croatia" | "191" | "hr" | "hrv" => Ok(Self::croatia()),
            "cuba" | "192" | "cu" | "cub" => Ok(Self::cuba()),
            "curacao" | "531" | "cw" | "cuw" => Ok(Self::curacao()),
            "cyprus" | "196" | "cy" | "cyp" => Ok(Self::cyprus()),
            "czechia" | "203" | "cz" | "cze" => Ok(Self::czechia()),
            "denmark" | "208" | "dk" | "dnk" => Ok(Self::denmark()),
            "djibouti" | "262" | "dj" | "dji" => Ok(Self::djibouti()),
            "dominica" | "212" | "dm" | "dma" => Ok(Self::dominica()),
            "dutchpartsintmaarten"
            | "dutch_part_sint_maarten"
            | "534"
            | "sx"
            | "sxm"
            | "stmaarten"
            | "sintmaarten" => Ok(Self::dutch_part_sint_maarten()),
            "ecuador" | "218" | "ec" | "ecu" => Ok(Self::ecuador()),
            "egypt" | "818" | "eg" | "egy" => Ok(Self::egypt()),
            "elsalvador" | "el_salvador" | "222" | "sv" | "slv" => Ok(Self::el_salvador()),
            "equatorialguinea" | "equatorial_guinea" | "226" | "gq" | "gnq" => {
                Ok(Self::equatorial_guinea())
            }
            "eritrea" | "232" | "er" | "eri" => Ok(Self::eritrea()),
            "estonia" | "233" | "ee" | "est" => Ok(Self::estonia()),
            "eswatini" | "748" | "sz" | "swz" => Ok(Self::eswatini()),
            "ethiopia" | "231" | "et" | "eth" => Ok(Self::ethiopia()),
            "federatedstatesofmicronesia"
            | "federated_states_of_micronesia"
            | "583"
            | "fm"
            | "fsm"
            | "micronesia" => Ok(Self::federated_states_of_micronesia()),
            "fiji" | "242" | "fj" | "fji" => Ok(Self::fiji()),
            "finland" | "246" | "fi" | "fin" => Ok(Self::finland()),
            "france" | "250" | "fr" | "fra" => Ok(Self::france()),
            "frenchguiana" | "french_guiana" | "254" | "gf" | "guf" => Ok(Self::french_guiana()),
            "frenchpartsaintmartin"
            | "french_part_saint_martin"
            | "663"
            | "mf"
            | "maf"
            | "stmartin"
            | "saintmartin" => Ok(Self::french_part_saint_martin()),
            "frenchpolynesia" | "french_polynesia" | "258" | "pf" | "pyf" => {
                Ok(Self::french_polynesia())
            }
            "gabon" | "266" | "ga" | "gab" => Ok(Self::gabon()),
            "georgia" | "268" | "ge" | "geo" => Ok(Self::georgia()),
            "germany" | "276" | "de" | "deu" => Ok(Self::germany()),
            "ghana" | "288" | "gh" | "gha" => Ok(Self::ghana()),
            "gibraltar" | "292" | "gi" | "gib" => Ok(Self::gibraltar()),
            "greece" | "300" | "gr" | "grc" => Ok(Self::greece()),
            "greenland" | "304" | "gl" | "grl" => Ok(Self::greenland()),
            "grenada" | "308" | "gd" | "grd" => Ok(Self::grenada()),
            "guadeloupe" | "312" | "gp" | "glp" => Ok(Self::guadeloupe()),
            "guam" | "316" | "gu" | "gum" => Ok(Self::guam()),
            "guatemala" | "320" | "gt" | "gtm" => Ok(Self::guatemala()),
            "guernsey" | "831" | "gg" | "ggy" => Ok(Self::guernsey()),
            "guinea" | "324" | "gn" | "gin" => Ok(Self::guinea()),
            "guineabissau" | "guinea_bissau" | "624" | "gw" | "gnb" => Ok(Self::guinea_bissau()),
            "guyana" | "328" | "gy" | "guy" => Ok(Self::guyana()),
            "haiti" | "332" | "ht" | "hti" => Ok(Self::haiti()),
            "heardislandandmcdonaldislands"
            | "heard_island_and_mc_donald_islands"
            | "334"
            | "hm"
            | "hmd"
            | "heardisland"
            | "mcdonaldislands" => Ok(Self::heard_island_and_mc_donald_islands()),
            "honduras" | "340" | "hn" | "hnd" => Ok(Self::honduras()),
            "hongkong" | "hong_kong" | "344" | "hk" | "hkg" => Ok(Self::hong_kong()),
            "hungary" | "348" | "hu" | "hun" => Ok(Self::hungary()),
            "iceland" | "352" | "is" | "isl" => Ok(Self::iceland()),
            "india" | "356" | "in" | "ind" => Ok(Self::india()),
            "indonesia" | "360" | "id" | "idn" => Ok(Self::indonesia()),
            "iraq" | "368" | "iq" | "irq" => Ok(Self::iraq()),
            "ireland" | "372" | "ie" | "irl" => Ok(Self::ireland()),
            "islamicrepublicofiran"
            | "islamic_republic_of_iran"
            | "364"
            | "ir"
            | "irn"
            | "iran" => Ok(Self::islamic_republic_of_iran()),
            "isleofman" | "isle_of_man" | "833" | "im" | "imn" => Ok(Self::isle_of_man()),
            "israel" | "376" | "il" | "isr" => Ok(Self::israel()),
            "italy" | "380" | "it" | "ita" => Ok(Self::italy()),
            "jamaica" | "388" | "jm" | "jam" => Ok(Self::jamaica()),
            "japan" | "392" | "jp" | "jpn" => Ok(Self::japan()),
            "jersey" | "832" | "je" | "jey" => Ok(Self::jersey()),
            "jordan" | "400" | "jo" | "jor" => Ok(Self::jordan()),
            "kazakhstan" | "398" | "kz" | "kaz" => Ok(Self::kazakhstan()),
            "kenya" | "404" | "ke" | "ken" => Ok(Self::kenya()),
            "kiribati" | "296" | "ki" | "kir" => Ok(Self::kiribati()),
            "kuwait" | "414" | "kw" | "kwt" => Ok(Self::kuwait()),
            "kyrgyzstan" | "417" | "kg" | "kgz" => Ok(Self::kyrgyzstan()),
            "latvia" | "428" | "lv" | "lva" => Ok(Self::latvia()),
            "lebanon" | "422" | "lb" | "lbn" => Ok(Self::lebanon()),
            "lesotho" | "426" | "ls" | "lso" => Ok(Self::lesotho()),
            "liberia" | "430" | "lr" | "lbr" => Ok(Self::liberia()),
            "libya" | "434" | "ly" | "lby" => Ok(Self::libya()),
            "liechtenstein" | "438" | "li" | "lie" => Ok(Self::liechtenstein()),
            "lithuania" | "440" | "lt" | "ltu" => Ok(Self::lithuania()),
            "luxembourg" | "442" | "lu" | "lux" => Ok(Self::luxembourg()),
            "macao" | "446" | "mo" | "mac" => Ok(Self::macao()),
            "madagascar" | "450" | "mg" | "mdg" => Ok(Self::madagascar()),
            "malawi" | "454" | "mw" | "mwi" => Ok(Self::malawi()),
            "malaysia" | "458" | "my" | "mys" => Ok(Self::malaysia()),
            "maldives" | "462" | "mv" | "mdv" => Ok(Self::maldives()),
            "mali" | "466" | "ml" | "mli" => Ok(Self::mali()),
            "malta" | "470" | "mt" | "mlt" => Ok(Self::malta()),
            "martinique" | "474" | "mq" | "mtq" => Ok(Self::martinique()),
            "mauritania" | "478" | "mr" | "mrt" => Ok(Self::mauritania()),
            "mauritius" | "480" | "mu" | "mus" => Ok(Self::mauritius()),
            "mayotte" | "175" | "yt" | "myt" => Ok(Self::mayotte()),
            "mexico" | "484" | "mx" | "mex" => Ok(Self::mexico()),
            "monaco" | "492" | "mc" | "mco" => Ok(Self::monaco()),
            "mongolia" | "496" | "mn" | "mng" => Ok(Self::mongolia()),
            "montenegro" | "499" | "me" | "mne" => Ok(Self::montenegro()),
            "montserrat" | "500" | "ms" | "msr" => Ok(Self::montserrat()),
            "morocco" | "504" | "ma" | "mar" => Ok(Self::morocco()),
            "mozambique" | "508" | "mz" | "moz" => Ok(Self::mozambique()),
            "myanmar" | "104" | "mm" | "mmr" => Ok(Self::myanmar()),
            "namibia" | "516" | "na" | "nam" => Ok(Self::namibia()),
            "nauru" | "520" | "nr" | "nru" => Ok(Self::nauru()),
            "nepal" | "524" | "np" | "npl" => Ok(Self::nepal()),
            "newcaledonia" | "new_caledonia" | "540" | "nc" | "ncl" => Ok(Self::new_caledonia()),
            "newzealand" | "new_zealand" | "554" | "nz" | "nzl" => Ok(Self::new_zealand()),
            "nicaragua" | "558" | "ni" | "nic" => Ok(Self::nicaragua()),
            "nigeria" | "566" | "ng" | "nga" => Ok(Self::nigeria()),
            "niue" | "570" | "nu" | "niu" => Ok(Self::niue()),
            "norfolkisland" | "norfolk_island" | "574" | "nf" | "nfk" => Ok(Self::norfolk_island()),
            "norway" | "578" | "no" | "nor" => Ok(Self::norway()),
            "oman" | "512" | "om" | "omn" => Ok(Self::oman()),
            "pakistan" | "586" | "pk" | "pak" => Ok(Self::pakistan()),
            "palau" | "585" | "pw" | "plw" => Ok(Self::palau()),
            "panama" | "591" | "pa" | "pan" => Ok(Self::panama()),
            "papuanewguinea" | "papua_new_guinea" | "598" | "pg" | "png" => {
                Ok(Self::papua_new_guinea())
            }
            "paraguay" | "600" | "py" | "pry" => Ok(Self::paraguay()),
            "peru" | "604" | "pe" | "per" => Ok(Self::peru()),
            "pitcairn" | "612" | "pn" | "pcn" => Ok(Self::pitcairn()),
            "poland" | "616" | "pl" | "pol" => Ok(Self::poland()),
            "portugal" | "620" | "pt" | "prt" => Ok(Self::portugal()),
            "puertorico" | "puerto_rico" | "630" | "pr" | "pri" => Ok(Self::puerto_rico()),
            "qatar" | "634" | "qa" | "qat" => Ok(Self::qatar()),
            "republicofnorthmacedonia"
            | "republic_of_north_macedonia"
            | "807"
            | "mk"
            | "mkd"
            | "macedonia" => Ok(Self::republic_of_north_macedonia()),
            "reunion" | "638" | "re" | "reu" => Ok(Self::reunion()),
            "romania" | "642" | "ro" | "rou" => Ok(Self::romania()),
            "rwanda" | "646" | "rw" | "rwa" => Ok(Self::rwanda()),
            "saintbarthelemy" | "saint_barthelemy" | "652" | "bl" | "blm" | "stbarthelemy" => {
                Ok(Self::saint_barthelemy())
            }
            "saintkittsandnevis" | "saint_kitts_and_nevis" | "659" | "kn" | "kna" | "stkitts" => {
                Ok(Self::saint_kitts_and_nevis())
            }
            "saintlucia" | "saint_lucia" | "662" | "lc" | "lca" | "stlucia" => {
                Ok(Self::saint_lucia())
            }
            "saintpierreandmiquelon"
            | "saint_pierre_and_miquelon"
            | "666"
            | "pm"
            | "spm"
            | "stpierre"
            | "saintpierre" => Ok(Self::saint_pierre_and_miquelon()),
            "saintvincentandthegrenadines"
            | "saint_vincent_and_the_grenadines"
            | "670"
            | "vc"
            | "vct"
            | "stvincent"
            | "saintvincent" => Ok(Self::saint_vincent_and_the_grenadines()),
            "samoa" | "882" | "ws" | "wsm" => Ok(Self::samoa()),
            "sanmarino" | "san_marino" | "674" | "sm" | "smr" => Ok(Self::san_marino()),
            "saotomeandprincipe" | "sao_tome_and_principe" | "678" | "st" | "stp" | "saotome" => {
                Ok(Self::sao_tome_and_principe())
            }
            "saudiarabia" | "saudi_arabia" | "682" | "sa" | "sau" => Ok(Self::saudi_arabia()),
            "senegal" | "686" | "sn" | "sen" => Ok(Self::senegal()),
            "serbia" | "688" | "rs" | "srb" => Ok(Self::serbia()),
            "seychelles" | "690" | "sc" | "syc" => Ok(Self::seychelles()),
            "sierraleone" | "sierra_leone" | "694" | "sl" | "sle" => Ok(Self::sierra_leone()),
            "singapore" | "702" | "sg" | "sgp" => Ok(Self::singapore()),
            "slovakia" | "703" | "sk" | "svk" => Ok(Self::slovakia()),
            "slovenia" | "705" | "si" | "svn" => Ok(Self::slovenia()),
            "solomonislands" | "solomon_islands" | "090" | "sb" | "slb" => {
                Ok(Self::solomon_islands())
            }
            "somalia" | "706" | "so" | "som" => Ok(Self::somalia()),
            "southafrica" | "south_africa" | "710" | "za" | "zaf" => Ok(Self::south_africa()),
            "southgeorgiaandthesouthsandwichislands"
            | "south_georgia_and_the_south_sandwich_islands"
            | "239"
            | "gs"
            | "sgs"
            | "southgeorgia"
            | "southsandwichislands" => Ok(Self::south_georgia_and_the_south_sandwich_islands()),
            "southsudan" | "south_sudan" | "728" | "ss" | "ssd" => Ok(Self::south_sudan()),
            "spain" | "724" | "es" | "esp" => Ok(Self::spain()),
            "srilanka" | "sri_lanka" | "144" | "lk" | "lka" => Ok(Self::sri_lanka()),
            "stateofpalestine" | "state_of_palestine" | "275" | "ps" | "pse" | "palestine" => {
                Ok(Self::state_of_palestine())
            }
            "suriname" | "740" | "sr" | "sur" => Ok(Self::suriname()),
            "svalbardandjanmayen" | "svalbard_and_jan_mayen" | "744" | "sj" | "sjm" => {
                Ok(Self::svalbard_and_jan_mayen())
            }
            "sweden" | "752" | "se" | "swe" => Ok(Self::sweden()),
            "switzerland" | "756" | "ch" | "che" => Ok(Self::switzerland()),
            "syrianarabrepublic" | "syrian_arab_republic" | "760" | "sy" | "syr" => {
                Ok(Self::syrian_arab_republic())
            }
            "taiwanprovinceofchina"
            | "taiwan_province_of_china"
            | "158"
            | "tw"
            | "twn"
            | "taiwan" => Ok(Self::taiwan_province_of_china()),
            "tajikistan" | "762" | "tj" | "tjk" => Ok(Self::tajikistan()),
            "thailand" | "764" | "th" | "tha" => Ok(Self::thailand()),
            "thebahamas" | "the_bahamas" | "044" | "bs" | "bhs" | "bahamas" => {
                Ok(Self::the_bahamas())
            }
            "thecaymanislands" | "the_cayman_islands" | "136" | "ky" | "cym" | "caymanislands" => {
                Ok(Self::the_cayman_islands())
            }
            "thecentralafricanrepublic"
            | "the_central_african_republic"
            | "140"
            | "cf"
            | "caf"
            | "centralafricanrepublic" => Ok(Self::the_central_african_republic()),
            "thecocoskeelingislands"
            | "the_cocos_keeling_islands"
            | "166"
            | "cc"
            | "cck"
            | "cocosislands"
            | "keelingislands" => Ok(Self::the_cocos_keeling_islands()),
            "thecomoros" | "the_comoros" | "174" | "km" | "com" | "comoros" => {
                Ok(Self::the_comoros())
            }
            "thecongo" | "the_congo" | "178" | "cg" | "cog" | "congo" => Ok(Self::the_congo()),
            "thecookislands" | "the_cook_islands" | "184" | "ck" | "cok" | "cookislands" => {
                Ok(Self::the_cook_islands())
            }
            "thedemocraticpeoplesrepublicofkorea"
            | "the_democratic_peoples_republic_of_korea"
            | "408"
            | "kp"
            | "prk"
            | "northkorea"
            | "democraticpeoplesrepublicofkorea" => {
                Ok(Self::the_democratic_peoples_republic_of_korea())
            }
            "thedemocraticrepublicofthecongo"
            | "the_democratic_republic_of_the_congo"
            | "180"
            | "cd"
            | "cod"
            | "democraticrepublicofthecongo" => Ok(Self::the_democratic_republic_of_the_congo()),
            "thedominicanrepublic"
            | "the_dominican_republic"
            | "214"
            | "do"
            | "dom"
            | "dominicanrepublic" => Ok(Self::the_dominican_republic()),
            "thefalklandislandsmalvinas"
            | "the_falkland_islands_malvinas"
            | "238"
            | "fk"
            | "flk"
            | "malvinas"
            | "falklandislands" => Ok(Self::the_falkland_islands_malvinas()),
            "thefaroeislands" | "the_faroe_islands" | "234" | "fo" | "fro" | "faroeislands" => {
                Ok(Self::the_faroe_islands())
            }
            "thefrenchsouthernterritories"
            | "the_french_southern_territories"
            | "260"
            | "tf"
            | "atf"
            | "frenchsouthernterritories" => Ok(Self::the_french_southern_territories()),
            "thegambia" | "the_gambia" | "270" | "gm" | "gmb" | "gabmia" => Ok(Self::the_gambia()),
            "theholysee" | "the_holy_see" | "336" | "va" | "vat" | "holysee" => {
                Ok(Self::the_holy_see())
            }
            "thelaopeoplesdemocraticrepublic"
            | "the_lao_peoples_democratic_republic"
            | "418"
            | "la"
            | "lao"
            | "laopeoplesdemocraticrepublic" => Ok(Self::the_lao_peoples_democratic_republic()),
            "themarshallislands"
            | "the_marshall_islands"
            | "584"
            | "mh"
            | "mhl"
            | "marshallislands" => Ok(Self::the_marshall_islands()),
            "thenetherlands" | "the_netherlands" | "528" | "nl" | "nld" | "netherlands"
            | "holland" => Ok(Self::the_netherlands()),
            "theniger" | "the_niger" | "562" | "ne" | "ner" | "niger" => Ok(Self::the_niger()),
            "thenorthernmarianaislands"
            | "the_northern_mariana_islands"
            | "580"
            | "mp"
            | "mnp"
            | "northernmarianaislands" => Ok(Self::the_northern_mariana_islands()),
            "thephilippines" | "the_philippines" | "608" | "ph" | "phl" | "philippines" => {
                Ok(Self::the_philippines())
            }
            "therepublicofkorea"
            | "the_republic_of_korea"
            | "410"
            | "kr"
            | "kor"
            | "southkorea"
            | "republicofkorea" => Ok(Self::the_republic_of_korea()),
            "therepublicofmoldova"
            | "the_republic_of_moldova"
            | "498"
            | "md"
            | "mda"
            | "moldova"
            | "republicofmoldova" => Ok(Self::the_republic_of_moldova()),
            "therussianfederation"
            | "the_russian_federation"
            | "643"
            | "ru"
            | "rus"
            | "russia"
            | "russianfederation" => Ok(Self::the_russian_federation()),
            "thesudan" | "the_sudan" | "729" | "sd" | "sdn" | "sudan" => Ok(Self::the_sudan()),
            "theturksandcaicosislands"
            | "the_turks_and_caicos_islands"
            | "796"
            | "tc"
            | "tca"
            | "turksandcaicosislands" => Ok(Self::the_turks_and_caicos_islands()),
            "theunitedarabemirates"
            | "the_united_arab_emirates"
            | "784"
            | "ae"
            | "are"
            | "unitedarabemirates" => Ok(Self::the_united_arab_emirates()),
            "theunitedkingdomofgreatbritainandnorthernireland"
            | "the_united_kingdom_of_great_britain_and_northern_ireland"
            | "826"
            | "gb"
            | "gbr"
            | "england"
            | "scotland"
            | "greatbritain"
            | "unitedkingdom"
            | "northernireland"
            | "unitedkingdomofgreatbritain"
            | "unitedkingdomofgreatbritainandnorthernireland" => {
                Ok(Self::the_united_kingdom_of_great_britain_and_northern_ireland())
            }
            "theunitedstatesminoroutlyingislands"
            | "the_united_states_minor_outlying_islands"
            | "581"
            | "um"
            | "umi"
            | "unitedstatesminoroutlyingislands" => {
                Ok(Self::the_united_states_minor_outlying_islands())
            }
            "theunitedstatesofamerica"
            | "the_united_states_of_america"
            | "840"
            | "us"
            | "usa"
            | "america"
            | "united states"
            | "unitedstatesofamerica" => Ok(Self::the_united_states_of_america()),
            "timorleste" | "timor_leste" | "626" | "tl" | "tls" => Ok(Self::timor_leste()),
            "togo" | "768" | "tg" | "tgo" => Ok(Self::togo()),
            "tokelau" | "772" | "tk" | "tkl" => Ok(Self::tokelau()),
            "tonga" | "776" | "to" | "ton" => Ok(Self::tonga()),
            "trinidadandtobago"
            | "trinidad_and_tobago"
            | "780"
            | "tt"
            | "tto"
            | "trinidad"
            | "tobago" => Ok(Self::trinidad_and_tobago()),
            "tunisia" | "788" | "tn" | "tun" => Ok(Self::tunisia()),
            "turkey" | "792" | "tr" | "tur" => Ok(Self::turkey()),
            "turkmenistan" | "795" | "tm" | "tkm" => Ok(Self::turkmenistan()),
            "tuvalu" | "798" | "tv" | "tuv" => Ok(Self::tuvalu()),
            "usvirginislands" | "us_virgin_islands" | "850" | "vi" | "vir" => {
                Ok(Self::us_virgin_islands())
            }
            "uganda" | "800" | "ug" | "uga" => Ok(Self::uganda()),
            "ukraine" | "804" | "ua" | "ukr" => Ok(Self::ukraine()),
            "unitedrepublicoftanzania"
            | "united_republic_of_tanzania"
            | "834"
            | "tz"
            | "tza"
            | "tanzania" => Ok(Self::united_republic_of_tanzania()),
            "uruguay" | "858" | "uy" | "ury" => Ok(Self::uruguay()),
            "uzbekistan" | "860" | "uz" | "uzb" => Ok(Self::uzbekistan()),
            "vanuatu" | "548" | "vu" | "vut" => Ok(Self::vanuatu()),
            "vietnam" | "704" | "vn" | "vnm" => Ok(Self::vietnam()),
            "wallisandfutuna" | "wallis_and_futuna" | "876" | "wf" | "wlf" => {
                Ok(Self::wallis_and_futuna())
            }
            "westernsahara" | "western_sahara" | "732" | "eh" | "esh" => Ok(Self::western_sahara()),
            "yemen" | "887" | "ye" | "yem" => Ok(Self::yemen()),
            "zambia" | "894" | "zm" | "zmb" => Ok(Self::zambia()),
            "zimbabwe" | "716" | "zw" | "zwe" => Ok(Self::zimbabwe()),
            e => Err(format!("Unknown String {}", e)),
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.long_name.replace(" ", ""))
    }
}
