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
	time timestamptz NOT NULL,
	device text,
	FOREIGN KEY (owner, device) REFERENCES devices(username, name)
);

create index locations_point_idx ON points using GIST(coordinates);

CREATE TABLE tracks (
	name text,
	owner text,
	device text,
	min_date timestamptz NOT NULL,
	max_date timestamptz NOT NULL,
	FOREIGN KEY (owner, device) REFERENCES devices(username, name),
	PRIMARY KEY (owner, name)
);

-- Example:
--
--INSERT INTO devices VALUES(decode('6375367961685468', 'hex'), 'dev', 'joz');
--
-- You are now able to authenticate using 'Authorization: Token Y3U2eWFoVGgK'
-- header.

CREATE TABLE trackpermissions (
	owner text,
	name text,
	viewer text,
	FOREIGN KEY (owner, device) REFERENCES devices(username, name),
	UNIQUE (ownerm device, viewer)
);