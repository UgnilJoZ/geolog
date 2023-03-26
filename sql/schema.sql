CREATE TABLE points (
	id BIGSERIAL PRIMARY KEY,
	owner text,
	coordinates geometry(Point,4326),
	elevation float,
	time timestamptz,
	device text
);

create index locations_point_idx ON points using GIST(coordinates);