-- Move Title to Subtitle if Series exists
UPDATE books SET subtitle = title WHERE series IS NOT NULL AND series != '';

-- Move Series to Title if Series exists
UPDATE books SET title = series WHERE series IS NOT NULL AND series != '';

-- Drop the column (SQLite supports DROP COLUMN since 3.35, assuming available)
ALTER TABLE books DROP COLUMN series;
