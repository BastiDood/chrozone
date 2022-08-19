const APP_ID = Deno.env.get('APP_ID');
const TOKEN = Deno.env.get('TOKEN');
const GUILD_ID = Deno.env.get('GUILD_ID');

// Ensure that `APP_ID` and `TOKEN` are available
if (!APP_ID || !TOKEN)
    throw new Error('missing environment variables');

const endpoint = GUILD_ID
    ? `https://discord.com/api/v10/applications/${APP_ID}/guilds/${GUILD_ID}/commands`
    : `https://discord.com/api/v10/applications/${APP_ID}/commands`;

const response = await fetch(endpoint, {
    method: 'PUT',
    headers: {
        Authorization: `Bot ${TOKEN}`,
        'Content-Type': 'application/json',
    },
    body: JSON.stringify([
        {
            name: 'epoch',
            description: 'Get the Unix timestamp from a date.',
            options: [
                {
                    type: 3,
                    name: 'timezone',
                    description: 'The timezone to base the date from. Autocompletions are enabled.',
                    required: true,
                    min_value: 1,
                    max_value: 1,
                },
                {
                    type: 4,
                    name: 'year',
                    description: 'Sets the year.',
                },
                {
                    type: 4,
                    name: 'month',
                    description: 'Sets the month.',
                    min_value: 1,
                    max_value: 12,
                },
                {
                    type: 4,
                    name: 'day',
                    description: 'Sets the day.',
                    min_value: 1,
                    max_value: 31,
                },
                {
                    type: 4,
                    name: 'hour',
                    description: 'Sets the hour (in military time).',
                    min_value: 0,
                    max_value: 23,
                },
                {
                    type: 4,
                    name: 'minute',
                    description: 'Sets the minute.',
                    min_value: 0,
                    max_value: 59,
                },
                {
                    type: 4,
                    name: 'second',
                    description: 'Sets the second.',
                    min_value: 0,
                    max_value: 60,
                },
            ],
        },
    ]),
});

const json = await response.json();
console.log(json);
