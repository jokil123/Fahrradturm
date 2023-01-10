import { Button, StyleSheet, Text, TextInput, View } from 'react-native';
const styles = StyleSheet.create({
    container: {
      flex: 1,
      backgroundColor: 'white',
      alignItems: 'center',
      justifyContent: 'center',
    },
    

    

    towercontainer: {
      flex: 1,
      backgroundColor: 'white',
      alignItems: 'center',
      ...Platform.select({
        ios: {
          justifyContent: 'top',
        },
        android: {
          justifyText: 'top',
        },
      })
    },

    towerheading: {
      padding: "5%",
      fontFamily: "Poppins",
      fontWeight: "400",
      fontSize: "30%",
      alignItems: 'center',
      justifyContent: 'center',
    },

    settingItem: {
      paddingTop: "5%",
      paddingLeft: "3%",
      flex: 1,
      marginBottom: "10%",
      flexDirection: "row",
      borderBottomColor: "black",
      borderBottomWidth: 1

    },

    settingtext: {
      fontFamily: "Poppins",
      fontWeight: "400",
      fontSize: 15,
      position: 'left',
      
    },
    settingcontainer: {
      paddingLeft: "3%",
      paddingTop: "10%",
      flex: 1,
      backgroundColor: 'white',
      height: "50%"
    },

    roundPicture: {
      width: 60,
      height: 60,
      borderRadius: 100,
      paddingRight: "5%"
    },

    topcontainer: {
      alignItems: 'center',
      paddingTop: "10%",
      flex: 1,
      backgroundColor: 'white',
      ...Platform.select({
        ios: {
          justifyContent: 'top',
        },
        android: {
          justifyText: 'top',
        },
      })


     
      
    },
    boxpicture: {
      height: 50,
      width: 50
    },
    homecontainer: {
      alignItems: 'center',
      paddingTop: "10%",
      flex: 1,
      backgroundColor: 'white',
      ...Platform.select({
        ios: {
          justifyContent: 'top',
        },
        android: {
          justifyText: 'top',
        },
      })},
    

    text: {
      fontFamily: "Poppins",
      fontWeight: "400",
      fontSize: 20,
      position: 'left',
      
    },
    map: {
      width: '100%',
      height: '100%',
    },
    towermap: {
      width: '100%',
      height: '40%',
    },
    headlinetext: {
      fontFamily: "Poppins",
      fontWeight: "400",
      fontSize: 25
    },
    input: {
      fontFamily: "Poppins",
      width: 250,
      height: 44,
      padding: 10,
      marginTop: 20,
      marginBottom: 10,
      backgroundColor: '#e8e8e8',
      borderColor: 'green',
      borderWidth: 2,
      borderRadius: 20,
    }
  });


  export {styles};