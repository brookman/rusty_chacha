import 'dart:io' as io;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:rusty_chacha/rusty_chacha.dart';

export 'src/api.dart';
export 'src/frb_generated.dart';

/// Use this to initialize RustyChaCha
class RustyChaCha {
  static bool _initialized = false;

  /// Call once before using
  static Future<void> init() async {
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

    _initialized = true;
  }
}
