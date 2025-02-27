import { initializeApp } from "firebase/app";
import {
  GoogleAuthProvider,
  getAuth,
  signInWithPopup,
  signOut,
} from "firebase/auth";

const firebase = {
  initializeApp,
  getAuth,
  signInWithPopup,
  signOut,
  GoogleAuthProvider,
};

declare global {
  interface Window {
    firebase: any;
  }
}

window.firebase = firebase;
