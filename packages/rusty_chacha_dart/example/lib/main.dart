import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:rusty_chacha_dart/rusty_chacha_dart.dart';

void main() async {
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
