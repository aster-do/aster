# Aster Dashboard

Send aggregated billables to the cost and earning frontend dashboards.

## API routes

- **GET** `/billables`: Get aggregated billables
  - Query parameters:
    - `operator` _(required)_: can be `min`, `max`, `mean`, `count` or `sum`
    - `frequency`: can be `hourly`, `dayly` or undefined. Undefined means getting only one aggreated billable.
    - `start`: the start date of the window using iso format.
    - `end`: the end date of the window using iso format.
    - `name`: the billables name.
