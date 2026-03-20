# Google Ads Editor Automation Reference

## Overview

Google Ads Editor is Google's free desktop application for managing Google Ads campaigns
offline. It communicates with Google's servers using the **Tangle API** -- an internal
Google protocol (`ads.api.tangle.*`) that is distinct from the public Google Ads API.
Because Editor authenticates via standard OAuth2 with the `https://www.googleapis.com/auth/adwords`
scope, it does **not** require a developer token. This makes it the only free path to
programmatically manage Google Ads without applying for (and waiting months for) API access.

This document covers everything discovered by reverse-engineering the Google Ads Editor
v14.12 binary, its SQLite databases, and its command-line interface. The goal is to enable
automation tooling that drives Editor headlessly from scripts or from Claude Code.

### Why This Matters

- **No developer token required** -- Editor uses first-party OAuth2, same as the web UI.
- **Full read/write access** -- every entity the web UI can touch, Editor can touch.
- **Offline staging** -- changes are staged locally in SQLite, validated, then posted.
- **Headless modes exist** -- the binary supports CSV import, download, post, and export
  without the GUI.

---

## Binary Details

### Location

```
/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor
```

### File Type

```
Mach-O universal binary with 2 architectures: [x86_64] [arm64]
```

### Build Info

| Field | Value |
|-------|-------|
| Bundle ID | `com.google.googleadseditorS` |
| Version | 14.12 (CFBundleShortVersionString: 14.12) |
| Build | 14.12.0 |
| Min macOS | 12.0 |
| Framework | Qt 6 (QtCore, QtGui, QtWidgets, QtSql, QtNetwork, QtWebEngineCore, etc.) |
| Binary size | ~175 MB universal |
| Source branch | `ads_editor_14_12_release_branch` |
| Source root | `/tmpfs/src/piper/branches/ads_editor_14_12_release_branch/googleclient/ads/adwords/editor/` |

### Key Source Files (from embedded paths)

```
src/commandline.cc          -- CLI flag parsing and command-line mode dispatch
src/editorapplication.cc    -- Main application class, headless runners
src/databaseactions.cc      -- Database mutation logic
src/databasereader.cc       -- Database read logic
src/databasewriter.cc       -- Database write logic
src/databasemigrationwriter.cc -- Schema migration
src/databasehelpertable.cc  -- Helper table operations
src/xmlimporter.cc          -- XML import logic
src/scheduledjobmanager.cc  -- Scheduled job execution

generated/appmodeenum.cc    -- AppMode enum definition
generated/stateenumenum.cc  -- Entity state enum
generated/tableidenum.cc    -- Table ID enum
generated/exportformatenum.cc -- Export format enum
generated/importsourcemethodenum.cc -- Import source methods
generated/scheduledjobargenum.cc -- Scheduled job arguments
generated/scheduledjobfreqenum.cc -- Scheduled job frequencies
generated/scheduledjobtypeenum.cc -- Scheduled job types
generated/biddingstrategyenum.cc -- Bidding strategy types
```

### Supporting Binaries

| File | Purpose |
|------|---------|
| `crashpad_handler` | Crash reporting daemon |
| `libhunspell.dylib` | Spell checking for ad text |
| `libquazip1-qt6.1.4.dylib` | ZIP archive support (.aea/.aes files) |

---

## App Modes (Headless Operations)

The binary defines an `AppMode` enum with 18 known values. Several of these enable
headless (no-GUI) operation:

| Enum Value | Purpose | Headless? |
|------------|---------|-----------|
| `kAppModeDefault` | Normal GUI launch | No |
| `kAppModeCommandLineInit` | Initialize command-line parameters | Internal |
| `kAppModeCSVImport` | Headless CSV import | **Yes** |
| `kAppModeDownload` | Headless account download | **Yes** |
| `kAppModePost` | Headless post (upload) changes | **Yes** |
| `kAppModeXmlExport` | Headless XML export | **Yes** |
| `kAppModeXmlImport` | Headless XML import | **Yes** |
| `kAppModeHtmlExport` | Headless HTML export | **Yes** |
| `kAppModeValidate` | Headless validation | **Yes** |
| `kAppModeAcceptProposals` | Headless proposal settlement | **Yes** |
| `kAppModeInfo` | Print info and exit | **Yes** |
| `kAppModeUsage` | Print usage and exit | **Yes** |
| `kAppModeFormatLog` | Format a log file | **Yes** |
| `kAppModeTest` | Internal testing mode | Internal |
| `kAppModeMicroBench` | Micro benchmarking | Internal |
| `kAppModeMultiAccountBenchmark` | Multi-account benchmarking | Internal |
| `kAppModePerformanceBench` | Performance benchmarking | Internal |
| `kAppModeExitOk` | Clean exit (return 0) | Internal |
| `kAppModeExitError` | Error exit (return non-zero) | Internal |

### Key Functions

Each headless mode maps to a method on the `EditorApplication` class:

```
RunCommandLineCSVImport()   -- kAppModeCSVImport
RunCommandLineDownload()    -- kAppModeDownload
RunCommandLineUpload()      -- kAppModePost
RunCommandLineXmlExport()   -- kAppModeXmlExport
RunCommandLineXmlImport()   -- kAppModeXmlImport
RunCommandLineHtmlExport()  -- kAppModeHtmlExport
RunHeadlessFileImport()     -- Generic file import dispatcher
RunHeadlessJob()            -- Generic headless job runner
RunHeadlessSettlement()     -- kAppModeAcceptProposals
ValidateCommandLineAccount() -- Account validation
```

---

## CLI Flags

The following flags were discovered via `strings` analysis. The binary uses single-dash
flags (not GNU double-dash style) and parses them in `commandline.cc`.

### Mode Selection Flags

| Flag | Mode | Description |
|------|------|-------------|
| `importCSV` / `importcsv` | kAppModeCSVImport | Import changes from a CSV file |
| `importXML` / `importxml` | kAppModeXmlImport | Import from XML archive (.aea/.aes) |
| `exportXml` / `exportxml` | kAppModeXmlExport | Export account to XML |
| `exportXmlShare` / `exportxmlshare` | kAppModeXmlExport | Export as sharing XML (.aes) |
| `exportXmlUpgrade` / `exportxmlupgrade` | kAppModeXmlExport | Export as upgrade XML |
| `exportHTML` / `exporthtml` | kAppModeHtmlExport | Export account summary as HTML |
| `acceptProposals` / `acceptproposals` | kAppModeAcceptProposals | Accept pending proposals |
| `formatLogFile` / `formatlogfile` | kAppModeFormatLog | Reformat a log file |

### Parameter Flags

| Flag | Type | Description |
|------|------|-------------|
| `-importFile` | path | Path to the CSV or XML file to import |
| `-exportFile` | path | Path for the exported output file |
| `-customerId` | integer | Target customer/account ID |
| `-logFile` | path | Path for the log file |
| `-logFileFilter` | string | Filter criteria for log output |
| `-noics` / `-noIcs` | boolean | Disable ICS (Incremental Change Sync) mode |
| `-data` | path | Override the data directory |
| `-endpoint` | URL | Override the API endpoint |
| `-exportCustomRulesTranslations` | path | Export custom validation rule translations to CSV |

### Important Notes

- **Commandline download requires `-noics`**: The binary explicitly states:
  `"Commandline download is not possible in ics mode. Use the -noics argument."`
- **The `importFile` flag is required for CSV import**: The binary checks:
  `"Expected an importFile argument."`
- **Headless jobs fail in default mode**: `"Headless jobs aren't allowed in default app mode."`
  -- you must specify a mode flag.

---

## OAuth2 / Authentication

Editor authenticates via OAuth2 with PKCE (code challenge):

| Detail | Value |
|--------|-------|
| OAuth2 endpoint | `https://oauth2.googleapis.com` |
| Scope | `https://www.googleapis.com/auth/adwords` |
| Additional scopes | `https://www.googleapis.com/auth/drive.file`, `https://www.googleapis.com/auth/drive.readonly`, `email` |
| Redirect URI | `localhost:8887` (local HTTP server) |
| PKCE | S256 code challenge method |
| Auth flow | `client_id=%1&response_type=code&redirect_uri=%1&code_challenge_method=S256&code_challenge=%2` |
| Token exchange | `redirect_uri=%1&code_verifier=%2` |

The local OAuth2 server listens on port 8887 and displays a success page upon
completion. Tokens are cached locally (likely in the system keychain or data directory).

### Tangle API

Editor does **not** use the public Google Ads REST API (`googleads.googleapis.com`).
Instead, it uses Google's internal **Tangle API**:

```
ads.api.tangle.Action
ads.api.tangle.ActionResponse
ads.api.tangle.Command
ads.api.tangle.CommandResponse
ads.api.tangle.Get
ads.api.tangle.GetResponse
ads.api.tangle.Mutate
ads.api.tangle.FieldMutate
ads.api.tangle.Collection
ads.api.tangle.ColumnInfo
ads.api.tangle.Error
ads.api.tangle.ClientInfo
```

Tangle uses protobuf over HTTPS. The protos are embedded in the binary:
- `ads/api/tangle/adwords_api.proto`
- `ads/api/tangle/client_info.proto`
- `ads/api/tangle/column_info.proto`
- `ads/api/tangle/output_only.proto`
- `ads/api/tangle/query_header.proto`
- `ads/api/tangle/query_output.proto`
- `ads/api/tangle/errors/cm/policy_errors.proto`

---

## SQLite Databases

### Data Directory

```
~/Library/Application Support/Google/Google-AdWords-Editor/735/
```

The `735` is the schema version (found in MetaInfo: `schemaVersion = '735'`,
`tableRevisionOverride = '732257333'`).

### Database Files

| File | Purpose |
|------|---------|
| `ape.db` | Main database -- accounts list, scheduled jobs, saved searches, global settings |
| `ape_{CUSTOMER_ID}.db` | Per-account database -- all campaign data for one account |
| `assets.db` | Image and media asset storage |

### Main Database (`ape.db`)

**Tables**: `Account`, `DownloadSelectionSet`, `GlobalUndoInfo`, `SavedSearch`,
`ScheduledJob`, `StatsColumnSet`, `sqlite_sequence`

The MetaInfo table is created with:
```sql
CREATE TABLE MetaInfo (
    id integer default 0 primary key,
    schemaVersion text default '',
    tableRevisionOverride text default '',
    validationRulesetVersion text default '',
    errorInfoVersion text default '',
    accountLevelLastSync integer default 0
);
-- Initial data:
INSERT INTO MetaInfo (schemaVersion, tableRevisionOverride) VALUES('735', '732257333');
```

The ScheduledJob table stores automated download/post jobs:
```sql
CREATE TABLE ScheduledJob (
    localId integer default 0 primary key AUTOINCREMENT,
    remoteId integer default 0,
    parentId integer default 0,
    state integer default 0,
    preAdvisoryState integer,
    status integer default 0,
    status_revert integer,
    status_history integer,
    customerId integer default 0,
    nextTime integer default 0,
    lastTime integer default 0,
    frequency integer default 0,
    jobType integer default 0,
    args blob default X'00000000'
);
```

### Account Database (`ape_{CUSTOMER_ID}.db`)

For our account: `ape_4200317041.db`

**Complete table list** (163 tables):

#### Core Entity Tables
| Table | Description |
|-------|-------------|
| Campaign | Campaign definitions |
| AdGroup | Ad group definitions |
| Keyword | Positive keywords (biddable) |
| KeywordNegative | Negative keywords |
| ResponsiveSearchAd | RSA ads |
| ExpandedTextAd | Legacy ETAs |
| TextAd | Legacy text ads |
| Budget | Shared and campaign-level budgets |
| BiddingStrategy | Portfolio bidding strategies |
| Label | Labels for organization |
| LabelEdge | Label-to-entity associations |
| ConversionTracker | Conversion actions |
| AccountSetting | Account-level settings |

#### Ad Types
| Table | Description |
|-------|-------------|
| AppAd | App promotion ads |
| AppImageAd | App image ads |
| BumperVideoAd | 6-second bumper video ads |
| CallOnlyAd | Call-only ads |
| DiscoveryAd | Demand Gen single-image ads |
| DiscoveryCarouselAd | Demand Gen carousel ads |
| DiscoveryCarouselDynamicAd | Demand Gen dynamic carousel ads |
| DiscoveryVideoAd | Demand Gen video ads |
| DynamicSearchAd | Dynamic search ads |
| ExpandedDynamicSearchAd | Expanded DSAs |
| GmailMultiProductAd | Gmail ads |
| ImageAd | Image/display ads |
| InDisplayVideoAd | In-feed video ads |
| InStreamVideoAd | Skippable in-stream video ads |
| MastheadVideoAd | YouTube Masthead ads |
| MultiAssetResponsiveDisplayAd | Responsive display ads |
| NonSkippableVideoAd | Non-skippable video ads |
| ProductAd | Shopping product ads |
| ResponsiveDisplayAd | Responsive display ads |
| TemplateAd | Template-based ads |
| UniversalAppAd | App campaign ads |
| UniversalAppEngagementAd | App engagement ads |
| UniversalAppPreRegistrationAd | App pre-registration ads |
| VideoActionAd | Video action campaign ads |
| VideoAd | Generic video ads |
| VideoAudioAd | Audio ads |

#### Targeting
| Table | Description |
|-------|-------------|
| Age / AgeNegative | Age demographic targeting |
| Gender / GenderNegative | Gender targeting |
| Income / IncomeNegative | Income tier targeting |
| Parental / ParentalNegative | Parental status targeting |
| Audience / AudienceNegative | Audience segment targeting |
| CombinedAudience | Combined audience segments |
| CustomAffinity | Custom affinity audiences |
| GeoTarget / GeoTargetNegative | Geographic targeting |
| GeoLocation | Location targets |
| Placement / PlacementNegative | Managed placement targeting |
| Topic / TopicNegative | Topic targeting |
| DynamicTarget / DynamicTargetNegative | DSA page feed targeting |
| ProductPartition | Shopping product group partitions |
| UserList | Remarketing/customer match lists |
| MobileApp / MobileAppNegative | Mobile app targeting |
| MobileAppCategory / MobileAppCategoryNegative | App category targeting |
| IPExclusion | IP address exclusions |
| LocationGroup / LocationGroupEdge | Location group targeting |
| VideoLineup / VideoLineupNegative | Video lineup targeting |
| YouTubeChannel / YouTubeChannelNegative | Channel targeting |
| YouTubeVideo / YouTubeVideoNegative | Video targeting |
| SearchTheme | Performance Max search themes |
| HotelPartition | Hotel group partitions |
| HotelItinerary | Hotel itinerary targeting |
| Persona | Audience personas |

#### Extensions
| Table | Description |
|-------|-------------|
| SitelinkV2 / SitelinkV2Edge | Sitelink extensions |
| CallExtensionV2 / CallExtensionV2Edge | Call extensions |
| CalloutV2 / CalloutV2Edge | Callout extensions |
| StructuredSnippetV2 / StructuredSnippetV2Edge | Structured snippets |
| PriceExtensionV2 / PriceExtensionV2Edge | Price extensions |
| PromotionExtensionV2 / PromotionExtensionV2Edge | Promotion extensions |
| AppExtensionV2 / AppExtensionV2Edge | App extensions |
| LeadFormExtension / LeadFormExtensionEdge | Lead form extensions |
| ImageExtensionLink | Image extensions |
| HotelCalloutV2 / HotelCalloutV2Edge | Hotel callout extensions |

#### Other
| Table | Description |
|-------|-------------|
| NegativeKeywordList / NegativeKeywordListEdge | Shared negative keyword lists |
| NegativePlacementList / NegativePlacementListEdge | Shared negative placement lists |
| BrandList / BrandListEdge / NegativeBrandListEdge | Brand lists |
| AssetGroup | Performance Max asset groups |
| AssetFolder | Asset organization |
| AssetReport / FueAssetReport | Asset performance reports |
| AssetSet | Asset sets |
| Image / ImageBackRef | Image assets and references |
| TextAsset | Text assets |
| Video | Video assets |
| BusinessLogo / BusinessName | Business identity |
| CustomGoal | Custom conversion goals |
| UnifiedGoal | Unified conversion goals |
| LiftMeasurement | Brand lift experiments |
| PMaxExperiment | Performance Max experiments |
| ShoppingMerchant | Linked Merchant Center accounts |
| ActionRule / ActionTrigger | Automated rules |
| Error / ErrorInfo | Validation errors |
| MetaInfo | Schema version and sync metadata |
| DownloadCheckpoint | Download progress tracking |
| TableRevision | Per-table revision tracking |
| OverviewInfo | Account overview data |
| SearchTerm | Search term report data |
| ServerSideSetting | Server-pushed settings |
| UndoInfo | Undo stack |
| ValidationRule | Custom validation rules |
| WorkflowEvent | Workflow event log |
| Suggestion* (40+ tables) | Google's optimization recommendations |

---

## Key Table Schemas

### Entity State Model

Every entity table follows the same pattern with these standard columns:

| Column | Type | Description |
|--------|------|-------------|
| `localId` | INTEGER | Local auto-increment primary key |
| `remoteId` | INTEGER | Server-side entity ID (0 = not yet synced) |
| `parentId` | INTEGER | Parent entity's localId (campaign for ad group, ad group for keyword, etc.) |
| `state` | INTEGER | Entity state enum (see below) |
| `preAdvisoryState` | INTEGER | State before advisory changes |

**State Values** (from `StateEnum`):

| Value | Enum | Meaning |
|-------|------|---------|
| 0 | `kStateNormal` | Synced from server, no local changes |
| 1 | `kStateEdited` | Has local modifications (pending post) |
| 2 | `kStateNew` | Created locally, not yet posted |
| 3 | `kStateNonExistent` | Deleted or non-existent |

### Field Change Tracking

Editable fields use a triple-column pattern:

```
fieldName           -- current value
fieldName_revert    -- original server value (NULL if unchanged)
fieldName_history   -- previous value for undo
```

When `fieldName_revert` is NULL, the field has not been locally modified.
When it has a value, the entity has been edited and the `_revert` value is what
the server last reported.

### Campaign Table

Key columns (excluding the `_revert` / `_history` variants):

| Column | Type | Description |
|--------|------|-------------|
| `name` | TEXT | Campaign name |
| `status` | INTEGER | 0=Unknown, 2=Enabled, 3=Paused, 4=Removed |
| `campaignType` | INTEGER | Campaign type enum |
| `budgetId` | INTEGER | Remote budget ID |
| `sharedBudgetLocalId` | INTEGER | Local budget reference |
| `budgetAmount` | INTEGER | Budget in micros (divide by 1,000,000 for currency) |
| `budgetPeriod` | INTEGER | Budget period enum |
| `biddingStrategyType` | INTEGER | Bidding strategy type |
| `biddingStrategyId` | INTEGER | Portfolio bidding strategy reference |
| `enhancedCPCEnabled` | INTEGER | Boolean flag |
| `maxCpa` | INTEGER | Target CPA in micros |
| `targetRoas` | REAL | Target ROAS ratio |
| `startDate` | INTEGER | YYYYMMDD format |
| `endDate` | INTEGER | YYYYMMDD format (default: 20371230) |
| `includeGoogleSearch` | INTEGER | Boolean |
| `includeSearchPartners` | INTEGER | Boolean |
| `includeDisplayNetwork` | INTEGER | Boolean |
| `languages` | TEXT | Language targeting codes |
| `geoTargets` | TEXT | Geo-targeting specifications |
| `adSchedule` | BLOB | Serialized ad schedule |
| `domainName` | TEXT | DSA domain |
| `shoppingMerchantId` | INTEGER | Merchant Center ID |
| `shoppingCountry` | TEXT | Shopping country of sale |
| `lastSync` | INTEGER | Last sync timestamp |

### AdGroup Table

Key columns:

| Column | Type | Description |
|--------|------|-------------|
| `name` | TEXT | Ad group name |
| `status` | INTEGER | 0=Unknown, 2=Enabled, 3=Paused, 4=Removed |
| `maxCpc` | INTEGER | Default max CPC bid in micros |
| `maxCpm` | INTEGER | Max CPM bid in micros |
| `maxCpv` | INTEGER | Max CPV bid in micros |
| `maxCpa` | INTEGER | Target CPA in micros |
| `targetRoas` | REAL | Target ROAS |
| `contentBidDimension` | INTEGER | Display bid dimension type |
| `optimizedTargeting` | INTEGER | Boolean (default: 1) |
| `adRotation` | INTEGER | Ad rotation type |
| `languages` | TEXT | Language targeting overrides |

### Keyword Table

Key columns:

| Column | Type | Description |
|--------|------|-------------|
| `text` | TEXT | Keyword text |
| `criterionType` | INTEGER | Match type (broad, phrase, exact) |
| `status` | INTEGER | 0=Unknown, 2=Enabled, 3=Paused, 4=Removed |
| `maxCpc` | INTEGER | Keyword-level bid override in micros |
| `qualityScore` | INTEGER | Quality score (0-10) |
| `firstPageCpc` | INTEGER | First page CPC estimate in micros |
| `topOfPageCpc` | INTEGER | Top of page CPC estimate |
| `firstPositionCpc` | INTEGER | First position CPC estimate |
| `approvalStatus` | INTEGER | Policy approval status |
| `disapprovalReasons` | TEXT | Disapproval reason text |
| `finalUrls` | BLOB | Serialized final URL list |
| `finalMobileUrls` | BLOB | Serialized mobile final URLs |

### ResponsiveSearchAd Table

Key columns:

| Column | Type | Description |
|--------|------|-------------|
| `status` | INTEGER | Ad status |
| `headline1` through `headline15` | TEXT | Up to 15 headlines |
| `headline1Pos` through `headline15Pos` | INTEGER | Pin position (0=auto, 1/2/3=pinned) |
| `description1` through `description4` | TEXT | Up to 4 descriptions |
| `description1Pos` through `description4Pos` | INTEGER | Pin position |
| `path1` / `path2` | TEXT | Display URL paths |
| `finalUrls` | BLOB | Serialized final URLs |
| `finalMobileUrls` | BLOB | Serialized mobile URLs |
| `adStrengthServer` | INTEGER | Server-computed ad strength |
| `adStrengthLocal` | INTEGER | Locally-computed ad strength |
| `systemHeadlines` | BLOB | Auto-generated headlines |
| `systemDescriptions` | BLOB | Auto-generated descriptions |

### Budget Table

| Column | Type | Description |
|--------|------|-------------|
| `name` | TEXT | Shared budget name |
| `budgetAmount` | INTEGER | Amount in micros |
| `status` | INTEGER | Budget status |
| `alignedBiddingStrategyId` | INTEGER | Linked portfolio bidding strategy |

### BiddingStrategy Table

| Column | Type | Description |
|--------|------|-------------|
| `type` | INTEGER | Strategy type enum |
| `name` | TEXT | Portfolio bidding strategy name |

### Label Table

| Column | Type | Description |
|--------|------|-------------|
| `name` | TEXT | Label name |
| `description` | TEXT | Label description |
| `color` | TEXT | Hex color (default: `#000000`) |
| `ownerCustomerId` | INTEGER | Owner account ID |
| `ownerName` | TEXT | Owner account name |

### ConversionTracker Table

| Column | Type | Description |
|--------|------|-------------|
| `name` | TEXT | Conversion action name |
| `type` | INTEGER | Conversion type |
| `preferred` | INTEGER | Is primary/preferred |
| `packageName` | TEXT | App package name |
| `appStore` | INTEGER | App store enum |

### AccountSetting Table

| Column | Type | Description |
|--------|------|-------------|
| `name` | TEXT | Account name |
| `currencyCode` | TEXT | Account currency (e.g., "USD") |
| `timeZone` | TEXT | Account timezone |
| `campaignsNotDownloaded` | INTEGER | Number of campaigns not yet downloaded |
| `optimizationScore` | REAL | Account optimization score |
| `autoTagging` | INTEGER | Auto-tagging enabled |
| `supportsConversions` | INTEGER | Conversion tracking supported |
| `creationTime` | INTEGER | Account creation timestamp |

---

## Read Path: Querying the SQLite Database

The database can be read directly while Editor is not running (or while it is, using
WAL mode). This gives instant access to all downloaded account data.

### Example Queries

**List all campaigns with their budgets:**
```sql
SELECT
    c.localId,
    c.remoteId,
    c.name,
    CASE c.status
        WHEN 2 THEN 'Enabled'
        WHEN 3 THEN 'Paused'
        WHEN 4 THEN 'Removed'
        ELSE 'Unknown'
    END as status,
    CAST(c.budgetAmount AS REAL) / 1000000.0 as budget_dollars,
    c.biddingStrategyType,
    c.campaignType,
    c.startDate,
    c.endDate
FROM Campaign c
WHERE c.status IN (2, 3)
ORDER BY c.name;
```

**List all ad groups with their campaigns:**
```sql
SELECT
    ag.localId,
    ag.remoteId,
    c.name as campaign_name,
    ag.name as adgroup_name,
    CASE ag.status
        WHEN 2 THEN 'Enabled'
        WHEN 3 THEN 'Paused'
        ELSE 'Other'
    END as status,
    CAST(ag.maxCpc AS REAL) / 1000000.0 as max_cpc_dollars
FROM AdGroup ag
JOIN Campaign c ON ag.parentId = c.localId
WHERE ag.status IN (2, 3)
ORDER BY c.name, ag.name;
```

**List all keywords with match types:**
```sql
SELECT
    k.localId,
    k.remoteId,
    c.name as campaign_name,
    ag.name as adgroup_name,
    k.text as keyword,
    k.criterionType as match_type,
    CASE k.status
        WHEN 2 THEN 'Enabled'
        WHEN 3 THEN 'Paused'
        ELSE 'Other'
    END as status,
    CAST(k.maxCpc AS REAL) / 1000000.0 as bid_dollars,
    k.qualityScore
FROM Keyword k
JOIN AdGroup ag ON k.parentId = ag.localId
JOIN Campaign c ON ag.parentId = c.localId
WHERE k.status IN (2, 3)
ORDER BY c.name, ag.name, k.text;
```

**List RSA ads with headlines:**
```sql
SELECT
    rsa.localId,
    rsa.remoteId,
    c.name as campaign_name,
    ag.name as adgroup_name,
    rsa.headline1,
    rsa.headline2,
    rsa.headline3,
    rsa.description1,
    rsa.description2,
    rsa.path1,
    rsa.path2,
    rsa.adStrengthServer
FROM ResponsiveSearchAd rsa
JOIN AdGroup ag ON rsa.parentId = ag.localId
JOIN Campaign c ON ag.parentId = c.localId
WHERE rsa.status IN (2, 3);
```

**Find all entities with pending local changes:**
```sql
-- Edited campaigns
SELECT 'Campaign' as type, localId, remoteId, name, state
FROM Campaign WHERE state = 1;

-- New campaigns
SELECT 'Campaign' as type, localId, remoteId, name, state
FROM Campaign WHERE state = 2;

-- Repeat for AdGroup, Keyword, ResponsiveSearchAd, etc.
```

**List shared budgets and their campaigns:**
```sql
SELECT
    b.localId,
    b.remoteId,
    b.name as budget_name,
    CAST(b.budgetAmount AS REAL) / 1000000.0 as amount_dollars,
    GROUP_CONCAT(c.name, '; ') as campaigns
FROM Budget b
LEFT JOIN Campaign c ON c.sharedBudgetLocalId = b.localId
GROUP BY b.localId;
```

**List labels and their assignments:**
```sql
SELECT
    l.name as label_name,
    l.color,
    le.sinkTableId,
    le.sinkId
FROM Label l
JOIN LabelEdge le ON le.sourceId = l.localId
WHERE l.status != 4;
```

### Database Path Construction

```python
import os

DATA_DIR = os.path.expanduser(
    "~/Library/Application Support/Google/Google-AdWords-Editor/735"
)
MAIN_DB = os.path.join(DATA_DIR, "ape.db")
ACCOUNT_DB = lambda cid: os.path.join(DATA_DIR, f"ape_{cid}.db")

# For our account:
db_path = ACCOUNT_DB(4200317041)
```

---

## Write Path: Headless CSV Import

### CSV Import Format

Editor's CSV import uses a specific format. The key columns depend on the entity type.
The binary references `kImportSourceMethodCsvFile` for file-based imports.

**Supported import sources:**

| Enum | Method |
|------|--------|
| `kImportSourceMethodCsvFile` | CSV file on disk |
| `kImportSourceMethodCsvPaste` | Pasted CSV text |
| `kImportSourceMethodCsvManual` | Manual entry |
| `kImportSourceMethodCsvArchive` | CSV from archive |
| `kImportSourceMethodCsvMixedInteractive` | Mixed interactive |
| `kImportSourceMethodAeaFile` | Editor archive (.aea) |
| `kImportSourceMethodAesFile` | Editor sharing (.aes) |
| `kImportSourceMethodSheetsFile` | Google Sheets |

### CSV Import Command

Based on the discovered flags, the headless CSV import command is:

```bash
"/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor" \
    -importCSV \
    -importFile /path/to/changes.csv \
    -customerId 4200317041 \
    -logFile /path/to/import.log
```

### CSV Format

The CSV format matches Google Ads Editor's "Make Multiple Changes" / bulk import format.
Headers must match the column names used in Editor's UI. Example for keywords:

```csv
Campaign,Ad group,Keyword,Match type,Max CPC,Status
"My Campaign","My Ad Group","example keyword","Broad","1.50","Enabled"
```

Example for RSAs:

```csv
Campaign,Ad group,Headline 1,Headline 2,Headline 3,Description 1,Description 2,Final URL,Path 1,Path 2,Status
"My Campaign","My Ad Group","Great Products","Buy Now","Free Shipping","Best deals on widgets.","Shop today and save big!","https://example.com","products","deals","Enabled"
```

### Error Handling

The binary produces specific error messages during CSV import:

- `"CSV headless import: failed to parse "` -- CSV parsing error
- `"CSV headless import: invalid account."` -- Wrong or missing customerId
- `"CSV headless import: no changes to account "` -- CSV parsed but no actionable changes

---

## Download / Upload Workflow

### Step 1: Download Account Data

```bash
# Download fresh data from Google servers
"/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor" \
    -download \
    -customerId 4200317041 \
    -noics \
    -logFile /tmp/download.log
```

**Note:** The `-noics` flag is **required** for command-line downloads. Without it:
`"Commandline download is not possible in ics mode. Use the -noics argument."`

This populates/refreshes the `ape_4200317041.db` SQLite database.

### Step 2: Read Current State

After download, query the SQLite database directly to understand the current account
state (see Read Path section above).

### Step 3: Make Changes via CSV Import

Prepare a CSV file with changes and import it headlessly:

```bash
"/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor" \
    -importCSV \
    -importFile /tmp/changes.csv \
    -customerId 4200317041 \
    -logFile /tmp/import.log
```

### Step 4: Validate (Optional)

The `kAppModeValidate` mode can be used to check for errors before posting:

```bash
"/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor" \
    -validate \
    -customerId 4200317041 \
    -logFile /tmp/validate.log
```

### Step 5: Post Changes to Google

```bash
# Upload/post staged changes to Google's servers
"/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor" \
    -post \
    -customerId 4200317041 \
    -logFile /tmp/post.log
```

### Step 6: Export (Optional)

```bash
# Export as XML archive
"/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor" \
    -exportXml \
    -exportFile /tmp/account_backup.aea \
    -customerId 4200317041

# Export as HTML summary
"/Applications/Google Ads Editor.app/Contents/Versions/14.12.4.0/Google Ads Editor.app/Contents/MacOS/Google Ads Editor" \
    -exportHTML \
    -exportFile /tmp/account_summary.html \
    -customerId 4200317041
```

### Export Formats

| Enum | Extension | Description |
|------|-----------|-------------|
| `kExportFormatCSV` | .csv | CSV spreadsheet |
| `kExportFormatHTML` | .html | HTML summary report |
| `kExportFormatXmlArchive` | .aea | Full archive (for backup/restore) |
| `kExportFormatXmlSharing` | .aes | Sharing archive (for sending to others) |
| `kExportFormatXmlUpgrade` | .aea | Upgrade-compatible archive |
| `kExportFormatSheets` | -- | Google Sheets export |
| `kExportFormatZIP` | .zip | Zipped export |
| `kExportFormatXmlActionRules` | .xml | Automated rules export |
| `kExportFormatXmlValidationRuleSet` | .xml | Validation rules export |

---

## Account Info

| Field | Value |
|-------|-------|
| MCC (Manager) Account ID | 7943900856 |
| MCC Account Name | GRAIsol |
| Client Account ID | 4200317041 |
| Client Account Name | Gumball Wraps |
| Authenticated User | griffin@graisol.com |
| Database Path | `~/Library/Application Support/Google/Google-AdWords-Editor/735/ape_4200317041.db` |
| Schema Version | 735 |
| Table Revision Override | 732257333 |

---

## Limitations and Caveats

### Confirmed Working

- **Binary analysis**: App modes, CLI flags, and function names are definitively present
  in the binary.
- **SQLite read access**: Database schema and data are fully readable and well-structured.
- **OAuth2 flow**: Editor uses standard OAuth2 with PKCE on localhost:8887.
- **Tangle API**: Confirmed as the backend protocol (not the public Google Ads API).

### Needs Testing

- **Exact CLI flag syntax**: The flags were extracted via `strings` analysis. The exact
  invocation syntax (single dash vs. double dash, flag=value vs. flag value) needs live
  testing. The binary appears to use single-dash without equals signs.

- **Headless mode authentication**: Whether headless modes can reuse an existing auth
  session (from a prior GUI login) or require interactive OAuth2.

- **Download flag name**: The exact flag to trigger `kAppModeDownload` is unclear. It
  might be `-download`, or it might require a mode-specific sub-flag.

- **Post flag name**: Similarly, the exact flag for `kAppModePost` needs confirmation.
  Candidates: `-post`, `-upload`, or something else. The function is
  `RunCommandLineUpload()`.

- **CSV column names**: The exact column header names expected by Editor's CSV import
  need validation against Editor's actual import dialog. Editor supports "Make Multiple
  Changes" with specific column names that may differ from API field names.

- **Concurrent access**: Whether the SQLite databases can be read while Editor is running
  (likely yes via WAL mode, but needs confirmation).

- **macOS Gatekeeper**: Running the binary directly from the command line may trigger
  macOS security warnings or require `xattr -cr` to clear quarantine flags.

### Known Limitations

- **Single-user access**: Editor locks the data directory; only one instance can run
  at a time.

- **OAuth2 token refresh**: Headless modes may fail if tokens have expired and no
  interactive browser is available for re-authentication.

- **No streaming API**: Editor downloads all data in bulk, not incrementally. Large
  accounts take significant time.

- **Version coupling**: The database schema version (735) is tightly coupled to the
  Editor version (14.12). Database files are not portable between major versions.

- **Post-only changes**: You cannot bypass the local staging model. Changes must be
  imported (via CSV or direct DB manipulation), then posted as a separate step.

---

## Wrapper Tool Design

### Proposed Architecture

A CLI wrapper (`gads-editor` or integrated into `gadscli`) that bridges Claude Code
to Google Ads Editor:

```
Claude Code / User
       |
       v
  gads-editor (Rust CLI wrapper)
       |
       +-- Read Path: SQLite queries directly on ape_*.db
       |
       +-- Write Path:
       |     1. Generate CSV from structured input
       |     2. Invoke Editor binary with -importCSV
       |     3. Invoke Editor binary with -post (or prompt user)
       |
       +-- Sync Path:
             1. Invoke Editor binary with -download -noics
             2. Read refreshed SQLite data
```

### Module Structure

```
src/editor/
    mod.rs          -- Public API
    binary.rs       -- Editor binary invocation (download, import, post)
    database.rs     -- SQLite read queries
    csv_writer.rs   -- CSV generation for Editor import
    schema.rs       -- Database schema constants
    types.rs        -- Rust structs matching DB tables
```

### Key Functions

```rust
/// Read all campaigns from the Editor database
fn list_campaigns(customer_id: u64) -> Result<Vec<Campaign>>;

/// Read all keywords for a campaign
fn list_keywords(customer_id: u64, campaign_id: i64) -> Result<Vec<Keyword>>;

/// Generate a CSV file for Editor import
fn generate_csv(changes: &[Change]) -> Result<PathBuf>;

/// Invoke Editor binary to download account data
fn download_account(customer_id: u64) -> Result<()>;

/// Invoke Editor binary to import CSV changes
fn import_csv(customer_id: u64, csv_path: &Path) -> Result<()>;

/// Invoke Editor binary to post staged changes
fn post_changes(customer_id: u64) -> Result<()>;
```

### Workflow Example

```bash
# 1. Download fresh data
gadscli editor download --customer-id 4200317041

# 2. List campaigns (reads SQLite directly)
gadscli editor campaigns --customer-id 4200317041

# 3. List keywords
gadscli editor keywords --customer-id 4200317041 --campaign "My Campaign"

# 4. Add keywords via CSV import
gadscli editor add-keywords \
    --customer-id 4200317041 \
    --campaign "My Campaign" \
    --ad-group "My Ad Group" \
    --keywords "widget,gadget,tool" \
    --match-type broad \
    --bid 1.50

# 5. Post changes to Google
gadscli editor post --customer-id 4200317041
```

### Safety Considerations

1. **Always download before reading** to ensure fresh data.
2. **Log all operations** to enable audit trails.
3. **Validate before posting** to catch errors early.
4. **Backup the database** before making changes (copy the .db file).
5. **Never modify the SQLite database directly** for writes -- always use CSV import
   to let Editor handle validation and change tracking.

---

## Appendix A: All Account Database Tables

Complete list of 163 tables in `ape_4200317041.db`:

```
AccountSetting          ActionRule              ActionTrigger
AdGroup                 Age                     AgeNegative
AppAd                   AppExtensionV2          AppExtensionV2Edge
AppImageAd              AssetFolder             AssetGroup
AssetReport             AssetSet                Audience
AudienceNegative        BiddingStrategy         BrandList
BrandListEdge           Budget                  BumperVideoAd
BusinessLogo            BusinessName            CallExtensionV2
CallExtensionV2Edge     CallOnlyAd              CalloutV2
CalloutV2Edge           Campaign                CombinedAudience
ConversionTracker       CustomAffinity          CustomGoal
DiscoveryAd             DiscoveryCarouselAd     DiscoveryCarouselDynamicAd
DiscoveryVideoAd        DownloadCheckpoint      DynamicSearchAd
DynamicTarget           DynamicTargetNegative   Error
ErrorInfo               ExpandedDynamicSearchAd ExpandedTextAd
FueAssetReport          Gender                  GenderNegative
GeoLocation             GeoTarget               GeoTargetNegative
GmailMultiProductAd     HotelCalloutV2          HotelCalloutV2Edge
HotelItinerary          HotelPartition          IPExclusion
Image                   ImageAd                 ImageBackRef
ImageExtensionLink      InDisplayVideoAd        InStreamVideoAd
Income                  IncomeNegative          Keyword
KeywordNegative         Label                   LabelEdge
LeadFormExtension       LeadFormExtensionEdge   LiftMeasurement
LocationGroup           LocationGroupEdge       MastheadVideoAd
MetaInfo                MobileApp               MobileAppCategory
MobileAppCategoryNegative MobileAppNegative     MultiAssetResponsiveDisplayAd
NegativeBrandListEdge   NegativeKeywordList     NegativeKeywordListEdge
NegativePlacementList   NegativePlacementListEdge NonSkippableVideoAd
OverviewInfo            PMaxExperiment          Parental
ParentalNegative        Persona                 Placement
PlacementNegative       PriceExtensionV2        PriceExtensionV2Edge
ProductAd               ProductPartition        PromotionExtensionV2
PromotionExtensionV2Edge ResponsiveDisplayAd    ResponsiveSearchAd
SearchTerm              SearchTheme             ServerSideSetting
ShoppingMerchant        SitelinkV2              SitelinkV2Edge
StructuredSnippetV2     StructuredSnippetV2Edge SuggestionAccountCallouts
SuggestionAccountClickToCall SuggestionAccountSitelink SuggestionAccountStructuredSnippets
SuggestionAdGroupNoAds  SuggestionAdGroupNoKeywords SuggestionAddAppConversionGoal
SuggestionAddResponsiveDisplayAds SuggestionAddRsa SuggestionAdsMobileAppInstall
SuggestionAfsOptIn      SuggestionBudgetRaising SuggestionBudgetReallocation
SuggestionCallouts      SuggestionClickToCall   SuggestionDisplayTargetingExpansion
SuggestionDsaToPmaxMigration SuggestionEnhancedSitelink SuggestionFacbMatchType
SuggestionFixCreativeDestinationIssues SuggestionFixEditorialIssues
SuggestionFixMisconfiguredAppDeepLinks SuggestionForecastingBudgetRaising
SuggestionForecastingSetCpaTarget SuggestionForecastingSetRoasTarget
SuggestionForecastingSharedBudgetRaising SuggestionGdaToPmaxMigration
SuggestionImageExtension SuggestionImproveAppDeepLinkCoverage
SuggestionImproveRdaAdStrength SuggestionInAppConversionBidding
SuggestionKeyword       SuggestionKeywordDeduping SuggestionLeadForm
SuggestionMarginalRoiBudgetRaising SuggestionMarginalRoiSharedBudgetRaising
SuggestionMaximizeClicksOptIn SuggestionMaximizeConversionValueOptIn
SuggestionMaximizeConversionsOptIn SuggestionNoTrafficKeywordRemoving
SuggestionOptimizeAdRotation SuggestionOther SuggestionPerformanceMaxOptIn
SuggestionPriceExtension SuggestionRsaImproveAdStrength
SuggestionSearchPlusOptIn SuggestionSetCpaTarget SuggestionSetRoasTarget
SuggestionSharedBudgetRaising SuggestionSitelink SuggestionStructuredSnippets
SuggestionSubscription  SuggestionSummary       SuggestionTargetCpaOptIn
SuggestionTargetCpaRaising SuggestionTargetImpressionShareOptIn
SuggestionTargetRoasLowering SuggestionTargetRoasOptIn
SuggestionVideoPartnersOptIn TableRevision       TemplateAd
TextAd                  TextAsset               Topic
TopicNegative           UndoInfo                UnifiedGoal
UniversalAppAd          UniversalAppEngagementAd UniversalAppPreRegistrationAd
UserList                ValidationRule          Video
VideoActionAd           VideoAd                 VideoAudioAd
VideoLineup             VideoLineupNegative     WorkflowEvent
YouTubeChannel          YouTubeChannelNegative  YouTubeVideo
YouTubeVideoNegative    sqlite_sequence
```

## Appendix B: Statistics Tables

The account database also contains detailed performance statistics tables
(created dynamically, found via CREATE TABLE strings in the binary):

### SearchStat / DisplayStat

Contain per-entity performance metrics with columns including:
`Clicks`, `Cost`, `Impressions`, `Ctr`, `AverageCpc`, `AverageCpm`,
`ConversionsOpt`, `BiddableConversionRate`, `AllConversions`,
`SearchImpressionShare`, `AbsoluteTopImpressionPercentage`,
`VideoViews`, `VideoViewRate`, and 100+ more metrics.

Each row is keyed by `(intervalId, entityId, tableId)` -- linking the stat to
a specific entity in a specific table for a specific time period.

## Appendix C: Source Files in Binary

All embedded source paths found in the binary (Google's internal Piper monorepo):

```
googleclient/ads/adwords/editor/
    generated/
        appmodeenum.cc/.h
        bidchangestyleenum.cc
        biddingstrategyenum.cc/.h
        contentbiddimensionenum.cc
        errorhidereasonenum.cc/.h
        exportformatenum.cc
        importsourcemethodenum.cc
        knowntemplateidenum.cc
        mastheadvideolayoutenum.cc
        scheduledjobargenum.cc/.h
        scheduledjobcampaignsenum.cc
        scheduledjobfreqenum.cc/.h
        scheduledjobpoststatusenum.cc
        scheduledjobtypeenum.cc
        serversidesettingrefreshconditionenum.cc
        serversidesettingsourceenum.cc
        showstateenum.cc/.h
        specialbidmodifiersenum.cc
        specialvideolineupenum.cc/.h
        stateenumenum.cc/.h
        tableidenum.cc/.h
        tablerefreshreasonenum.cc
        validationresultenum.cc
        validationrulekindenum.cc
        validationruleseverityenum.cc
    src/
        adbuilderadswidget.cc
        adbuilderkeywordswidget.cc
        biddingstrategycache.cc
        commandline.cc
        containerid.h
        containeridex.h
        databaseactions.cc
        databasehelpertable.cc
        databasemigrationwriter.cc
        databasereader.cc
        databasewriter.cc
        displaylanguagewidget.cc
        editorapplication.cc
        entitymainwidget.cc
        entitytabledataprovider.cc
        scheduledjobmanager.cc
        xmlimporter.cc
```
