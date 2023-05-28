use serde::Deserialize;

#[cfg(feature = "async-graphql")]
use async_graphql::{Enum, SimpleObject};

/// https://developers.google.com/maps/documentation/places/web-service/details#PlacesDetailsStatus
#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum Response {
    /// `OK` indicating the API request was successful.
    /// Includes `PlaceResult` in `result`.
    #[serde(rename = "OK")]
    Ok { result: PlaceResult },

    /// `ZERO_RESULTS` indicating that the referenced location, place_id, was valid but no longer refers to a valid result.
    /// This may occur if the establishment is no longer in business.
    #[serde(rename = "ZERO_RESULTS")]
    ZeroResults,

    /// `INVALID_REQUEST` indicating the API request was malformed
    #[serde(rename = "INVALID_REQUEST")]
    InvalidRequest,

    /// `OVER_QUERY_LIMIT` indicating any of the following:
    /// - You have exceeded the QPS limits.
    /// - Billing has not been enabled on your account.
    /// - The monthly $200 credit, or a self-imposed usage cap, has been exceeded.
    /// - The provided method of payment is no longer valid (for example, a credit card has expired).
    #[serde(rename = "OVER_QUERY_LIMIT")]
    OverQueryLimit,

    /// `REQUEST_DENIED` indicating that your request was denied, generally because:
    /// - The request is missing an API key.
    /// - The `key` parameter is invalid.
    #[serde(rename = "REQUEST_DENIED")]
    RequestDenied { error_message: String },

    /// `UNKNOWN_ERROR` indicating an unknown error
    #[serde(rename = "UNKNOWN_ERROR")]
    UnknownError,
}

/// Attributes describing a place
/// https://developers.google.com/maps/documentation/places/web-service/details#Place
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(SimpleObject))]
pub struct PlaceResult {
    pub address_components: Option<Vec<AddressComponent>>,
    pub adr_address: Option<String>,
    pub formatted_address: Option<String>,
    pub geometry: Option<PlaceGeometry>,
    pub icon: Option<String>,
    pub icon_background_color: Option<String>,
    pub icon_mask_base_uri: Option<String>,
    pub name: Option<String>,
    pub place_id: Option<String>,
    pub reference: Option<String>,
    pub types: Option<Vec<AddressType>>,
    pub url: Option<String>,
    pub utc_offset: Option<i32>,
    pub vicinity: Option<String>,
    pub photos: Option<Vec<PlacePhoto>>,
}

impl PlaceResult {
    /// Obtain address component based on `AddressType`
    pub fn address_component(&self, address_type: &AddressType) -> Option<&AddressComponent> {
        match self.address_components.as_ref() {
            Some(ac) => ac
                .iter()
                .find(|&c| match c.types.iter().find(|&t| t == address_type) {
                    Some(_) => true,
                    None => false,
                }),
            None => None,
        }
    }

    /// Obtain street number from `address_components`
    pub fn street_number(&self) -> Option<&str> {
        match self.address_component(&AddressType::StreetNumber) {
            Some(ac) => Some(&ac.long_name),
            None => None,
        }
    }

    /// Obtain route from `address_components`
    pub fn route(&self) -> Option<&str> {
        match self.address_component(&AddressType::Route) {
            Some(ac) => Some(&ac.long_name),
            None => None,
        }
    }

    /// Obtain sublocality from `address_components`
    pub fn sublocality(&self) -> Option<&str> {
        match self.address_component(&AddressType::Sublocality) {
            Some(ac) => Some(&ac.long_name),
            None => None,
        }
    }

    /// Obtain postal code from `address_components`
    pub fn postal_code(&self) -> Option<&str> {
        match self.address_component(&AddressType::PostalCode) {
            Some(ac) => Some(&ac.long_name),
            None => None,
        }
    }

    /// Obtain city from `address_components`
    pub fn city(&self) -> Option<&str> {
        match self.address_component(&AddressType::Locality) {
            Some(ac) => Some(&ac.long_name),
            None => None,
        }
    }

    /// Obtain state from `address_components`
    pub fn state(&self) -> Option<&str> {
        match self.address_component(&AddressType::AdministrativeAreaLevel1) {
            Some(ac) => Some(&ac.long_name),
            None => None,
        }
    }

    /// Obtain country from `address_components`
    pub fn country(&self) -> Option<&str> {
        match self.address_component(&AddressType::Country) {
            Some(ac) => Some(&ac.long_name),
            None => None,
        }
    }

    /// Obtain country code from `address_components`
    pub fn country_code(&self) -> Option<&str> {
        match self.address_component(&AddressType::Country) {
            Some(ac) => Some(&ac.short_name),
            None => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(SimpleObject))]
pub struct PlaceGeometry {
    pub location: PlaceLatLng,
    pub viewport: Viewport,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(SimpleObject))]
pub struct Viewport {
    pub northeast: PlaceLatLng,
    pub southwest: PlaceLatLng,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(SimpleObject))]
pub struct PlaceLatLng {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(SimpleObject))]
pub struct AddressComponent {
    long_name: String,
    short_name: String,
    types: Vec<AddressType>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(SimpleObject))]
pub struct PlacePhoto {
    pub height: i32,
    pub width: i32,
    pub html_attributions: Vec<String>,
    pub photo_reference: Option<String>,
}

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "async-graphql", derive(Enum))]
pub enum AddressType {
    #[serde(rename = "accounting")]
    Accounting,

    #[serde(rename = "airport")]
    Airport,

    #[serde(rename = "amusement_park")]
    AmusementPark,

    #[serde(rename = "aquarium")]
    Aquarium,

    #[serde(rename = "art_gallery")]
    ArtGallery,

    #[serde(rename = "atm")]
    Atm,

    #[serde(rename = "bakery")]
    Bakery,

    #[serde(rename = "bank")]
    Bank,

    #[serde(rename = "bar")]
    Bar,

    #[serde(rename = "beauty_salon")]
    BeautySalon,

    #[serde(rename = "bicycle_store")]
    BicycleStore,

    #[serde(rename = "book_store")]
    BookStore,

    #[serde(rename = "bowling_alley")]
    BowlingAlley,

    #[serde(rename = "bus_station")]
    BusStation,

    #[serde(rename = "cafe")]
    Cafe,

    #[serde(rename = "campground")]
    Campground,

    #[serde(rename = "car_dealer")]
    CarDealer,

    #[serde(rename = "car_rental")]
    CarRental,

    #[serde(rename = "car_repair")]
    CarRepair,

    #[serde(rename = "car_wash")]
    CarWash,

    #[serde(rename = "casino")]
    Casino,

    #[serde(rename = "cemetery")]
    Cemetery,

    #[serde(rename = "church")]
    Church,

    #[serde(rename = "city_hall")]
    CityHall,

    #[serde(rename = "clothing_store")]
    ClothingStore,

    #[serde(rename = "convenience_store")]
    ConvenienceStore,

    #[serde(rename = "courthouse")]
    Courthouse,

    #[serde(rename = "dentist")]
    Dentist,

    #[serde(rename = "department_store")]
    DepartmentStore,

    #[serde(rename = "doctor")]
    Doctor,

    #[serde(rename = "drugstore")]
    Drugstore,

    #[serde(rename = "electrician")]
    Electrician,

    #[serde(rename = "electronics_store")]
    ElectronicsStore,

    #[serde(rename = "embassy")]
    Embassy,

    #[serde(rename = "fire_station")]
    FireStation,

    #[serde(rename = "florist")]
    Florist,

    #[serde(rename = "funeral_home")]
    FuneralHome,

    #[serde(rename = "furniture_store")]
    FurnitureStore,

    #[serde(rename = "gas_station")]
    GasStation,

    #[serde(rename = "gym")]
    Gym,

    #[serde(rename = "hair_care")]
    HairCare,

    #[serde(rename = "hardware_store")]
    HardwareStore,

    #[serde(rename = "hindu_temple")]
    HinduTemple,

    #[serde(rename = "home_goods_store")]
    HomeGoodsStore,

    #[serde(rename = "hospital")]
    Hospital,

    #[serde(rename = "insurance_agency")]
    InsuranceAgency,

    #[serde(rename = "jewelry_store")]
    JewelryStore,

    #[serde(rename = "laundry")]
    Laundry,

    #[serde(rename = "lawyer")]
    Lawyer,

    #[serde(rename = "library")]
    Library,

    #[serde(rename = "light_rail_station")]
    LightRailStation,

    #[serde(rename = "liquor_store")]
    LiquorStore,

    #[serde(rename = "local_government_office")]
    LocalGovernmentOffice,

    #[serde(rename = "locksmith")]
    Locksmith,

    #[serde(rename = "lodging")]
    Lodging,

    #[serde(rename = "meal_delivery")]
    MealDelivery,

    #[serde(rename = "meal_takeaway")]
    MealTakeaway,

    #[serde(rename = "mosque")]
    Mosque,

    #[serde(rename = "movie_rental")]
    MovieRental,

    #[serde(rename = "movie_theater")]
    MovieTheater,

    #[serde(rename = "moving_company")]
    MovingCompany,

    #[serde(rename = "museum")]
    Museum,

    #[serde(rename = "night_club")]
    NightClub,

    #[serde(rename = "painter")]
    Painter,

    #[serde(rename = "park")]
    Park,

    #[serde(rename = "parking")]
    Parking,

    #[serde(rename = "pet_store")]
    PetStore,

    #[serde(rename = "pharmacy")]
    Pharmacy,

    #[serde(rename = "physiotherapist")]
    Physiotherapist,

    #[serde(rename = "plumber")]
    Plumber,

    #[serde(rename = "police")]
    Police,

    #[serde(rename = "post_office")]
    PostOffice,

    #[serde(rename = "primary_school")]
    PrimarySchool,

    #[serde(rename = "real_estate_agency")]
    RealEstateAgency,

    #[serde(rename = "restaurant")]
    Restaurant,

    #[serde(rename = "roofing_contractor")]
    RoofingContractor,

    #[serde(rename = "rv_park")]
    RvPark,

    #[serde(rename = "school")]
    School,

    #[serde(rename = "secondary_school")]
    SecondarySchool,

    #[serde(rename = "shoe_store")]
    ShoeStore,

    #[serde(rename = "shopping_mall")]
    ShoppingMall,

    #[serde(rename = "spa")]
    Spa,

    #[serde(rename = "stadium")]
    Stadium,

    #[serde(rename = "storage")]
    Storage,

    #[serde(rename = "store")]
    Store,

    #[serde(rename = "subway_station")]
    SubwayStation,

    #[serde(rename = "supermarket")]
    Supermarket,

    #[serde(rename = "synagogue")]
    Synagogue,

    #[serde(rename = "taxi_stand")]
    TaxiStand,

    #[serde(rename = "tourist_attraction")]
    TouristAttraction,

    #[serde(rename = "train_station")]
    TrainStation,

    #[serde(rename = "transit_station")]
    TransitStation,

    #[serde(rename = "travel_agency")]
    TravelAgency,

    #[serde(rename = "university")]
    University,

    #[serde(rename = "veterinary_care")]
    VeterinaryCare,

    #[serde(rename = "zoo")]
    Zoo,

    #[serde(rename = "administrative_area_level_1")]
    AdministrativeAreaLevel1,

    #[serde(rename = "administrative_area_level_2")]
    AdministrativeAreaLevel2,

    #[serde(rename = "administrative_area_level_3")]
    AdministrativeAreaLevel3,

    #[serde(rename = "administrative_area_level_4")]
    AdministrativeAreaLevel4,

    #[serde(rename = "administrative_area_level_5")]
    AdministrativeAreaLevel5,

    #[serde(rename = "archipelago")]
    Archipelago,

    #[serde(rename = "colloquial_area")]
    ColloquialArea,

    #[serde(rename = "continent")]
    Continent,

    #[serde(rename = "country")]
    Country,

    #[serde(rename = "establishment")]
    Establishment,

    #[serde(rename = "finance")]
    Finance,

    #[serde(rename = "floor")]
    Floor,

    #[serde(rename = "food")]
    Food,

    #[serde(rename = "general_contractor")]
    GeneralContractor,

    #[serde(rename = "geocode")]
    Geocode,

    #[serde(rename = "health")]
    Health,

    #[serde(rename = "intersection")]
    Intersection,

    #[serde(rename = "landmark")]
    Landmark,

    #[serde(rename = "locality")]
    Locality,

    #[serde(rename = "natural_feature")]
    NaturalFeature,

    #[serde(rename = "neighborhood")]
    Neighborhood,

    #[serde(rename = "place_of_worship")]
    PlaceOfWorship,

    #[serde(rename = "plus_code")]
    PlusCode,

    #[serde(rename = "point_of_interest")]
    PointOfInterest,

    #[serde(rename = "political")]
    Political,

    #[serde(rename = "post_box")]
    PostBox,

    #[serde(rename = "postal_code")]
    PostalCode,

    #[serde(rename = "postal_code_prefix")]
    PostalCodePrefix,

    #[serde(rename = "postal_code_suffix")]
    PostalCodeSuffix,

    #[serde(rename = "postal_town")]
    PostalTown,

    #[serde(rename = "premise")]
    Premise,

    #[serde(rename = "room")]
    Room,

    #[serde(rename = "route")]
    Route,

    #[serde(rename = "street_address")]
    StreetAddress,

    #[serde(rename = "street_number")]
    StreetNumber,

    #[serde(rename = "sublocality")]
    Sublocality,

    #[serde(rename = "sublocality_level_1")]
    SublocalityLevel1,

    #[serde(rename = "sublocality_level_2")]
    SublocalityLevel2,

    #[serde(rename = "sublocality_level_3")]
    SublocalityLevel3,

    #[serde(rename = "sublocality_level_4")]
    SublocalityLevel4,

    #[serde(rename = "sublocality_level_5")]
    SublocalityLevel5,

    #[serde(rename = "subpremise")]
    Subpremise,

    #[serde(rename = "town_square")]
    TownSquare,
}