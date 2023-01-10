import { StatusBar } from 'expo-status-bar';
import { Alert, Button, FlatList, StyleSheet, Text, View, Image, ListItem } from 'react-native';


import {styles} from "./../assets/style/style";
import  {getTowers, getActivity}  from '../components/fetch';
import { useEffect} from 'react';
import { useFocusEffect } from '@react-navigation/native';
import { auth } from '../firebase';
import React, {useState} from 'react';
var i = 0;
var towers=[];


export default function Home({navigation}) {

  const [activitydata, setData] = useState([
   
 ]);

 useFocusEffect(() => {
    
    getData()
    
     
   })
   
   function getData() {
    towers=[]
   
     getTowers(TowerRetrived)
     try {
     getActivity(auth.currentUser.uid, ActivityRetrived)
    
     }
     catch {
      //Alert.alert("biite anmelden")
     }
   };
 
   function TowerRetrived(TowerList) {
     towers.push(TowerList)
     global.tower=towers;
     
   }

   function ActivityRetrived(ActivityList) {
    setData(ActivityList)
    
    
  }



  function ActivityList() {
    return(
   <View style={styles.homecontainer}>  
    <FlatList style={{height: 100}}
        data = {activitydata}
        keyExtractor={(item) => item.location}
        extraData={activitydata}
        renderItem = {({item}) => 
        
        <View style={{height:100}} key={item.boxfk} onStartShouldSetResponder={() => navigation.navigate("boxinfo",{box:item})} >
       <Text>{item.boxfk}</Text></View>  
        
        }
        /></View> )
  }

  function UserStatus(props) {
    const isLoggedIn = props.isLoggedIn;
    if (isLoggedIn) {
      return <ActivityList/>
    } else {
      
    }
    return <View style={styles.container}><Text>Please log in or create a Account to see your Activities</Text><Image 
    source={require('./../assets/images/tower.jpg')}
    style={{width: 350, height: 350}}
   
  /></View>
    }


  return (
        <UserStatus isLoggedIn={auth.currentUser}/>
  );
}

/*
 <FlatList 
        data = {activitydata}
        keyExtractor={(item) => item.boxfk}
        extraData={activitydata}
        renderItem = {({item}) => 
        <View>
            <Text key={item.boxfk}>asdf</Text>
        </View>
        }
        />
*/