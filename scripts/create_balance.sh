nats req cc.commands.balance "`cat ./create_balance_cmd.json | jq -c`" -s connect.cosmonic.sh --creds ~/.cosmo/user.creds
