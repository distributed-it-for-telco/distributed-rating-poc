homeDir=$PWD
echo $homeDir
##===========Metaverse agent================
echo "#################################################################"
echo "Building commons component"
cd $homeDir/commons
rm -rf wit/deps
wit-deps
wash build
##===========Metaverse agent================
echo "#################################################################"
echo "Building rating interface"
cd $homeDir/rating-inerface
rm -rf wit/deps
wit-deps
wash build
##===========Usage Collector Orange================
echo "#################################################################"
echo "Building usage-collector-orange"
cd $homeDir/usage-collectors/usage-collector-orange
rm -rf wit/deps
wit-deps
wash build
##===========Usage collector orange connectivity================
echo "#################################################################"
echo "Building usage-collector-orange-connectivity"
cd $homeDir/usage-collectors/usage-collector-orange-connectivity
rm -rf wit/deps
wit-deps
wash build
##===========Usage collector video provider================
echo "#################################################################"
echo "Building usage-collector-video-provider"
cd $homeDir/usage-collectors/usage-collector-video-provider
rm -rf wit/deps
wit-deps
wash build
##===========Balance Manager================
echo "#################################################################"
echo "Building balance-management"
cd $homeDir/balance-management
rm -rf wit/deps
wit-deps
wash build
wac plug ./build/balance_manager_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/balance_manager_s.wasm
##===========Metaverse agent================
echo "#################################################################"
echo "Building streaming-rating-agent"
cd $homeDir/rating-agents/streaming-rating-agent
rm -rf wit/deps
wit-deps
wash build
##===========Metaverse agent================
echo "#################################################################"
echo "Building video-rating-agent"
cd $homeDir/rating-agents/video-rating-agent
rm -rf wit/deps
wit-deps
wash build
##===========Metaverse agent================
echo "#################################################################"
echo "Building orange-vod-metaverse-agent"
cd $homeDir/rating-agents/orange-vod-metaverse-agent
rm -rf wit/deps
wit-deps
wash build
##===========Video on demand agent=================
echo "#################################################################"
echo "Building orange-vod-rating-agent"
cd $homeDir/rating-agents/orange-vod-rating-agent
rm -rf wit/deps
wit-deps
wash build
wac plug ./build/orange_vod_ratingagent_s.wasm --plug ../../commons/build/rating_commons_s.wasm --plug ../../usage-collectors/usage-collector-orange/build/usage_collector_orange_s.wasm -o ./build/orange_vod_ratingagent_s.wasm
##===========Video on demand agent=================
echo "#################################################################"
echo "Building aws-stor-rating-agent"
cd $homeDir/rating-agents/aws-stor-rating-agent
rm -rf wit/deps
wit-deps
wash build
wac plug ./build/aws_stor_rating_agent_s.wasm --plug ../../commons/build/rating_commons_s.wasm -o ./build/orange_vod_ratingagent_s.wasm
##===========Rating Coordinator==============
echo "#################################################################"
echo "Building rating-coordinator"
cd $homeDir/rating-coordinator
rm -rf wit/deps
wit-deps
wash build
wac plug ./build/rating_coordinator_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/rating_coordinator_s.wasm
##===========Api gateway=====================
echo "#################################################################"
echo "Building api-gateway"
cd $homeDir/api-gateway
rm -rf wit/deps
wit-deps
wash build
wac plug ./build/api_gateway_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/api_gateway_s.wasm
##===========================================
cd $homeDir
wash up -d
sleep 5
wash app delete rating && wash app deploy wadm.yaml 
wash down