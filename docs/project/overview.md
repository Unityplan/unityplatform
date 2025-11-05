This is written like a **technical design summary** or early whitepaper draft you could hand to a developer team or potential collaborator.

---

# 🌐 Global Communication & Learning Platform (With Decentralization in Holochain in the future)

### Modern Frontend Tech and Rust Microservices Backend (In the future we can migrate the frontend (and frontend api) to Holochain DNA modules for full decentralization)

---

## 🧭 Vision Overview

We are designing a **decentralized digital ecosystem** that allows users across multiple countries to **communicate, learn and collaborate** without relying on centralized servers or data silos.

Each country manages its **own local infrastructure** (for sovereignty and scalability), while individual users own their **personal data** through distributed technologies like **Holochain (future)**.

Global interaction: 
 - Personal P2P and small group chats with direct E2E encrypted chats. 
 - Forums build with Matrix protocol for collaboration with bridge through **Matrix federation**
 - User permissions evolve dynamically based on **verified learning achievements** earned through a decentralized network of LMS servers.
 - All content is **multilingual**, with real-time translation enabling seamless global communication.
 - The system is designed to be **offline-first**, allowing users to continue learning and communicating even with intermittent connectivity.
 - The platform emphasizes **transparency, verifiability, and user sovereignty** through cryptographic credentials and decentralized identity (if possible, else later enforced with Holochain).
 - The architecture supports **federated country nodes**, enabling local autonomy while fostering global collaboration.
 - The platform is built with **modern frontend technologies** (React, Shadcn, Tailwind) for a smooth user experience, while the backend leverages **Rust microservices** for performance and security.
 - In the future, we aim to migrate key components to **Holochain DNA modules** to achieve full decentralization and user data ownership. With Tauri for cross-platform desktop/mobile apps.
 - While we wait for Holochain we can still integrate mobile app with Tauri besides the web frontend.
 - We will use ipfs for file storage to prepare for future decentralization.

---

## 🧩 Core Goals

| Goal                               | Description                                                                                                                          |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| **Data sovereignty**               | Each user owns their own data; countries manage their own infrastructure.                                                            |
| **Decentralized architecture**     | No single point of failure or global database.                                                                                       |
| **Multilingual communication**     | Users communicate globally, each seeing messages in their preferred language.                                                        |
| **Progressive learning system**    | Completing courses grants access to new forums, content, or permissions.                                                             |
| **Transparency & verifiability**   | Achievements, roles, and credentials are verifiable (cryptographically if possible or else this is a future feature with Holochain)  |
| **Federated global collaboration** | Local autonomy + global interoperability + possibility to work with external groups (Matrix bridges)                                 |


## 🛠️ Key Technologies
| Technology            | Purpose                                                |
| --------------------- | ------------------------------------------------------ |
| Docker                | Containerization and deployment                        |
| Matrix Protocol       | Decentralized communication and collaboration          |
| Rust Microservices    | High-performance backend services                      |
| React + Vite          | Modern, responsive frontend development                |
| IPFS                  | Decentralized file storage                             |
| Tauri (Future)        | Cross-platform desktop and mobile applications         |
| Holochain (future)    | Decentralized data ownership and application logic     |



---

## 🏗️ Organizational Structure (Inverted Pyramid Model)

The platform follows an **inverted pyramid architecture** where **users have the highest priority** and global systems have the lowest. This user-centric approach ensures data sovereignty and empowers individuals while maintaining organizational coordination.

### Hierarchy Levels (Priority Order)

```
┌─────────────────────────────────────┐
│         1. USERS (Highest)          │  ← Individual data ownership, personal control
├─────────────────────────────────────┤
│         2. COMMUNITIES              │  ← Local groups, learning circles, teams
├─────────────────────────────────────┤
│  3. TERRITORIES (Countries/Nations) │  ← Autonomous infrastructure, local governance
├─────────────────────────────────────┤
│      4. CONTINENTS (Optional)       │  ← Regional coordination (if needed)
├─────────────────────────────────────┤
│       5. GLOBAL (Lowest)            │  ← Federation, standards, shared resources
└─────────────────────────────────────┘
```

### Territory Definition

**Territories** include:
- **Countries**: Sovereign nations (e.g., Denmark, Canada, Kenya)
- **First Nations**: Indigenous territories with self-governance (e.g., Navajo Nation, Sámi territories)
- **Autonomous Regions**: Self-governing regions within larger nations

Each territory operates with **full autonomy** over:
- User management and invitations
- Local content and curriculum
- Language preferences and translations
- Community structure and policies
- Data residency and compliance requirements

---

#### 🆔 Territory ID Format Standard

UnityPlan uses a standardized territory identification system that respects sovereignty for both countries and First Nations.

##### **Countries** (ISO 3166-1 Alpha-2)

Uses standard two-letter country codes:

| ID | Name | Notes |
|----|------|-------|
| `US` | United States | ISO 3166-1 |
| `CA` | Canada | ISO 3166-1 |
| `AU` | Australia | ISO 3166-1 |
| `NZ` | New Zealand | ISO 3166-1 |
| `MX` | Mexico | ISO 3166-1 |
| `GB` | United Kingdom | ISO 3166-1 |
| `FR` | France | ISO 3166-1 |
| `DK` | Denmark | ISO 3166-1 |
| `NO` | Norway | ISO 3166-1 |
| `SE` | Sweden | ISO 3166-1 |

**Total**: 249 countries (ISO 3166-1 standard)

##### **First Nations** ({NAME}-FN-{COUNTRY})

Format prioritizes First Nation name, followed by FN marker, followed by country code for **geographic context only**:

**Format**: `{NAME}-FN-{COUNTRY}`

**CRITICAL**: The country code in a First Nation ID is **GEOGRAPHIC METADATA ONLY** and does **NOT** create a parent-child relationship. First Nations are **top-level sovereign territories** with `parent_territory = None`.

**Examples**:

| ID | Name | Country Context | parent_territory | Notes |
|----|------|-----------------|------------------|-------|
| `HAIDA-FN-CA` | Haida Nation | Canada (geographic) | `None` | Independent, Pacific Northwest |
| `NAVAJO-FN-US` | Navajo Nation | United States (geographic) | `None` | Independent, largest US tribe |
| `CREE-FN-CA` | Cree Nation | Canada (geographic) | `None` | Independent, largest FN in Canada |
| `CHEROKEE-FN-US` | Cherokee Nation | United States (geographic) | `None` | Independent, second largest US tribe |
| `YOLNGU-FN-AU` | Yolngu people | Australia (geographic) | `None` | Independent, Northern Territory |
| `MAORI-FN-NZ` | Māori | New Zealand (geographic) | `None` | Independent, Indigenous Polynesian |
| `ZAPOTEC-FN-MX` | Zapotec people | Mexico (geographic) | `None` | Independent, Oaxaca region |
| `INUIT-FN-CA` | Inuit | Canada (geographic) | `None` | Independent, Arctic regions |
| `SAMI-FN-NO` | Sámi people | Norway (geographic) | `None` | Independent (also in SE, FI, RU) |

**Sovereignty Principles**:
- ✅ First Nation name comes **first** (respects sovereignty)
- ✅ `FN` marker clearly identifies as First Nation
- ✅ Country code provides **geographic context** (prevents name collisions)
- ✅ **`parent_territory = None`** (top-level, equal to countries)
- ✅ First Nations control their own communities (e.g., `HAIDA-FN-CA-MASSETT`)
- ✅ Self-identification respected (registered name is authoritative)

**Why Keep Country Code?**
1. **Prevents name collisions**: `EAGLE-FN-CA` vs `EAGLE-FN-US` are distinct
2. **Geographic context**: Helps users understand location
3. **No power implication**: Code is metadata, NOT hierarchy
4. **Practical**: Matches how First Nations often identify themselves internationally

**What This Means for Governance**:
- TeacherRegistrar for `CA` **CANNOT** manage `HAIDA-FN-CA` (separate hierarchies)
- TeacherRegistrar for `HAIDA-FN-CA` **CANNOT** manage `CA` (separate hierarchies)
- Each is sovereign within their own hierarchy
- Platform respects Indigenous self-determination

##### **Communities** ({PARENT}-{NAME})

Communities are nested within countries or First Nations:

**Format**: `{PARENT_ID}-{COMMUNITY_NAME}`

**Examples**:

| ID | Name | Parent | Type |
|----|------|--------|------|
| `US-CA-SF` | San Francisco | United States → California | City |
| `HAIDA-FN-CA-MASSETT` | Massett | Haida Nation (Canada) | Village |
| `CA-BC-VANCOUVER` | Vancouver | Canada → British Columbia | City |
| `NAVAJO-FN-US-WINDOW-ROCK` | Window Rock | Navajo Nation (US) | Capital |
| `AU-NSW-SYDNEY` | Sydney | Australia → New South Wales | City |
| `DK-COPENHAGEN` | Copenhagen | Denmark | City |
| `NO-OSLO` | Oslo | Norway | City |
| `SE-STOCKHOLM` | Stockholm | Sweden | City |

**Hierarchy Depth**: Unlimited (communities can nest within communities)

---

### Territory Autonomy & Data Architecture

Each territory operates with **full autonomy** over:
- User management and invitations
- Local content and curriculum
- Language preferences and translations
- Community structure and policies
- Data residency and compliance requirements

### Role System & Permission Model

The platform uses a **badge-based permission system** where access to courses, forums, and administrative functions is granted through earning badges (primarily via course completion). The role hierarchy flows from **global (bottom/foundation) → territory → community → user (top)**, reflecting the inverted pyramid model.

---

#### 🎯 **Core Permission Principle: Badge-Based Access**

**Badges are the primary permission mechanism:**
- Users earn badges by completing courses
- Badges unlock access to:
  - Advanced courses (prerequisite chains)
  - Forums and discussion topics
  - Content creation and moderation abilities
  - Administrative roles at various levels
- Badges can also be manually assigned by authorized users
- Some badges require annual renewal (e.g., Code of Conduct)

---

#### 📜 **Foundation: Code of Conduct (Mandatory)**

**The Code of Conduct course is the gateway to all platform participation:**

- ✅ **Mandatory for all users** - must be completed before any other activity
- 🔄 **Annual renewal required** - retake every year to maintain "Code of Conduct" badge
- 🔒 **Without this badge, users can only**:
  - View their own profile (read-only)
  - Edit basic profile information
  - Delete their account
- ✅ **With this badge, users can**:
  - Edit their profile
  - Comment on forum topics they have access to
  - Enroll in and complete courses they have access to
  - Participate in community activities they belong to

**Enforcement:**
- Users cannot take any courses or participate in forums without current Code of Conduct badge
- Badge expiration sends notifications 30, 14, and 7 days before expiry
- Expired badge locks user out of courses/forums or any other future extention, until renewal

---

#### 🌍 **Level 1: Global Roles (Foundation/Platform Level)**

Global roles manage platform-wide infrastructure, content templates, and standards that territories can adopt or customize.

##### **1.1 Platform Administrator**
**Purpose:** System maintenance and oversight with minimal intervention in content/governance

**Permissions:**
- ✅ **Read access** to all data across the entire platform (for analysis and troubleshooting)
- ✅ **Maintenance operations**: Server management, database optimization, security updates
- ✅ **Monitoring**: Analytics, performance metrics, system health
- ❌ **Limited write access** within web interface - only what's necessary for platform operations
- ❌ **Cannot modify** user content, courses, or governance decisions without explicit authorization

**Accountability:**
- All administrative actions logged in immutable audit trail
- Cannot override territory sovereignty or user data ownership

##### **1.2 Infrastructure & DevOps Roles**
- **Database Administrator**: Database maintenance, backups, performance tuning
- **Security Engineer**: Security audits, vulnerability management, incident response
- **DevOps Engineer**: CI/CD pipelines, container orchestration, deployment automation
- **Monitoring Specialist**: Observability setup, alerting, incident detection
- **Network Engineer**: Network architecture, VPNs, firewalls
- **Techical Support**: Initial troubleshooting of infrastructure issues and user requests

- **Permissions**: Similar to Platform Administrator with focus on infrastructure tasks

##### **1.3 Global LMS Content Creators**
**Purpose:** Create foundational course templates that territories can use, customize, or ignore

**Permissions:**
- ✅ Create, edit, or replace courses in the **global course library**
- ✅ Define which badges courses award upon completion
- ✅ Set course prerequisites and learning paths
- ❌ Cannot force territories to use global courses

**Course Update Policies:**
- **Edit existing course**: Previous participants receive notification to retake (optional)
- **Replace course**: Previous participants **must retake** within timeframe defined in course settings
  - If not retaken: User loses associated certification/badge/permissions
  - Grace period and deadline configurable per course
  - Automated reminders sent at intervals

**Territory Autonomy:**
- Global courses are **optional templates** - territories can choose to:
  - Use global course as-is
  - Customize/adapt for local context
  - Create entirely separate courses
  - Hide global courses from their users
- Changes about hiding global courses require **100% unanimous vote** among all users with this role at that level

##### **1.4 Global Forum Structure Creators**
**Purpose:** Design foundational forum categories and structure

**Permissions:**
- ✅ Create new forum categories and subcategories
- ✅ Edit forum descriptions and rules
- ✅ Close forums (make read-only) - **nothing is ever deleted** for transparency
- ✅ Set badge requirements for forum access
- ❌ Cannot delete forums or forum content

**Territory Autonomy:**
- Territories can hide, customize, or create alternatives to global forum structure
- Requires **100% unanimous vote** among forum structure creators, at that level

##### **1.5 Global Forum Topic Creators**
**Purpose:** Create discussion topics in global forums

**Permissions:**
- ✅ Create new forum topics
- ✅ Edit their own topics
- ✅ Set topic-specific rules or badge requirements
- ❌ Standard users can only **comment** on topics, not create topics (at global level)

##### **1.6 Global Forum Moderators**
**Purpose:** Maintain forum quality and enforce Code of Conduct

**Permissions:**
- ✅ Moderate comments (flag, warn, but not delete)
- ✅ Issue warnings to users violating Code of Conduct
- ✅ Escalate severe violations to Platform Administrators
- ✅ Close topics that become unproductive (read-only)
- ❌ Cannot delete content (transparency principle)

**Warning System:**
- First warning: User notified, comment flagged
- Second warning: Temporary forum access restriction (e.g., 7 days)
- Third warning: Case escalated to Territory Manager or Platform Admin

- Requires **100% unanimous vote** among forum mederators to close topics, at that level

---

#### 🗺️ **Level 2: Territory Roles**

Territory roles manage Country/First Nation-specific content, communities, and governance.

##### **2.1 Territory Manager**
**Purpose:** Bridge global coordination with local autonomy; manage territory infrastructure and users

**Permissions:**
- ✅ **Multi-Territory Management**: Can manage multiple territories (especially during initial rollout)
- ✅ **User Invitations**: Invite and onboard new users to their territories
- ✅ **Settings Configuration**: Configure language, preferred translation language, timezone, localization
- ✅ **Co-Management**: Invite other users to be Territory Managers for their territories
- ✅ **Community Oversight**: Create and manage communities within territories
- ✅ **Delegation**: Assign Community Managers, LMS Creators, Forum Moderators at territory level
- ✅ **Content Curation**: Choose which global courses/forums to show, hide, or customize
- ✅ **Territory-Specific Content**: Create courses and forums specific to their territory

**Authority Limitation:**
- Territory Managers have full permissions **until** another user with the same role is assigned above them
- Once a higher-level manager exists, permissions become delegated/shared

##### **2.2 Territory LMS Content Creator**
- Same permissions as Global LMS Content Creator, but scoped to their territory
- Can customize global courses for local context
- Course changes require unanimous vote if multiple creators exist

##### **2.3 Territory Forum Structure Creator**
- Same permissions as Global Forum Structure Creator, scoped to territory
- Can hide/show global forums or create territory-specific forums

##### **2.4 Territory Forum Moderator**
- Moderates territory-specific forums
- Same powers as Global Forum Moderators, scoped to territory

---

#### 🏘️ **Level 3: Community Roles**

Communities are smaller groups within territories (learning circles, teams, local chapters, guilds).

##### **3.1 Community Manager**
**Permissions:**
- ✅ Manage specific communities within a territory
- ✅ Moderate discussions and forums within their community (if no other Community Forum Moderator exists)
- ✅ Assign learning paths to community members (if learning circles or guilds)
- ✅ Review member achievements (if learning circles or guilds)
- ✅ Create community-specific courses and forums (if learning circles or guilds or no other Community LMS Content Creator exists)
- ✅ Invite users to join the community

**Authority in Inverted Pyramid:**
- Community Managers have **higher authority** than Territory Managers (communities are above territories in the pyramid structure)
- Territory Managers **cannot override** Community Manager decisions within that community - but the territory manager will work as community managers until a community manager is elected or assigned.
- Community self-governance is paramount - only community members can elect/remove their managers

**Democratic Election:**
- Community Managers are assigned by a territory manager or the community manager below this community
 or **elected by community members** via unanimous vote (100% agreement)
- Any community member can nominate themselves or another member
- All community members must vote; abstentions count as "no"
- Community can also vote to remove a Community Manager (100% agreement, excluding the manager being voted on)

##### **3.2 Community LMS Content Creator**
- Create and manage courses for their community
- Can customize territory or global courses for community needs

**Democratic Election:**
- Elected by community members via unanimous vote
- Can be removed by community vote (100% agreement, excluding the creator)

##### **3.3 Community Forum Moderator**
- Moderate community-specific forums and discussions

**Democratic Election:**
- Elected by community members via unanimous vote
- Can be removed by community vote (100% agreement, excluding the moderator)

---

#### 🗳️ **Community-Level Democratic Role Elections**

Communities have **complete control** over who manages them, ensuring true unified governance.

**Election Process:**

```rust
struct CommunityRoleElection {
    id: Uuid,
    community_id: Uuid,
    election_type: ElectionType,
    nominee_user_id: Uuid,
    nominated_by: Uuid,
    role: CommunityRole,  // Manager, LMS Creator, or Forum Moderator
    voting_period_ends: DateTime<Utc>,
    eligible_voters: Vec<Uuid>,  // All community members
    votes: Vec<Vote>,
    status: ElectionStatus,
}

enum ElectionType {
    ElectToRole,      // Vote someone into a role
    RemoveFromRole,   // Vote to remove someone from a role
}

enum CommunityRole {
    CommunityManager,
    LMSContentCreator,
    ForumModerator,
}

async fn nominate_for_community_role(
    nominator_id: Uuid,
    nominee_id: Uuid,
    community_id: Uuid,
    role: CommunityRole,
) -> Result<CommunityRoleElection> {
    // Verify nominator is community member
    verify_community_membership(nominator_id, community_id).await?;
    
    // Verify nominee is community member
    verify_community_membership(nominee_id, community_id).await?;
    
    // Get all community members (eligible voters)
    let members = get_community_members(community_id).await?;
    
    // Create election
    let election = community_role_elections.insert(CommunityRoleElection {
        id: Uuid::new_v4(),
        community_id,
        election_type: ElectionType::ElectToRole,
        nominee_user_id: nominee_id,
        nominated_by: nominator_id,
        role,
        voting_period_ends: Utc::now() + Duration::days(14),
        eligible_voters: members.iter().map(|m| m.user_id).collect(),
        votes: vec![],  // No auto-vote for nominator
        status: ElectionStatus::Active,
    })?;
    
    // Notify all community members
    for member in members {
        notify_user(member.user_id, Notification::CommunityRoleElection {
            election_id: election.id,
            nominee: nominee_id,
            role: role.clone(),
            deadline: election.voting_period_ends,
        }).await?;
    }
    
    Ok(election)
}

async fn vote_on_community_role(
    voter_id: Uuid,
    election_id: Uuid,
    approve: bool,
) -> Result<()> {
    let election = community_role_elections.find(election_id)?;
    
    // Verify voter is eligible
    if !election.eligible_voters.contains(&voter_id) {
        return Err(Error::NotEligibleToVote);
    }
    
    // Check if already voted
    if election.votes.iter().any(|v| v.user_id == voter_id) {
        return Err(Error::AlreadyVoted);
    }
    
    // Record vote
    community_role_elections.add_vote(election_id, Vote {
        user_id: voter_id,
        approved: approve,
        voted_at: Utc::now(),
    })?;
    
    // Check if all votes are in
    let updated_election = community_role_elections.find(election_id)?;
    if updated_election.votes.len() == updated_election.eligible_voters.len() {
        // All voted - tally results
        finalize_community_election(election_id).await?;
    }
    
    Ok(())
}

async fn finalize_community_election(election_id: Uuid) -> Result<ElectionOutcome> {
    let election = community_role_elections.find(election_id)?;
    
    // Require 100% unanimous approval
    let all_approved = election.votes.iter().all(|v| v.approved);
    
    let outcome = if all_approved {
        match election.election_type {
            ElectionType::ElectToRole => {
                // Assign role to nominee
                assign_community_role(
                    election.nominee_user_id,
                    election.community_id,
                    election.role.clone(),
                ).await?;
                
                ElectionOutcome::Elected
            }
            ElectionType::RemoveFromRole => {
                // Remove role from user
                remove_community_role(
                    election.nominee_user_id,
                    election.community_id,
                    election.role.clone(),
                ).await?;
                
                ElectionOutcome::Removed
            }
        }
    } else {
        ElectionOutcome::Rejected
    };
    
    // Update election status
    community_role_elections.update(election_id, ElectionStatus::Completed(outcome))?;
    
    // Notify community members of outcome
    notify_community_election_outcome(election.community_id, election_id, outcome).await?;
    
    // Log in audit trail
    audit_log.insert(AuditEntry {
        event_type: AuditEventType::CommunityRoleElection,
        actor_id: election.nominated_by,
        target_id: Some(election.nominee_user_id),
        scope: Scope::Community(election.community_id),
        details: json!({
            "role": election.role,
            "outcome": outcome,
            "total_votes": election.votes.len(),
            "approved_votes": election.votes.iter().filter(|v| v.approved).count(),
        }),
        timestamp: Utc::now(),
    })?;
    
    Ok(outcome)
}

// Vote to remove existing community role holder
async fn propose_remove_community_role(
    proposer_id: Uuid,
    target_user_id: Uuid,
    community_id: Uuid,
    role: CommunityRole,
) -> Result<CommunityRoleElection> {
    // Verify proposer is community member
    verify_community_membership(proposer_id, community_id).await?;
    
    // Verify target currently holds the role
    verify_has_community_role(target_user_id, community_id, &role).await?;
    
    // Get all community members EXCEPT the target user (they can't vote on their own removal)
    let all_members = get_community_members(community_id).await?;
    let eligible_voters: Vec<Uuid> = all_members
        .iter()
        .filter(|m| m.user_id != target_user_id)
        .map(|m| m.user_id)
        .collect();
    
    // Create removal election
    let election = community_role_elections.insert(CommunityRoleElection {
        id: Uuid::new_v4(),
        community_id,
        election_type: ElectionType::RemoveFromRole,
        nominee_user_id: target_user_id,
        nominated_by: proposer_id,
        role,
        voting_period_ends: Utc::now() + Duration::days(14),
        eligible_voters,  // Target user excluded
        votes: vec![],
        status: ElectionStatus::Active,
    })?;
    
    // Notify eligible voters (not including target)
    for voter_id in &election.eligible_voters {
        notify_user(*voter_id, Notification::CommunityRoleRemovalVote {
            election_id: election.id,
            target_user: target_user_id,
            role: role.clone(),
            deadline: election.voting_period_ends,
        }).await?;
    }
    
    // Notify target user they're being voted on
    notify_user(target_user_id, Notification::RoleRemovalProposed {
        community_id,
        role: role.clone(),
        proposer: proposer_id,
    }).await?;
    
    Ok(election)
}
```

**Key Features of Community Elections:**

1. **100% Unanimous Requirement**: Every community member must vote "yes" for election/removal to succeed
2. **Self-Nomination Allowed**: Community members can nominate themselves or others
3. **Removal Process**: Community can vote to remove role holders (excluding the person being voted on)
4. **Transparency**: All elections logged in immutable audit trail
5. **Notification System**: All members notified of pending elections with deadlines
6. **Voting Period**: 14 days for community to vote (configurable)
7. **Abstention = No**: Not voting counts as voting "no" (ensures active participation)

**Benefits:**
- ✅ **Community Self-Governance**: Communities control their own leadership
- ✅ **Accountability**: Role holders can be removed if community loses confidence
- ✅ **Democratic Legitimacy**: 100% agreement ensures strong mandate
- ✅ **Prevents Power Concentration**: No single person can assign/remove roles unilaterally
- ✅ **Unified Empowerment**: Aligns with inverted pyramid model (users have highest authority)

---

#### 👤 **Level 4: Individual User Roles (Highest Priority)**

Individual users are at the **top** of the inverted pyramid - they have ultimate control over their own data.
User roles determine what actions they can take based on badges earned by participating or completing courses,
and by that everthing is available to everybody as long as they have the willingness to learn to recieve the badges.

##### **4.1 Standard User/Learner**
**Baseline permissions (with Code of Conduct badge):**
- ✅ Enroll in and complete courses (based on badges/prerequisites)
- ✅ **Comment** on forum topics (cannot create topics at at any level without specific badge allowing it)
- ✅ View and edit their own profile
- ✅ Manage privacy settings and data sharing preferences related to their account
- ✅ Delete their account and data
- ✅ Participate in community activities (based on community membership)

**Without Code of Conduct badge:**
- ❌ No course access
- ❌ No forum participation
- ✅ Can only view/edit profile and delete account
- ✅ Can take Code of Conduct course to regain full access

##### **4.2 Teacher/Instructor**
**Earned via badge (e.g., "Certified Instructor" badge):**
- ✅ Teach courses within their community

##### **4.3 LMS Content Creator**
**Earned via badge (e.g., "Content Contributor" badge):**
- ✅ Contribute learning materials (videos, documents, interactive content)
- ✅ Submit content for review and inclusion in courses
- ✅ Earn attribution for contributed content

##### **4.4 Community Forum Topic Creator**
**Earned via badge at community level:**
- ✅ Create new discussion topics in community forums.

---

#### ⚖️ **Democratic Governance & Change Control**

**For roles that can modify shared resources (courses, forums):**

**Change Authority:**
- If **only one user** has a role at a given level → they have full authority
- If **multiple users** have the same role at the same level → changes require **100% unanimous vote**

**Voting applies to:**
- Hiding or showing global content at territory/community level
- Editing existing courses or forums
- Changing course prerequisites or badge assignments
- Major structural changes to learning paths

**Implementation:**
- Proposed changes enter a voting period (e.g., 7-14 days)
- All users with that role at that level are notified
- Must achieve 100% agreement to proceed
- Abstentions count as "no" votes
- Prevents unilateral changes to shared resources

---

#### 🎓 **Badge System & Course Prerequisites**

**Badge Acquisition:**
1. **Primary method**: Complete a course that awards the badge
2. **Secondary method**: Manually assigned by authorized user (Territory Manager, Community Manager, etc.)
3. **Annual renewal**: Some badges (Code of Conduct) require periodic re-certification

**Prerequisites & Course Chains:**
- Courses can require specific badges before enrollment
- Courses can be linked in prerequisite chains:
  ```
  Code of Conduct → Basic Communication → Advanced Collaboration → Moderator Training
  ```
- Users cannot skip prerequisite courses
- Losing a prerequisite badge (e.g., due to course replacement) may lock access to other courses or forums

**Badge-Gated Forums:**
- Forums can require specific badges for access
- Example: "Advanced Instructors" forum requires "Certified Instructor" badge
- Prevents topic/comment access without proper credentials

---

#### 🔄 **Summary: Permission Hierarchy**

```
┌─────────────────────────────────────────────────────────┐
│  USERS (Highest - Data Ownership)                       │
│  • Own their data, privacy, account                     │
│  • Participate based on earned badges                   │
│  • Code of Conduct badge required for activity          │
├─────────────────────────────────────────────────────────┤
│  COMMUNITY LEVEL                                         │
│  • Community Managers, LMS Creators, Moderators         │
│  • Can customize territory content for local groups     │
│  • Authority until higher level manager assigned        │
├─────────────────────────────────────────────────────────┤
│  TERRITORY LEVEL                                         │
│  • Territory Managers, LMS Creators, Forum Managers     │
│  • Choose to use/hide/customize global content          │
│  • 100% unanimous vote required for shared changes      │
├─────────────────────────────────────────────────────────┤
│  GLOBAL LEVEL (Foundation - Lowest Priority)            │
│  • Platform Admins, DevOps, LMS Content Creators        │
│  • Provide optional templates and infrastructure        │
│  • Cannot override territory autonomy                   │
│  • Read-only access except for maintenance              │
└─────────────────────────────────────────────────────────┘
```

**Key Principles:**
1. **Badge-based access** drives all permissions
2. **Code of Conduct is mandatory** and renewed annually
3. **Nothing is deleted** - only closed/archived for transparency
4. **100% unanimous votes** required for shared resource changes
5. **Territory autonomy** - can hide/customize global content
6. **Users own their data** - highest priority in the system

---

## 🧱 Core Components

### 1. **Territory Infrastructure & Data Isolation**

#### **Multi-Tenant PostgreSQL Schema Architecture**

**Why Schema-Based Isolation?**
Schema-based multi-tenancy is the **best practice** for your use case because it provides:

1. **Easy Future Migration**: Each territory's data lives in its own PostgreSQL schema, making it trivial to migrate to dedicated servers later
   ```bash
   # Export single territory
   pg_dump --schema=territory_canada mydb > canada.sql
   # Restore on new server
   psql -h canada-server.example.com mydb < canada.sql
   ```

2. **Resource Efficiency**: Start with shared infrastructure (1 server or 1 per continent), scale independently as territories grow

3. **Strong Isolation**: Schema boundaries + Row-Level Security (RLS) prevent cross-territory data access

4. **Independent Backups**: Back up and restore individual territories without affecting others

5. **Performance**: Better than row-level tenancy (`WHERE territory_id = ?`) which requires filtering every query

#### **Database Schema Structure**

```sql
-- Global schema (shared across all territories)
global:
  - users                    -- Global user accounts (SSO, authentication)
  - user_profiles            -- User profile information and preferences
  - user_languages           -- Language proficiency levels
  - user_social_links        -- Social media and website links
  - user_privacy_settings    -- Privacy and visibility controls
  - user_social_groups       -- Custom social familiarity groups
  - user_notification_prefs  -- Notification preferences
  - user_news_feed_prefs     -- News feed settings
  - user_follows             -- User following relationships
  - p2p_conversations        -- P2P encrypted conversation metadata
  - project_updates          -- User daily project updates/sharing
  - update_reactions         -- Reactions to project updates
  - update_comments          -- Comments on project updates
  - user_badges              -- User badge assignments with expiration tracking
  - territories              -- Territory registry and metadata
  - territory_managers       -- Territory manager assignments (multi-territory support)
  - global_settings          -- Platform-wide configuration
  - audit_log                -- System-wide audit trail (immutable, never deleted)
  - role_assignments         -- Global role assignments (Platform Admin, DevOps, etc.)

-- Public schema (shared reference data)
public:
  - translations             -- Translation memory and term database
  - course_templates         -- Global course templates (optional for territories)
  - course_versions          -- Course version history (edits/replacements tracked)
  - badges                   -- Badge definitions (name, description, permissions granted)
  - badge_prerequisites      -- Badge dependency chains
  - credential_standards     -- Open Badges, Verifiable Credential schemas
  - forum_categories_global  -- Global forum structure templates (optional for territories)
  - code_of_conduct_versions -- Code of Conduct course versions with annual tracking
  - categories               -- Category definitions (Science, Leadership, etc.)
  - category_memberships     -- Which courses/forums belong to which categories
  - topic_tool_types         -- 27 collaboration tools: voting, whiteboard, proposals, elections, etc.

-- Per-Territory schemas (isolated)
territory_{id}:
  - territory_settings       -- Language, timezone, localization preferences
  - matrix_server_config     -- Local Matrix homeserver configuration
  - user_activity_feed       -- Aggregated feed of user activities
  - communities              -- Communities within this territory
  - community_members        -- User-community memberships with role assignments
  - courses                  -- Territory-specific courses (can reference or customize global)
  - course_enrollments       -- User course progress and completion tracking
  - course_categories        -- Territory-specific category assignments
  - achievements             -- User learning credentials and badge awards within territory
  - forums                   -- Territory and community forums (Matrix room references)
  - forum_categories         -- Territory-specific forum category assignments
  - forum_topics             -- Forum discussion topics
  - forum_comments           -- User comments on topics (never deleted, only flagged)
  - forum_hidden_levels      -- Which forums are hidden at which community/territory levels
  - topic_collaborations     -- Voting, whiteboard, brainstorming session data
  - moderation_actions       -- Warning system logs for Code of Conduct violations
  - invitations              -- Pending user invitations to territory
  - voting_sessions          -- Democratic voting for content/structure changes
  - unhiding_votes           -- Votes to unhide forums at community/territory level
  - content_visibility       -- Which global courses/forums are shown/hidden
```

**Key Schema Features:**
- `user_profiles` stores comprehensive profile data (display name, avatar, bio, location, birthdate)
- `user_languages` tracks language proficiency levels (spoken/written/listening/reading)
- `user_social_links` unlimited social media links with 30+ predefined types (LinkedIn, GitHub, Twitter, etc.)
- `user_privacy_settings` granular visibility controls with 7 privacy presets (Private → Public)
- `user_social_groups` custom familiarity groups (Family, Close Friends, Friends, Acquaintances)
- `user_notification_prefs` notification preferences per category with quiet hours
- `user_news_feed_prefs` customizable feed algorithm and content sources
- `p2p_conversations` metadata for 2-10 user encrypted conversations (Signal Protocol)
- `project_updates` daily project sharing with visibility controls
- `user_badges` table tracks badge ownership, acquisition date, and expiration (for annual renewals like Code of Conduct)
- `badges` table defines permissions each badge grants (forum access, course creation, moderation, etc.)
- `badge_prerequisites` enables course chains and badge dependencies
- `course_versions` tracks when courses are edited vs. replaced (triggers different notification/retake policies)
- `moderation_actions` implements the three-strike warning system
- `voting_sessions` supports 100% unanimous voting for role holders making shared changes
- `audit_log` is append-only, ensuring complete transparency (nothing deleted)
- `categories` and `category_memberships` enable discovery system with roadmaps
- `forum_hidden_levels` tracks which communities/territories hid which forums
- `unhiding_votes` manages democratic process to unhide forums
- `matrix_server_config` configures each territory's local Matrix homeserver
- `topic_collaborations` stores data for 27 collaboration tools (voting, proposals, elections, events, etc.)
- `topic_tool_types` defines all available topic collaboration types
- `user_activity_feed` aggregates user activities for news feed generation

#### **Infrastructure Evolution Path**

**Phase 1: Initial Rollout (3-5 Territories)**
- Single PostgreSQL server with multiple schemas
- All territories share infrastructure
- Territory Managers manage 1-3 territories each
- Deploy in one geographic location

**Phase 2: Regional Expansion (10-20 Territories)**
- 1 server per continent (Americas, Europe, Asia-Pacific, Africa)
- Migrate territory schemas to regional servers based on user location
- Reduced latency for regional users
- Territory Managers still can manage multiple territories

**Phase 3: Territory Autonomy (50+ Territories)**
- Large territories get dedicated servers
- Smaller territories remain on shared regional infrastructure
- Full data sovereignty for territories that want it
- Cross-territory federation via Matrix protocol

#### **Benefits of This Approach**

✅ **Start Small**: Launch with minimal infrastructure investment  
✅ **Scale Gradually**: Add servers as territories grow  
✅ **Easy Migration**: Move territories to new servers without downtime  
✅ **Data Sovereignty**: Clear boundaries for compliance and governance  
✅ **Territory Manager Flexibility**: Manage multiple territories from single interface  
✅ **Future-Proof**: Smooth path to Holochain migration (each territory becomes a DNA)  

#### **Cross-Server Territory Management**

When territories are distributed across multiple database servers, Territory Managers need a **unified interface** that transparently handles cross-server operations. This is achieved through a **Service Registry + API Gateway** pattern, which already fits with your Rust microservices architecture.

**Architecture Components:**

1. **Territory Registry Service (Global)**
   - Central registry mapping each territory to its database server location
   - Stores territory metadata: name, server URL, schema name, region
   - Replicated across regions for high availability
   - Updated when territories are migrated to new servers

   ```sql
   -- Global territory registry (lives on central coordination server)
   CREATE TABLE territories (
       id UUID PRIMARY KEY,
       name VARCHAR(255) NOT NULL,
       database_server VARCHAR(255) NOT NULL,  -- 'postgres://eu-server-1:5432/platform'
       schema_name VARCHAR(255) NOT NULL,      -- 'territory_denmark'
       region VARCHAR(50),                      -- 'europe', 'americas', 'asia-pacific'
       created_at TIMESTAMPTZ DEFAULT NOW()
   );
   
   CREATE TABLE territory_managers (
       manager_user_id UUID,
       territory_id UUID,
       role VARCHAR(50) DEFAULT 'manager',
       assigned_at TIMESTAMPTZ DEFAULT NOW(),
       PRIMARY KEY (manager_user_id, territory_id)
   );
   ```

2. **API Gateway with Dynamic Routing**
   - Territory Manager authenticates once (SSO via OIDC)
   - Gateway retrieves manager's authorized territories from registry
   - Routes each request to the appropriate backend server based on territory context
   - Maintains connection pools to all active database servers
   - Handles failover if a server becomes unavailable

   ```rust
   // Example backend service logic
   async fn get_territory_communities(
       territory_id: Uuid,
       manager_id: Uuid,
   ) -> Result<Vec<Community>> {
       // 1. Verify manager has permission for this territory
       verify_manager_access(manager_id, territory_id).await?;
       
       // 2. Get territory's database location from registry
       let territory = territory_registry.get(territory_id).await?;
       
       // 3. Get connection pool for this database server
       let pool = get_connection_pool(&territory.database_server)?;
       
       // 4. Set schema and query
       let communities = sqlx::query_as::<_, Community>(
           &format!("SET search_path = {}; SELECT * FROM communities", territory.schema_name)
       )
       .fetch_all(pool)
       .await?;
       
       Ok(communities)
   }
   ```

3. **Multi-Database Connection Pooling**
   - Backend maintains separate connection pools for each database server
   - Pools are created on-demand when first territory on a server is accessed
   - Lazy loading: only connect to servers with territories the manager actually uses
   - Connection pool health monitoring and automatic reconnection

4. **Frontend Territory Switcher**
   - Territory Manager sees dropdown/sidebar with all their territories
   - Switching territory sends new requests with `territory_id` header/context
   - Backend transparently routes to correct server
   - Manager doesn't need to know about server distribution

**Benefits of This Approach:**

✅ **Transparent Federation**: Territory Managers work the same way regardless of server distribution  
✅ **Centralized Authentication**: Single sign-on works across all servers  
✅ **Independent Scaling**: Each database server scales independently  
✅ **Graceful Migration**: Move territories between servers without changing application code  
✅ **Performance**: Requests only go to relevant servers (no broadcast queries)  
✅ **Fault Tolerance**: If one regional server fails, territories on other servers remain accessible  

**Migration Workflow Example:**

```bash
# 1. Export territory from source server
pg_dump --host=server1.example.com \
        --schema=territory_denmark \
        platform_db > denmark.sql

# 2. Restore on destination server
psql --host=eu-server.example.com \
     platform_db < denmark.sql

# 3. Update territory registry (single UPDATE query)
UPDATE territories 
SET database_server = 'postgres://eu-server.example.com:5432/platform'
WHERE id = 'denmark-uuid';

# 4. Territory Manager's next request automatically routes to new server
```

**Implementation Notes:**

- **Service Discovery**: Use Consul, etcd, or Kubernetes service discovery for database server locations
- **Caching**: Cache territory → server mappings in memory with TTL (5-10 minutes)
- **Monitoring**: Track cross-server query latency and connection pool health
- **Transaction Boundaries**: Cross-server transactions are not supported (by design - territories are independent)
- **Eventual Consistency**: Changes to territory registry propagate within seconds

#### **Additional Infrastructure Components**

- **Service Mesh**: Rust microservices orchestrated via Docker Compose with Traefik/Linkerd for mTLS
- **Sovereignty**: Territories control their infrastructure, data residency, and compliance requirements
- **Scalability**: Each territory (or server hosting multiple territories) scales independently based on demand
- **Federation Protocol**: Matrix protocol enables secure inter-territory communication and collaboration
- **Time-Series Data**: TimescaleDB for metrics, analytics, and activity tracking
- **Backup Strategy**: Per-schema backups enable independent disaster recovery
- **Access Control**: PostgreSQL Row-Level Security (RLS) enforces territory boundaries at database level

---

### 2. **Badge-Based Permission & Access Control System**

The badge system is the **cornerstone of platform permissions**, controlling access to courses, forums, content creation, and administrative functions.

#### **Badge Architecture**

**Badge Definition:**
```rust
struct Badge {
    id: Uuid,
    name: String,              // "Code of Conduct", "Certified Instructor", "Forum Moderator"
    description: String,
    category: BadgeCategory,   // Governance, Learning, Community, Admin
    permissions: Vec<Permission>,
    requires_renewal: bool,    // Annual renewal required (e.g., Code of Conduct)
    renewal_period_days: Option<u32>, // 365 for annual badges
    visual_icon: String,       // Badge icon/image URL
}

enum Permission {
    // Course permissions
    EnrollInCourses,
    CreateCourses { scope: Scope },
    EditCourses { scope: Scope },
    
    // Forum permissions
    CommentOnTopics,
    CreateTopics { scope: Scope },
    ModerateForum { scope: Scope },
    
    // Administrative permissions
    ManageTerritory { territory_id: Uuid },
    ManageCommunity { community_id: Uuid },
    InviteUsers { scope: Scope },
    AssignBadges { badge_categories: Vec<BadgeCategory> },
}

enum Scope {
    Global,
    Territory(Uuid),
    Community(Uuid),
}
```

#### **Code of Conduct Badge (Mandatory Foundation)**

**Special Status:**
- **Required** for all platform participation beyond profile management
- **Annual renewal** - expires every 365 days
- **Automated notifications**: 30, 14, 7 days before expiration
- **Automatic lockout**: Upon expiration, user loses all participation permissions

**Implementation:**
```rust
async fn check_code_of_conduct_status(user_id: Uuid) -> Result<BadgeStatus> {
    let badge = user_badges
        .filter(user_id.eq(user_id))
        .filter(badge_name.eq("Code of Conduct"))
        .first()?;
    
    if badge.expires_at < Utc::now() {
        // Lock user out of courses and forums
        return Ok(BadgeStatus::Expired);
    }
    
    // Send reminders if approaching expiration
    let days_until_expiry = (badge.expires_at - Utc::now()).num_days();
    if days_until_expiry <= 30 {
        send_renewal_reminder(user_id, days_until_expiry).await?;
    }
    
    Ok(BadgeStatus::Active)
}
```

**User Experience Without Code of Conduct Badge:**
- ❌ Cannot enroll in any courses
- ❌ Cannot view or comment on forums
- ❌ Cannot join communities
- ✅ Can view/edit own profile
- ✅ Can delete account
- ✅ Can take Code of Conduct course to (re)activate

#### **Badge Acquisition Methods**

1. **Course Completion (Primary)**
   ```rust
   async fn complete_course(user_id: Uuid, course_id: Uuid) -> Result<Vec<Badge>> {
       let course = courses.find(course_id)?;
       let awarded_badges = course.badges_awarded;
       
       for badge in &awarded_badges {
           assign_badge(user_id, badge.id, BadgeSource::CourseCompletion(course_id)).await?;
       }
       
       // Emit event for permission system update
       event_bus.publish(Event::BadgesAwarded { user_id, badges: awarded_badges }).await?;
       
       Ok(awarded_badges)
   }
   ```

2. **Manual Assignment (Secondary)**
   - Territory Managers can assign badges to users in their territories
   - Community Managers can assign community-scoped badges
   - Requires appropriate permission badge to assign others
   - All manual assignments logged in audit trail

3. **Achievement-Based (Automated)**
   - Participation milestones (e.g., "100 Forum Comments" badge)
   - Contribution recognition (e.g., "Content Creator - Bronze" after 10 approved materials)
   - Consistency rewards (e.g., "Daily Learner" for 30-day streak)

#### **Course Prerequisites & Badge Chains**

**Prerequisite Enforcement:**
```rust
async fn check_enrollment_eligibility(user_id: Uuid, course_id: Uuid) -> Result<bool> {
    let course = courses.find(course_id)?;
    let user_badges = get_user_badges(user_id).await?;
    
    // Must have Code of Conduct badge
    if !user_badges.contains("Code of Conduct") {
        return Err(Error::CodeOfConductRequired);
    }
    
    // Check course prerequisites
    for required_badge in &course.prerequisite_badges {
        if !user_badges.contains(required_badge) {
            return Err(Error::MissingPrerequisite {
                required: required_badge.clone(),
                course: course.name.clone(),
            });
        }
    }
    
    Ok(true)
}
```

**Example Learning Path:**
```
1. Code of Conduct (mandatory, annual)
   ↓
2. Platform Basics
   ↓
3. Effective Communication → Badge: "Community Communicator"
   ↓
4. Advanced Collaboration (requires "Community Communicator")
   ↓
5. Moderator Training → Badge: "Forum Moderator"
   ↓ (enables moderation permissions)
6. Leadership & Governance → Badge: "Community Leader"
   ↓ (enables community management permissions)
```

#### **Badge-Gated Forums**

Forums can require specific badges for access:

```rust
struct Forum {
    id: Uuid,
    name: String,
    scope: Scope,  // Global, Territory, Community
    required_badges: Vec<Uuid>,  // Badges needed to view/participate
    comment_permission_badge: Option<Uuid>,  // Additional badge for commenting
    topic_creation_badge: Option<Uuid>,      // Badge for creating topics
    visibility: ForumVisibility,
}

async fn check_forum_access(user_id: Uuid, forum_id: Uuid, action: ForumAction) -> Result<bool> {
    let forum = forums.find(forum_id)?;
    let user_badges = get_user_badges(user_id).await?;
    
    // Check base forum access
    for required_badge in &forum.required_badges {
        if !user_badges.contains(required_badge) {
            return Ok(false);
        }
    }
    
    // Check action-specific permissions
    match action {
        ForumAction::View => Ok(true),
        ForumAction::Comment => {
            if let Some(badge) = forum.comment_permission_badge {
                Ok(user_badges.contains(&badge))
            } else {
                Ok(true)  // No additional badge required
            }
        }
        ForumAction::CreateTopic => {
            if let Some(badge) = forum.topic_creation_badge {
                Ok(user_badges.contains(&badge))
            } else {
                // Default: only Forum Topic Creators role can create topics at global/territory level
                Ok(has_role(user_id, Role::ForumTopicCreator).await?)
            }
        }
    }
}
```

#### **Badge Expiration & Renewal**

**Annual Renewal System:**
```rust
// Background job runs daily
async fn check_badge_expirations() {
    let expiring_soon = user_badges
        .filter(expires_at.is_not_null())
        .filter(expires_at.lt(Utc::now() + Duration::days(30)))
        .load()?;
    
    for badge in expiring_soon {
        let days_remaining = (badge.expires_at - Utc::now()).num_days();
        
        match days_remaining {
            30 | 14 | 7 => {
                send_notification(badge.user_id, NotificationType::BadgeExpiring {
                    badge_name: badge.name,
                    days_remaining,
                    renewal_course_url: badge.renewal_course_url,
                }).await?;
            }
            0 => {
                // Badge expired - revoke permissions
                revoke_badge(badge.user_id, badge.id).await?;
                
                // If Code of Conduct, lock user out
                if badge.name == "Code of Conduct" {
                    lock_user_participation(badge.user_id).await?;
                    send_notification(badge.user_id, NotificationType::CodeOfConductExpired).await?;
                }
            }
            _ => {}
        }
    }
}
```

**Impact of Badge Loss:**
- User loses associated permissions immediately
- Access to badge-gated forums removed
- Cannot enroll in courses requiring that badge
- If prerequisite badge lost, may lose access to advanced courses already enrolled in
- Audit log records all badge revocations with reason

---

### 2b. **Category System & Content Discovery**

The category system organizes courses and forums into intuitive groupings with **badge-based visibility** and **roadmap discovery** for locked content.

#### **Category Architecture**

**Category Types:**
```rust
struct Category {
    id: Uuid,
    name: String,  // "Science & Technology", "Leadership & Governance", "Cultural Studies"
    description: String,
    icon: String,
    color_scheme: String,  // For UI theming
    required_badge: Option<Uuid>,  // Badge needed to see this category
    parent_category: Option<Uuid>,  // Hierarchical nesting
    scope: Scope,  // Global, Territory, or Community
    display_order: i32,
}

struct CategoryMembership {
    category_id: Uuid,
    resource_type: ResourceType,
    resource_id: Uuid,
}

enum ResourceType {
    Forum,
    Course,
    LearningPath,  // Curated sequence of courses
}
```

**Hierarchical Categories Example:**
```
Science & Technology (requires: Science Explorer badge)
├─ Physics
│  ├─ Classical Mechanics (public)
│  ├─ Quantum Physics (requires: Advanced Physics badge)
│  └─ Astrophysics (requires: Physics Mastery badge)
├─ Computer Science
│  ├─ Programming Basics (public)
│  ├─ Web Development (requires: Programmer badge)
│  └─ AI & Machine Learning (requires: Advanced CS badge)
└─ Engineering
   ├─ Mechanical Engineering (public)
   └─ Electrical Engineering (requires: Engineering Fundamentals badge)
```

#### **Visual Discovery System**

Users browse content with **three visibility states**:

**1. Accessible Content (Full Color)**
- User has all required badges
- Click to view/enter forum or enroll in course
- Shows: Title, description, member count, activity level

**2. Locked Content (Greyed-Out Silhouette)**
- User missing required badges
- Displayed as semi-transparent "silhouette" with lock icon
- Shows: Title (partial), estimated unlock time, lock icon
- Click to view **detailed roadmap**

**3. Hidden Content (Not Shown)**
- Content hidden at user's territory/community level
- Does NOT appear in category listings
- Exception: Can be discovered via search with "show hidden" filter
- Shows warning: "This content is hidden at your community/territory level"

#### **Roadmap System**

When a user clicks on locked content, they see a **detailed roadmap** to unlock it:

```
╔════════════════════════════════════════════════════════════╗
║  🔒 Advanced Quantum Physics Forum                        ║
║                                                            ║
║  Unlock Progress: ██████░░░░ 60%                          ║
╚════════════════════════════════════════════════════════════╝

📋 Requirements to Unlock:

✅ Code of Conduct Badge
   └─ You have this badge (renews in 187 days)

✅ Basic Physics Badge
   └─ Earned by completing "Physics 101" (completed 3 months ago)

✅ Intermediate Physics Badge  
   └─ Earned by completing "Physics 201" (completed 1 month ago)

🔒 Advanced Physics Badge  
   ├─ Complete course: "Advanced Physics" 
   │  ├─ Prerequisites: Intermediate Physics Badge ✅
   │  ├─ Duration: 8 weeks
   │  ├─ Effort: 6-8 hours/week
   │  └─ Next cohort starts: March 15, 2025
   └─ OR complete: "Self-Paced Advanced Physics"
      ├─ Prerequisites: Intermediate Physics Badge ✅
      ├─ Duration: Self-paced (avg. 10 weeks)
      └─ Start anytime

🔒 Quantum Mechanics Fundamentals Badge
   ├─ Complete course: "Quantum 101"
   │  ├─ Prerequisites: Advanced Physics Badge 🔒 (unlock first)
   │  ├─ Duration: 6 weeks
   │  └─ Effort: 8-10 hours/week
   └─ Estimated availability: After completing Advanced Physics

───────────────────────────────────────────────────────────

⏱️  Estimated Time to Unlock: 14-18 weeks
📚 Recommended Path: [View Optimal Learning Sequence]
👥 1,247 users have unlocked this forum
💡 Tip: Join "Physics Study Group" community for peer support

[Enroll in "Advanced Physics"] [Bookmark for Later] [Ask Question]
```

#### **Implementation**

**Category View Generation:**
```rust
async fn get_user_category_view(user_id: Uuid, scope: Scope) -> Result<Vec<CategoryView>> {
    let user_badges = get_user_badges(user_id).await?;
    let user_territory = get_user_territory(user_id).await?;
    let user_community = get_user_community(user_id).await?;
    
    let categories = match scope {
        Scope::Global => categories.filter(scope.eq(Scope::Global)).load()?,
        Scope::Territory(id) => categories
            .filter(scope.eq(Scope::Global).or(scope.eq(Scope::Territory(id))))
            .load()?,
        Scope::Community(id) => categories.load()?,  // All categories visible
    };
    
    let mut category_views = vec![];
    
    for category in categories {
        // Check if user can see this category
        let category_accessible = if let Some(required_badge) = category.required_badge {
            user_badges.iter().any(|b| b.id == required_badge && !b.is_expired())
        } else {
            true
        };
        
        // Get resources in this category
        let resources = get_category_resources(category.id).await?;
        let mut accessible_resources = vec![];
        let mut locked_resources = vec![];
        
        for resource in resources {
            // Check if resource is hidden at user's level
            if is_resource_hidden(user_id, &resource).await? {
                continue;  // Don't show hidden resources in normal view
            }
            
            let can_access = check_resource_access(user_id, &resource).await?;
            
            if can_access {
                accessible_resources.push(ResourceView {
                    resource,
                    state: AccessState::Accessible,
                    roadmap: None,
                });
            } else {
                // Generate roadmap for locked resource
                let roadmap = generate_access_roadmap(user_id, &resource).await?;
                locked_resources.push(ResourceView {
                    resource: resource.to_silhouette(),  // Partial info only
                    state: AccessState::Locked,
                    roadmap: Some(roadmap),
                });
            }
        }
        
        category_views.push(CategoryView {
            category,
            accessible: category_accessible,
            accessible_resources,
            locked_resources,
            total_resources: accessible_resources.len() + locked_resources.len(),
            unlock_progress: calculate_unlock_progress(&accessible_resources, &locked_resources),
        });
    }
    
    Ok(category_views)
}

async fn generate_access_roadmap(user_id: Uuid, resource: &Resource) -> Result<AccessRoadmap> {
    let user_badges = get_user_badges(user_id).await?;
    let required_badges = get_resource_required_badges(resource).await?;
    
    let mut steps = vec![];
    let mut total_estimated_weeks = 0;
    let mut completed_count = 0;
    
    for badge in required_badges {
        if user_badges.iter().any(|b| b.id == badge.id && !b.is_expired()) {
            steps.push(RoadmapStep {
                badge: badge.clone(),
                status: StepStatus::Completed,
                estimated_weeks: 0,
                unlock_options: vec![],
            });
            completed_count += 1;
        } else {
            // Find all ways to earn this badge
            let courses = find_courses_awarding_badge(badge.id).await?;
            let manual_assignment = can_badge_be_manually_assigned(user_id, badge.id).await?;
            
            let estimated_weeks = courses.iter()
                .filter(|c| check_course_prerequisites(user_id, c.id).await.is_ok())
                .map(|c| c.estimated_duration_weeks)
                .min()
                .unwrap_or(4);
            
            total_estimated_weeks += estimated_weeks;
            
            let mut unlock_options = courses.iter()
                .map(|c| UnlockOption::Course(c.clone()))
                .collect::<Vec<_>>();
            
            if manual_assignment {
                unlock_options.push(UnlockOption::ManualAssignment {
                    who_can_assign: get_badge_assigners(badge.id).await?,
                });
            }
            
            steps.push(RoadmapStep {
                badge: badge.clone(),
                status: StepStatus::Locked,
                estimated_weeks,
                unlock_options,
            });
        }
    }
    
    // Calculate optimal learning path
    let recommended_path = calculate_optimal_learning_path(user_id, &steps).await?;
    
    // Get community stats
    let users_who_unlocked = count_users_with_access(resource).await?;
    
    Ok(AccessRoadmap {
        steps,
        total_estimated_weeks,
        completed_count,
        total_count: required_badges.len(),
        progress_percentage: (completed_count as f32 / required_badges.len() as f32 * 100.0) as u8,
        recommended_path,
        users_who_unlocked,
        helpful_communities: find_related_communities(resource).await?,
    })
}
```

**Benefits:**
- ✅ **Intuitive Discovery**: Browse by interest areas, not just lists
- ✅ **Clear Progression**: Visual roadmaps show exactly how to unlock content
- ✅ **Motivation**: See progress bars and achievable goals
- ✅ **Transparency**: Nothing truly hidden - users know what exists
- ✅ **Flexibility**: Multiple paths to unlock content
- ✅ **Community Support**: See who else has unlocked content, find study groups

---

### 3. **Decentralized Communication System**

#### **Personal & Small Group Chat (E2E Encrypted)**
- Direct peer-to-peer messaging with end-to-end encryption
- Small group chats (2-10 participants) for private collaboration
- WebSocket gateway (`tokio-tungstenite`) for real-time message delivery
- Offline-first architecture with message queuing and synchronization
- **Requires**: Active Code of Conduct badge for participation

#### **Matrix-Based Forums & Collaboration**

Matrix protocol enables **federated, decentralized forums** with hierarchical permissions tied to the badge system and cross-territory data sovereignty.

**Forum Hierarchy & Scope:**
```
Global Forums (optional templates)
  ├─ Territory Forums (can customize or hide global)
  │   └─ Community Forums (local to communities)
  └─ Cross-Territory Federated Forums (via Matrix federation)
```

**Cross-Territory Federation & Data Sovereignty:**

Each territory operates its own **Matrix homeserver**, ensuring user data stays within their territory while enabling global collaboration:

- **Local Data Storage**: Users post only to their territory's Matrix server
- **Global Visibility**: All territories' Matrix servers federate to show content from other territories
- **Read-Only Federation**: Users see all global/territory forums but interact only through their local server
- **Data Residency**: User messages remain on their territory's server; other territories only receive read replicas
- **Seamless Experience**: Users access global forums through their local Matrix server without knowing about cross-territory mechanics

**Example Flow:**
1. User in Denmark creates topic in Global Forum → stored on Denmark's Matrix server
2. User in Canada views same topic → Denmark's server federates content to Canada's server (read-only)
3. User in Kenya replies → reply stored on Kenya's Matrix server
4. All users see unified conversation, but data stays in respective territories

**Forum Visibility & Hiding System:**

Forums created at lower levels (global/territory) can be **hidden** at higher levels (community), but with important transparency features:

- **Hidden Forums Remain Visible**: Users can still **see and read** hidden forum titles/descriptions
- **Participation Blocked**: Users from communities that hid the forum cannot post or comment
- **Discovery & Transparency**: Users can browse hidden forums to understand what's available
- **Democratic Unhiding**: Communities can vote (100% unanimous) to unhide forums at each level
  - Community votes to unhide at community level
  - If forum was hidden at territory level, territory must also vote to unhide
  - Unhiding cascades up hierarchy until reaching the level where forum was created

**Example Hiding Scenario:**
```
Global Forum: "Advanced Political Theory"
  ├─ Territory A: Visible (users can participate)
  ├─ Territory B: Hidden (users can read but not participate)
  │   ├─ Community B1: Hidden (inherited from territory)
  │   └─ Community B2: Votes to unhide → still blocked until Territory B unhides
  └─ Territory C: Visible
      ├─ Community C1: Hidden (users can read but not participate)
      └─ Community C2: Visible (users can participate)
```

**Forum Access Modes:**

1. **Public Forums** (marked "readable to all"):
   - Any user can view without badges
   - Code of Conduct badge required to comment/participate
   - Good for announcements, general information

2. **Badge-Gated Forums**:
   - Require specific badges to view/participate
   - Users without badges see "greyed out silhouette" with roadmap to access
   - Encourages learning progression

**Key Features:**
- **Badge-Gated Access**: Forums require specific badges (e.g., "Advanced Learner" forum requires completion of prerequisite courses)
- **Category Tagging**: Forums grouped by categories tied to badge permissions (see Category System below)
- **Hierarchical Permissions**: 
  - Global/Territory: Only Forum Topic Creators role can create topics; all users can comment
  - Community: Community members with appropriate badges can create topics
- **Matrix Federation**: Enables cross-territory collaboration while maintaining data sovereignty
- **Transparency**: Hidden forums remain visible (read-only) for discovery and unhiding requests
- **No Deletion Policy**: Content can be flagged/archived but never deleted (transparency principle)
- **Moderation**: Forum Moderators can warn users, flag content, close topics (read-only)
- **Real-time Collaboration**: Typing indicators, read receipts, reactions, threaded discussions
- **Topic Types as Tools**: Voting, whiteboard, brainstorming, linked courses, video, hiring, development tools

**Forum Permission Matrix:**

| Action | Global/Territory | Community | Required Badge |
|--------|-----------------|-----------|----------------|
| **View Forum (Public)** | Any user | Any user | None (if marked "readable to all") |
| **View Forum (Gated)** | Badge required | Badge required | Forum-specific badge |
| **View Hidden Forum** | Read-only (no participation) | Read-only (no participation) | Any user can see it exists |
| **Comment on Topic** | Any user with CoC badge | Community member with CoC | Code of Conduct (active) |
| **Create Topic** | Forum Topic Creator role | Community member | Topic Creator badge (community) |
| **Moderate** | Forum Moderator role | Community Moderator | Forum Moderator badge |
| **Close Topic** | Forum Moderator+ | Community Moderator+ | Moderator badge |
| **Hide/Unhide Forum** | 100% unanimous vote | 100% unanimous vote | Territory/Community Manager+ |

**Topic Types & Collaboration Tools:**

Forums support various topic types that act as **collaboration tools**, organized by category:

**Governance & Decision-Making:**
1. **Discussion**: Standard threaded conversations
2. **Voting**: Democratic decision-making with vote tallying
3. **Proposals/RFC**: Formal proposals for community/territory decisions with structured format (rationale, impact analysis, voting period)
4. **Elections**: Democratic elections for moderators/managers with candidate profiles and voting periods
5. **Polls/Surveys**: Quick lightweight feedback (non-binding, for gathering opinions)
6. **Consensus Building**: Tools for achieving 100% unanimous agreement

**Learning & Collaboration:**
7. **Linked Course Discussion**: Discussion tied to specific courses
8. **Q&A**: Stack Overflow-style questions with accepted answers
9. **Peer Review**: Structured peer assessment for assignments/badges
10. **Study Groups**: Organize learning cohorts, schedule peer study sessions
11. **Mentorship Matching**: Connect mentors with learners, track mentorship relationships
12. **Wiki/Knowledge Base**: Collaborative documentation building
13. **Showcase/Portfolio**: Share completed work, get community feedback

**Community Management:**
14. **Events/Calendar**: Schedule gatherings, workshops, deadlines with RSVP tracking
15. **Announcements**: Official read-only communications from managers/moderators
16. **Resource Sharing**: Physical/digital resource lending, booking (equipment, rooms, tools)

**Collaboration & Productivity:**
17. **Whiteboard**: Real-time collaborative canvas
18. **Brainstorming**: Idea collection and voting
19. **Video Conference**: Integrated video calls
20. **Project Management**: Task tracking, milestones, deliverables
21. **Hiring/Jobs**: Recruitment and project staffing

**Platform Operations:**
22. **Development**: Code collaboration, issue tracking for development work
23. **Bug Reports/Issues**: Platform bug tracking and resolution
24. **Accessibility Feedback**: Dedicated channel for accessibility issues and improvements
25. **Translation/Localization**: Collaborative translation work for multilingual platform

**Engagement & Gamification:**
26. **Emergency/Alerts**: High-priority urgent communications (safety, security)
27. **Challenges/Competitions**: Learning challenges, skill competitions with leaderboards

**Future Extensions:**
- **Marketplace/Exchange**: Barter services, trade resources within community (will be a separate extension like LMS and Forum)

**Matrix Room Mapping:**
```rust
struct ForumMatrixRoom {
    forum_id: Uuid,
    matrix_room_id: String,  // Matrix room identifier (on each territory's server)
    scope: Scope,            // Global, Territory, Community
    required_badges: Vec<Uuid>,
    readable_to_all: bool,   // Public forum (no badge required to read)
    federated: bool,         // Cross-territory federation enabled
    encryption_enabled: bool, // E2E encryption for sensitive forums
    hidden_at_levels: Vec<HiddenLevel>,  // Which territories/communities hid this forum
    category_tags: Vec<Uuid>, // Categories this forum belongs to
    topic_tools_enabled: Vec<TopicToolType>, // Which collaboration tools are available
}

struct HiddenLevel {
    scope: Scope,  // Territory or Community that hid the forum
    hidden_at: DateTime<Utc>,
    hidden_by: Uuid,  // User who initiated the hiding vote
    can_read: bool,  // Always true - users can see hidden forums
    can_participate: bool,  // Always false - cannot post/comment
}

// When user tries to join forum
async fn join_forum(user_id: Uuid, forum_id: Uuid) -> Result<MatrixRoomInvite> {
    let forum = forums.find(forum_id)?;
    let user_territory = get_user_territory(user_id).await?;
    
    // 1. Check if forum is hidden at user's territory/community level
    if is_forum_hidden_for_user(user_id, forum_id).await? {
        return Err(Error::ForumHiddenAtYourLevel {
            message: "You can read this forum but cannot participate. Your community/territory has hidden it.",
            unhide_process: "Request a vote in your community to unhide this forum.",
        });
    }
    
    // 2. Check badge requirements (unless readable to all)
    if !forum.readable_to_all {
        check_forum_access(user_id, forum_id, ForumAction::View).await?;
    }
    
    // 3. Get Matrix credentials for user
    let matrix_user = get_or_create_matrix_user(user_id).await?;
    
    // 3. Invite to Matrix room
    let room = forums.find(forum_id)?;
    matrix_client.invite_user(room.matrix_room_id, matrix_user.id).await?;
    
    Ok(MatrixRoomInvite { room_id: room.matrix_room_id })
}
```

**Moderation & Warning System:**

Implements the three-strike warning system for Code of Conduct violations:

```rust
async fn issue_warning(
    moderator_id: Uuid,
    user_id: Uuid,
    comment_id: Uuid,
    reason: String,
) -> Result<ModerationAction> {
    // Verify moderator has permission
    verify_permission(moderator_id, Permission::ModerateForum).await?;
    
    // Count existing warnings
    let warning_count = moderation_actions
        .filter(target_user_id.eq(user_id))
        .filter(action_type.eq(ActionType::Warning))
        .filter(created_at.gt(Utc::now() - Duration::days(90))) // 90-day window
        .count()?;
    
    let action = match warning_count {
        0 => ModerationAction::FirstWarning {
            comment_flagged: true,
            notification_sent: true,
        },
        1 => ModerationAction::SecondWarning {
            comment_flagged: true,
            forum_restriction_days: 7,
        },
        2 => ModerationAction::ThirdWarning {
            case_escalated_to: EscalationTarget::TerritoryManager,
            temporary_suspension_pending: true,
        },
        _ => ModerationAction::EscalationRequired,
    };
    
    // Log in immutable audit trail
    audit_log.insert(AuditEntry {
        moderator_id,
        target_user_id: user_id,
        action: action.clone(),
        reason,
        comment_id: Some(comment_id),
        timestamp: Utc::now(),
    })?;
    
    // Execute action (notify user, restrict access, escalate)
    execute_moderation_action(user_id, &action).await?;
    
    Ok(action)
}
```

**Category System & Discovery:**

Forums and courses are organized by **category tags** that work with badge permissions to create intuitive discovery and progression paths.

**Category Structure:**
```rust
struct Category {
    id: Uuid,
    name: String,  // "Science & Technology", "Leadership", "Cultural Studies"
    description: String,
    icon: String,
    required_badge: Option<Uuid>,  // Badge needed to see this category
    parent_category: Option<Uuid>,  // Hierarchical categories
    scope: Scope,  // Global, Territory, or Community category
}

struct CategoryMembership {
    category_id: Uuid,
    resource_type: ResourceType,  // Forum or Course
    resource_id: Uuid,
}

enum ResourceType {
    Forum,
    Course,
}
```

**Discovery & Roadmap System:**

Users see categorized content with **visual progression indicators**:

1. **Accessible Content** (full visibility):
   - Forums/courses user has badges to access
   - Shown with full details, click to enter

2. **Locked Content** (greyed-out silhouettes):
   - Forums/courses user doesn't have badges for yet
   - Shown as "greyed out silhouettes" with lock icon
   - Click to see **roadmap** for how to gain access
   
3. **Roadmap View** (prerequisite chain):
   ```
   🔒 Advanced Quantum Physics Forum
   
   To unlock this forum, you need:
   ├─ ✅ Code of Conduct Badge (you have this)
   ├─ ✅ Basic Physics Badge (you have this)
   ├─ 🔒 Intermediate Physics Badge
   │   └─ Required: Complete "Intermediate Physics" course
   │       ├─ Prerequisite: Basic Physics Badge ✅
   │       └─ Duration: 6 weeks
   └─ 🔒 Quantum Mechanics Fundamentals Badge
       └─ Required: Complete "Quantum 101" course
           ├─ Prerequisite: Intermediate Physics Badge 🔒
           └─ Duration: 8 weeks
   
   Estimated time to unlock: 14 weeks
   Recommended learning path: [View detailed path]
   ```

**Category Permissions:**
- Categories themselves can require badges
- Users without category badge see category as greyed-out
- All nested content appears locked until category badge is earned
- Encourages structured learning progression

**Implementation:**
```rust
async fn get_user_category_view(user_id: Uuid) -> Result<Vec<CategoryView>> {
    let user_badges = get_user_badges(user_id).await?;
    let all_categories = categories.load()?;
    
    let mut category_views = vec![];
    
    for category in all_categories {
        // Check if user can see this category
        let accessible = if let Some(required_badge) = category.required_badge {
            user_badges.contains(&required_badge)
        } else {
            true  // No badge required for category
        };
        
        // Get forums and courses in this category
        let resources = get_category_resources(category.id).await?;
        let mut accessible_resources = vec![];
        let mut locked_resources = vec![];
        
        for resource in resources {
            let resource_accessible = check_resource_access(user_id, &resource).await?;
            
            if resource_accessible {
                accessible_resources.push(resource);
            } else {
                // Generate roadmap for locked resource
                let roadmap = generate_access_roadmap(user_id, &resource).await?;
                locked_resources.push(LockedResource {
                    resource,
                    roadmap,
                    display_as_silhouette: true,
                });
            }
        }
        
        category_views.push(CategoryView {
            category,
            accessible,
            accessible_resources,
            locked_resources,
        });
    }
    
    Ok(category_views)
}

async fn generate_access_roadmap(
    user_id: Uuid,
    resource: &Resource,
) -> Result<AccessRoadmap> {
    let user_badges = get_user_badges(user_id).await?;
    let required_badges = get_resource_required_badges(resource).await?;
    
    let mut steps = vec![];
    let mut total_estimated_weeks = 0;
    
    for badge in required_badges {
        if user_badges.contains(&badge.id) {
            steps.push(RoadmapStep {
                badge: badge.clone(),
                status: StepStatus::Completed,
                estimated_weeks: 0,
            });
        } else {
            // Find courses that award this badge
            let courses = find_courses_awarding_badge(badge.id).await?;
            let estimated_weeks = courses.iter()
                .map(|c| c.estimated_duration_weeks)
                .min()
                .unwrap_or(4);
            
            total_estimated_weeks += estimated_weeks;
            
            steps.push(RoadmapStep {
                badge: badge.clone(),
                status: StepStatus::Locked,
                estimated_weeks,
                unlock_via: courses,
            });
        }
    }
    
    Ok(AccessRoadmap {
        steps,
        total_estimated_weeks,
        recommended_path: calculate_optimal_path(user_id, &steps).await?,
    })
}
```

---

### 4. **Progressive Learning Management System (LMS)**

The LMS implements a **federated, badge-driven learning architecture** where courses award badges that unlock further learning and platform capabilities. Courses are organized by **categories** (see Category System above) for intuitive discovery.

#### **Course Hierarchy & Autonomy**

**Multi-Level Course Structure:**
```
Global Course Library (optional templates)
  ├─ Territory Courses (can customize, hide, or create new)
  │   └─ Community Courses (hyper-local content)
  └─ Course Variants (translations, cultural adaptations)
```

**Course Organization:**
- **Category Tags**: Courses grouped by subject area (Science, Leadership, Arts, etc.)
- **Learning Paths**: Curated sequences of courses within categories
- **Prerequisite Chains**: Badge dependencies create natural progression
- **Discovery**: Users browse by category, see locked courses as "greyed silhouettes"
- **Roadmaps**: Click locked course to see detailed unlock path

**Territory Autonomy Implementation:**
```rust
struct CourseVisibility {
    course_id: Uuid,
    scope: Scope,
    source: CourseSource,  // Global, Territory, Community
    customizations: Option<CourseCustomization>,
    hidden: bool,          // Territory chose to hide this course
    replacement: Option<Uuid>, // Territory replaced with this course
    category_tags: Vec<Uuid>,  // Categories this course belongs to
}

enum CourseSource {
    GlobalTemplate,
    TerritoryCustomized { based_on: Uuid },  // References global course
    TerritoryOriginal,
    CommunityCustomized { based_on: Uuid },
    CommunityOriginal,
}
```

**Democratic Course Management:**
When multiple LMS Content Creators exist at the same level (global/territory/community), changes require **100% unanimous vote**:

```rust
async fn propose_course_change(
    proposer_id: Uuid,
    course_id: Uuid,
    change: CourseChange,
) -> Result<VotingSession> {
    let course = courses.find(course_id)?;
    
    // Find all users with LMS Content Creator role at this scope
    let creators = role_assignments
        .filter(role.eq(Role::LMSContentCreator))
        .filter(scope.eq(course.scope))
        .load()?;
    
    if creators.len() == 1 {
        // Single creator - can make change immediately
        apply_course_change(course_id, change).await?;
        return Ok(VotingSession::AutoApproved);
    }
    
    // Multiple creators - create voting session
    let session = voting_sessions.insert(VotingSession {
        id: Uuid::new_v4(),
        proposal_type: ProposalType::CourseChange(change),
        proposer_id,
        required_voters: creators.iter().map(|c| c.user_id).collect(),
        votes: vec![Vote { user_id: proposer_id, approved: true }], // Proposer auto-votes yes
        voting_period_ends: Utc::now() + Duration::days(14),
        unanimous_required: true,
        status: VotingStatus::Active,
    })?;
    
    // Notify all other creators
    for creator in creators {
        if creator.user_id != proposer_id {
            notify_vote_required(creator.user_id, session.id).await?;
        }
    }
    
    Ok(session)
}
```

#### **Course Edit vs. Replace Policies**

**Course Edit (Minor Changes):**
- Typo fixes, clarifications, additional examples
- Previous completers receive **optional** notification to review changes
- Badge/certification remains valid
- No mandatory retake required

**Course Replace (Major Changes):**
- Curriculum overhaul, new learning objectives, different badge requirements
- Previous completers **must retake** within configured timeframe
- If not retaken by deadline: Badge revoked, permissions lost
- Automated reminders at intervals

```rust
async fn handle_course_replacement(
    old_course_id: Uuid,
    new_course_id: Uuid,
    retake_deadline_days: u32,
) -> Result<()> {
    // Find all users who completed old course
    let previous_completers = course_enrollments
        .filter(course_id.eq(old_course_id))
        .filter(status.eq(CompletionStatus::Completed))
        .load()?;
    
    let deadline = Utc::now() + Duration::days(retake_deadline_days as i64);
    
    for completer in previous_completers {
        // Create mandatory retake enrollment
        course_enrollments.insert(CourseEnrollment {
            user_id: completer.user_id,
            course_id: new_course_id,
            status: EnrollmentStatus::MandatoryRetake,
            deadline: Some(deadline),
            reason: RetakeReason::CourseReplaced { old_course: old_course_id },
        })?;
        
        // Send notification
        notify_user(completer.user_id, Notification::MandatoryCourseRetake {
            course_name: new_course.name,
            deadline,
            consequence: "Badge revocation and permission loss".to_string(),
        }).await?;
    }
    
    // Schedule background job to check deadlines
    schedule_retake_deadline_check(new_course_id, deadline).await?;
    
    Ok(())
}

// Background job runs daily
async fn check_retake_deadlines() {
    let overdue = course_enrollments
        .filter(status.eq(EnrollmentStatus::MandatoryRetake))
        .filter(deadline.lt(Utc::now()))
        .load()?;
    
    for enrollment in overdue {
        // Revoke badges from old course
        revoke_course_badges(enrollment.user_id, enrollment.old_course_id).await?;
        
        // User loses permissions granted by those badges
        recalculate_user_permissions(enrollment.user_id).await?;
        
        // Notify user
        notify_user(enrollment.user_id, Notification::BadgeRevoked {
            reason: "Failed to complete mandatory course retake",
        }).await?;
    }
}
```

#### **Federated Course Network**

**Cross-Territory Course Sharing:**
- Territories can **share courses** with other territories (with attribution)
- Courses can be **remixed** and localized while maintaining version link
- **Content versioning** tracks course evolution and attribution
- Support for multiple formats: video, interactive, SCORM, xAPI, H5P

```rust
struct CourseSharing {
    course_id: Uuid,
    origin_territory: Uuid,
    shared_with: Vec<Uuid>,  // Territory IDs
    sharing_terms: SharingTerms,
    attribution_required: bool,
    allow_modifications: bool,
    license: ContentLicense,  // CC BY-SA, CC BY-NC, etc.
}
```

#### **Course Content Structure & Types**

Courses are structured as **collections of content items** organized into sections, supporting multiple content types for rich, engaging learning experiences.

**Course Architecture:**
```rust
struct Course {
    id: Uuid,
    name: String,
    description: String,
    scope: Scope,  // Global, Territory, Community
    category_tags: Vec<Uuid>,
    
    // Content organization
    sections: Vec<CourseSection>,
    
    // Access control
    required_badges: Vec<Uuid>,  // Prerequisites
    badges_awarded: Vec<Uuid>,   // What badges this course grants
    
    // Enrollment
    enrollment_type: EnrollmentType,
    max_enrollments: Option<u32>,
    
    // Certification
    requires_certification: bool,
    certification_survey_id: Option<Uuid>,
    passing_score: Option<f32>,  // Percentage required to pass
    
    // Metadata
    estimated_duration_weeks: u32,
    effort_hours_per_week: u32,
    language: String,
    difficulty_level: DifficultyLevel,
    instructor_ids: Vec<Uuid>,
    
    // Linked resources
    linked_forum_id: Option<Uuid>,  // Dedicated course forum
    
    // Visibility
    published: bool,
    promoted: bool,  // Featured on homepage
    
    // Gamification
    karma_points_on_completion: u32,
    
    // Attribution
    author_ids: Vec<Uuid>,
    origin_course_id: Option<Uuid>,  // If customized from another
    license: ContentLicense,
}

struct CourseSection {
    id: Uuid,
    course_id: Uuid,
    title: String,
    description: Option<String>,
    sequence: i32,  // Display order
    content_items: Vec<ContentItem>,
}

struct ContentItem {
    id: Uuid,
    section_id: Uuid,
    title: String,
    description: Option<String>,
    sequence: i32,
    content_type: ContentType,
    content_data: ContentData,
    
    // Completion tracking
    is_optional: bool,
    estimated_duration_minutes: u32,
    
    // Access
    requires_previous_completion: bool,  // Must complete previous items first
}

enum ContentType {
    // Text & Documents
    Article,           // Rich text document
    Document,          // PDF, Word, etc.
    Book,              // Multi-page book format with chapters
    
    // Media
    Video,             // Embedded or uploaded video
    Audio,             // Podcast, audio lecture, voice recording
    Presentation,      // PDF slides or embedded deck
    Infographic,       // Image-based content
    Animation,         // GIF, animated content, motion graphics
    Model3D,           // Interactive 3D models
    
    // Interactive Content
    InteractiveH5P,    // H5P interactive content
    Simulation,        // Virtual labs, interactive scenarios
    InteractiveDiagram,// Clickable, zoomable diagrams
    Flashcards,        // Spaced repetition learning cards
    DragAndDrop,       // Drag-and-drop activities
    Matching,          // Matching exercises
    Timeline,          // Interactive timelines
    BranchingScenario, // Choose-your-own-path learning
    CaseStudy,         // Structured problem-solving scenarios
    
    // Assessments
    Quiz,              // Knowledge check
    PracticeExercise,  // Non-graded practice
    CodeExercise,      // Programming challenges with auto-grading
    SelfAssessment,    // Reflection exercises
    CertificationExam, // Final assessment
    
    // Collaboration
    Assignment,        // User submission required
    PeerAssessment,    // Structured peer grading
    GroupProject,      // Collaborative workspace
    Discussion,        // Forum-style discussion
    Wiki,              // Collaborative document creation
    Glossary,          // Collaborative term database
    Portfolio,         // Collection of user work over time
    
    // Real-Time Interaction
    LiveSession,       // Scheduled live video/webinar
    Whiteboard,        // Collaborative drawing/brainstorming
    Poll,              // Real-time polling/surveys
    BreakoutRoom,      // Small group video sessions
    
    // Reference Materials
    ExternalLink,      // Link to external resource
    ResourceLibrary,   // Searchable resource database
    Bibliography,      // Reading lists with links
    
    // Advanced/Emerging Tech
    VirtualReality,    // VR content
    AugmentedReality,  // AR content
    AdaptivePath,      // AI-driven personalized learning path
    
    // Standards & Integrations
    SCORM,             // SCORM package
    xAPI,              // xAPI/Tin Can content
    LTI,               // Learning Tools Interoperability
    ExternalTool,      // Google Workspace, Office 365, etc.
}

enum ContentData {
    // Text & Documents
    Article { 
        html_content: String,
        reading_time_minutes: u32,
    },
    
    Document {
        ipfs_cid: String,
        mime_type: String,
        file_size_bytes: u64,
        download_url: String,
        page_count: Option<u32>,
    },
    
    Book {
        ipfs_cid: String,
        format: BookFormat,  // PDF, EPUB, MOBI
        isbn: Option<String>,
        author: String,
        publisher: Option<String>,
        page_count: u32,
        chapters: Vec<BookChapter>,
    },
    
    // Media
    Video {
        source: VideoSource,
        url: String,
        duration_seconds: u32,
        thumbnail_url: Option<String>,
        transcript_url: Option<String>,
        subtitle_tracks: Vec<SubtitleTrack>,
    },
    
    Audio {
        source: AudioSource,
        url: String,
        duration_seconds: u32,
        transcript_url: Option<String>,
        audio_type: AudioType,  // Podcast, Lecture, Interview
    },
    
    Presentation {
        source: PresentationSource,
        url: String,
        page_count: u32,
        thumbnail_url: Option<String>,
    },
    
    Infographic {
        ipfs_cid: String,
        width: u32,
        height: u32,
        alt_text: String,
    },
    
    Animation {
        ipfs_cid: String,
        format: AnimationFormat,  // GIF, WebM, MP4
        duration_seconds: u32,
        width: u32,
        height: u32,
    },
    
    Model3D {
        ipfs_cid: String,
        format: Model3DFormat,  // GLTF, OBJ, FBX
        thumbnail_url: String,
        viewer_url: String,  // Web-based 3D viewer
        interactive: bool,
    },
    
    // Interactive Content
    InteractiveH5P {
        h5p_content_id: String,
        h5p_type: String,  // Interactive Video, Quiz, Timeline, etc.
    },
    
    Simulation {
        simulation_type: SimulationType,  // PhysicsLab, ChemistryLab, Business, etc.
        launch_url: String,
        parameters: serde_json::Value,
        requires_webgl: bool,
    },
    
    InteractiveDiagram {
        ipfs_cid: String,
        diagram_type: DiagramType,  // FlowChart, MindMap, ConceptMap
        editable: bool,
        collaboration_enabled: bool,
    },
    
    Flashcards {
        deck_id: Uuid,
        cards: Vec<Flashcard>,
        spaced_repetition_enabled: bool,
    },
    
    DragAndDrop {
        background_image_url: String,
        draggable_items: Vec<DraggableItem>,
        drop_zones: Vec<DropZone>,
        attempts_allowed: u32,
    },
    
    Matching {
        items_left: Vec<MatchItem>,
        items_right: Vec<MatchItem>,
        correct_pairs: Vec<(Uuid, Uuid)>,
        attempts_allowed: u32,
    },
    
    Timeline {
        events: Vec<TimelineEvent>,
        theme: TimelineTheme,
        interactive: bool,
    },
    
    BranchingScenario {
        scenario_id: Uuid,
        start_scene_id: Uuid,
        scenes: Vec<ScenarioScene>,
        choices: Vec<ScenarioChoice>,
    },
    
    CaseStudy {
        title: String,
        description: String,
        background_info: String,
        questions: Vec<CaseStudyQuestion>,
        resources: Vec<String>,  // URLs to supporting materials
    },
    
    // Assessments
    Quiz {
        quiz_id: Uuid,
        questions: Vec<QuizQuestion>,
        attempts_allowed: u32,
        passing_score_percentage: f32,
        randomize_questions: bool,
        show_correct_answers: bool,
        time_limit_minutes: Option<u32>,
    },
    
    PracticeExercise {
        exercise_type: ExerciseType,  // Math, Writing, Problem-Solving
        instructions: String,
        hints: Vec<String>,
        auto_graded: bool,
        solution_available: bool,
    },
    
    CodeExercise {
        programming_language: String,
        starter_code: String,
        test_cases: Vec<TestCase>,
        execution_environment: CodeEnvironment,
        time_limit_seconds: u32,
        memory_limit_mb: u32,
    },
    
    SelfAssessment {
        criteria: Vec<AssessmentCriterion>,
        reflection_prompts: Vec<String>,
        rubric_url: Option<String>,
    },
    
    CertificationExam {
        survey_id: Uuid,
        attempts_allowed: u32,
        passing_score_percentage: f32,
        certificate_template_id: Uuid,
        time_limit_minutes: u32,
        proctoring_required: bool,
    },
    
    // Collaboration
    Assignment {
        instructions: String,
        submission_types: Vec<SubmissionType>,
        due_date: Option<DateTime<Utc>>,
        max_file_size_mb: u32,
        peer_review_required: bool,
        group_work: bool,
    },
    
    PeerAssessment {
        assignment_id: Uuid,
        review_criteria: Vec<AssessmentCriterion>,
        reviews_required: u32,
        anonymous: bool,
        calibration_phase: bool,
    },
    
    GroupProject {
        project_description: String,
        min_group_size: u32,
        max_group_size: u32,
        milestones: Vec<ProjectMilestone>,
        collaboration_tools: Vec<String>,  // Wiki, Forum, Whiteboard
    },
    
    Discussion {
        forum_topic_id: Uuid,
        minimum_posts: u32,
        graded: bool,
        rubric_id: Option<Uuid>,
    },
    
    Wiki {
        wiki_id: Uuid,
        initial_content: String,
        allow_student_editing: bool,
        version_controlled: bool,
    },
    
    Glossary {
        glossary_id: Uuid,
        terms: Vec<GlossaryTerm>,
        allow_student_contributions: bool,
        requires_approval: bool,
    },
    
    Portfolio {
        portfolio_id: Uuid,
        sections: Vec<PortfolioSection>,
        template_id: Option<Uuid>,
        sharing_settings: SharingSettings,
    },
    
    // Real-Time Interaction
    LiveSession {
        scheduled_start: DateTime<Utc>,
        duration_minutes: u32,
        video_conference_url: String,
        recording_url: Option<String>,
        max_participants: Option<u32>,
        features: Vec<LiveSessionFeature>,  // Chat, ScreenShare, Breakout
    },
    
    Whiteboard {
        whiteboard_id: Uuid,
        collaboration_enabled: bool,
        tools_available: Vec<WhiteboardTool>,
        max_participants: u32,
    },
    
    Poll {
        question: String,
        options: Vec<PollOption>,
        allow_multiple_selections: bool,
        anonymous: bool,
        show_results: PollResultsVisibility,
    },
    
    BreakoutRoom {
        parent_session_id: Uuid,
        room_count: u32,
        duration_minutes: u32,
        assignment_method: RoomAssignmentMethod,  // Random, Manual, SelfSelect
        activity_instructions: String,
    },
    
    // Reference Materials
    ExternalLink {
        url: String,
        link_type: ExternalLinkType,
        preview_image_url: Option<String>,
        description: Option<String>,
    },
    
    ResourceLibrary {
        library_id: Uuid,
        resources: Vec<Resource>,
        categories: Vec<String>,
        searchable: bool,
    },
    
    Bibliography {
        citations: Vec<Citation>,
        citation_style: CitationStyle,  // APA, MLA, Chicago, IEEE
        annotated: bool,
    },
    
    // Advanced/Emerging Tech
    VirtualReality {
        vr_scene_url: String,
        platform: VRPlatform,  // WebVR, Oculus, SteamVR
        headset_required: bool,
        fallback_2d_url: Option<String>,
    },
    
    AugmentedReality {
        ar_content_url: String,
        marker_image_url: Option<String>,  // For marker-based AR
        platform: ARPlatform,  // WebXR, ARCore, ARKit
    },
    
    AdaptivePath {
        path_id: Uuid,
        assessment_id: Uuid,  // Initial diagnostic
        learning_paths: Vec<LearningPath>,
        adaptation_algorithm: AdaptationAlgorithm,
    },
    
    // Standards & Integrations
    SCORM {
        scorm_package_url: String,
        scorm_version: String,  // 1.2, 2004
    },
    
    xAPI {
        launch_url: String,
        activity_id: String,
    },
    
    LTI {
        lti_version: String,  // 1.1, 1.3
        launch_url: String,
        consumer_key: String,
        shared_secret: String,  // Encrypted
        custom_parameters: serde_json::Value,
    },
    
    ExternalTool {
        tool_type: ExternalToolType,  // GoogleDocs, Office365, Padlet, etc.
        embed_url: String,
        oauth_required: bool,
        iframe_settings: IFrameSettings,
    },
}

enum VideoSource {
    YouTube,
    Vimeo,
    PeerTube,  // Self-hosted federated video
    DirectUpload,  // Stored in IPFS
    ExternalEmbed,
}

enum AudioSource {
    DirectUpload,  // IPFS
    ExternalURL,
    Podcast,  // RSS feed
    Streaming,  // Live audio
}

enum AudioType {
    Podcast,
    Lecture,
    Interview,
    AudioBook,
    Music,
}

enum BookFormat {
    PDF,
    EPUB,
    MOBI,
    HTML,
}

struct BookChapter {
    title: String,
    start_page: u32,
    duration_minutes: Option<u32>,  // For audiobooks
}

enum AnimationFormat {
    GIF,
    WebM,
    MP4,
    SVG,
}

enum Model3DFormat {
    GLTF,
    GLB,
    OBJ,
    FBX,
    STL,
}

enum SimulationType {
    PhysicsLab,
    ChemistryLab,
    BiologyLab,
    BusinessSimulation,
    EngineeringCAD,
    MedicalProcedure,
    FlightSimulator,
    Custom,
}

enum DiagramType {
    FlowChart,
    MindMap,
    ConceptMap,
    ProcessDiagram,
    OrgChart,
    NetworkDiagram,
}

struct Flashcard {
    id: Uuid,
    front: String,
    back: String,
    image_url: Option<String>,
    audio_url: Option<String>,
    tags: Vec<String>,
}

struct DraggableItem {
    id: Uuid,
    content: String,
    image_url: Option<String>,
}

struct DropZone {
    id: Uuid,
    label: String,
    correct_item_ids: Vec<Uuid>,
    x: f32,  // Position coordinates
    y: f32,
}

struct MatchItem {
    id: Uuid,
    content: String,
    image_url: Option<String>,
}

struct TimelineEvent {
    id: Uuid,
    title: String,
    date: DateTime<Utc>,
    description: String,
    media_url: Option<String>,
}

enum TimelineTheme {
    Historical,
    Scientific,
    Personal,
    Project,
}

struct ScenarioScene {
    id: Uuid,
    title: String,
    description: String,
    media_url: Option<String>,
    feedback: Option<String>,
}

struct ScenarioChoice {
    id: Uuid,
    from_scene_id: Uuid,
    choice_text: String,
    to_scene_id: Uuid,
    score_impact: i32,
}

struct CaseStudyQuestion {
    id: Uuid,
    question_text: String,
    question_type: QuestionType,
    rubric: Option<String>,
}

enum QuestionType {
    OpenEnded,
    Analytical,
    DecisionMaking,
    RolePlay,
}

enum ExerciseType {
    Mathematics,
    Writing,
    ProblemSolving,
    CriticalThinking,
    DataAnalysis,
}

struct TestCase {
    id: Uuid,
    input: String,
    expected_output: String,
    hidden: bool,  // Not shown to student
    points: u32,
}

enum CodeEnvironment {
    Python,
    JavaScript,
    Java,
    CPlusPlus,
    Rust,
    SQL,
    Custom { image: String },  // Docker image
}

struct AssessmentCriterion {
    id: Uuid,
    criterion: String,
    description: String,
    max_points: u32,
    levels: Vec<RubricLevel>,
}

struct RubricLevel {
    level: String,  // Excellent, Good, Fair, Poor
    points: u32,
    description: String,
}

struct ProjectMilestone {
    id: Uuid,
    title: String,
    description: String,
    due_date: DateTime<Utc>,
    deliverables: Vec<String>,
}

struct GlossaryTerm {
    id: Uuid,
    term: String,
    definition: String,
    contributor_id: Option<Uuid>,
    approved: bool,
    links: Vec<String>,
}

struct PortfolioSection {
    id: Uuid,
    title: String,
    description: String,
    artifacts: Vec<PortfolioArtifact>,
}

struct PortfolioArtifact {
    id: Uuid,
    artifact_type: ArtifactType,
    title: String,
    description: String,
    url: String,
    reflection: Option<String>,
}

enum ArtifactType {
    Document,
    Image,
    Video,
    Link,
    CodeProject,
}

enum SharingSettings {
    Private,
    CourseInstructors,
    CourseParticipants,
    Public,
}

enum LiveSessionFeature {
    Chat,
    ScreenShare,
    Whiteboard,
    BreakoutRooms,
    Recording,
    Polling,
    HandRaise,
}

enum WhiteboardTool {
    Pen,
    Highlighter,
    Eraser,
    Shapes,
    Text,
    Images,
    StickyNotes,
}

struct PollOption {
    id: Uuid,
    text: String,
    votes: u32,
}

enum PollResultsVisibility {
    Hidden,  // Only instructor sees
    AfterVoting,  // Show after user votes
    Live,  // Real-time results
}

enum RoomAssignmentMethod {
    Random,
    Manual,
    SelfSelect,
    BySkillLevel,
}

struct Resource {
    id: Uuid,
    title: String,
    resource_type: ResourceType,
    url: String,
    description: Option<String>,
    tags: Vec<String>,
}

enum ResourceType {
    Article,
    Video,
    PDF,
    Website,
    Tool,
    Dataset,
}

struct Citation {
    id: Uuid,
    authors: Vec<String>,
    title: String,
    publication: String,
    year: u32,
    url: Option<String>,
    annotation: Option<String>,
}

enum CitationStyle {
    APA,
    MLA,
    Chicago,
    IEEE,
    Harvard,
}

enum VRPlatform {
    WebVR,
    WebXR,
    Oculus,
    SteamVR,
    PSVR,
}

enum ARPlatform {
    WebXR,
    ARCore,
    ARKit,
    MagicLeap,
}

struct LearningPath {
    id: Uuid,
    name: String,
    content_sequence: Vec<Uuid>,  // ContentItem IDs
    difficulty_level: DifficultyLevel,
}

enum AdaptationAlgorithm {
    RuleBased,
    BayesianKnowledgeTracing,
    ItemResponseTheory,
    MachineLearning,
}

enum ExternalToolType {
    GoogleDocs,
    GoogleSheets,
    GoogleSlides,
    Office365Word,
    Office365Excel,
    Office365PowerPoint,
    Padlet,
    Miro,
    Figma,
    CodePen,
    Replit,
    Custom,
}

struct IFrameSettings {
    width: String,  // "100%", "800px"
    height: String,
    allow_fullscreen: bool,
    sandbox_attributes: Vec<String>,
}

// Forum Topic Collaboration Tools
enum TopicToolType {
    // Governance & Decision-Making
    Discussion,           // Standard threaded conversations
    Voting,              // Democratic decision-making with vote tallying
    ProposalRFC,         // Formal proposals with structured format
    Elections,           // Democratic elections with candidate profiles
    PollsSurveys,        // Quick lightweight feedback (non-binding)
    ConsensusBuilding,   // Tools for achieving 100% unanimous agreement
    
    // Learning & Collaboration
    LinkedCourseDiscussion,  // Discussion tied to specific courses
    QA,                      // Stack Overflow-style Q&A with accepted answers
    PeerReview,              // Structured peer assessment
    StudyGroups,             // Organize learning cohorts
    MentorshipMatching,      // Connect mentors with learners
    WikiKnowledgeBase,       // Collaborative documentation
    ShowcasePortfolio,       // Share work, get feedback
    
    // Community Management
    EventsCalendar,      // Schedule gatherings with RSVP
    Announcements,       // Official read-only communications
    ResourceSharing,     // Physical/digital resource lending and booking
    
    // Collaboration & Productivity
    Whiteboard,          // Real-time collaborative canvas
    Brainstorming,       // Idea collection and voting
    VideoConference,     // Integrated video calls
    ProjectManagement,   // Task tracking, milestones, deliverables
    HiringJobs,          // Recruitment and project staffing
    
    // Platform Operations
    Development,         // Code collaboration, issue tracking
    BugReportsIssues,    // Platform bug tracking
    AccessibilityFeedback,  // Accessibility issues and improvements
    TranslationLocalization, // Collaborative translation work
    
    // Engagement & Gamification
    EmergencyAlerts,     // High-priority urgent communications
    ChallengesCompetitions, // Learning challenges with leaderboards
}

struct TopicCollaborationData {
    topic_id: Uuid,
    tool_type: TopicToolType,
    data: serde_json::Value,  // Tool-specific data structure
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Example: Voting topic data
struct VotingTopicData {
    title: String,
    description: String,
    options: Vec<VotingOption>,
    voting_type: VotingType,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    requires_unanimous: bool,  // For consensus building
    eligible_voters: EligibleVoters,
}

struct VotingOption {
    id: Uuid,
    option_text: String,
    votes: u32,
    voters: Vec<Uuid>,
}

enum VotingType {
    SingleChoice,
    MultipleChoice { max_selections: u32 },
    RankedChoice,
    Approval,  // Vote yes/no on each option
}

enum EligibleVoters {
    AllMembers,
    RoleHolders { role: Role },
    BadgeHolders { badge_id: Uuid },
    CommunityMembers { community_id: Uuid },
    Custom { user_ids: Vec<Uuid> },
}

// Example: Event topic data
struct EventTopicData {
    title: String,
    description: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    location: EventLocation,
    max_attendees: Option<u32>,
    rsvp_list: Vec<RSVP>,
    requires_badge: Option<Uuid>,
}

struct RSVP {
    user_id: Uuid,
    status: RSVPStatus,
    responded_at: DateTime<Utc>,
    notes: Option<String>,
}

enum RSVPStatus {
    Going,
    Maybe,
    NotGoing,
}

enum EventLocation {
    Physical { address: String, coordinates: Option<(f64, f64)> },
    Virtual { meeting_url: String },
    Hybrid { address: String, meeting_url: String },
}

// Example: Proposal/RFC topic data
struct ProposalTopicData {
    title: String,
    description: String,
    rationale: String,
    impact_analysis: String,
    implementation_plan: Option<String>,
    estimated_cost: Option<String>,
    timeline: Option<String>,
    author_id: Uuid,
    status: ProposalStatus,
    discussion_period_end: DateTime<Utc>,
    voting_period_start: Option<DateTime<Utc>>,
    voting_period_end: Option<DateTime<Utc>>,
}

enum ProposalStatus {
    Draft,
    UnderDiscussion,
    InVoting,
    Approved,
    Rejected,
    Withdrawn,
    Implemented,
}

// Example: Study Group topic data
struct StudyGroupTopicData {
    course_id: Option<Uuid>,
    subject: String,
    description: String,
    schedule: Vec<StudySession>,
    members: Vec<Uuid>,
    max_members: Option<u32>,
    facilitator_id: Option<Uuid>,
}

struct StudySession {
    id: Uuid,
    date_time: DateTime<Utc>,
    duration_minutes: u32,
    topic: String,
    location: EventLocation,
    completed: bool,
}

enum PresentationSource {
    GoogleSlides,
    PDFUpload,  // Stored in IPFS
    SlideShare,
    SpeakerDeck,
}

enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

enum EnrollmentType {
    Open,           // Anyone can enroll (with prerequisites)
    InviteOnly,     // Requires invitation
    Cohort,         // Scheduled cohorts with start/end dates
    SelfPaced,      // Enroll anytime, no deadlines
}

struct QuizQuestion {
    id: Uuid,
    question_text: String,
    question_type: QuizQuestionType,
    points: u32,
    explanation: Option<String>,  // Shown after answer
}

enum QuizQuestionType {
    MultipleChoice {
        options: Vec<QuizOption>,
        correct_option_id: Uuid,
    },
    MultipleAnswer {
        options: Vec<QuizOption>,
        correct_option_ids: Vec<Uuid>,
    },
    TrueFalse {
        correct_answer: bool,
    },
    FillInBlank {
        correct_answers: Vec<String>,  // Accept multiple variations
        case_sensitive: bool,
    },
    Essay {
        min_words: Option<u32>,
        max_words: Option<u32>,
        requires_manual_grading: bool,
    },
}

struct QuizOption {
    id: Uuid,
    text: String,
    sequence: i32,
}

struct SubtitleTrack {
    language: String,  // ISO language code
    url: String,
    label: String,  // Display name
}
```

**Course Progression & Completion Tracking:**
```rust
struct UserCourseEnrollment {
    id: Uuid,
    user_id: Uuid,
    course_id: Uuid,
    
    // Enrollment details
    enrolled_at: DateTime<Utc>,
    enrollment_type: EnrollmentType,
    cohort_id: Option<Uuid>,
    
    // Progress tracking
    status: EnrollmentStatus,
    completed_content_items: Vec<Uuid>,
    current_content_item_id: Option<Uuid>,
    progress_percentage: f32,
    
    // Time tracking
    time_spent_minutes: u32,
    last_accessed_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    
    // Assessment results
    quiz_attempts: Vec<QuizAttempt>,
    certification_score: Option<f32>,
    certification_passed: bool,
    certificate_issued_at: Option<DateTime<Utc>>,
    
    // Gamification
    karma_earned: u32,
}

enum EnrollmentStatus {
    Active,
    Completed,
    Dropped,
    MandatoryRetake,  // Course was replaced
    Expired,          // Certification expired
}

struct QuizAttempt {
    id: Uuid,
    content_item_id: Uuid,
    attempted_at: DateTime<Utc>,
    score_percentage: f32,
    passed: bool,
    answers: Vec<QuizAnswer>,
    time_spent_seconds: u32,
}
```

**Content Delivery Features:**

1. **Fullscreen Learning Mode**
   - Distraction-free content viewing
   - Navigation between content items
   - Progress indicator
   - Note-taking sidebar
   - Bookmarking

2. **Adaptive Content Delivery**
   - Skip completed items automatically
   - Suggest next logical item based on progress
   - Personalized recommendations

3. **Offline Support**
   - Download courses for offline viewing
   - Sync progress when reconnected
   - Cache videos, PDFs, and documents locally

4. **Accessibility Features**
   - Screen reader support
   - Keyboard navigation
   - Transcripts for videos
   - Subtitles/captions
   - High contrast mode
   - Adjustable font sizes

5. **Social Learning Features**
   - Course-specific forums (linked Matrix rooms)
   - Peer discussions on content items
   - Study groups within courses
   - Collaborative notes

6. **Gamification**
   - Karma points for completion
   - Streaks for daily engagement
   - Leaderboards (opt-in)
   - Achievement badges
   - Completion certificates

#### **Course-Linked Forums**

Each course can have a **dedicated forum** (Matrix room) for discussion:

```rust
async fn create_course_forum(course_id: Uuid) -> Result<Uuid> {
    let course = courses.find(course_id)?;
    
    // Create Matrix room for course discussion
    let forum = forums.insert(Forum {
        id: Uuid::new_v4(),
        name: format!("{} - Discussion Forum", course.name),
        scope: course.scope,
        required_badges: course.required_badges.clone(),  // Same access as course
        linked_course_id: Some(course_id),
        readable_to_all: false,  // Badge-gated
        topic_tools_enabled: vec![
            TopicToolType::Discussion,
            TopicToolType::QA,  // Question & Answer format
            TopicToolType::PeerReview,
        ],
    })?;
    
    // Create Matrix room on each territory's server
    for territory in get_all_territories().await? {
        create_matrix_room_for_forum(&territory, &forum).await?;
    }
    
    // Update course with forum link
    courses.update(course_id, |c| c.linked_forum_id = Some(forum.id))?;
    
    Ok(forum.id)
}
```

**Forum Integration Benefits:**
- Students can ask questions about specific content items
- Instructors can post announcements
- Peer-to-peer learning and support
- Archived for future cohorts
- Searchable Q&A repository

#### **Achievement & Credential System**

**Badge Awards Upon Completion:**
```rust
async fn complete_course_and_award_badges(
    user_id: Uuid,
    course_id: Uuid,
) -> Result<Vec<Badge>> {
    let course = courses.find(course_id)?;
    
    // Verify all requirements met (quizzes passed, assignments submitted, etc.)
    verify_completion_requirements(user_id, course_id).await?;
    
    // Award badges
    let mut awarded_badges = vec![];
    for badge_id in &course.badges_awarded {
        let badge = assign_badge(
            user_id,
            *badge_id,
            BadgeSource::CourseCompletion(course_id),
        ).await?;
        
        awarded_badges.push(badge);
        
        // If badge has expiration (e.g., Code of Conduct), set expiry
        if badge.requires_renewal {
            set_badge_expiration(
                user_id,
                *badge_id,
                Utc::now() + Duration::days(badge.renewal_period_days.unwrap() as i64),
            ).await?;
        }
    }
    
    // Issue certification if required
    if course.requires_certification {
        let certification = issue_course_certificate(user_id, course_id).await?;
        awarded_badges.push(certification.badge);
    }
    
    // Issue cryptographic credential (if enabled)
    if course.issue_verifiable_credential {
        issue_verifiable_credential(user_id, course_id, &awarded_badges).await?;
    }
    
    // Unlock new courses/forums based on new badges
    recalculate_user_permissions(user_id).await?;
    
    // Award karma points
    award_karma_points(user_id, course.karma_points_on_completion).await?;
    
    // Notify user of achievements
    notify_user(user_id, Notification::CourseCompleted {
        course: course.name,
        badges: awarded_badges.clone(),
        karma_earned: course.karma_points_on_completion,
    }).await?;
    
    Ok(awarded_badges)
}
```

**Cross-Territory Credential Recognition:**
- Badges earned in one territory are **globally recognized**
- Verifiable Credentials (future) use cryptographic signatures
- Integration with external standards: **Open Badges 2.0/3.0**, **W3C Verifiable Credentials**
- Interoperability with external learning platforms and employers

#### **Adaptive Learning Paths**

```rust
async fn recommend_next_courses(user_id: Uuid) -> Result<Vec<Course>> {
    let user_badges = get_user_badges(user_id).await?;
    let completed_courses = get_completed_courses(user_id).await?;
    
    // Find courses user is eligible for but hasn't taken
    let available_courses = courses
        .filter(|c| {
            // Has all prerequisite badges
            c.prerequisite_badges.iter().all(|b| user_badges.contains(b))
        })
        .filter(|c| {
            // Hasn't completed yet
            !completed_courses.contains(&c.id)
        })
        .load()?;
    
    // Rank by relevance (user's interests, learning style, territory priorities)
    let ranked = rank_courses_by_relevance(user_id, available_courses).await?;
    
    Ok(ranked)
}
```

**Analytics Dashboard:**
- **For Learners**: Progress tracking, badge collection, recommended next steps
- **For Educators**: Course completion rates, struggle points, engagement metrics
- **For Territory Managers**: Territory-wide learning trends, popular courses, achievement statistics

---

### 5. **Multilingual Real-Time Translation**

- **Automatic Translation**: Messages and content translated in real-time to user's preferred language
- **Context-Aware**: Translation preserves technical terms, cultural nuances, and domain-specific vocabulary
- **Translation Memory**: Shared translation database improves consistency across the platform
- **Human Review Layer**: Community-driven translation corrections and improvements
- **Language Learning Mode**: Optional side-by-side view for language learners

**Territory Language Preferences:**
Each territory configures:
- **Primary Language**: Default language for territory
- **Preferred Translation Language**: Automatic translation target for incoming content
- **Supported Languages**: Additional languages available to users in that territory

**Implementation:**
```rust
async fn translate_forum_message(
    message: &str,
    source_lang: Language,
    target_lang: Language,
    context: TranslationContext,
) -> Result<String> {
    // Check translation memory cache first
    if let Some(cached) = translation_memory.get(message, source_lang, target_lang) {
        return Ok(cached);
    }
    
    // Use AI translation service with context awareness
    let translated = translation_service.translate(
        message,
        source_lang,
        target_lang,
        context,  // Course content, forum discussion, technical docs, etc.
    ).await?;
    
    // Store in translation memory
    translation_memory.store(message, source_lang, target_lang, translated.clone())?;
    
    Ok(translated)
}
```

---

### 6. **Decentralized Identity & Authentication**

#### **Current Implementation**

**OpenID Connect (OIDC) Single Sign-On:**
```rust
struct AuthenticationFlow {
    // User authenticates once via OIDC provider
    oidc_provider: String,  // Keycloak, Auth0, etc.
    
    // JWT tokens for session management
    access_token: JWTToken,
    refresh_token: JWTToken,
    
    // Badge-based permissions embedded in token claims
    claims: TokenClaims {
        user_id: Uuid,
        badges: Vec<Uuid>,
        roles: Vec<Role>,
        territories: Vec<Uuid>,  // Territories user has access to
        code_of_conduct_expires: DateTime<Utc>,
    },
}

// Middleware verifies badge permissions on every request
async fn check_permission(token: &JWTToken, required_permission: Permission) -> Result<bool> {
    let claims = verify_and_decode_token(token)?;
    
    // Check Code of Conduct expiration first
    if claims.code_of_conduct_expires < Utc::now() {
        return Err(Error::CodeOfConductExpired);
    }
    
    // Check if user's badges grant required permission
    let user_permissions = get_permissions_from_badges(&claims.badges).await?;
    
    Ok(user_permissions.contains(&required_permission))
}
```

**Multi-Factor Authentication (MFA):**
- TOTP (Time-based One-Time Password) support
- WebAuthn for passwordless authentication
- Backup codes for account recovery
- Required for Platform Administrator and high-privilege roles

**Cross-Server SSO:**
When territories are on different servers, SSO still works:
- Central OIDC provider issues tokens valid across all servers
- API Gateway validates tokens and routes to appropriate territory server
- User maintains single session across all territories they manage

#### **Future Holochain Integration**

**Self-Sovereign Identity (SSI):**
- Users control their own identity without central authority
- Decentralized Identifiers (DIDs) replace centralized user IDs
- Verifiable Credentials for badges and achievements
- Privacy-preserving selective disclosure (prove you have a badge without revealing identity)

**Migration Path:**
```rust
// Hybrid mode: Support both centralized and decentralized identity
enum UserIdentity {
    Centralized {
        user_id: Uuid,
        oidc_provider: String,
    },
    Decentralized {
        did: String,  // did:key:z6Mk... or did:holo:...
        public_key: PublicKey,
    },
    Hybrid {
        // User has both - transitioning to decentralized
        centralized: Uuid,
        did: String,
    },
}
```

#### **User Profile System**

**Profile Data Structure:**
```rust
struct UserProfile {
    // Core Identity
    user_id: Uuid,
    username: String,  // Unique, changeable (with rate limiting)
    display_name: String,
    avatar_url: Option<String>,  // IPFS CID or URL
    bio: Option<String>,
    location: Option<String>,  // City, Territory, or general location
    birthdate: Option<NaiveDate>,  // For age verification, privacy-controlled
    
    // Contact Information (privacy-controlled)
    email: String,  // Primary (verified)
    secondary_emails: Vec<String>,
    phone: Option<String>,
    
    // Social Links (unlimited with type detection)
    social_links: Vec<SocialLink>,
    
    // Language Preferences
    language_preferences: LanguagePreferences,
    
    // UI Preferences
    theme: ThemePreference,
    accessibility: AccessibilitySettings,
    
    // Privacy & Visibility
    privacy_settings: ProfilePrivacySettings,
    
    // Notification Preferences
    notification_settings: NotificationSettings,
    
    // News Feed Preferences
    news_feed_settings: NewsFeedSettings,
    
    // Metadata
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_seen: DateTime<Utc>,
}

struct SocialLink {
    id: Uuid,
    link_type: SocialLinkType,
    url: String,
    display_label: Option<String>,
    visibility: ProfileVisibility,  // Control per-link
}

enum SocialLinkType {
    // Professional
    LinkedIn,
    GitHub,
    GitLab,
    Portfolio,
    PersonalWebsite,
    
    // Social Media
    Twitter,
    Mastodon,
    Bluesky,
    Facebook,
    Instagram,
    TikTok,
    YouTube,
    Twitch,
    Discord,
    Telegram,
    Signal,
    WhatsApp,
    
    // Academic/Research
    ORCID,
    ResearchGate,
    GoogleScholar,
    Academia,
    
    // Creative
    Behance,
    Dribbble,
    DeviantArt,
    ArtStation,
    SoundCloud,
    Bandcamp,
    
    // Other
    Medium,
    Substack,
    Patreon,
    KoFi,
    Custom { 
        name: String,
        icon_url: Option<String>,
    },
}

struct LanguagePreferences {
    // Primary language for UI and content
    preferred_language: String,  // ISO 639-1 code (e.g., "en", "fr", "es")
    
    // Ordered list of secondary languages with proficiency levels
    secondary_languages: Vec<LanguageProficiency>,
    
    // Fallback to English if preferred languages unavailable
    fallback_to_english: bool,  // Default: true
    
    // Auto-translate content not in preferred languages
    auto_translate: bool,  // Default: true
    translation_provider: TranslationProvider,
}

struct LanguageProficiency {
    language_code: String,  // ISO 639-1
    language_name: String,  // "Spanish", "Français", etc.
    spoken_level: ProficiencyLevel,
    written_level: ProficiencyLevel,
    listening_level: ProficiencyLevel,
    reading_level: ProficiencyLevel,
}

enum ProficiencyLevel {
    Native,
    Fluent,
    Advanced,
    Intermediate,
    Basic,
    Learning,
}

enum TranslationProvider {
    DeepL,
    LibreTranslate,  // Self-hosted, privacy-focused
    Google,
    Microsoft,
}

struct ThemePreference {
    mode: ThemeMode,
    accent_color: Option<String>,  // Hex color for personalization
    font_size: FontSize,
    high_contrast: bool,
}

enum ThemeMode {
    Light,
    Dark,
    Auto,  // System preference
    Custom { background: String, text: String },
}

enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

struct AccessibilitySettings {
    // Visual
    screen_reader_enabled: bool,
    reduce_motion: bool,
    high_contrast_mode: bool,
    color_blind_mode: Option<ColorBlindMode>,
    
    // Text
    dyslexic_font: bool,
    increased_letter_spacing: bool,
    
    // Audio
    captions_always_on: bool,
    audio_descriptions: bool,
    sign_language_preferred: bool,
    
    // Interaction
    keyboard_navigation_enhanced: bool,
    focus_indicators_enhanced: bool,
    
    // Reading
    text_to_speech_enabled: bool,
    reading_guide: bool,
    simplified_ui: bool,
}

enum ColorBlindMode {
    Protanopia,    // Red-blind
    Deuteranopia,  // Green-blind
    Tritanopia,    // Blue-blind
    Monochromacy,  // Complete color blindness
}

struct ProfilePrivacySettings {
    // Overall visibility preset
    preset: PrivacyPreset,
    
    // Custom visibility per field (overrides preset)
    custom_visibility: HashMap<ProfileField, ProfileVisibility>,
    
    // Who can see your profile
    profile_visibility: ProfileVisibility,
    
    // Who can contact you
    contact_visibility: ProfileVisibility,
    
    // Who can see your activity
    activity_visibility: ProfileVisibility,
    
    // Social familiarity groups
    social_groups: Vec<SocialGroup>,
}

enum PrivacyPreset {
    Private,         // Only me
    FamilyOnly,      // Family group only
    CloseFriends,    // Family + close friends
    Friends,         // Family + close friends + friends
    Acquaintances,   // All above + acquaintances
    Community,       // All above + community members
    Territory,       // All above + territory members
    Public,          // Everyone
    Custom,          // Fully customized per field
}

enum ProfileVisibility {
    OnlyMe,
    Family,
    CloseFriends,
    Friends,
    Acquaintances,
    CommunityMembers,
    TerritoryMembers,
    BadgeHolders { badge_id: Uuid },  // Only users with specific badge
    CustomGroup { group_id: Uuid },
    Public,
}

enum ProfileField {
    DisplayName,
    Avatar,
    Bio,
    Location,
    Birthdate,
    Email,
    Phone,
    SocialLinks,
    Badges,
    CourseProgress,
    Achievements,
    CommunityMemberships,
    ActivityFeed,
    Friends,
    ProjectUpdates,
}

struct SocialGroup {
    id: Uuid,
    name: String,
    description: Option<String>,
    familiarity_level: FamiliarityLevel,
    members: Vec<Uuid>,  // User IDs
    created_by: Uuid,
    created_at: DateTime<Utc>,
}

enum FamiliarityLevel {
    Family,
    CloseFriends,
    Friends,
    Acquaintances,
    Professional,
    Community,
    Custom,
}

struct NotificationSettings {
    // Delivery channels
    email_notifications: bool,
    push_notifications: bool,
    in_app_notifications: bool,
    
    // Notification categories
    course_updates: NotificationPreference,
    forum_activity: NotificationPreference,
    badge_awards: NotificationPreference,
    direct_messages: NotificationPreference,
    mentions: NotificationPreference,
    friend_activity: NotificationPreference,
    community_announcements: NotificationPreference,
    election_voting: NotificationPreference,
    
    // Digest options
    daily_digest: bool,
    weekly_summary: bool,
    
    // Quiet hours
    quiet_hours_enabled: bool,
    quiet_hours_start: Option<NaiveTime>,
    quiet_hours_end: Option<NaiveTime>,
}

enum NotificationPreference {
    All,           // Every update
    Important,     // Only important updates
    Mentions,      // Only when mentioned
    Off,           // No notifications
}

struct NewsFeedSettings {
    // Content sources
    show_course_updates: bool,
    show_forum_activity: bool,
    show_friend_activity: bool,
    show_community_posts: bool,
    show_badge_awards: bool,
    show_project_updates: bool,
    
    // Filtered by participation
    followed_categories: Vec<Uuid>,
    followed_courses: Vec<Uuid>,
    followed_forums: Vec<Uuid>,
    followed_topics: Vec<Uuid>,
    followed_users: Vec<Uuid>,
    
    // Feed algorithm
    feed_algorithm: FeedAlgorithm,
    
    // Languages to show
    show_languages: Vec<String>,  // ISO codes
}

enum FeedAlgorithm {
    Chronological,        // Latest first
    Relevance,            // Based on your interests
    Popular,              // Most engaged content
    MostDiscussed,        // Most comments
    Custom,               // User-defined weights
}
```

**Privacy Preset Configurations:**
```rust
impl PrivacyPreset {
    fn get_default_visibility(&self) -> HashMap<ProfileField, ProfileVisibility> {
        use ProfileField::*;
        use ProfileVisibility::*;
        
        match self {
            PrivacyPreset::Private => hashmap! {
                DisplayName => OnlyMe,
                Avatar => OnlyMe,
                Bio => OnlyMe,
                Location => OnlyMe,
                Birthdate => OnlyMe,
                Email => OnlyMe,
                Phone => OnlyMe,
                SocialLinks => OnlyMe,
                Badges => OnlyMe,
                CourseProgress => OnlyMe,
                Achievements => OnlyMe,
                CommunityMemberships => OnlyMe,
                ActivityFeed => OnlyMe,
                Friends => OnlyMe,
                ProjectUpdates => OnlyMe,
            },
            PrivacyPreset::FamilyOnly => hashmap! {
                DisplayName => Family,
                Avatar => Family,
                Bio => Family,
                Location => Family,
                Birthdate => Family,
                Email => OnlyMe,
                Phone => OnlyMe,
                SocialLinks => Family,
                Badges => Family,
                CourseProgress => Family,
                Achievements => Family,
                CommunityMemberships => Family,
                ActivityFeed => Family,
                Friends => Family,
                ProjectUpdates => Family,
            },
            PrivacyPreset::Friends => hashmap! {
                DisplayName => Friends,
                Avatar => Friends,
                Bio => Friends,
                Location => Friends,
                Birthdate => Friends,
                Email => OnlyMe,
                Phone => CloseFriends,
                SocialLinks => Friends,
                Badges => Friends,
                CourseProgress => Friends,
                Achievements => Friends,
                CommunityMemberships => Friends,
                ActivityFeed => Friends,
                Friends => Friends,
                ProjectUpdates => Friends,
            },
            PrivacyPreset::Public => hashmap! {
                DisplayName => Public,
                Avatar => Public,
                Bio => Public,
                Location => Public,
                Birthdate => OnlyMe,
                Email => OnlyMe,
                Phone => OnlyMe,
                SocialLinks => Public,
                Badges => Public,
                CourseProgress => Public,
                Achievements => Public,
                CommunityMemberships => Public,
                ActivityFeed => CommunityMembers,
                Friends => Public,
                ProjectUpdates => Public,
            },
            // ... other presets
            _ => HashMap::new(),
        }
    }
}
```

**Peer-to-Peer Encrypted Communication (Non-Matrix):**
```rust
// For small group direct messaging (2-10 users) with end-to-end encryption
struct P2PConversation {
    id: Uuid,
    participants: Vec<Uuid>,  // 2-10 users
    encryption_type: EncryptionType,
    
    // Signal Protocol for forward secrecy
    ratchet_state: Option<RatchetState>,
    
    // Conversation metadata
    created_at: DateTime<Utc>,
    last_message_at: DateTime<Utc>,
    
    // Message storage (local only, not on server)
    local_storage_path: String,
}

enum EncryptionType {
    SignalProtocol,  // For 1-on-1 and small groups
    OlmMegolm,       // Alternative for larger groups
    WebRTC,          // For real-time voice/video
}

struct RatchetState {
    // Signal Protocol double ratchet state
    root_key: Vec<u8>,
    chain_keys: HashMap<Uuid, Vec<u8>>,  // Per participant
    message_keys: Vec<Vec<u8>>,
}

// P2P messaging without central server (WebRTC data channels)
async fn send_p2p_message(
    conversation_id: Uuid,
    message: &str,
    recipients: Vec<Uuid>,
) -> Result<()> {
    let conversation = get_p2p_conversation(conversation_id)?;
    
    // Encrypt message with Signal Protocol
    let encrypted_messages = encrypt_for_recipients(
        message,
        &recipients,
        &conversation.ratchet_state,
    )?;
    
    // Send via WebRTC data channels (peer-to-peer)
    for (recipient_id, encrypted_msg) in encrypted_messages {
        if let Some(peer_connection) = get_peer_connection(recipient_id).await? {
            peer_connection.send_data_channel_message(encrypted_msg).await?;
        } else {
            // Fallback: Store encrypted message for later delivery
            queue_offline_message(recipient_id, encrypted_msg).await?;
        }
    }
    
    Ok(())
}
```

**Daily Project Updates & Personal Sharing:**
```rust
struct ProjectUpdate {
    id: Uuid,
    user_id: Uuid,
    project_id: Option<Uuid>,  // Link to a project if structured
    
    // Content
    title: String,
    content: String,  // Markdown
    media: Vec<MediaAttachment>,
    tags: Vec<String>,
    
    // Categorization
    update_type: UpdateType,
    visibility: ProfileVisibility,
    
    // Engagement
    reactions: HashMap<Reaction, Vec<Uuid>>,  // User IDs who reacted
    comments: Vec<Comment>,
    
    // Metadata
    created_at: DateTime<Utc>,
    edited_at: Option<DateTime<Utc>>,
}

enum UpdateType {
    DailyProgress,
    Milestone,
    Problem,
    Solution,
    Learning,
    Idea,
    Question,
    Showcase,
    Custom { category: String },
}

struct MediaAttachment {
    id: Uuid,
    media_type: MediaType,
    url: String,  // IPFS CID or URL
    thumbnail_url: Option<String>,
    caption: Option<String>,
}

enum MediaType {
    Image,
    Video,
    Audio,
    Document,
    Code,
    Link,
}

enum Reaction {
    Like,
    Love,
    Celebrate,
    Support,
    Insightful,
    Curious,
    Custom { emoji: String },
}

struct Comment {
    id: Uuid,
    user_id: Uuid,
    content: String,
    created_at: DateTime<Utc>,
    edited_at: Option<DateTime<Utc>>,
    replies: Vec<Comment>,  // Nested comments
}

// Example: User shares daily progress on learning project
async fn create_project_update(
    user_id: Uuid,
    project_id: Option<Uuid>,
    update: ProjectUpdateInput,
) -> Result<ProjectUpdate> {
    // Check user's privacy settings for project updates
    let privacy_settings = get_user_privacy_settings(user_id).await?;
    
    let update = ProjectUpdate {
        id: Uuid::new_v4(),
        user_id,
        project_id,
        title: update.title,
        content: update.content,
        media: update.media,
        tags: update.tags,
        update_type: update.update_type,
        visibility: privacy_settings.get_visibility(ProfileField::ProjectUpdates),
        reactions: HashMap::new(),
        comments: Vec::new(),
        created_at: Utc::now(),
        edited_at: None,
    };
    
    // Store update
    db.project_updates.insert(&update)?;
    
    // Notify followers if visibility allows
    notify_update_followers(user_id, &update).await?;
    
    Ok(update)
}
```

#### **Future Holochain Integration**

**Profile DNA (holochain-open-dev/profiles):**
```rust
// Integration with holochain-open-dev/profiles
// https://github.com/holochain-open-dev/profiles

use holochain_open_dev::profiles::{Profile, ProfilesStore};

#[hdk_extern]
pub fn create_profile(profile: ProfileInput) -> ExternResult<Profile> {
    // User-owned profile data stored in their source chain
    let profile = Profile {
        agent_pub_key: agent_info()?.agent_latest_pubkey,
        nickname: profile.nickname,
        avatar: profile.avatar,  // IPFS CID
        fields: profile.custom_fields,  // Flexible schema
        created_at: sys_time()?,
    };
    
    create_entry(&EntryTypes::Profile(profile.clone()))?;
    
    // Create link for profile discovery
    create_link(
        Path::from("all_profiles").path_entry_hash()?,
        profile.agent_pub_key.clone(),
        LinkTypes::AllProfiles,
        (),
    )?;
    
    Ok(profile)
}

// Hybrid mode: Sync centralized profile to Holochain
async fn sync_profile_to_holochain(user_id: Uuid) -> Result<()> {
    let profile = get_user_profile(user_id).await?;
    let holochain_agent = get_user_holochain_agent(user_id).await?;
    
    // Create profile in Holochain DNA
    holochain_agent.call_zome(
        "profiles",
        "create_profile",
        ProfileInput {
            nickname: profile.display_name,
            avatar: profile.avatar_url,
            custom_fields: hashmap! {
                "bio" => profile.bio,
                "location" => profile.location,
                "social_links" => serde_json::to_string(&profile.social_links)?,
            },
        },
    ).await?;
    
    Ok(())
}
```

#### **Database Schema Updates**

```sql
-- Global schema
global:
  - user_profiles            -- User profile information and preferences
  - user_languages           -- Language proficiency levels
  - user_social_links        -- Social media and website links
  - user_privacy_settings    -- Privacy and visibility controls
  - user_social_groups       -- Custom social familiarity groups
  - user_notification_prefs  -- Notification preferences
  - user_news_feed_prefs     -- News feed settings
  - p2p_conversations        -- P2P encrypted conversation metadata
  - project_updates          -- User daily project updates/sharing
  - update_reactions         -- Reactions to project updates
  - update_comments          -- Comments on project updates
  - user_follows             -- User following relationships

-- Territory schema additions
territory_{id}:
  - user_activity_feed       -- Aggregated feed of user activities
```

---

### 6b. **Peer-to-Peer Communication Infrastructure**

For small group encrypted messaging (2-10 users) outside of Matrix, the platform requires infrastructure for WebRTC peer connections and Signal Protocol key management.

#### **Signal Protocol Key Server**

**Pre-Key Distribution & Management:**
```rust
// Signal Protocol requires a key server for initial handshake
struct SignalKeyServer {
    // Users upload pre-keys that others can fetch to initiate encrypted sessions
    pre_key_store: PreKeyStore,
    signed_pre_key_store: SignedPreKeyStore,
    identity_key_store: IdentityKeyStore,
}

struct PreKeyBundle {
    user_id: Uuid,
    device_id: u32,  // Users can have multiple devices
    registration_id: u32,
    identity_key: PublicKey,
    signed_pre_key: SignedPreKey,
    pre_keys: Vec<PreKey>,
}

// User uploads pre-keys on registration or device addition
async fn upload_pre_keys(user_id: Uuid, device_id: u32, bundle: PreKeyBundle) -> Result<()> {
    // Verify signature on signed pre-key
    verify_signed_pre_key(&bundle.signed_pre_key, &bundle.identity_key)?;
    
    // Store pre-keys for retrieval
    key_server.store_pre_key_bundle(user_id, device_id, bundle).await?;
    
    // Rotate signed pre-key weekly for forward secrecy
    schedule_key_rotation(user_id, device_id, Duration::days(7)).await?;
    
    Ok(())
}

// Other users fetch pre-keys to initiate encrypted conversation
async fn fetch_pre_key_bundle(target_user_id: Uuid, device_id: u32) -> Result<PreKeyBundle> {
    let bundle = key_server.get_pre_key_bundle(target_user_id, device_id).await?;
    
    // Mark pre-key as used (one-time keys)
    key_server.mark_pre_key_used(target_user_id, device_id, &bundle.pre_keys[0]).await?;
    
    Ok(bundle)
}
```

**Session Management:**
```rust
struct SessionStore {
    // Stores Double Ratchet state per conversation
    sessions: HashMap<ConversationId, SessionState>,
}

struct SessionState {
    conversation_id: Uuid,
    participants: Vec<Uuid>,
    root_key: Vec<u8>,
    chain_keys: HashMap<Uuid, ChainKey>,
    
    // For forward secrecy
    message_keys: Vec<MessageKey>,
    skipped_message_keys: HashMap<u32, MessageKey>,
    
    // X3DH initial key agreement
    shared_secret: Vec<u8>,
}

// Establish encrypted session using X3DH + Double Ratchet
async fn establish_p2p_session(
    initiator_id: Uuid,
    recipient_id: Uuid,
) -> Result<SessionState> {
    // 1. Fetch recipient's pre-key bundle
    let bundle = fetch_pre_key_bundle(recipient_id, 0).await?;
    
    // 2. Perform X3DH key agreement
    let shared_secret = perform_x3dh(
        initiator_id,
        &bundle.identity_key,
        &bundle.signed_pre_key,
        &bundle.pre_keys[0],
    )?;
    
    // 3. Initialize Double Ratchet
    let (root_key, chain_key) = initialize_ratchet(&shared_secret)?;
    
    let session = SessionState {
        conversation_id: Uuid::new_v4(),
        participants: vec![initiator_id, recipient_id],
        root_key,
        chain_keys: hashmap! { recipient_id => chain_key },
        message_keys: Vec::new(),
        skipped_message_keys: HashMap::new(),
        shared_secret,
    };
    
    // Store session locally (not on server)
    store_session_locally(&session)?;
    
    Ok(session)
}
```

**Rust Dependencies:**
```toml
[dependencies]
libsignal-protocol = "0.1"  # Signal Protocol implementation
x25519-dalek = "2.0"        # Elliptic curve Diffie-Hellman
ed25519-dalek = "2.0"       # Signature verification
```

#### **WebRTC Signaling & NAT Traversal Infrastructure**

**STUN Servers (NAT Discovery):**
```rust
struct StunServerConfig {
    // Public STUN servers for discovering public IP/port
    servers: Vec<StunServer>,
}

struct StunServer {
    url: String,  // stun:stun.l.google.com:19302
    priority: u8,
}

// Default configuration using public STUN servers
fn default_stun_config() -> StunServerConfig {
    StunServerConfig {
        servers: vec![
            StunServer { url: "stun:stun.l.google.com:19302".into(), priority: 1 },
            StunServer { url: "stun:stun1.l.google.com:19302".into(), priority: 2 },
            StunServer { url: "stun:stun2.l.google.com:19302".into(), priority: 3 },
            StunServer { url: "stun:stun.services.mozilla.com:3478".into(), priority: 4 },
        ],
    }
}
```

**TURN Servers (Relay for Restricted NATs):**
```rust
struct TurnServerConfig {
    // Self-hosted TURN servers for relaying when direct P2P fails
    servers: Vec<TurnServer>,
}

struct TurnServer {
    url: String,          // turn:turn.example.com:3478
    username: String,     // Temporary credentials
    credential: String,   // Time-limited token
    realm: String,
    deployment: TurnDeployment,
}

enum TurnDeployment {
    Global,                    // Shared TURN server
    Regional { region: String },  // Per-continent TURN
    Territory { id: Uuid },    // Territory-specific TURN
}

// Generate temporary TURN credentials (expires in 24 hours)
async fn generate_turn_credentials(user_id: Uuid) -> Result<TurnCredentials> {
    let timestamp = Utc::now().timestamp() + 86400;  // 24 hours
    let username = format!("{}:{}", timestamp, user_id);
    
    // HMAC-SHA1 of username with shared secret
    let credential = generate_turn_credential(&username, &TURN_SECRET)?;
    
    Ok(TurnCredentials {
        username,
        credential,
        ttl: 86400,
        urls: get_turn_server_urls(user_id).await?,
    })
}

// Deploy TURN servers using coturn
// Docker compose configuration:
/*
services:
  coturn:
    image: coturn/coturn:latest
    ports:
      - "3478:3478/tcp"
      - "3478:3478/udp"
      - "49152-65535:49152-65535/udp"  # Relay ports
    environment:
      - TURN_SHARED_SECRET=${TURN_SECRET}
      - TURN_REALM=turn.unityplan.org
      - TURN_EXTERNAL_IP=${SERVER_PUBLIC_IP}
    volumes:
      - ./coturn.conf:/etc/coturn/turnserver.conf
    restart: unless-stopped
*/
```

**WebRTC Signaling Server:**
```rust
// Signaling server for WebRTC offer/answer exchange
struct SignalingServer {
    // WebSocket connections per user
    connections: Arc<Mutex<HashMap<Uuid, WebSocketConnection>>>,
}

#[derive(Serialize, Deserialize)]
enum SignalingMessage {
    Offer {
        from: Uuid,
        to: Uuid,
        sdp: String,  // Session Description Protocol
        conversation_id: Uuid,
    },
    Answer {
        from: Uuid,
        to: Uuid,
        sdp: String,
        conversation_id: Uuid,
    },
    IceCandidate {
        from: Uuid,
        to: Uuid,
        candidate: String,
        conversation_id: Uuid,
    },
    ConnectionEstablished {
        conversation_id: Uuid,
    },
}

// Handle WebRTC signaling over WebSocket
async fn handle_signaling_message(
    ws: &mut WebSocket,
    msg: SignalingMessage,
) -> Result<()> {
    match msg {
        SignalingMessage::Offer { from, to, sdp, conversation_id } => {
            // Forward offer to recipient
            if let Some(recipient_ws) = get_user_websocket(to).await? {
                recipient_ws.send(json!({
                    "type": "offer",
                    "from": from,
                    "sdp": sdp,
                    "conversation_id": conversation_id,
                })).await?;
            } else {
                // Recipient offline - queue for delivery
                queue_signaling_message(to, msg).await?;
            }
        },
        SignalingMessage::Answer { from, to, sdp, conversation_id } => {
            // Forward answer to initiator
            if let Some(initiator_ws) = get_user_websocket(to).await? {
                initiator_ws.send(json!({
                    "type": "answer",
                    "from": from,
                    "sdp": sdp,
                    "conversation_id": conversation_id,
                })).await?;
            }
        },
        SignalingMessage::IceCandidate { from, to, candidate, conversation_id } => {
            // Exchange ICE candidates for NAT traversal
            if let Some(peer_ws) = get_user_websocket(to).await? {
                peer_ws.send(json!({
                    "type": "ice_candidate",
                    "from": from,
                    "candidate": candidate,
                    "conversation_id": conversation_id,
                })).await?;
            }
        },
        SignalingMessage::ConnectionEstablished { conversation_id } => {
            // P2P connection established, signaling server no longer needed
            // All future messages go directly peer-to-peer via WebRTC data channel
            log::info!("P2P connection established for conversation {}", conversation_id);
        },
    }
    
    Ok(())
}
```

**Client-Side WebRTC Setup:**
```typescript
// Frontend: Establish P2P connection with WebRTC
async function establishP2PConnection(
  recipientId: string,
  conversationId: string
): Promise<RTCPeerConnection> {
  // 1. Get ICE servers (STUN/TURN)
  const iceServers = await fetch('/api/webrtc/ice-servers').then(r => r.json());
  
  // 2. Create peer connection
  const pc = new RTCPeerConnection({
    iceServers: [
      { urls: 'stun:stun.l.google.com:19302' },
      { 
        urls: iceServers.turn.urls,
        username: iceServers.turn.username,
        credential: iceServers.turn.credential
      }
    ]
  });
  
  // 3. Create data channel for messages
  const dataChannel = pc.createDataChannel('messages', {
    ordered: true,
    maxRetransmits: 3
  });
  
  dataChannel.onopen = () => {
    console.log('P2P data channel open');
    // Now send encrypted messages directly via dataChannel.send()
  };
  
  dataChannel.onmessage = (event) => {
    // Decrypt message with Signal Protocol
    const decrypted = decryptSignalMessage(event.data, conversationId);
    displayMessage(decrypted);
  };
  
  // 4. Create and send offer
  const offer = await pc.createOffer();
  await pc.setLocalDescription(offer);
  
  // Send via signaling server (WebSocket)
  signalingSocket.send({
    type: 'offer',
    to: recipientId,
    sdp: offer.sdp,
    conversation_id: conversationId
  });
  
  // 5. Handle ICE candidates
  pc.onicecandidate = (event) => {
    if (event.candidate) {
      signalingSocket.send({
        type: 'ice_candidate',
        to: recipientId,
        candidate: event.candidate,
        conversation_id: conversationId
      });
    }
  };
  
  return pc;
}
```

#### **Infrastructure Requirements Summary**

**Per-Territory Deployment:**
```yaml
# Signal Protocol Key Server
signal-key-server:
  cpu: 2 cores
  ram: 4GB
  storage: 50GB SSD  # For pre-key storage
  bandwidth: Low (only key exchange)
  deployment: One per territory

# WebRTC Signaling Server
webrtc-signaling:
  cpu: 2 cores
  ram: 4GB
  storage: 10GB
  bandwidth: Low (WebSocket only, not message relay)
  deployment: One per territory
  
# TURN Server (coturn)
turn-server:
  cpu: 4 cores
  ram: 8GB
  storage: 20GB
  bandwidth: HIGH (relays actual messages when P2P fails)
  deployment: One per region (continent)
  estimated_usage: 10-20% of conversations (most use direct P2P)
  
# STUN Servers
stun-servers:
  deployment: Use public Google/Mozilla STUN servers
  cost: Free
```

**Bandwidth Considerations:**
- **Direct P2P (80-90% of connections)**: No server bandwidth used
- **TURN Relay (10-20% of connections)**: Server relays encrypted messages
- **Estimate**: 100 concurrent TURN-relayed conversations = ~50 Mbps
- **Optimization**: Deploy regional TURN servers to reduce latency

**Benefits of P2P Infrastructure:**
1. **Privacy**: Messages don't touch servers (when direct P2P works)
2. **Scalability**: Server load doesn't grow with message volume
3. **Low Latency**: Direct peer-to-peer connection
4. **Encryption**: End-to-end via Signal Protocol
5. **Small Groups**: Perfect for 2-10 user conversations without Matrix overhead

**Fallback Strategy:**
- If P2P connection fails after 10 seconds, offer to continue via Matrix (federated but server-based)
- User can choose privacy (P2P but may fail) vs reliability (Matrix always works)

---

### 7. **Distributed File Storage (IPFS)**

**Content-Addressed Storage for Course Materials:**

```rust
struct CourseFile {
    ipfs_cid: String,  // Content Identifier (e.g., QmX... or bafybei...)
    filename: String,
    mime_type: String,
    size_bytes: u64,
    uploaded_by: Uuid,
    territory_scope: Option<Uuid>,  // Territory-specific or global
    pinned_on_nodes: Vec<String>,   // Which IPFS nodes have this pinned
}

async fn upload_course_material(
    file: FileUpload,
    uploader_id: Uuid,
    scope: Scope,
) -> Result<CourseFile> {
    // Verify uploader has Content Creator badge
    verify_badge(uploader_id, "Content Creator").await?;
    
    // Upload to IPFS
    let cid = ipfs_client.add(file.data).await?;
    
    // Pin on multiple nodes for redundancy
    pin_on_territory_nodes(cid, scope).await?;
    
    // Store metadata in database
    let course_file = course_files.insert(CourseFile {
        ipfs_cid: cid.clone(),
        filename: file.name,
        mime_type: file.mime_type,
        size_bytes: file.size,
        uploaded_by: uploader_id,
        territory_scope: scope.territory_id(),
        pinned_on_nodes: get_pinning_nodes(scope).await?,
    })?;
    
    Ok(course_file)
}
```

**Benefits:**
- **Deduplication**: Same file uploaded multiple times only stored once
- **Integrity**: Content hash guarantees file hasn't been tampered with
- **Resilience**: Files pinned on multiple nodes survive individual node failures
- **Offline Access**: Users can pin important course materials locally
- **Bandwidth Optimization**: Popular files can be served from nearby IPFS nodes
- **Future Decentralization**: Seamless transition when migrating to Holochain

**Content Types:**
- Course videos, PDFs, presentations
- User-uploaded assignments and projects
- Forum attachments
- Badge/certificate images
- Territory-specific media assets

---

### 8. **Event-Driven Architecture & Audit Trail**

#### **Message Bus (NATS) for Event Distribution**

**Event Types:**
```rust
enum PlatformEvent {
    // Badge events
    BadgeAwarded { user_id: Uuid, badge_id: Uuid, source: BadgeSource },
    BadgeRevoked { user_id: Uuid, badge_id: Uuid, reason: String },
    BadgeExpiring { user_id: Uuid, badge_id: Uuid, days_remaining: u32 },
    
    // Course events
    CourseCompleted { user_id: Uuid, course_id: Uuid, badges_awarded: Vec<Uuid> },
    CourseEdited { course_id: Uuid, editor_id: Uuid, change_type: CourseChangeType },
    CourseReplaced { old_course: Uuid, new_course: Uuid, retake_deadline: DateTime<Utc> },
    MandatoryRetakeRequired { user_id: Uuid, course_id: Uuid, deadline: DateTime<Utc> },
    
    // Forum events
    TopicCreated { forum_id: Uuid, topic_id: Uuid, creator_id: Uuid },
    CommentFlagged { comment_id: Uuid, moderator_id: Uuid, reason: String },
    UserWarned { user_id: Uuid, moderator_id: Uuid, warning_count: u32 },
    
    // Administrative events
    TerritoryMigrated { territory_id: Uuid, from_server: String, to_server: String },
    VoteRequired { voting_session_id: Uuid, voters: Vec<Uuid>, proposal: String },
    VoteCompleted { voting_session_id: Uuid, outcome: VoteOutcome },
    
    // Permission events
    PermissionsRecalculated { user_id: Uuid, new_permissions: Vec<Permission> },
    RoleAssigned { user_id: Uuid, role: Role, scope: Scope, assigned_by: Uuid },
}

// Services subscribe to relevant events
async fn subscribe_to_events() {
    let mut subscriber = nats_client.subscribe("platform.events.*").await?;
    
    while let Some(message) = subscriber.next().await {
        let event: PlatformEvent = serde_json::from_slice(&message.data)?;
        
        match event {
            PlatformEvent::BadgeAwarded { user_id, badge_id, .. } => {
                // Recalculate user permissions
                recalculate_user_permissions(user_id).await?;
                
                // Check if badge unlocks new courses/forums
                unlock_new_content(user_id, badge_id).await?;
            }
            
            PlatformEvent::CourseReplaced { old_course, new_course, retake_deadline } => {
                // Create mandatory retake enrollments
                handle_course_replacement(old_course, new_course, retake_deadline).await?;
            }
            
            PlatformEvent::VoteRequired { voting_session_id, voters, .. } => {
                // Notify voters
                for voter_id in voters {
                    notify_vote_required(voter_id, voting_session_id).await?;
                }
            }
            
            _ => {}
        }
    }
}
```

#### **Immutable Audit Trail**

**All significant actions logged permanently:**
```rust
struct AuditEntry {
    id: Uuid,
    timestamp: DateTime<Utc>,
    event_type: AuditEventType,
    actor_id: Uuid,  // Who performed the action
    target_id: Option<Uuid>,  // What was affected
    scope: Scope,
    details: serde_json::Value,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

enum AuditEventType {
    // Administrative
    RoleAssigned, RoleRevoked, TerritoryCreated, TerritoryMigrated,
    
    // Content management
    CourseCreated, CourseEdited, CourseReplaced, CourseFlagged,
    ForumCreated, ForumClosed, TopicCreated, CommentPosted, CommentFlagged,
    
    // User actions
    UserInvited, UserJoined, BadgeAssigned, BadgeRevoked,
    WarningIssued, UserSuspended, AccountDeleted,
    
    // Governance
    VoteProposed, VoteCast, VoteCompleted,
    
    // Security
    LoginSuccess, LoginFailed, MFAEnabled, PasswordReset, PermissionDenied,
}

// Audit log is append-only, never deleted
async fn log_audit_event(event: AuditEntry) -> Result<()> {
    audit_log.insert(event)?;
    
    // Also publish to event bus for real-time monitoring
    event_bus.publish(Event::AuditLogEntry(event)).await?;
    
    Ok(())
}
```

**Transparency Features:**
- **Public Audit Log**: Certain events (role assignments, course changes, voting outcomes) are publicly viewable
- **User Access**: Users can view audit log entries related to their own account
- **Territory Transparency**: Territory Managers can view all actions within their territories
- **Immutability**: Records cannot be modified or deleted, ensuring accountability

**Future: Cryptographically Signed Events (Holochain):**
- Each event signed with actor's private key
- Hash chain links events for tamper detection
- Distributed verification across nodes
- Complete transparency without central authority

---

### 9. **Background Jobs & Automation Engine**

**Scheduled & Event-Driven Tasks:**

```rust
// Automated badge expiration checks
#[scheduled("0 2 * * *")]  // Run at 2 AM daily
async fn check_badge_expirations() {
    // Implementation shown earlier
}

// Mandatory course retake deadline enforcement
#[scheduled("0 3 * * *")]  // Run at 3 AM daily
async fn check_retake_deadlines() {
    // Implementation shown earlier
}

// Voting session deadline checks
#[scheduled("*/30 * * * *")]  // Every 30 minutes
async fn check_voting_deadlines() {
    let expired_votes = voting_sessions
        .filter(status.eq(VotingStatus::Active))
        .filter(voting_period_ends.lt(Utc::now()))
        .load()?;
    
    for session in expired_votes {
        // Tally votes
        let outcome = if session.unanimous_required {
            // 100% required - check if all voted yes
            let all_voted_yes = session.votes.len() == session.required_voters.len()
                && session.votes.iter().all(|v| v.approved);
            
            if all_voted_yes {
                VoteOutcome::Approved
            } else {
                VoteOutcome::Rejected
            }
        } else {
            // Simple majority
            let yes_votes = session.votes.iter().filter(|v| v.approved).count();
            if yes_votes > session.required_voters.len() / 2 {
                VoteOutcome::Approved
            } else {
                VoteOutcome::Rejected
            }
        };
        
        // Apply outcome
        match outcome {
            VoteOutcome::Approved => apply_proposal(session.proposal_type).await?,
            VoteOutcome::Rejected => notify_proposal_rejected(session).await?,
        }
        
        // Update status
        voting_sessions.update(session.id, VotingStatus::Completed(outcome))?;
        
        // Publish event
        event_bus.publish(Event::VoteCompleted {
            voting_session_id: session.id,
            outcome,
        }).await?;
    }
}

// Course content synchronization across territories
#[event_triggered(PlatformEvent::CourseShared)]
async fn sync_shared_course(event: CourseSharingEvent) {
    let course = courses.find(event.course_id)?;
    
    for territory_id in event.shared_with_territories {
        // Replicate course to territory schema
        replicate_course_to_territory(course.id, territory_id).await?;
        
        // Notify Territory Managers
        notify_territory_managers(territory_id, Notification::NewCourseAvailable {
            course_name: course.name.clone(),
        }).await?;
    }
}

// Notification digest aggregation
#[scheduled("0 8 * * *")]  // 8 AM daily
async fn send_daily_notification_digests() {
    let users_with_pending = get_users_with_pending_notifications().await?;
    
    for user_id in users_with_pending {
        let notifications = get_pending_notifications(user_id).await?;
        
        send_email_digest(user_id, notifications).await?;
        
        mark_notifications_as_sent(notifications).await?;
    }
}
```

**Automation Categories:**
1. **Badge Management**: Expiration checks, renewal reminders, permission updates
2. **Course Management**: Retake deadlines, completion certificates, recommendation updates
3. **Governance**: Vote deadline enforcement, proposal execution, notification distribution
4. **Content Sync**: Cross-territory course replication, translation updates, IPFS pinning
5. **Analytics**: Daily/weekly reports, engagement metrics, learning statistics
6. **Maintenance**: Database cleanup, log rotation, cache invalidation

---

### 10. **Cross-Platform Client Applications**

#### **Current: Web Frontend**

**Technology Stack:**
- **React 19**: Component-based UI with modern hooks, concurrent rendering
- **Vite**: Lightning-fast dev server, optimized production builds
- **TailwindCSS 4.1.16**: Utility-first styling, responsive design system
- **ShadCN 3.5.0**: Accessible component library with theming support
- **TanStack Router 1.134.10**: Type-safe client-side routing with data loading
- **TypeScript**: Full type safety across codebase

**Key Frontend Features:**

```typescript
// Badge-aware UI components
function CourseCatalog() {
  const { userBadges } = useUserBadges();
  const { courses } = useCourses();
  
  // Filter courses by badge requirements
  const availableCourses = courses.filter(course =>
    course.prerequisiteBadges.every(badge => userBadges.includes(badge))
  );
  
  const lockedCourses = courses.filter(course =>
    !course.prerequisiteBadges.every(badge => userBadges.includes(badge))
  );
  
  return (
    <div>
      <CourseList courses={availableCourses} locked={false} />
      <CourseList courses={lockedCourses} locked={true} showRequirements />
    </div>
  );
}

// Territory switcher for multi-territory managers
function TerritorySwitcher() {
  const { managedTerritories } = useTerritoryManager();
  const [currentTerritory, setCurrentTerritory] = useCurrentTerritory();
  
  return (
    <Select value={currentTerritory} onChange={setCurrentTerritory}>
      {managedTerritories.map(territory => (
        <Option key={territory.id} value={territory.id}>
          {territory.name} ({territory.userCount} users)
        </Option>
      ))}
    </Select>
  );
}

// Code of Conduct expiration banner
function CodeOfConductBanner() {
  const { codeOfConductExpires } = useUserBadges();
  const daysUntilExpiry = differenceInDays(codeOfConductExpires, new Date());
  
  if (daysUntilExpiry > 30) return null;
  
  return (
    <Banner variant="warning">
      Your Code of Conduct certification expires in {daysUntilExpiry} days.
      <Link to="/courses/code-of-conduct">Renew now</Link>
    </Banner>
  );
}
```

**Progressive Web App (PWA):**
- Offline-first architecture with service workers
- Local caching of courses for offline learning
- Background sync for course progress
- Push notifications for badge expirations, votes, warnings

#### **Future: Tauri Desktop & Mobile**

**Cross-Platform Native Applications:**
- **Platforms**: Windows, macOS, Linux (desktop), iOS, Android (mobile)
- **Benefits**:
  - Smaller bundle size vs Electron (~600KB vs ~150MB)
  - Better performance (native WebView, not Chromium)
  - Native OS integration (file system, notifications, system tray)
  - Offline-first with local database (SQLite)

**Offline Learning Capabilities:**
```rust
// Tauri backend - download courses for offline access
#[tauri::command]
async fn download_course_for_offline(course_id: String) -> Result<(), String> {
    // Download course materials from IPFS
    let course = api_client.get_course(&course_id).await?;
    
    // Download and cache all files
    for file in &course.files {
        let content = ipfs_client.get(&file.ipfs_cid).await?;
        local_storage.store(&file.ipfs_cid, content)?;
    }
    
    // Store course metadata in local SQLite
    local_db.insert_course(course)?;
    
    Ok(())
}

// Sync progress when back online
#[tauri::command]
async fn sync_offline_progress() -> Result<(), String> {
    let offline_progress = local_db.get_unsynced_progress()?;
    
    for progress in offline_progress {
        api_client.update_course_progress(progress).await?;
        local_db.mark_synced(progress.id)?;
    }
    
    Ok(())
}
```

---

### 11. **Future: Holochain Integration**

#### **Full Decentralization Roadmap**

**Phase 1: Hybrid Mode (Years 1-2)**
- Maintain current Rust microservices backend
- Add Holochain nodes for specific use cases:
  - Personal learning portfolios (user-owned data)
  - Badge/credential verification (distributed trust)
  - Community governance voting (transparent, tamper-proof)

**Phase 2: Gradual Migration (Years 2-4)**
- Migrate frontend API calls to Holochain DNA functions
- User authentication transitions to Decentralized Identifiers (DIDs)
- Course materials stored on IPFS, referenced in Holochain
- Territory autonomy increases as local Holochain conductors deployed

**Phase 3: Full Decentralization (Year 4+)**
- All application logic in Holochain DNA modules
- No centralized servers required (optional caching/gateway nodes)
- Users run their own Holochain conductor (desktop/mobile app)
- Complete data sovereignty and censorship resistance

#### **Holochain DNA Modules**

The platform will leverage existing **holochain-open-dev** community modules to accelerate development and ensure interoperability:

**1. Profiles Module ([holochain-open-dev/profiles](https://github.com/holochain-open-dev/profiles)):**
- User profile management (nickname, avatar, custom fields)
- Decentralized identity and profile discovery
- Integrates with our UserProfile system for migration
- Already designed in Section 6 (User Profile System)

**2. Peer Status Module ([holochain-open-dev/peer-status](https://github.com/holochain-open-dev/peer-status)):**
- Agent online/offline/busy status tracking
- Real-time presence indicators
- Integration with P2P messaging (Section 6b)
- Supports peer discovery for direct connections

**3. Notifications Module ([holochain-open-dev/notifications](https://github.com/holochain-open-dev/notifications)):**
- External notification management (email, SMS, WhatsApp)
- Integrates with our NotificationSettings (Section 6)
- Supports notification preferences and quiet hours
- Handles both in-app and external notifications

**4. File Storage Module ([holochain-open-dev/file-storage](https://github.com/holochain-open-dev/file-storage)):**
- Store and retrieve files in Holochain DHT
- Complements IPFS for small files and user-generated content
- Integration path: IPFS for large course materials, Holochain for user files
- Automatic deduplication and content addressing

**Migration Strategy for Community Modules:**

```rust
// Phase 1: Hybrid integration
// Current centralized system bridges to Holochain modules

// Profile sync: PostgreSQL → holochain-open-dev/profiles
async fn sync_profile_to_holochain(user_id: Uuid) -> Result<()> {
    let profile = get_user_profile(user_id).await?;
    let holochain_agent = get_user_holochain_agent(user_id).await?;
    
    holochain_agent.call_zome(
        "profiles",
        "create_profile",
        ProfileInput {
            nickname: profile.display_name,
            avatar: profile.avatar_url,
            fields: hashmap! {
                "bio" => profile.bio,
                "location" => profile.location,
                "social_links" => serde_json::to_string(&profile.social_links)?,
                "language_prefs" => serde_json::to_string(&profile.language_preferences)?,
            },
        },
    ).await?;
    
    Ok(())
}

// Peer status integration: WebRTC + holochain-open-dev/peer-status
async fn update_peer_status(status: PeerStatus) -> Result<()> {
    // Update centralized database
    db.user_status.update(user_id, status).await?;
    
    // Sync to Holochain for P2P discovery
    holochain_agent.call_zome(
        "peer_status",
        "set_status",
        status,
    ).await?;
    
    Ok(())
}

// Notification routing: Central → holochain-open-dev/notifications
async fn send_notification(notification: Notification) -> Result<()> {
    match notification.delivery_method {
        DeliveryMethod::InApp => {
            // Use centralized system
            send_in_app_notification(notification).await?;
        },
        DeliveryMethod::External { channel } => {
            // Route through Holochain notifications module
            holochain_agent.call_zome(
                "notifications",
                "send_notification",
                NotificationInput {
                    recipient: notification.user_id,
                    channel,  // Email, SMS, WhatsApp
                    message: notification.content,
                },
            ).await?;
        },
    }
    Ok(())
}

// File storage routing: IPFS vs Holochain
async fn store_file(file: UploadedFile) -> Result<String> {
    // Large files (>1MB): IPFS
    if file.size > 1_000_000 {
        let ipfs_cid = ipfs_client.add_file(&file.data).await?;
        return Ok(format!("ipfs://{}", ipfs_cid));
    }
    
    // Small files (<1MB): Holochain file-storage
    let file_hash = holochain_agent.call_zome(
        "file_storage",
        "create_file",
        FileInput {
            name: file.name,
            content: file.data,
            mime_type: file.mime_type,
        },
    ).await?;
    
    Ok(format!("holochain://{}", file_hash))
}
```

**Custom Platform DNA Modules:**

```rust
// Example: Badge verification DNA
#[hdk_extern]
pub fn issue_badge(badge_award: BadgeAward) -> ExternResult<ActionHash> {
    // Verify issuer has authority
    verify_issuer_authority(&badge_award.issuer, &badge_award.badge_type)?;
    
    // Create badge entry (cryptographically signed)
    let badge_hash = create_entry(EntryTypes::Badge(Badge {
        recipient: badge_award.recipient,
        badge_type: badge_award.badge_type,
        issued_by: badge_award.issuer,
        issued_at: sys_time()?,
        expires_at: badge_award.expires_at,
        evidence: badge_award.evidence,  // Link to course completion
    }))?;
    
    // Link to recipient's badge collection
    create_link(
        badge_award.recipient.clone(),
        badge_hash.clone(),
        LinkTypes::RecipientToBadge,
        (),
    )?;
    
    Ok(badge_hash)
}

#[hdk_extern]
pub fn verify_badge(query: BadgeQuery) -> ExternResult<Vec<Badge>> {
    // Get all badges for user
    let links = get_links(query.user_agent_pub_key, LinkTypes::RecipientToBadge, None)?;
    
    let mut badges = Vec::new();
    for link in links {
        let badge: Badge = get(link.target, GetOptions::default())?
            .ok_or(wasm_error!("Badge not found"))?
            .entry()
            .to_app_option()?
            .ok_or(wasm_error!("Invalid badge entry"))?;
        
        // Check if badge is still valid (not expired)
        if badge.expires_at.is_none() || badge.expires_at.unwrap() > sys_time()? {
            badges.push(badge);
        }
    }
    
    Ok(badges)
}
```

#### **Holochain Benefits for This Platform**

1. **User Data Sovereignty**: Each user's learning portfolio stored in their own source chain
2. **Censorship Resistance**: No single authority can delete content or revoke access
3. **Cryptographic Verification**: All badges, credentials, and achievements cryptographically provable
4. **Privacy-Preserving**: Users control who sees their data (selective disclosure)
5. **Territory Autonomy**: Each territory can run independent DNA while maintaining interoperability
6. **Democratic Governance**: Voting and decision-making built into DNA logic, transparent and tamper-proof
7. **Offline-First**: Holochain works offline by default, syncs when connectivity restored
8. **No Servers Required**: Eliminates hosting costs and single points of failure

#### **Migration Path from Current Architecture**

**Database → Holochain:**
```
PostgreSQL Schemas → Holochain DNA Modules
  territory_{id} → Territory DNA instance
  global → Shared coordination DNA
  user data → Personal source chains
```

**Authentication:**
```
OIDC + JWT → Decentralized Identifiers (DIDs)
  Centralized IdP → Self-sovereign identity
  Password-based → Cryptographic key pairs
```

**File Storage:**
```
IPFS (current) → IPFS (unchanged)
  Already decentralized, seamless integration
  Holochain stores IPFS CIDs, not actual files
```

**Event Bus:**
```
NATS (current) → Holochain Signals
  Centralized pub/sub → P2P gossiping
  Server coordination → Agent-to-agent messaging
```

This architecture provides a clear **evolution path** from centralized Rust microservices to fully decentralized Holochain, without requiring a complete rewrite upfront. The badge-based permission system, immutable audit trails, and democratic governance are already designed to align with Holochain's agent-centric model.

---

### 12. **Planned Platform Extensions**

The platform is designed with a modular architecture allowing separate extensions to be developed and integrated independently:

#### **Core Extensions (Separate Modules)**

**1. Learning Management System (LMS)** - ✅ Currently Designed
- 40+ content types (video, interactive, VR/AR, code exercises, etc.)
- Course/section/content hierarchy
- Certification and badge awards
- Progress tracking and gamification
- Offline-first learning mode

**2. Communication & Forums** - ✅ Currently Designed
- Matrix-based federated forums
- 27 topic collaboration tool types
- Cross-territory federation
- Democratic moderation system
- Category-based discovery

**3. Marketplace & Resource Exchange** - 🔮 Future Extension
- Barter and trade services within communities
- Physical/digital resource sharing
- Time banking system
- Skills exchange marketplace
- Community currency integration
- Reputation-based trust system
- Escrow and dispute resolution
- Integration with local economies

**4. Community Resource Management** - 🔮 Future Extension
- Physical space booking (meeting rooms, maker spaces, equipment)
- Tool library management
- Vehicle/equipment sharing
- Community garden plot allocation
- Inventory tracking for shared resources

**5. Healthcare & Wellness** - 🔮 Future Extension
- Peer support groups
- Mental health resources
- Traditional knowledge sharing
- Holistic wellness programs
- Health data sovereignty

**6. Cultural Preservation** - 🔮 Future Extension
- Language learning and preservation
- Traditional arts and crafts documentation
- Oral history archival
- Cultural ceremony coordination
- Elder knowledge repository

Each extension follows the platform's core principles:
- Badge-based access control
- Democratic governance (100% unanimous voting)
- Data sovereignty at territory level
- Transparency (no deletion, only hiding)
- Cross-territory federation where appropriate
- Inverted pyramid authority model

---

## 🎯 Implementation Summary

This platform combines:
- **Badge-driven permissions** for granular, learner-centric access control
- **Category-based discovery** with visual roadmaps showing unlock paths for locked content
- **Cross-territory federation** where users interact via local Matrix servers while seeing global content
- **Democratic governance** requiring unanimous votes for shared resources
- **Territory autonomy** with ability to customize or reject global content (forums remain visible when hidden)
- **Federated architecture** enabling cross-territory collaboration while maintaining data sovereignty
- **Transparency & accountability** through immutable audit trails and no content deletion
- **27 topic collaboration tools** organized in 6 categories:
  - Governance & Decision-Making (6): Discussion, Voting, Proposals/RFC, Elections, Polls, Consensus Building
  - Learning & Collaboration (7): Course Discussion, Q&A, Peer Review, Study Groups, Mentorship, Wiki, Showcase
  - Community Management (3): Events/Calendar, Announcements, Resource Sharing
  - Collaboration & Productivity (5): Whiteboard, Brainstorming, Video Conference, Project Management, Hiring
  - Platform Operations (4): Development, Bug Reports, Accessibility Feedback, Translation
  - Engagement & Gamification (2): Emergency Alerts, Challenges/Competitions
- **Comprehensive user profiles** with privacy controls:
  - 7 privacy presets from Private to Public with custom field-level visibility
  - Social familiarity groups (Family, Close Friends, Friends, Acquaintances, etc.)
  - Multi-language support with proficiency tracking
  - Unlimited social links with 30+ platform integrations
  - Customizable news feed with category/course/forum/user following
- **P2P encrypted messaging** (2-10 users):
  - Signal Protocol encryption with forward secrecy
  - WebRTC data channels for peer-to-peer delivery
  - STUN/TURN infrastructure for NAT traversal
  - Fallback to Matrix for reliability
- **Daily project updates** with structured sharing and engagement features
- **Progressive LMS with 40+ content types**:
  - Text & Documents (3), Media (6), Interactive (9), Assessments (5)
  - Collaboration (7), Real-Time (4), Reference (3), VR/AR (3), Standards (4)
  - Offline-first learning, gamification, accessibility features
- **User sovereignty** as the highest priority in the inverted pyramid model
- **Intuitive progression** with greyed-out silhouettes showing what's possible to unlock
- **Holochain migration path** leveraging community modules:
  - holochain-open-dev/profiles for decentralized identity
  - holochain-open-dev/peer-status for presence tracking
  - holochain-open-dev/notifications for external notifications
  - holochain-open-dev/file-storage for small user files
- **Future-proof decentralization** with clear migration path to full Holochain deployment

The result is a **scalable, privacy-focused, globally collaborative learning and communication platform** that respects local autonomy while fostering global connection and maintaining user data sovereignty at every territory.