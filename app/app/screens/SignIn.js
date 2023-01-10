import { StatusBar } from 'expo-status-bar';
import { Alert, Button, StyleSheet, Text, TextInput, View } from 'react-native';
import React, {useState} from 'react';
import {styles} from "./../assets/style/style";
import { auth } from './../firebase';



export default function SignInScreen ({navigation}) {

const [email, setEmail] = useState('');
const [password, setPassword] = useState('');




const createAccount = () => { 
  auth
  .createUserWithEmailAndPassword(email, password)
  .then(() => {
    Alert.alert("User signed in and logged in")
    navigation.navigate('settings',{created: true})

  })
  .catch(error => {
    if (error.code === 'auth/email-already-in-use') {
      Alert.alert('That email address is already in use!');
    }

    if (error.code === 'auth/invalid-email') {
      Alert.alert('That email address is invalid!');
    }

    Alert.alert(error.message);
  });

}

  return (
    <View style={styles.container}>
      <Text style={styles.headlinetext}>Create a account!</Text>
     
     <TextInput
      value={email}
      onChangeText={(email) => setEmail(email)}
      placeholder={'Email'}
      style={styles.input}
    />
     <TextInput
      value={password}
      onChangeText={(password) => setPassword(password)}
      placeholder={'Password'}
      style={styles.input}
    />
  <Button title='create Account' onPress={createAccount}></Button>
  <Text>
{"\n"}
{"\n"}
</Text>
      
    </View>
  );
}
