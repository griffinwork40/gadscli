-- Create a test SQLite database matching Google Ads Editor's schema
-- Used by integration tests in tests/integration/editor_tests.rs

-- AccountSetting
CREATE TABLE IF NOT EXISTS AccountSetting (
    name TEXT,
    currencyCode TEXT,
    timeZone TEXT,
    optimizationScore REAL
);
INSERT INTO AccountSetting (name, currencyCode, timeZone, optimizationScore)
VALUES ('Test Account', 'USD', 'America/New_York', 0.85);

-- Campaign
CREATE TABLE IF NOT EXISTS Campaign (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    name TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 2,
    campaignType INTEGER,
    budgetAmount INTEGER,
    biddingStrategyType INTEGER,
    startDate INTEGER,
    endDate INTEGER,
    state INTEGER NOT NULL DEFAULT 0,
    status_revert INTEGER,
    budgetAmount_revert INTEGER
);
INSERT INTO Campaign (remoteId, name, status, campaignType, budgetAmount, biddingStrategyType, state)
VALUES (100001, 'Search Campaign Alpha', 2, 0, 50000000, 0, 0);
INSERT INTO Campaign (remoteId, name, status, campaignType, budgetAmount, biddingStrategyType, state)
VALUES (100002, 'Display Campaign Beta', 3, 1, 25000000, 1, 0);
INSERT INTO Campaign (remoteId, name, status, campaignType, budgetAmount, state)
VALUES (NULL, 'New PMax Campaign', 2, 6, 75000000, 2);

-- AdGroup (parentId uses container encoding: tableType << 32 | localId, Campaign=2)
CREATE TABLE IF NOT EXISTS AdGroup (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    name TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 2,
    maxCpc INTEGER,
    state INTEGER NOT NULL DEFAULT 0,
    status_revert INTEGER
);
-- parentId for campaign localId=1: (2 << 32) | 1 = 8589934593
INSERT INTO AdGroup (remoteId, parentId, name, status, maxCpc, state)
VALUES (200001, 8589934593, 'Brand Keywords', 2, 1500000, 0);
INSERT INTO AdGroup (remoteId, parentId, name, status, maxCpc, state)
VALUES (200002, 8589934593, 'Generic Keywords', 2, 2000000, 0);
-- parentId for campaign localId=2: (2 << 32) | 2 = 8589934594
INSERT INTO AdGroup (remoteId, parentId, name, status, state)
VALUES (200003, 8589934594, 'Display Targeting', 3, 0);

-- Keyword (parentId uses container encoding: AdGroup=4)
CREATE TABLE IF NOT EXISTS Keyword (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    text TEXT NOT NULL,
    criterionType INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 0,
    maxCpc INTEGER,
    qualityScore INTEGER,
    state INTEGER NOT NULL DEFAULT 0,
    status_revert INTEGER
);
-- parentId for ad group localId=1: (4 << 32) | 1 = 17179869185
INSERT INTO Keyword (remoteId, parentId, text, criterionType, status, maxCpc, qualityScore, state)
VALUES (300001, 17179869185, 'buy shoes online', 0, 2, 1500000, 7, 0);
INSERT INTO Keyword (remoteId, parentId, text, criterionType, status, maxCpc, qualityScore, state)
VALUES (300002, 17179869185, 'best running shoes', 1, 2, 2000000, 8, 0);
-- parentId for ad group localId=2: (4 << 32) | 2 = 17179869186
INSERT INTO Keyword (remoteId, parentId, text, criterionType, status, maxCpc, state)
VALUES (300003, 17179869186, 'cheap sneakers', 2, 3, 1000000, 1);

-- ResponsiveSearchAd (parentId uses container encoding: AdGroup=4)
CREATE TABLE IF NOT EXISTS ResponsiveSearchAd (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 2,
    headline1 TEXT,
    headline2 TEXT,
    headline3 TEXT,
    headline4 TEXT,
    headline5 TEXT,
    headline6 TEXT,
    headline7 TEXT,
    headline8 TEXT,
    headline9 TEXT,
    headline10 TEXT,
    headline11 TEXT,
    headline12 TEXT,
    headline13 TEXT,
    headline14 TEXT,
    headline15 TEXT,
    description1 TEXT,
    description2 TEXT,
    description3 TEXT,
    description4 TEXT,
    path1 TEXT,
    path2 TEXT,
    finalUrls BLOB,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO ResponsiveSearchAd (remoteId, parentId, status, headline1, headline2, headline3, description1, description2, path1, path2, state)
VALUES (400001, 17179869185, 2, 'Buy Shoes Now', 'Free Shipping', 'Top Brands', 'Shop the best shoes online.', 'Free returns on all orders.', 'shoes', 'buy', 0);
INSERT INTO ResponsiveSearchAd (parentId, status, headline1, headline2, headline3, description1, state)
VALUES (17179869185, 2, 'New RSA Ad', 'Great Deals', 'Shop Today', 'Find amazing deals today.', 2);

-- Budget
CREATE TABLE IF NOT EXISTS Budget (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    name TEXT,
    budgetAmount INTEGER,
    status INTEGER NOT NULL DEFAULT 2,
    state INTEGER NOT NULL DEFAULT 0,
    budgetAmount_revert INTEGER
);
INSERT INTO Budget (remoteId, name, budgetAmount, status, state)
VALUES (500001, 'Daily Budget - Search', 50000000, 2, 0);
INSERT INTO Budget (remoteId, name, budgetAmount, status, state)
VALUES (500002, 'Daily Budget - Display', 25000000, 2, 0);

-- Label
CREATE TABLE IF NOT EXISTS Label (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    color TEXT,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO Label (remoteId, name, description, color, state)
VALUES (600001, 'High Priority', 'High priority campaigns', '#FF0000', 0);
INSERT INTO Label (remoteId, name, description, color, state)
VALUES (600002, 'Seasonal', 'Seasonal promotions', '#00FF00', 0);
INSERT INTO Label (name, description, state)
VALUES ('New Label', 'Just created', 2);

-- KeywordNegative
CREATE TABLE IF NOT EXISTS KeywordNegative (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    text TEXT NOT NULL,
    criterionType INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 2,
    state INTEGER NOT NULL DEFAULT 0
);
-- Negative keywords at campaign level: parentId for campaign localId=1: (2 << 32) | 1 = 8589934593
INSERT INTO KeywordNegative (remoteId, parentId, text, criterionType, status, state)
VALUES (700001, 8589934593, 'free shoes', 1, 2, 0);
INSERT INTO KeywordNegative (remoteId, parentId, text, criterionType, status, state)
VALUES (700002, 8589934593, 'used shoes', 2, 2, 0);

-- BiddingStrategy
CREATE TABLE IF NOT EXISTS BiddingStrategy (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    name TEXT NOT NULL,
    strategyType INTEGER,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO BiddingStrategy (remoteId, name, strategyType, state)
VALUES (800001, 'Target CPA Strategy', 2, 0);
INSERT INTO BiddingStrategy (remoteId, name, strategyType, state)
VALUES (800002, 'Maximize Clicks', 0, 0);

-- SitelinkV2
CREATE TABLE IF NOT EXISTS SitelinkV2 (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    linkText TEXT NOT NULL,
    finalUrls TEXT,
    description1 TEXT,
    description2 TEXT,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO SitelinkV2 (remoteId, parentId, linkText, finalUrls, description1, description2, state)
VALUES (900001, 8589934593, 'Shop Now', 'https://example.com/shop', 'Browse our collection', 'Free shipping available', 0);
INSERT INTO SitelinkV2 (remoteId, parentId, linkText, finalUrls, description1, state)
VALUES (900002, 8589934593, 'Contact Us', 'https://example.com/contact', 'Get in touch today', 0);

-- CalloutV2
CREATE TABLE IF NOT EXISTS CalloutV2 (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    text TEXT NOT NULL,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO CalloutV2 (remoteId, parentId, text, state)
VALUES (1000001, 8589934593, 'Free Shipping', 0);
INSERT INTO CalloutV2 (remoteId, parentId, text, state)
VALUES (1000002, 8589934593, '24/7 Support', 0);

-- StructuredSnippetV2
CREATE TABLE IF NOT EXISTS StructuredSnippetV2 (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    header TEXT NOT NULL,
    "values" TEXT,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO StructuredSnippetV2 (remoteId, parentId, header, "values", state)
VALUES (1100001, 8589934593, 'Brands', 'Nike, Adidas, Puma', 0);
INSERT INTO StructuredSnippetV2 (remoteId, parentId, header, "values", state)
VALUES (1100002, 8589934593, 'Types', 'Running, Walking, Hiking', 0);

-- GeoTarget
CREATE TABLE IF NOT EXISTS GeoTarget (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    locationId INTEGER,
    locationName TEXT,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO GeoTarget (remoteId, parentId, locationId, locationName, state)
VALUES (1200001, 8589934593, 1014221, 'New York, NY', 0);
INSERT INTO GeoTarget (remoteId, parentId, locationId, locationName, state)
VALUES (1200002, 8589934593, 1014895, 'Los Angeles, CA', 0);

-- Audience
CREATE TABLE IF NOT EXISTS Audience (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    audienceId INTEGER,
    audienceName TEXT,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO Audience (remoteId, parentId, audienceId, audienceName, state)
VALUES (1300001, 8589934593, 80432, 'In-Market: Shoes', 0);
INSERT INTO Audience (remoteId, parentId, audienceId, audienceName, state)
VALUES (1300002, 8589934593, 80433, 'In-Market: Athletic Wear', 0);

-- Placement
CREATE TABLE IF NOT EXISTS Placement (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    url TEXT NOT NULL,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO Placement (remoteId, parentId, url, state)
VALUES (1400001, 8589934594, 'shoes.example.com', 0);
INSERT INTO Placement (remoteId, parentId, url, state)
VALUES (1400002, 8589934594, 'sports.example.com', 0);

-- SearchTerm
CREATE TABLE IF NOT EXISTS SearchTerm (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    parentId INTEGER NOT NULL,
    searchTerm TEXT NOT NULL,
    keywordText TEXT
);
INSERT INTO SearchTerm (parentId, searchTerm, keywordText)
VALUES (17179869185, 'buy running shoes online', 'buy shoes online');
INSERT INTO SearchTerm (parentId, searchTerm, keywordText)
VALUES (17179869185, 'best shoes for running', 'best running shoes');

-- NegativeKeywordList
CREATE TABLE IF NOT EXISTS NegativeKeywordList (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    name TEXT NOT NULL,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO NegativeKeywordList (remoteId, name, state)
VALUES (1500001, 'Brand Exclusions', 0);
INSERT INTO NegativeKeywordList (remoteId, name, state)
VALUES (1500002, 'Competitor Terms', 0);

-- AssetGroup (parentId links to campaign)
CREATE TABLE IF NOT EXISTS AssetGroup (
    localId INTEGER PRIMARY KEY AUTOINCREMENT,
    remoteId INTEGER,
    parentId INTEGER NOT NULL,
    name TEXT NOT NULL,
    state INTEGER NOT NULL DEFAULT 0
);
INSERT INTO AssetGroup (remoteId, parentId, name, state)
VALUES (1600001, 8589934593, 'Main Asset Group', 0);
INSERT INTO AssetGroup (remoteId, parentId, name, state)
VALUES (1600002, 8589934593, 'Secondary Assets', 0);
