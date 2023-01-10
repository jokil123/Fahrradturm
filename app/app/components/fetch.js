import { Button, StyleSheet, Text, View, Image, Alert } from 'react-native';
import { firestore } from './../firebase';
import { collection, query, where, onSnapshot } from "firebase/firestore";
import React, { useEffect } from 'react';


async function getTowers(TowerRetrived) {
    var towerdata = []
    const q = query(collection(firestore, "tower"));
    const unsubscribe = onSnapshot(q, (querySnapshot) => {
        querySnapshot.forEach((doc) => {

            const towerDoc = doc.data()
            TowerRetrived(towerDoc)
            
  });
  
});
}

async function getActivity(document, ActivityRetrived) {

    const snapshot = await firestore.collection('user').doc(document).collection('rentals').get()
    const activitydata=[]
    snapshot.docs.map(doc => activitydata.push(doc.data()))
    ActivityRetrived(activitydata)
}


export {getTowers, getActivity}