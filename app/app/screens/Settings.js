import { StatusBar } from 'expo-status-bar';
import { Image, TouchableOpacity ,Alert, StyleSheet, Text, View, Button } from 'react-native';
import {styles} from "./../assets/style/style";
import addUser from '../components/addUser';
import { auth } from '../firebase';
import React, {useState} from 'react';

export default function Settings({navigation, route}) {

  function SettingLink(props) { 
    return (
      <View onStartShouldSetResponder={() => navigation.navigate(props.navigate)} style={styles.settingItem}>
    
          <View  style={{width:"30%"}}>
            <Image style={styles.roundPicture} source={props.picturelink}></Image>
          </View>
          <View style={{width:"70%", paddingTop: 13, }}>
            <Text style={styles.settingtext}>{props.heading}</Text>
            <Text>{props.info}</Text>
          </View>
        
      </View>
    )
  }
  
  function UserStatus(props) {
  const isLoggedIn = props.isLoggedIn;
  if (isLoggedIn) {
    return  (<SettingLink navigate={"account"} picturelink={require('./../assets/images/profile.png')} heading='Profile Information' info='Name, Email, Security'/>
    )
  } else {
    
  }
  return <SettingLink navigate={"login"} picturelink={require('./../assets/images/profile.png')} heading='Log in' info='Log in or create a new Account'/>
  }
  
  if(route.params) {
    addUser(auth.currentUser.uid, auth.currentUser.email)
  }
 


  return (

 
    <View style={styles.settingcontainer}>
      <View style={{height: 300}}>
        <Text style={styles.text}>Account Settings</Text>
        <UserStatus isLoggedIn={auth.currentUser}/>
        <Text style={styles.text}>Other Settings</Text>
        <SettingLink navigate={"account"} picturelink={require('./../assets/images/profile.png')} heading='Profile Information' info='Name, Email, Security'/>
      </View>
    </View>
  );
}

