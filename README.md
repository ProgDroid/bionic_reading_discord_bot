# Bionic Reading Discord Bot

Discord bot that takes text and converts to Bionic Reading format

Uses [Bionic Reading API Wrapper](https://github.com/ProgDroid/bionic_reading_api) to convert text to [Bionic Reading](https://bionic-reading.com/) formatted Markdown.

## Set Up

Create the Discord application as per the [Discord docs](https://discord.com/developers/docs/getting-started#step-1-creating-an-app).
Once the application is created, invite it to the server you wish to run this on.

Afterwards, you will need to register the command in advance of using this. This can be done with a curl request as follows:
```shell
export APPLICATION_ID=<discord application ID>
export BOT_TOKEN=<discord bot token>

curl -X POST https://discord.com/api/v10/applications/$APPLICATION_ID/commands -H "Authorization: Bot $BOT_TOKEN" -H "Content-Type: application/json" '{"name":"convert","description":"Converts given text to a Bionic Reading highlighted text","options":[{"type":3,"name":"text_value","description":"The text you want to convert","required":true,"min_length":1},{"type":3,"name":"fixation_value","description":"Fixation level (length of highlights)","required":false,"choices":[{"name":"Weakest","value":"1"},{"name":"Weak","value":"2"},{"name":"Average","value":"3"},{"name":"Strong","value":"4"},{"name":"Strongest","value":"5"}]},{"type":3,"name":"saccade_value","description":"Saccade level (amount of highlights)","required":false,"choices":[{"name":"Fewest","value":"10"},{"name":"Few","value":"20"},{"name":"Average","value":"30"},{"name":"More","value":"40"},{"name":"Most","value":"50"}]}]}'
```

### Running locally

You will need Docker.

1. Run `docker build --tag bionic_reading_discord_bot .`
2. Run `docker run --rm -p 8080:8080 --env-file ./.env bionic_reading_discord_bot:latest`

Once the container is running, you can test by hitting the `api/discord/interactions` endpoint:

```shell
curl -X POST http://localhost:8080/api/discord/interactions -H "Content-Type: application/json" -d 'post body here'
```

You can find an example interaction payload [here](https://discord.com/developers/docs/interactions/application-commands#slash-commands-example-interaction).

### Running on Cloud Run

<!-- Set up Secrets -->

<!-- TODO -->