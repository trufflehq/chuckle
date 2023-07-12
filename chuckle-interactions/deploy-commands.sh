curl -X PUT https://discord.com/api/applications/$DISCORD_APPLICATION_ID/commands \
	-H "Authorization: Bot $DISCORD_TOKEN" \
	-H "content-type: application/json" \
	-d @./commands.lock.json
