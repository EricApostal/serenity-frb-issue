// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../../frb_generated.dart';
import 'id.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These types are ignored because they are neither used by any `pub` functions nor (for structs and enums) marked `#[frb(unignore)]`: `EmojiIdentifierParseError`, `ImageHashInner`, `ImageHashParseError`, `IncidentUpdate`, `Incident`, `Maintenance`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `clone`, `clone`, `clone`, `clone`, `clone`, `clone`, `clone`, `cmp`, `deserialize`, `eq`, `eq`, `eq`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `from_str`, `from_str`, `hash`, `partial_cmp`, `serialize`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<EmojiIdentifier>>
abstract class EmojiIdentifier implements RustOpaqueInterface {
  bool get animated;

  EmojiId get id;

  String get name;

  set animated(bool animated);

  set id(EmojiId id);

  set name(String name);

  /// Generates a URL to the emoji's image.
  Future<String> url();
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ImageHash>>
abstract class ImageHash implements RustOpaqueInterface {
  /// Returns if the linked image is animated, which means the hash starts with `a_`.
  ///
  /// # Example
  /// ```rust
  /// use serenity::model::misc::ImageHash;
  ///
  /// let animated_hash: ImageHash = "a_e3c0db7f38777778fb43081f8746ebc9".parse().unwrap();
  /// assert!(animated_hash.is_animated());
  /// ```
  Future<bool> isAnimated();
}
