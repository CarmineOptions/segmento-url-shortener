CREATE OR REPLACE FUNCTION base62_encode(num BIGINT)
RETURNS TEXT
LANGUAGE plpgsql
IMMUTABLE
STRICT
AS $$
DECLARE
  alphabet TEXT := '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
  base INT := 62;
  result TEXT := '';
  n BIGINT := num;
BEGIN
  IF n = 0 THEN
    RETURN '0';
  END IF;

  WHILE n > 0 LOOP
    result := substr(alphabet, (n % base)::INT + 1, 1) || result;
    n := n / base;
  END LOOP;

  RETURN result;
END;
$$;

CREATE TABLE links (
  id BIGSERIAL PRIMARY KEY,
  short_code TEXT GENERATED ALWAYS AS (base62_encode(id)) STORED NOT NULL,
  target_url TEXT NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Avoid one letter short codes
ALTER SEQUENCE links_id_seq RESTART WITH 3844;
