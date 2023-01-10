// Import the functions you need from the SDKs you need

import { getAuth } from "firebase/auth";
import AsyncStorage from '@react-native-async-storage/async-storage';
import { initializeAuth, getReactNativePersistence} from 'firebase/auth/react-native';
import { useState } from 'react';
import firebase from 'firebase/compat/app';
import 'firebase/compat/auth';
import 'firebase/compat/firestore';
import { doc, setDoc, Timestamp } from "firebase/firestore";

import {useAuthState} from 'react-firebase-hooks/auth';
import {useCollectionData} from 'react-firebase-hooks/firestore';


const firebaseConfig = {
  apiKey: "AIzaSyABZ7cwwms2GT4eKrznaa7B4yaEC8zAdR0",
  authDomain: "fahrradturmtest.firebaseapp.com",
  projectId: "fahrradturmtest",
  storageBucket: "fahrradturmtest.appspot.com",
  messagingSenderId: "75931027863",
  appId: "1:75931027863:web:1b08fd61af87bac1e45cd2",
  measurementId: "G-PJJCQRHBHD"
};

// Initialize Firebase
const app = firebase.initializeApp(firebaseConfig);
const auth = firebase.auth();
const firestore = firebase.firestore();


/*const auth = getAuth(app);*/
  export { auth, firestore };