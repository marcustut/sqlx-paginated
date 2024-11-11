pub static COLUMN_PROTECTION_BLOCKED_POSTGRES: [&str; 13] = [
    // System schemas and tables
    "pg_",
    "information_schema.",
    // System columns
    "oid",
    "tableoid",
    "xmin",
    "xmax",
    "cmin",
    "cmax",
    "ctid",
    // Other sensitive prefixes
    "pg_catalog",
    "pg_toast",
    "pg_temp",
    "pg_internal",
];
