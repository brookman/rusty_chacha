// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Compression {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() uncompressed,
    required TResult Function(int? compressionLevel) zstd,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? uncompressed,
    TResult? Function(int? compressionLevel)? zstd,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? uncompressed,
    TResult Function(int? compressionLevel)? zstd,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Compression_Uncompressed value) uncompressed,
    required TResult Function(Compression_Zstd value) zstd,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Compression_Uncompressed value)? uncompressed,
    TResult? Function(Compression_Zstd value)? zstd,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Compression_Uncompressed value)? uncompressed,
    TResult Function(Compression_Zstd value)? zstd,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CompressionCopyWith<$Res> {
  factory $CompressionCopyWith(
          Compression value, $Res Function(Compression) then) =
      _$CompressionCopyWithImpl<$Res, Compression>;
}

/// @nodoc
class _$CompressionCopyWithImpl<$Res, $Val extends Compression>
    implements $CompressionCopyWith<$Res> {
  _$CompressionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Compression_UncompressedImplCopyWith<$Res> {
  factory _$$Compression_UncompressedImplCopyWith(
          _$Compression_UncompressedImpl value,
          $Res Function(_$Compression_UncompressedImpl) then) =
      __$$Compression_UncompressedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Compression_UncompressedImplCopyWithImpl<$Res>
    extends _$CompressionCopyWithImpl<$Res, _$Compression_UncompressedImpl>
    implements _$$Compression_UncompressedImplCopyWith<$Res> {
  __$$Compression_UncompressedImplCopyWithImpl(
      _$Compression_UncompressedImpl _value,
      $Res Function(_$Compression_UncompressedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Compression_UncompressedImpl extends Compression_Uncompressed {
  const _$Compression_UncompressedImpl() : super._();

  @override
  String toString() {
    return 'Compression.uncompressed()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Compression_UncompressedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() uncompressed,
    required TResult Function(int? compressionLevel) zstd,
  }) {
    return uncompressed();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? uncompressed,
    TResult? Function(int? compressionLevel)? zstd,
  }) {
    return uncompressed?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? uncompressed,
    TResult Function(int? compressionLevel)? zstd,
    required TResult orElse(),
  }) {
    if (uncompressed != null) {
      return uncompressed();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Compression_Uncompressed value) uncompressed,
    required TResult Function(Compression_Zstd value) zstd,
  }) {
    return uncompressed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Compression_Uncompressed value)? uncompressed,
    TResult? Function(Compression_Zstd value)? zstd,
  }) {
    return uncompressed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Compression_Uncompressed value)? uncompressed,
    TResult Function(Compression_Zstd value)? zstd,
    required TResult orElse(),
  }) {
    if (uncompressed != null) {
      return uncompressed(this);
    }
    return orElse();
  }
}

abstract class Compression_Uncompressed extends Compression {
  const factory Compression_Uncompressed() = _$Compression_UncompressedImpl;
  const Compression_Uncompressed._() : super._();
}

/// @nodoc
abstract class _$$Compression_ZstdImplCopyWith<$Res> {
  factory _$$Compression_ZstdImplCopyWith(_$Compression_ZstdImpl value,
          $Res Function(_$Compression_ZstdImpl) then) =
      __$$Compression_ZstdImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int? compressionLevel});
}

/// @nodoc
class __$$Compression_ZstdImplCopyWithImpl<$Res>
    extends _$CompressionCopyWithImpl<$Res, _$Compression_ZstdImpl>
    implements _$$Compression_ZstdImplCopyWith<$Res> {
  __$$Compression_ZstdImplCopyWithImpl(_$Compression_ZstdImpl _value,
      $Res Function(_$Compression_ZstdImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? compressionLevel = freezed,
  }) {
    return _then(_$Compression_ZstdImpl(
      compressionLevel: freezed == compressionLevel
          ? _value.compressionLevel
          : compressionLevel // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$Compression_ZstdImpl extends Compression_Zstd {
  const _$Compression_ZstdImpl({this.compressionLevel}) : super._();

  @override
  final int? compressionLevel;

  @override
  String toString() {
    return 'Compression.zstd(compressionLevel: $compressionLevel)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Compression_ZstdImpl &&
            (identical(other.compressionLevel, compressionLevel) ||
                other.compressionLevel == compressionLevel));
  }

  @override
  int get hashCode => Object.hash(runtimeType, compressionLevel);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Compression_ZstdImplCopyWith<_$Compression_ZstdImpl> get copyWith =>
      __$$Compression_ZstdImplCopyWithImpl<_$Compression_ZstdImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() uncompressed,
    required TResult Function(int? compressionLevel) zstd,
  }) {
    return zstd(compressionLevel);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? uncompressed,
    TResult? Function(int? compressionLevel)? zstd,
  }) {
    return zstd?.call(compressionLevel);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? uncompressed,
    TResult Function(int? compressionLevel)? zstd,
    required TResult orElse(),
  }) {
    if (zstd != null) {
      return zstd(compressionLevel);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Compression_Uncompressed value) uncompressed,
    required TResult Function(Compression_Zstd value) zstd,
  }) {
    return zstd(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Compression_Uncompressed value)? uncompressed,
    TResult? Function(Compression_Zstd value)? zstd,
  }) {
    return zstd?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Compression_Uncompressed value)? uncompressed,
    TResult Function(Compression_Zstd value)? zstd,
    required TResult orElse(),
  }) {
    if (zstd != null) {
      return zstd(this);
    }
    return orElse();
  }
}

abstract class Compression_Zstd extends Compression {
  const factory Compression_Zstd({final int? compressionLevel}) =
      _$Compression_ZstdImpl;
  const Compression_Zstd._() : super._();

  int? get compressionLevel;
  @JsonKey(ignore: true)
  _$$Compression_ZstdImplCopyWith<_$Compression_ZstdImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
