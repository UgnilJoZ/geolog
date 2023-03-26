# API documentation
## Point
A point is a struct uniquely identified by a randomly chosen Uint64 ID.

It has an UTC timestamp, a WGS84 lon-lat position and a device reference.
Optional additional fields are elevation (above the WGS84 reference ellipsoid)
and a GNSS precision float in meters.

## Tracks
A track is a collection of points. It is defined by a timespan, a user and a
device reference. It may cache its shape.

## Permission model
Each track is owned by a user. Other users may be allowed to see it.

## Endpoints
### `POST /v1/points`
#### JSON request body
```json
[
    {
        coordinates: [0.0, 1.0],
        elevation: null,
        time: "1970-01-01 00:00",
        device: "mobile"
    }
]
```
#### Response body
```json
{
    id: 7005050,
}
```

### `GET /v1/points`
#### Arguments
fromdate, todate, device, minlat, maxlat, minlon, maxlon, limit
#### JSON response body
```json
[
    {
        id: 7005050,
        coordinates: [0.0, 1.0],
        elevation: null,
        time: "1970-01-01 00:00",
        device: "mobile"
    }
]
```