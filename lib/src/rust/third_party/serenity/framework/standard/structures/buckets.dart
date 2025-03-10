// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../../../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These types are ignored because they are neither used by any `pub` functions nor (for structs and enums) marked `#[frb(unignore)]`: `Bucket`, `Ratelimit`, `RevertBucket`, `TicketCounter`, `UnitRatelimit`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `fmt`, `fmt`, `fmt`, `fmt`
// These functions are ignored (category: IgnoreBecauseOwnerTyShouldIgnore): `give`, `give`, `take`, `take`
// These functions have error during generation (see debug logs or enable `stop_on_error: true` for more details): `await_ratelimits`, `check`, `delay_action`, `delay`, `limit_for`, `limit`, `time_span`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<BucketBuilder>>
abstract class BucketBuilder implements RustOpaqueInterface {
  static Future<BucketBuilder> default_() =>
      RustLib.instance.api
          .serenityFrameworkStandardStructuresBucketsBucketBuilderDefault();

  /// A bucket collecting tickets per channel category.
  ///
  /// This requires the cache, as messages do not contain their channel's category.
  static Future<BucketBuilder> newCategory() =>
      RustLib.instance.api
          .serenityFrameworkStandardStructuresBucketsBucketBuilderNewCategory();

  /// A bucket collecting tickets per channel.
  static Future<BucketBuilder> newChannel() =>
      RustLib.instance.api
          .serenityFrameworkStandardStructuresBucketsBucketBuilderNewChannel();

  /// A bucket collecting tickets per command invocation.
  static Future<BucketBuilder> newGlobal() =>
      RustLib.instance.api
          .serenityFrameworkStandardStructuresBucketsBucketBuilderNewGlobal();

  /// A bucket collecting tickets per guild.
  static Future<BucketBuilder> newGuild() =>
      RustLib.instance.api
          .serenityFrameworkStandardStructuresBucketsBucketBuilderNewGuild();

  /// A bucket collecting tickets per user.
  static Future<BucketBuilder> newUser() =>
      RustLib.instance.api
          .serenityFrameworkStandardStructuresBucketsBucketBuilderNewUser();
}

/// Decides what a bucket will use to collect tickets for.
enum LimitedFor {
  /// The bucket will collect tickets for every invocation of a command.
  global,

  /// The bucket will collect tickets per user.
  user,

  /// The bucket will collect tickets per guild.
  guild,

  /// The bucket will collect tickets per channel.
  channel,

  /// The bucket will collect tickets per category.
  ///
  /// This requires the cache, as messages do not contain their channel's category.
  category;

  /// We use the previous behaviour of buckets as default.
  static Future<LimitedFor> default_() =>
      RustLib.instance.api
          .serenityFrameworkStandardStructuresBucketsLimitedForDefault();
}

/// Action taken for the command invocation.
enum RateLimitAction {
  /// Invocation has been delayed.
  delayed,

  /// Tried to delay invocation but maximum of delays reached.
  failedDelay,

  /// Cancelled the invocation due to time or ticket reasons.
  cancelled,
}

/// Contains information about a rate limit.
class RateLimitInfo {
  /// Time to elapse in order to invoke a command again.
  final Duration rateLimit;

  /// Amount of active delays by this target.
  final int activeDelays;

  /// Maximum delays that this target can invoke.
  final int maxDelays;

  /// Whether this is the first time the rate limit info has been returned for this target
  /// without the rate limit to elapse.
  final bool isFirstTry;

  /// How the command invocation has been treated by the framework.
  final RateLimitAction action;

  const RateLimitInfo({
    required this.rateLimit,
    required this.activeDelays,
    required this.maxDelays,
    required this.isFirstTry,
    required this.action,
  });

  /// Gets the duration of the rate limit in microseconds.
  Future<BigInt> asMicros() => RustLib.instance.api
      .serenityFrameworkStandardStructuresBucketsRateLimitInfoAsMicros(
        that: this,
      );

  /// Gets the duration of the rate limit in milliseconds.
  Future<BigInt> asMillis() => RustLib.instance.api
      .serenityFrameworkStandardStructuresBucketsRateLimitInfoAsMillis(
        that: this,
      );

  /// Gets the duration of the rate limit in seconds.
  Future<BigInt> asSecs() => RustLib.instance.api
      .serenityFrameworkStandardStructuresBucketsRateLimitInfoAsSecs(
        that: this,
      );

  @override
  int get hashCode =>
      rateLimit.hashCode ^
      activeDelays.hashCode ^
      maxDelays.hashCode ^
      isFirstTry.hashCode ^
      action.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RateLimitInfo &&
          runtimeType == other.runtimeType &&
          rateLimit == other.rateLimit &&
          activeDelays == other.activeDelays &&
          maxDelays == other.maxDelays &&
          isFirstTry == other.isFirstTry &&
          action == other.action;
}
