import 'dart:io';

import 'package:flutter/material.dart';
import 'package:midi_player/src/rust/api/midi_playback.dart';
import 'package:midi_player/src/rust/api/simple.dart';
import 'package:midi_player/src/rust/frb_generated.dart';
import 'package:file_picker/file_picker.dart';

Future<void> main() async {
  await RustLib.init();

  FilePickerResult? sf2Result = await FilePicker.platform.pickFiles(dialogTitle: 'Select SoundFont2 file (.sf2)');
  await loadSoundFont(soundFontData: File(sf2Result!.paths.first!).readAsBytesSync());

  FilePickerResult? midiResult = await FilePicker.platform.pickFiles(dialogTitle: 'Select MIDI file (.mid)');
  await loadMidiData(midiData: File(midiResult!.paths.first!).readAsBytesSync());

  startPlayback();

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
            'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`',
          ),
        ),
      ),
    );
  }
}
