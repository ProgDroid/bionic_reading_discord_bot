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

## Running locally

You will need Docker.

1. Copy `.env.dist` to `.env` and replace the variables with the respective values (`PORT` is optional and defaults to 8080).
2. Run `docker build --tag bionic_reading_discord_bot .`.
3. Run `docker run --rm -p 8080:8080 --env-file ./.env bionic_reading_discord_bot:latest`.

Once the container is running, you can test by hitting the `api/discord/interactions` endpoint:

```shell
curl -X POST http://localhost:8080/api/discord/interactions -H "Content-Type: application/json" -d 'post body here'
```

You can find an example interaction payload [here](https://discord.com/developers/docs/interactions/application-commands#slash-commands-example-interaction).

## Hosting on Cloud Run

This was intended to be hosted on Cloud Run (hence it not using WebSockets).

It expects to find its configuration values in Secret Manager, set up in the same project with the handles found in the `.env.dist` file. Each secret should contain the respective value.

It also expects an Artifact Registry repository to be created in advance. Create one with format `Docker` and select a region you will use later for the Cloud Run service.

Once the secrets and Artifact Registry repository are set up, deploy via the `gcloud` CLI or using the actions set up in this repo if you fork this repo.

If you use the actions in this repo, you will have to set up the following as secrets in GitHub for the actions to work:
- `GCP_APP_NAME` with the desired application image name.
- `GCP_CREDENTIALS` with the contents of the `key.json` file generated following [these steps](https://github.com/GoogleCloudPlatform/community/blob/master/archived/cicd-cloud-run-github-actions/index.md#cloud-run).
- `GCP_EMAIL` with the service account email that you've created for the deployment.
- `GCP_PROJECT_ID` with the Google Cloud Project ID.
- `GCP_ARTIFACT_REPO` with the name of the Artifact Register repository.
- `GCP_REGION` with the desired server region.

Finally, go into the Discord bot in the [Developer Portal](https://discord.com/developers/applications) and set the Interactions Endpoint URL as `https://<your-cloud-run-url-here>/api/discord/interactions`.

## How to use

Go into a server with the bot in it and type `/convert`. This should bring up the command. The `text_value` box will contain the text you want to convert. This field is mandatory.

There are 2 additional fields which are optional:
- `fixation_value` sets the length of the highlights and you can pick between Weakest, Weak, Average, Strong and Strongest.
- `saccade_value` sets the amount of highlights in the text and you can pick between Fewest, Few, Average, More and Most.

Sensible defaults will be used if these are not included.
