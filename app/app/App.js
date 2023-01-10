import { StatusBar } from 'expo-status-bar';
import { Button, StyleSheet, Text, View } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import Home from "./screens/Home";
import Settings from "./screens/Settings";
import Ionicons from 'react-native-vector-icons/Ionicons';
import Map from "./screens/Map";
import Services from "./screens/Services";
import SignInScreen from './screens/SignIn';
import LogInScreen from './screens/LogIn';
import TowerInfo from './screens/Tower';
import Account from './screens/Account';
import BoxInfo from './screens/Boxinfo';
import { useFonts } from 'expo-font';
const loggedin = true;
const Tab = createBottomTabNavigator();
const Stack = createNativeStackNavigator();


function SettingsStackScreen() {
  
  return (
    <Stack.Navigator screenOptions={({ route }) => ({
          
      //headerShown: false,   
      headerTitleStyle: {
          fontFamily: "Poppins",
          fontWeight: "400",
        fontSize: 21
        } })}>
      <Stack.Screen name="settings" options={{ title: 'Settings' }} component={Settings} />
      <Stack.Screen name="signin"  options={{ title: 'Sign in' }}  component={SignInScreen} />
      <Stack.Screen name="login" options={{ title: 'Log in' }} component={LogInScreen} />
      <Stack.Screen name="account" options={{ title: 'Account' }} component={Account} />
      
    </Stack.Navigator>
  );
}

function MapStackScreen() {
  return (
    <Stack.Navigator screenOptions={({ route }) => ({ 
      headerTitleStyle: {
        fontFamily: "Poppins",
        fontWeight: "400",
      fontSize: 21
      }  
       })}>
      <Stack.Screen name="map" options={{ title: 'Map' }} component={Map} />
      <Stack.Screen name="tower" options={{ title: '' }} component={TowerInfo} />
    </Stack.Navigator>
  );
}

function ServiceStackScreen() {
  return (
    <Stack.Navigator screenOptions={({ route }) => ({ 
      headerTitleStyle: {
        fontFamily: "Poppins",
        fontWeight: "400",
      fontSize: 21
      }  
       })}>
      <Stack.Screen name="services" options={{ title: 'Services' }} component={Services} />
    </Stack.Navigator>
  );
}

function HomeStackScreen() {
  return (
    <Stack.Navigator screenOptions={({ route }) => ({ 
      headerTitleStyle: {
        fontFamily: "Poppins",
        fontWeight: "400",
      fontSize: 21
      }  
       })}>
      <Stack.Screen name="map" options={{ title: 'Home' }} component={Home} />
      <Stack.Screen name="boxinfo" options={{ title: 'Box' }} component={BoxInfo} />
    </Stack.Navigator>
  );
}

export default function App() {
  const [loaded] = useFonts({
    Poppins: require('./assets/fonts/Poppins-Bold.ttf'),
  });

  if (!loaded) {
    return null;
  }

  return (
    
    <NavigationContainer>
      
    <Tab.Navigator screenOptions={({ route }) => ({
          
          headerShown: false,     
          
          tabBarIcon: ({ focused, color, size }) => {
            let iconName;

            if (route.name === 'Home') {
              iconName = 'menu';
            } else if (route.name === 'Settings') {
              iconName = 'settings';
            }
            else if (route.name === 'Map') {
              iconName = 'map';
            }
            else if (route.name === 'Services') {
              iconName = 'grid';
              
            }

            // You can return any component that you like here!
            return <Ionicons name={iconName} size={size} color={color} />;
          },
          tabBarActiveTintColor: '#24AA52',
          tabBarInactiveTintColor: '#292D32',
        })}>
      <Tab.Screen name="Home" component={HomeStackScreen} />
      <Tab.Screen name="Map" component={MapStackScreen} />
      <Tab.Screen name="Services" component={ServiceStackScreen} />
      <Tab.Screen name="Settings"component={SettingsStackScreen} />
      
      
    </Tab.Navigator>
 
   
      
  </NavigationContainer>
);

      }