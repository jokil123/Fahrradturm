import { StatusBar } from 'expo-status-bar';
import { Alert, Button, StyleSheet, Text, TextInput, View, Image } from 'react-native';
import {styles} from "../assets/style/style";


export default function BoxInfo({route, navigation}) {
  const {box} = route.params;
  console.log(box)

  return (
    <View style={styles.towercontainer}>
      <Text style={styles.towerheading}>{box.boxfk}</Text>
 
    </View>
  );
}

