// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import

import 'dart:convert';
import 'dart:typed_data';

import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:ffi' as ffi;

abstract class Turtlebot2 {
  Future<List<String>> availableTutlebots({dynamic hint});

  Stream<String> spawnTurtlebot({dynamic hint});

  Future<void> sendToTurtlebot({dynamic hint});

  Future<List<Feedback>> receiveFromTurtlebot({dynamic hint});
}

class BasicSensor {
  final bool valid;
  final int timeStamp;
  final int bumper;
  final int wheelDrop;
  final int cliff;
  final int leftEncoder;
  final int rightEncoder;
  final int leftPwm;
  final int rightPwm;
  final int button;
  final int charger;
  final int battery;
  final int overcurrentFlags;

  BasicSensor({
    required this.valid,
    required this.timeStamp,
    required this.bumper,
    required this.wheelDrop,
    required this.cliff,
    required this.leftEncoder,
    required this.rightEncoder,
    required this.leftPwm,
    required this.rightPwm,
    required this.button,
    required this.charger,
    required this.battery,
    required this.overcurrentFlags,
  });
}

class Cliff {
  final bool valid;
  final int rightCliffSensor;
  final int centralCliffSensor;
  final int leftCliffSensor;

  Cliff({
    required this.valid,
    required this.rightCliffSensor,
    required this.centralCliffSensor,
    required this.leftCliffSensor,
  });
}

class ControllerInfo {
  final bool valid;
  final int isUserConfigured;
  final int pGain;
  final int iGain;
  final int dGain;

  ControllerInfo({
    required this.valid,
    required this.isUserConfigured,
    required this.pGain,
    required this.iGain,
    required this.dGain,
  });
}

class Current {
  final bool valid;
  final int leftMotor;
  final int rightMotor;

  Current({
    required this.valid,
    required this.leftMotor,
    required this.rightMotor,
  });
}

class DockingIR {
  final bool valid;
  final int rightSignal;
  final int centralSignal;
  final int leftSignal;

  DockingIR({
    required this.valid,
    required this.rightSignal,
    required this.centralSignal,
    required this.leftSignal,
  });
}

class Feedback {
  final int epochTimeStamp;
  final BasicSensor basicSensor;
  final DockingIR dockingIr;
  final InertialSensor inertialSensor;
  final Cliff cliff;
  final Current current;
  final HardwareVersion hardwareVersion;
  final FirmwareVersion firmwareVersion;
  final Gyro gyro;
  final GeneralPurposeInput generalPurposeInput;
  final UniqueDeviceId uniqueDeviceId;
  final ControllerInfo controllerInfo;

  Feedback({
    required this.epochTimeStamp,
    required this.basicSensor,
    required this.dockingIr,
    required this.inertialSensor,
    required this.cliff,
    required this.current,
    required this.hardwareVersion,
    required this.firmwareVersion,
    required this.gyro,
    required this.generalPurposeInput,
    required this.uniqueDeviceId,
    required this.controllerInfo,
  });
}

class FirmwareVersion {
  final bool valid;
  final int patch;
  final int minor;
  final int major;

  FirmwareVersion({
    required this.valid,
    required this.patch,
    required this.minor,
    required this.major,
  });
}

class GeneralPurposeInput {
  final bool valid;
  final int dCh0;
  final int aCh0;
  final int aCh1;
  final int aCh2;
  final int aCh3;

  GeneralPurposeInput({
    required this.valid,
    required this.dCh0,
    required this.aCh0,
    required this.aCh1,
    required this.aCh2,
    required this.aCh3,
  });
}

class Gyro {
  final bool valid;
  final int frameId;
  final int followedDataLength;
  final int x0;
  final int y0;
  final int z0;
  final int x1;
  final int y1;
  final int z1;
  final int x2;
  final int y2;
  final int z2;

  Gyro({
    required this.valid,
    required this.frameId,
    required this.followedDataLength,
    required this.x0,
    required this.y0,
    required this.z0,
    required this.x1,
    required this.y1,
    required this.z1,
    required this.x2,
    required this.y2,
    required this.z2,
  });
}

class HardwareVersion {
  final bool valid;
  final int patch;
  final int minor;
  final int major;

  HardwareVersion({
    required this.valid,
    required this.patch,
    required this.minor,
    required this.major,
  });
}

class InertialSensor {
  final bool valid;
  final int angle;
  final int angleRate;

  InertialSensor({
    required this.valid,
    required this.angle,
    required this.angleRate,
  });
}

class UniqueDeviceId {
  final bool valid;
  final int udid0;
  final int udid1;
  final int udid2;

  UniqueDeviceId({
    required this.valid,
    required this.udid0,
    required this.udid1,
    required this.udid2,
  });
}

class Turtlebot2Impl extends FlutterRustBridgeBase<Turtlebot2Wire>
    implements Turtlebot2 {
  factory Turtlebot2Impl(ffi.DynamicLibrary dylib) =>
      Turtlebot2Impl.raw(Turtlebot2Wire(dylib));

  Turtlebot2Impl.raw(Turtlebot2Wire inner) : super(inner);

  Future<List<String>> availableTutlebots({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_available_tutlebots(port),
        parseSuccessData: _wire2api_StringList,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "available_tutlebots",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Stream<String> spawnTurtlebot({dynamic hint}) =>
      executeStream(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_spawn_turtlebot(port),
        parseSuccessData: _wire2api_String,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "spawn_turtlebot",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> sendToTurtlebot({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_send_to_turtlebot(port),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "send_to_turtlebot",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<List<Feedback>> receiveFromTurtlebot({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_receive_from_turtlebot(port),
        parseSuccessData: _wire2api_list_feedback,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "receive_from_turtlebot",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  // Section: api2wire

  // Section: api_fill_to_wire

}

// Section: wire2api
String _wire2api_String(dynamic raw) {
  return raw as String;
}

List<String> _wire2api_StringList(dynamic raw) {
  return (raw as List<dynamic>).cast<String>();
}

BasicSensor _wire2api_basic_sensor(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 13)
    throw Exception('unexpected arr length: expect 13 but see ${arr.length}');
  return BasicSensor(
    valid: _wire2api_bool(arr[0]),
    timeStamp: _wire2api_u32(arr[1]),
    bumper: _wire2api_u32(arr[2]),
    wheelDrop: _wire2api_u32(arr[3]),
    cliff: _wire2api_u32(arr[4]),
    leftEncoder: _wire2api_u32(arr[5]),
    rightEncoder: _wire2api_u32(arr[6]),
    leftPwm: _wire2api_u32(arr[7]),
    rightPwm: _wire2api_u32(arr[8]),
    button: _wire2api_u32(arr[9]),
    charger: _wire2api_u32(arr[10]),
    battery: _wire2api_u32(arr[11]),
    overcurrentFlags: _wire2api_u32(arr[12]),
  );
}

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
}

Cliff _wire2api_cliff(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return Cliff(
    valid: _wire2api_bool(arr[0]),
    rightCliffSensor: _wire2api_u32(arr[1]),
    centralCliffSensor: _wire2api_u32(arr[2]),
    leftCliffSensor: _wire2api_u32(arr[3]),
  );
}

ControllerInfo _wire2api_controller_info(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 5)
    throw Exception('unexpected arr length: expect 5 but see ${arr.length}');
  return ControllerInfo(
    valid: _wire2api_bool(arr[0]),
    isUserConfigured: _wire2api_u32(arr[1]),
    pGain: _wire2api_u32(arr[2]),
    iGain: _wire2api_u32(arr[3]),
    dGain: _wire2api_u32(arr[4]),
  );
}

Current _wire2api_current(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 3)
    throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
  return Current(
    valid: _wire2api_bool(arr[0]),
    leftMotor: _wire2api_u32(arr[1]),
    rightMotor: _wire2api_u32(arr[2]),
  );
}

DockingIR _wire2api_docking_ir(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return DockingIR(
    valid: _wire2api_bool(arr[0]),
    rightSignal: _wire2api_u32(arr[1]),
    centralSignal: _wire2api_u32(arr[2]),
    leftSignal: _wire2api_u32(arr[3]),
  );
}

Feedback _wire2api_feedback(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 12)
    throw Exception('unexpected arr length: expect 12 but see ${arr.length}');
  return Feedback(
    epochTimeStamp: _wire2api_i32(arr[0]),
    basicSensor: _wire2api_basic_sensor(arr[1]),
    dockingIr: _wire2api_docking_ir(arr[2]),
    inertialSensor: _wire2api_inertial_sensor(arr[3]),
    cliff: _wire2api_cliff(arr[4]),
    current: _wire2api_current(arr[5]),
    hardwareVersion: _wire2api_hardware_version(arr[6]),
    firmwareVersion: _wire2api_firmware_version(arr[7]),
    gyro: _wire2api_gyro(arr[8]),
    generalPurposeInput: _wire2api_general_purpose_input(arr[9]),
    uniqueDeviceId: _wire2api_unique_device_id(arr[10]),
    controllerInfo: _wire2api_controller_info(arr[11]),
  );
}

FirmwareVersion _wire2api_firmware_version(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return FirmwareVersion(
    valid: _wire2api_bool(arr[0]),
    patch: _wire2api_u32(arr[1]),
    minor: _wire2api_u32(arr[2]),
    major: _wire2api_u32(arr[3]),
  );
}

GeneralPurposeInput _wire2api_general_purpose_input(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 6)
    throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
  return GeneralPurposeInput(
    valid: _wire2api_bool(arr[0]),
    dCh0: _wire2api_u32(arr[1]),
    aCh0: _wire2api_u32(arr[2]),
    aCh1: _wire2api_u32(arr[3]),
    aCh2: _wire2api_u32(arr[4]),
    aCh3: _wire2api_u32(arr[5]),
  );
}

Gyro _wire2api_gyro(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 12)
    throw Exception('unexpected arr length: expect 12 but see ${arr.length}');
  return Gyro(
    valid: _wire2api_bool(arr[0]),
    frameId: _wire2api_u32(arr[1]),
    followedDataLength: _wire2api_u32(arr[2]),
    x0: _wire2api_u32(arr[3]),
    y0: _wire2api_u32(arr[4]),
    z0: _wire2api_u32(arr[5]),
    x1: _wire2api_u32(arr[6]),
    y1: _wire2api_u32(arr[7]),
    z1: _wire2api_u32(arr[8]),
    x2: _wire2api_u32(arr[9]),
    y2: _wire2api_u32(arr[10]),
    z2: _wire2api_u32(arr[11]),
  );
}

HardwareVersion _wire2api_hardware_version(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return HardwareVersion(
    valid: _wire2api_bool(arr[0]),
    patch: _wire2api_u32(arr[1]),
    minor: _wire2api_u32(arr[2]),
    major: _wire2api_u32(arr[3]),
  );
}

int _wire2api_i32(dynamic raw) {
  return raw as int;
}

InertialSensor _wire2api_inertial_sensor(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 3)
    throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
  return InertialSensor(
    valid: _wire2api_bool(arr[0]),
    angle: _wire2api_u32(arr[1]),
    angleRate: _wire2api_u32(arr[2]),
  );
}

List<Feedback> _wire2api_list_feedback(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_feedback).toList();
}

int _wire2api_u32(dynamic raw) {
  return raw as int;
}

int _wire2api_u8(dynamic raw) {
  return raw as int;
}

Uint8List _wire2api_uint_8_list(dynamic raw) {
  return raw as Uint8List;
}

UniqueDeviceId _wire2api_unique_device_id(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return UniqueDeviceId(
    valid: _wire2api_bool(arr[0]),
    udid0: _wire2api_u32(arr[1]),
    udid1: _wire2api_u32(arr[2]),
    udid2: _wire2api_u32(arr[3]),
  );
}

void _wire2api_unit(dynamic raw) {
  return;
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class Turtlebot2Wire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  Turtlebot2Wire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  Turtlebot2Wire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_available_tutlebots(
    int port_,
  ) {
    return _wire_available_tutlebots(
      port_,
    );
  }

  late final _wire_available_tutlebotsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_available_tutlebots');
  late final _wire_available_tutlebots =
      _wire_available_tutlebotsPtr.asFunction<void Function(int)>();

  void wire_spawn_turtlebot(
    int port_,
  ) {
    return _wire_spawn_turtlebot(
      port_,
    );
  }

  late final _wire_spawn_turtlebotPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_spawn_turtlebot');
  late final _wire_spawn_turtlebot =
      _wire_spawn_turtlebotPtr.asFunction<void Function(int)>();

  void wire_send_to_turtlebot(
    int port_,
  ) {
    return _wire_send_to_turtlebot(
      port_,
    );
  }

  late final _wire_send_to_turtlebotPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_send_to_turtlebot');
  late final _wire_send_to_turtlebot =
      _wire_send_to_turtlebotPtr.asFunction<void Function(int)>();

  void wire_receive_from_turtlebot(
    int port_,
  ) {
    return _wire_receive_from_turtlebot(
      port_,
    );
  }

  late final _wire_receive_from_turtlebotPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_receive_from_turtlebot');
  late final _wire_receive_from_turtlebot =
      _wire_receive_from_turtlebotPtr.asFunction<void Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
