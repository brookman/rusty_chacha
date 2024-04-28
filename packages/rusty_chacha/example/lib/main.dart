import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:collection/collection.dart';
import 'package:rusty_chacha/rusty_chacha.dart';

void _encrypt() async {
  final data = Uint8List.fromList([1, 2, 3, 4, 5]);
  print('data: $data');

  // Create and use a ChaCha20Poly1305 cipher with a random key:
  RustyChaCha20Poly1305 cipher = await RustyChaCha.create();

  Uint8List encrypted = await cipher.encrypt(cleartext: data);
  print('encrypted: $encrypted');

  Uint8List decrypted = await cipher.decrypt(ciphertext: encrypted);
  print('decrypted: $decrypted');

  assert(const ListEquality().equals(data, decrypted));
}

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rusty ChaCha 💃🦀 demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Rusty ChaCha 💃🦀 demo'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      floatingActionButton: const FloatingActionButton(
        onPressed: _encrypt,
        tooltip: 'Encrypt',
        child: Icon(Icons.lock),
      ),
    );
  }
}
