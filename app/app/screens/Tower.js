import { StatusBar } from 'expo-status-bar';
import { Alert, Button, StyleSheet, Text, TextInput, View, Image } from 'react-native';
import {styles} from "../assets/style/style";
import MapView from 'react-native-maps';
import { Marker } from 'react-native-maps';

export default function TowerInfo({route, navigation}) {
  const {tower} = route.params;
  var initialRegion= {
    latitude:tower.location.latitude,
    longitude: tower.location.longitude,
    latitudeDelta: 0.00922,
    longitudeDelta: 0.00421,
  };
  
  //navigation.setOptions({ title: tower.name });
  return (
    <View style={styles.towercontainer}>
      <Text style={styles.towerheading}>{tower.name}</Text>
      <MapView loadingEnabled = {true} style={styles.towermap} showsUserLocation={true} initialRegion={initialRegion}>
      <Marker
      coordinate = {{latitude: tower.location.latitude,longitude: tower.location.longitude}}
      >
          <Image 
            source={require('./../assets/bikeicon.png')}
            style={{width: 70, height: 70}}
           
          />
          
      </Marker>
        </MapView>
    </View>
  );
}

