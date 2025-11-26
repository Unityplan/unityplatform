use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GeoPoint {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CoverageArea {
    Circle {
        center: GeoPoint,
        radius: f64, // in meters
    },
    Polygon {
        coordinates: Vec<GeoPoint>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CommunityType {
    Zone,
    Neighborhood,
    Guild,
    StudyGroup,
    /// A container community that groups other communities together
    Group,
}

impl sqlx::Type<sqlx::Postgres> for CommunityType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("varchar")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CommunityType {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let s: &str = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s {
            "zone" => Ok(CommunityType::Zone),
            "neighborhood" => Ok(CommunityType::Neighborhood),
            "guild" => Ok(CommunityType::Guild),
            "study_group" => Ok(CommunityType::StudyGroup),
            "group" => Ok(CommunityType::Group),
            _ => Err(format!("Unknown community type: {}", s).into()),
        }
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for CommunityType {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let s = match self {
            CommunityType::Zone => "zone",
            CommunityType::Neighborhood => "neighborhood",
            CommunityType::Guild => "guild",
            CommunityType::StudyGroup => "study_group",
            CommunityType::Group => "group",
        };
        <&str as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Community {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub community_type: CommunityType,
    pub territory_id: Option<String>,
    pub parent_community_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    #[schema(value_type = Option<CoverageArea>)]
    pub coverage_area: Option<sqlx::types::Json<serde_json::Value>>,
    pub member_count: i32,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommunityRole {
    Admin,
    Moderator,
    Member,
}

impl sqlx::Type<sqlx::Postgres> for CommunityRole {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for CommunityRole {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let s = match self {
            CommunityRole::Admin => "admin",
            CommunityRole::Moderator => "moderator",
            CommunityRole::Member => "member",
        };
        <String as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&s.to_string(), buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CommunityRole {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s: String = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "admin" => Ok(CommunityRole::Admin),
            "moderator" => Ok(CommunityRole::Moderator),
            "member" => Ok(CommunityRole::Member),
            _ => Err(format!("Invalid community role: {}", s).into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunityMember {
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CommunitySettings {
    pub community_id: Uuid,
    pub inherit_requirements: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RequirementContext {
    View,
    Participate,
    Admin,
}

impl sqlx::Type<sqlx::Postgres> for RequirementContext {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("varchar")
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for RequirementContext {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let s = match self {
            RequirementContext::View => "view",
            RequirementContext::Participate => "participate",
            RequirementContext::Admin => "admin",
        };
        <String as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&s.to_string(), buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for RequirementContext {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s: String = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "view" => Ok(RequirementContext::View),
            "participate" => Ok(RequirementContext::Participate),
            "admin" => Ok(RequirementContext::Admin),
            _ => Err(format!("Invalid requirement context: {}", s).into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CommunityBadgeRequirement {
    pub id: Uuid,
    pub community_id: Uuid,
    pub badge_id: Uuid,
    pub context: RequirementContext,
    pub created_at: DateTime<Utc>,
}

/// A badge requirement with inheritance information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EffectiveBadgeRequirement {
    pub id: Uuid,
    pub community_id: Uuid,
    pub badge_id: Uuid,
    pub context: RequirementContext,
    pub created_at: DateTime<Utc>,
    /// Name of the badge
    pub badge_name: String,
    /// Slug of the badge
    pub badge_slug: String,
    /// Name of the community that defines this requirement
    pub source_community_name: String,
    /// Whether this requirement is inherited from a parent community
    pub is_inherited: bool,
    /// How many levels up this requirement comes from (0 = direct, 1 = parent, etc.)
    pub inheritance_depth: i32,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema, sqlx::FromRow)]
pub struct EffectiveManager {
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub source: String,
    pub distance: i32,
}
