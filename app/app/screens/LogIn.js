import { StatusBar } from 'expo-status-bar';
import { Alert, Button, StyleSheet, Text, TextInput, View } from 'react-native';
import React, {useState} from 'react';
import {styles} from "./../assets/style/style";
import { auth } from './../firebase';


export default function LogInScreen({navigation}) {

const pressHandler = () => {
    navigation.navigate('signin')
}
const [email, setEmail] = useState('');
const [password, setPassword] = useState('');

const SignIn = () => {
  auth
  .signInWithEmailAndPassword(email, password)
  .then((userCredential) => {
      const user = userCredential.user;
      Alert.alert("signed in");
      navigation.navigate('settings',{created: false})
    })
    .catch((error) => {
      const errorCode = error.code;
      const errorMessage = error.message;
      Alert.alert(errorMessage);
    });
  }
  return (
    <View style={styles.container}>
      <Text style={styles.headlinetext}>Please log in</Text>
     
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
      <Button onPress={SignIn} title='log in'></Button>
      <Text>
{"\n"}
{"\n"}
{"\n"}
{"\n"}
<Text>Don't have an Account yet?</Text>
</Text>
      <Button title='Register' onPress={pressHandler}></Button>
    </View>
  );
}

