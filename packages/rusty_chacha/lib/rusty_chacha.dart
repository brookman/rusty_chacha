import 'dart:io' as io;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:rusty_chacha/rusty_chacha.dart';

export 'src/api.dart';
export 'src/frb_generated.dart';

/// Use this to initialize RustyChaCha
class RustyChaCha {
  static bool _initialized = false;

  static Future<RustyChaCha20Poly1305> createChaCha20Poly1305({
    required Uint8List key,
    Compression? compression,
  }) async {
    await _ensureInitialized();
    return RustyChaCha20Poly1305.createInternal(key: key, compression: compression);
  }

  static Future<RustyXChaCha20Poly1305> createXChaCha20Poly1305({
    required Uint8List key,
    Compression? compression,
  }) async {
    await _ensureInitialized();
    return RustyXChaCha20Poly1305.createInternal(key: key, compression: compression);
  }

  static Future<void> _ensureInitialized() async {
    if (_initialized) {
      return;
    }
    if (io.Platform.isIOS || io.Platform.isMacOS) {
      // TODO(brookman): Use dynamic linking for iOS and MacOS?
      final lib = ExternalLibrary.process(iKnowHowToUseIt: true);
      await RustLib.init(externalLibrary: lib);
    } else {
      await RustLib.init();
    }

    await RustLib.init();
    _initialized = true;
  }
}
