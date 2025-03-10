// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import '../../lib.dart';
import 'model/gateway.dart';
import 'model/user.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These types are ignored because they are neither used by any `pub` functions nor (for structs and enums) marked `#[frb(unignore)]`: `ChunkGuildFilter`, `ReconnectType`, `ShardAction`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `clone`, `clone`, `clone`, `clone`, `cmp`, `eq`, `eq`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `from`, `hash`, `partial_cmp`
// These functions have error during generation (see debug logs or enable `stop_on_error: true` for more details): `competing`, `custom`, `listening`, `playing`, `streaming`, `watching`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ActivityData>>
abstract class ActivityData implements RustOpaqueInterface {
  ActivityType get kind;

  String get name;

  String? get state;

  Url? get url;

  set kind(ActivityType kind);

  set name(String name);

  set state(String? state);

  set url(Url? url);
}

/// Indicates the current connection stage of a [`Shard`].
///
/// This can be useful for knowing which shards are currently "down"/"up".
enum ConnectionStage {
  /// Indicator that the [`Shard`] is normally connected and is not in, e.g., a resume phase.
  connected,

  /// Indicator that the [`Shard`] is connecting and is in, e.g., a resume phase.
  connecting,

  /// Indicator that the [`Shard`] is fully disconnected and is not in a reconnecting phase.
  disconnected,

  /// Indicator that the [`Shard`] is currently initiating a handshake.
  handshake,

  /// Indicator that the [`Shard`] has sent an IDENTIFY packet and is awaiting a READY packet.
  identifying,

  /// Indicator that the [`Shard`] has sent a RESUME packet and is awaiting a RESUMED packet.
  resuming;

  /// Whether the stage is a form of connecting.
  ///
  /// This will return `true` on:
  /// - [`Connecting`][`ConnectionStage::Connecting`]
  /// - [`Handshake`][`ConnectionStage::Handshake`]
  /// - [`Identifying`][`ConnectionStage::Identifying`]
  /// - [`Resuming`][`ConnectionStage::Resuming`]
  ///
  /// All other variants will return `false`.
  ///
  /// # Examples
  ///
  /// Assert that [`ConnectionStage::Identifying`] is a connecting stage:
  ///
  /// ```rust
  /// use serenity::gateway::ConnectionStage;
  ///
  /// assert!(ConnectionStage::Identifying.is_connecting());
  /// ```
  ///
  /// Assert that [`ConnectionStage::Connected`] is _not_ a connecting stage:
  ///
  /// ```rust
  /// use serenity::gateway::ConnectionStage;
  ///
  /// assert!(!ConnectionStage::Connected.is_connecting());
  /// ```
  Future<bool> isConnecting() => RustLib.instance.api
      .serenityGatewayConnectionStageIsConnecting(that: this);
}

/// Presence data of the current user.
class PresenceData {
  /// The current activity, if present
  final ActivityData? activity;

  /// The current online status
  final OnlineStatus status;

  const PresenceData({this.activity, required this.status});

  static Future<PresenceData> default_() =>
      RustLib.instance.api.serenityGatewayPresenceDataDefault();

  @override
  int get hashCode => activity.hashCode ^ status.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PresenceData &&
          runtimeType == other.runtimeType &&
          activity == other.activity &&
          status == other.status;
}
