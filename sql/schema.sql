CREATE TABLE devices (
	token bytea UNIQUE,
	name text,
	username text,
	PRIMARY KEY (username, name)
);

CREATE TABLE points (
	id bigserial PRIMARY KEY,
	owner text,
	coordinates geometry(Point,4326),
	elevation float,
	time timestamptz,
	device text,
	FOREIGN KEY (owner, device) REFERENCES devices(username, name)
);

create index locations_point_idx ON points using GIST(coordinates);