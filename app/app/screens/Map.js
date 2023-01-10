import { StatusBar } from 'expo-status-bar';
import { Button, StyleSheet, Text, View, Image, Alert, FlatList, ListItem } from 'react-native';
import {styles} from "./../assets/style/style";
import MapView from 'react-native-maps';
import { Marker } from 'react-native-maps';
import BikeMarker from '../components/fetch';
import { useEffect} from 'react';
import React, {useState} from 'react';
import  {getTowers}  from '../components/fetch';


var initialRegion= {
  latitude:47.4,
  longitude: 9.7,
  latitudeDelta: 0.922,
  longitudeDelta: 0.421,
};




export default function Map({navigation}) {


  const pressHandler = () => {
      
  }
  
  const myArray=global.tower;


  return (
    <View style={styles.container}>
<MapView loadingEnabled = {true} style={styles.map} showsUserLocation={true} initialRegion={initialRegion}>
{myArray.map(marker => (
  
    <Marker
      key={marker.name}
      coordinate = {{latitude: marker.location.latitude,longitude: marker.location.longitude}}
      onPress={() => navigation.navigate('tower',{tower: marker})}
      >
          <Image 
            source={require('./../assets/bikeicon.png')}
            style={{width: 70, height: 70}}
           
          />
          
      </Marker>
 ))}
              
    </MapView>  
    </View>
  );

}
  

  
/*
     <FlatList 
        data = {myArray}
        keyExtractor={(item) => item.name}
        extraData={myArray}
        renderItem = {({item}) => 
        <View>
            <Text key={item.name}>{item.location.latitude}</Text>
        </View>
        }
        />
*/
