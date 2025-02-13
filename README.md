# House finder
Just my personal script to find myself a house to buy.

## How to use
1. Clone the repository
2. Build with `cargo build --release`
3. Run binary `./target/release/house_finder` with arguments.

For example:
```shell
./target/release/house_finder \
  --publishing-time-search-criteria=WITHIN_ONE_DAY \
  --price_max=1000000 \
  --cities=FI_UUSIMAA_ESPOO \
  --cities=FI_UUSIMAA_HELSINKI \
  --location-latitude=<LOCATION-LATITUDE> \
  --location-longitude=<LOCATION-LONGITUDE> \
  --open-route-service-token '<OPEN-ROUTE-SERVICE-AUTHORIZATION-TOKEN>' \
  --telegram-bot-token '<TELEGRAM-BOT-TOKEN>' \
  --telegram-user-id <TELEGRAM-USER-ID> \
  --cache-etuovi-html \
  --cache-elisa-fixed-broadband-products
```

## Arguments
- `--publishing-time-search-criteria` - (Optional) (Default: ANY_DAY) Search criteria for publishing time. One of: ANY_DAY, WITHIN_ONE_DAY, WITHIN_TWO_DAYS, WITHIN_SEVEN_DAYS or WITHIN_TWO_WEEKS
- `--price_max` - (Optional) Max price in euros.
- `--cities` - (Optional) Cities to search for. Allows multiple.
- `--location-latitude` - (Optional) (Requires: --location-longitude) Latitude to calculate distance against.
- `--location-longitude` - (Optional) (Requires: --location-latitude) Longitude to calculate distance against.
- `--open-route-service-token` - (Optional) [OpenRouteService](https://openrouteservice.org/) authorization token: https://openrouteservice.org/sign-up/
- `--telegram-bot-token` - (Optional) (Requires: --telegram-user-id) Telegram bot token from [BotFather](https://telegram.me/BotFather).
- `--telegram-user-id` - (Optional) (Requires: --telegram-bot-token) Your Telegram user ID.
- `--cache-elisa-fixed-broadband-products` - (Optional) If given stores all of Elisa fixedBroadbandProducts request data to cache directory in same directory as executable.
- `--cache-etuovi-announcements` - (Optional) If given stores all of Etuovi announcement search request data to cache directory in same directory as executable.
- `--cache-etuovi-html` - (Optional) If given stores all of Etuovi property page HTML to cache directory in same directory as executable.
- `--house-min-square-meters` - (Optional) If given, only get houses that are bigger than this. Only if house or total area are given. Compare with total area only if house area is not given.

## Info

### Cache
1. Writes cache to same directory executable is in.
   - If run with `cargo run` directory will be created in `./target/debug/cache` directory.
1. Will always cache following data:
  - [OpenRouteService](https://openrouteservice.org/) biking distance, because it includes coordinates and should not change.
  - Elisa address search results, because those should not change, because are just postal code and street address as Elisa's own identifier.
1. You can always manually remove cache directories.
1. With following arguments you can cache more data:
  - `--cache-elisa-fixed-broadband-products` If given stores all of Elisa fixedBroadbandProducts request data to cache directory in same directory as executable.
    - You should use this, because offered products should not change all the time.
  - `--cache-etuovi-announcements` - If given stores all of Etuovi announcement search request data to cache directory in same directory as executable.
    - This is usually only for development use, because it used and there is new results with this it will never get them for same search parameters.
  - `--cache-etuovi-html` - If given stores all of Etuovi property page HTML to cache directory in same directory as executable.
    - You should use this, because sellers wont update their pages all the time.
1. Using caches you leviate load on services and also the script will run mutch faster, because requests wont be rate limited.

### Rate limit
Requests are rate limited.
Rate limits are service specific.
So for example all calls to Elisa are rate limited together.

#### OpenRouteServices
Request to [OpenRouteService](https://openrouteservice.org/) are only rate limited by 40 calls per minute.

#### Every other service
Requests to every other service are rate limited to once per second so we don't overload their servers with this script.
