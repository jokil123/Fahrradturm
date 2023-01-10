import { StatusBar } from 'expo-status-bar';
import { Alert, Button, StyleSheet, Text, TextInput, View } from 'react-native';
import React, {useState} from 'react';
import {styles} from "./../assets/style/style";
import { auth } from './../firebase';



export default function Account ({navigation}) {

const LogOut = () => { 
    auth
    .signOut()
    .then(() => navigation.navigate("settings"));

}

  return (
    <View style={styles.container}>
      <Text>{auth.currentUser.email}</Text>
  <Button title='Logout' onPress={LogOut}></Button>

      
    </View>
  );
}
