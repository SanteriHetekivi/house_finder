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
  --cottage-latitude=<COTTAGE-LATITUDE> \
  --cottage-longitude=<COTTAGE-LONGITUDE> \
  --open-route-service-token '<OPEN-ROUTE-SERVICE-AUTHORIZATION-TOKEN>' \
  --telegram-bot-token '<TELEGRAM-BOT-TOKEN>' \
  --telegram-user-id <TELEGRAM-USER-ID>
```

## Arguments
- `--publishing-time-search-criteria` - (Optional) (Default: ANY_DAY) Search criteria for publishing time. One of: ANY_DAY, WITHIN_ONE_DAY, WITHIN_TWO_DAYS, WITHIN_SEVEN_DAYS or WITHIN_TWO_WEEKS
- `--price_max` - (Optional) Max price in euros.
- `--cities` - Cities to search for. Allows multiple.
- `--cottage-latitude` - Latitude to summer cottage.
- `--cottage-longitude` - Longitude to summer cottage.
- `--open-route-service-token` - (Optional) [OpenRouteService](https://openrouteservice.org/) authorization token: https://openrouteservice.org/sign-up/
- `--telegram-bot-token` - (Optional) (Requires: --telegram-user-id) Telegram bot token from [BotFather](https://telegram.me/BotFather).
- `--telegram-user-id` - (Optional) (Requires: --telegram-bot-token) Your Telegram user ID.
- `--cache` - (Optional) If given stores all of the request data to cache directory in same directory as executable.

## Info

### Cache
1. Writes cache to same directory executable is in.
   - If run with `cargo run` directory will be created in `./target/debug/cache` directory.
1. Even without `--cache` argument always caches [OpenRouteService](https://openrouteservice.org/) biking distance, because it includes coordinates and should not change.

### Rate limit
Requests are rate limited.
Rate limits are second- and top-domain specific.
So for example if I made the call to www.google.com program would rate limit it together with all the google.com requests.

#### OpenRouteServices
Request to [OpenRouteService](https://openrouteservice.org/) are only rate limited by 40 calls per minute.

#### Every other service
Requests to every other service are rate limited to once per second.
