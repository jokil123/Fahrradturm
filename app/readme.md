npx expo install firebase
npx expo install expo-location
npx expo install react-native-maps
npx expo install expo-splash-screen
npm install @react-navigation/native-stack
npm install @react-navigation/bottom-tabs 
npm install @react-navigation/native  
npm i react-native-switch-selector --save
npm i moment
npm install geolib
  
npx expo customize metro.config.js

const { getDefaultConfig } = require('@expo/metro-config');

const defaultConfig = getDefaultConfig(__dirname);
defaultConfig.resolver.assetExts.push('cjs');

module.exports = defaultConfig;
