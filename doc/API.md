# API documentation
## Point
A point is a struct uniquely identified by a randomly chosen Uint64 ID.

It has an UTC timestamp, a WGS84 lon-lat position and a device reference.
Optional additional fields are elevation (above the WGS84 reference ellipsoid)
and a GNSS precision float in meters.

## Tracks
A track is a named collection of points. It is defined by a timespan, a user and a
device reference. It may cache its shape.

## Permission model
Each track is owned by a user. Other users may be allowed to see it.

## Authentication
The `Authorization` header follows the form `Token abccdeff`, where `abccdeff`
is the base64 representation of a record ID in the `devices` table.

## Endpoints
### `POST /v1/points`
#### JSON request body
```json
[
    {
        "coordinates": [0.0, 1.0],
        "elevation": 4.0,
        "time": "1970-01-01T00:00Z",
        "device": "mobile"
    }
]
```
#### Response body
```json
{
    id: 7005050,
}
```

#### Example
```sh
curl -v -H "Content-Type: application/json" -X POST 127.1:8080/points  -H 'Authorization: Token Y3U2eWFoVGgK' -d '[{"coordinates": [3.0, 4.0],"time": "2017-12-24T18:21Z","elevation": 4,"device": "dev"}]'
```

### `GET /v1/points`
#### Arguments
min\_date, max\_date, device, minlat, maxlat, minlon, maxlon, limit.

minlon,maxlon,minlat,maxlat can only be provided alltogether or be absent.

Same with min\_date and max\_date.

#### JSON response body
```json
[
    {
        "id": 7005050,
        "coordinates": [0.0, 1.0],
        "elevation": null,
        "time": "1970-01-01T00:00Z",
        "device": "mobile"
    }
]
```

#### Example
```sh
curl -v -H "Content-Type: application/json" -X GET '127.1:8080/points?device=dev&minlon=0.0&maxlon=8.0&minlat=0.0&maxlat=9.0'  -H 'Authorization: Token Y3U2eWFoVGgK' | jq
```

### `PUT /v1/tracks/{name}`
#### JSON request body
```json
{
    "device": "mobile",
    "min_date": "1970-01-01T00:00Z",
    "max_date": "1971-01-01T00:00Z"
}
```

#### Example
```sh
curl -v -H "Content-Type: application/json" -X PUT 127.1:8080/tracks/track1  -H 'Authorization: Token Y3U2eWFoVGgK' -d '{"device":"dev","min_date": "1970-01-01T00:00Z","max_date": "2010-01-01T00:00Z"}
```

### `GET /v1/tracks/{name}`
#### JSON response body
This will include a point list like returned by `GET /v1/points`.

```json
{
    "definition": {
        "device": "mobile",
        "from": "1970-01-01T00:00Z",
        "to": "1971-01-01 00:00"
    },
    "points": [
        {
            "id": 7005050,
            "coordinates": [0.0, 1.0],
            "elevation": null,
            "time": "1970-01-01T00:00Z",
            "device": "mobile"
        }
    ]
```

#### Example
```sh
curl -v -H "Content-Type: application/json" 127.1:8080/tracks/track1 -H 'Authorization: Token Y3U2eWFoVGgK' | jq
```
