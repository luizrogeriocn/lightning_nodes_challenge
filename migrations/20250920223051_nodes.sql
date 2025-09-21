CREATE TABLE IF NOT EXISTS nodes (
  public_key  TEXT PRIMARY KEY,
  alias       TEXT NOT NULL,
  channels    INTEGER NOT NULL,
  capacity    INTEGER NOT NULL,
  first_seen  INTEGER NOT NULL,
  updated_at  INTEGER NOT NULL,
  city        TEXT CHECK (city IS NULL OR json_valid(city)),
  country     TEXT CHECK (country IS NULL OR json_valid(country)),
  iso_code    TEXT CHECK (iso_code IS NULL OR length(iso_code) = 2),
  subdivision TEXT CHECK (subdivision IS NULL OR json_valid(subdivision))
);
