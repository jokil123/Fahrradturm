import { firestore } from './../firebase';
import { collection, query, where, onSnapshot } from "firebase/firestore";


export default function addUser(id, email) {
    
    const res = firestore.collection('user').doc(id).set({email: email});
}