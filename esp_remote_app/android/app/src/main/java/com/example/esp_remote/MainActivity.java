package com.example.esp_remote;

import androidx.annotation.NonNull;
import io.flutter.embedding.android.FlutterActivity;
import io.flutter.embedding.engine.FlutterEngine;
import io.flutter.plugins.GeneratedPluginRegistrant;


public class MainActivity extends FlutterActivity {
 static {
    System.loadLibrary("native");
 }
 @Override
 public void configureFlutterEngine(@NonNull FlutterEngine flutterEngine) {
 GeneratedPluginRegistrant.registerWith(flutterEngine);
 }
}
