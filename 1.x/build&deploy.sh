homeDir=$PWD
echo $homeDir
##===========Metaverse agent================
cd $homeDir/metaverse-rating-agent
wit-deps
wash build
##===========Vodafone agent=================
cd $homeDir/orange-vod-rating-agent
wit-deps
wash build
##===========Balance Manager================
cd $homeDir/balance-management
wit-deps
wash build
##===========Rating Coordinator==============
cd $homeDir/rating-coordinator
wit-deps
wash build
##===========Api gateway=====================
cd $homeDir/api-gateway
wit-deps
wash build
##===========================================
cd $homeDir
wash up -d
sleep 5
wash app delete rating && wash app deploy wadm.yaml 
wash down