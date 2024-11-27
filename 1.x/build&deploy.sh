homeDir=$PWD
echo $homeDir
##===========Metaverse agent================
cd $homeDir/commons
wit-deps
wash build
##===========Metaverse agent================
cd $homeDir/rating-inerface
wit-deps
wash build
##===========Usage Collector Orange================
cd $homeDir/usage-collectors/usage-collector-orange
wit-deps
wash build
##===========Usage collector orange connectivity================
cd $homeDir/usage-collectors/usage-collector-orange-connectivity
wit-deps
wash build
##===========Usage collector video provider================
cd $homeDir/usage-collectors/usage-collector-video-provider
wit-deps
wash build
##===========Balance Manager================
cd $homeDir/balance-management
wit-deps
wash build
wac plug ./build/balance_manager_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/balance_manager_s.wasm
##===========Metaverse agent================
cd $homeDir/rating-agents/streaming-rating-agent
wit-deps
wash build
##===========Metaverse agent================
cd $homeDir/rating-agents/video-rating-agent
wit-deps
wash build
##===========Metaverse agent================
cd $homeDir/rating-agents/metaverse-rating-agent
wit-deps
wash build
##===========Video on demand agent=================
cd $homeDir/rating-agents/orange-vod-rating-agent
wit-deps
wash build
wac plug ./build/orange_vod_ratingagent_s.wasm --plug ../../commons/build/rating_commons_s.wasm -o ./build/orange_vod_ratingagent_s.wasm
##===========Video on demand agent=================
cd $homeDir/rating-agents/aws-stor-rating-agent
wit-deps
wash build
wac plug ./build/aws_stor_rating_agent_s.wasm --plug ../../commons/build/rating_commons_s.wasm -o ./build/orange_vod_ratingagent_s.wasm
##===========Rating Coordinator==============
cd $homeDir/rating-coordinator
wit-deps
wash build
##===========Api gateway=====================
cd $homeDir/api-gateway
wit-deps
wash build
wac plug ./build/api_gateway_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/api_gateway_s.wasm
##===========================================
cd $homeDir
wash up -d
sleep 5
wash app delete rating && wash app deploy wadm.yaml 
wash down