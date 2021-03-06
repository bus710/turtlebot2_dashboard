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
  Stream<String> spawnTurtlebot({dynamic hint});

  Future<List<Feedback>> receiveFromTurtlebot({dynamic hint});

  Future<List<String>> searchPortCommand({dynamic hint});

  Future<void> openPortCommand({required String serialPort, dynamic hint});

  Future<void> closePortCommand({dynamic hint});

  Future<void> baseControlCommand(
      {required int speed, required int radius, dynamic hint});

  Future<void> soundCommand(
      {required int freq,
      required int amp,
      required int duration,
      dynamic hint});

  Future<void> soundSequenceCommand({required int seq, dynamic hint});

  Future<void> requestExtraCommand(
      {required bool hwVer,
      required bool fwVer,
      required bool udid,
      dynamic hint});

  Future<void> generalPurposeOutputCommand(
      {required bool dOutCh0,
      required bool dOutCh1,
      required bool dOutCh2,
      required bool dOutCh3,
      required bool power3V3,
      required bool power5V0,
      required bool power12V5A,
      required bool power12V1A5,
      required bool redLed1,
      required bool redLed2,
      required bool greenLed1,
      required bool greenLed2,
      dynamic hint});

  Future<void> setControllerGainCommand(
      {required bool isUserConfigured,
      required int p,
      required double i,
      required int d,
      dynamic hint});

  Future<void> getControllerGain({dynamic hint});
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
  final String epochTimeStamp;
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

  Future<List<String>> searchPortCommand({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_search_port_command(port),
        parseSuccessData: _wire2api_StringList,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "search_port_command",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> openPortCommand({required String serialPort, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) =>
            inner.wire_open_port_command(port, _api2wire_String(serialPort)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "open_port_command",
          argNames: ["serialPort"],
        ),
        argValues: [serialPort],
        hint: hint,
      ));

  Future<void> closePortCommand({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_close_port_command(port),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "close_port_command",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> baseControlCommand(
          {required int speed, required int radius, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_base_control_command(
            port, _api2wire_u16(speed), _api2wire_u16(radius)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "base_control_command",
          argNames: ["speed", "radius"],
        ),
        argValues: [speed, radius],
        hint: hint,
      ));

  Future<void> soundCommand(
          {required int freq,
          required int amp,
          required int duration,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_sound_command(port, _api2wire_u8(freq),
            _api2wire_u8(amp), _api2wire_u8(duration)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "sound_command",
          argNames: ["freq", "amp", "duration"],
        ),
        argValues: [freq, amp, duration],
        hint: hint,
      ));

  Future<void> soundSequenceCommand({required int seq, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) =>
            inner.wire_sound_sequence_command(port, _api2wire_u8(seq)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "sound_sequence_command",
          argNames: ["seq"],
        ),
        argValues: [seq],
        hint: hint,
      ));

  Future<void> requestExtraCommand(
          {required bool hwVer,
          required bool fwVer,
          required bool udid,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) =>
            inner.wire_request_extra_command(port, hwVer, fwVer, udid),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "request_extra_command",
          argNames: ["hwVer", "fwVer", "udid"],
        ),
        argValues: [hwVer, fwVer, udid],
        hint: hint,
      ));

  Future<void> generalPurposeOutputCommand(
          {required bool dOutCh0,
          required bool dOutCh1,
          required bool dOutCh2,
          required bool dOutCh3,
          required bool power3V3,
          required bool power5V0,
          required bool power12V5A,
          required bool power12V1A5,
          required bool redLed1,
          required bool redLed2,
          required bool greenLed1,
          required bool greenLed2,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_general_purpose_output_command(
            port,
            dOutCh0,
            dOutCh1,
            dOutCh2,
            dOutCh3,
            power3V3,
            power5V0,
            power12V5A,
            power12V1A5,
            redLed1,
            redLed2,
            greenLed1,
            greenLed2),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "general_purpose_output_command",
          argNames: [
            "dOutCh0",
            "dOutCh1",
            "dOutCh2",
            "dOutCh3",
            "power3V3",
            "power5V0",
            "power12V5A",
            "power12V1A5",
            "redLed1",
            "redLed2",
            "greenLed1",
            "greenLed2"
          ],
        ),
        argValues: [
          dOutCh0,
          dOutCh1,
          dOutCh2,
          dOutCh3,
          power3V3,
          power5V0,
          power12V5A,
          power12V1A5,
          redLed1,
          redLed2,
          greenLed1,
          greenLed2
        ],
        hint: hint,
      ));

  Future<void> setControllerGainCommand(
          {required bool isUserConfigured,
          required int p,
          required double i,
          required int d,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_set_controller_gain_command(
            port,
            isUserConfigured,
            _api2wire_u32(p),
            _api2wire_f32(i),
            _api2wire_u32(d)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "set_controller_gain_command",
          argNames: ["isUserConfigured", "p", "i", "d"],
        ),
        argValues: [isUserConfigured, p, i, d],
        hint: hint,
      ));

  Future<void> getControllerGain({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_get_controller_gain(port),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "get_controller_gain",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  // Section: api2wire
  ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
    return _api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  int _api2wire_bool(bool raw) {
    return raw ? 1 : 0;
  }

  double _api2wire_f32(double raw) {
    return raw;
  }

  int _api2wire_u16(int raw) {
    return raw;
  }

  int _api2wire_u32(int raw) {
    return raw;
  }

  int _api2wire_u8(int raw) {
    return raw;
  }

  ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

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
    epochTimeStamp: _wire2api_String(arr[0]),
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

  void wire_search_port_command(
    int port_,
  ) {
    return _wire_search_port_command(
      port_,
    );
  }

  late final _wire_search_port_commandPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_search_port_command');
  late final _wire_search_port_command =
      _wire_search_port_commandPtr.asFunction<void Function(int)>();

  void wire_open_port_command(
    int port_,
    ffi.Pointer<wire_uint_8_list> serial_port,
  ) {
    return _wire_open_port_command(
      port_,
      serial_port,
    );
  }

  late final _wire_open_port_commandPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_open_port_command');
  late final _wire_open_port_command = _wire_open_port_commandPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_close_port_command(
    int port_,
  ) {
    return _wire_close_port_command(
      port_,
    );
  }

  late final _wire_close_port_commandPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_close_port_command');
  late final _wire_close_port_command =
      _wire_close_port_commandPtr.asFunction<void Function(int)>();

  void wire_base_control_command(
    int port_,
    int speed,
    int radius,
  ) {
    return _wire_base_control_command(
      port_,
      speed,
      radius,
    );
  }

  late final _wire_base_control_commandPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Uint16, ffi.Uint16)>>('wire_base_control_command');
  late final _wire_base_control_command =
      _wire_base_control_commandPtr.asFunction<void Function(int, int, int)>();

  void wire_sound_command(
    int port_,
    int freq,
    int amp,
    int duration,
  ) {
    return _wire_sound_command(
      port_,
      freq,
      amp,
      duration,
    );
  }

  late final _wire_sound_commandPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Uint8, ffi.Uint8,
              ffi.Uint8)>>('wire_sound_command');
  late final _wire_sound_command =
      _wire_sound_commandPtr.asFunction<void Function(int, int, int, int)>();

  void wire_sound_sequence_command(
    int port_,
    int seq,
  ) {
    return _wire_sound_sequence_command(
      port_,
      seq,
    );
  }

  late final _wire_sound_sequence_commandPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint8)>>(
          'wire_sound_sequence_command');
  late final _wire_sound_sequence_command =
      _wire_sound_sequence_commandPtr.asFunction<void Function(int, int)>();

  void wire_request_extra_command(
    int port_,
    bool hw_ver,
    bool fw_ver,
    bool udid,
  ) {
    return _wire_request_extra_command(
      port_,
      hw_ver ? 1 : 0,
      fw_ver ? 1 : 0,
      udid ? 1 : 0,
    );
  }

  late final _wire_request_extra_commandPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Uint8, ffi.Uint8,
              ffi.Uint8)>>('wire_request_extra_command');
  late final _wire_request_extra_command = _wire_request_extra_commandPtr
      .asFunction<void Function(int, int, int, int)>();

  void wire_general_purpose_output_command(
    int port_,
    bool d_out_ch0,
    bool d_out_ch1,
    bool d_out_ch2,
    bool d_out_ch3,
    bool power_3v3,
    bool power_5v0,
    bool power_12v5a,
    bool power_12v1a5,
    bool red_led1,
    bool red_led2,
    bool green_led1,
    bool green_led2,
  ) {
    return _wire_general_purpose_output_command(
      port_,
      d_out_ch0 ? 1 : 0,
      d_out_ch1 ? 1 : 0,
      d_out_ch2 ? 1 : 0,
      d_out_ch3 ? 1 : 0,
      power_3v3 ? 1 : 0,
      power_5v0 ? 1 : 0,
      power_12v5a ? 1 : 0,
      power_12v1a5 ? 1 : 0,
      red_led1 ? 1 : 0,
      red_led2 ? 1 : 0,
      green_led1 ? 1 : 0,
      green_led2 ? 1 : 0,
    );
  }

  late final _wire_general_purpose_output_commandPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8,
              ffi.Uint8)>>('wire_general_purpose_output_command');
  late final _wire_general_purpose_output_command =
      _wire_general_purpose_output_commandPtr.asFunction<
          void Function(int, int, int, int, int, int, int, int, int, int, int,
              int, int)>();

  void wire_set_controller_gain_command(
    int port_,
    bool is_user_configured,
    int p,
    double i,
    int d,
  ) {
    return _wire_set_controller_gain_command(
      port_,
      is_user_configured ? 1 : 0,
      p,
      i,
      d,
    );
  }

  late final _wire_set_controller_gain_commandPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Uint8, ffi.Uint32, ffi.Float,
              ffi.Uint32)>>('wire_set_controller_gain_command');
  late final _wire_set_controller_gain_command =
      _wire_set_controller_gain_commandPtr
          .asFunction<void Function(int, int, int, double, int)>();

  void wire_get_controller_gain(
    int port_,
  ) {
    return _wire_get_controller_gain(
      port_,
    );
  }

  late final _wire_get_controller_gainPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_controller_gain');
  late final _wire_get_controller_gain =
      _wire_get_controller_gainPtr.asFunction<void Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list(
    int len,
  ) {
    return _new_uint_8_list(
      len,
    );
  }

  late final _new_uint_8_listPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list');
  late final _new_uint_8_list = _new_uint_8_listPtr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

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

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;

const int CMD_LEN_BASE_CONTROL = 7;

const int CMD_LEN_SOUND = 6;

const int CMD_LEN_SOUND_SEQUENCE = 4;

const int CMD_LEN_REQUEST_EXTRA = 5;

const int CMD_LEN_GENERAL_PURPOSE_OUTPUT = 5;

const int CMD_LEN_SET_CONTROLLER_GAIN = 16;

const int CMD_LEN_GET_CONTROLLER_GAIN = 4;

const int CMD_SIZE_BASE_CONTROL = 4;

const int CMD_SIZE_SOUND = 3;

const int CMD_SIZE_SOUND_SEQUENCE = 1;

const int CMD_SIZE_REQUEST_EXTRA = 2;

const int CMD_SIZE_GENERAL_PURPOSE_OUTPUT = 2;

const int CMD_SIZE_SET_CONTROLLER_GAIN = 13;

const int CMD_SIZE_GET_CONTROLLER_GAIN = 1;

const int FDB_SIZE_BASIC_SENSOR_DATA = 15;

const int FDB_SIZE_DOCKING_IR = 3;

const int FDB_SIZE_INERTIAL_SENSOR = 7;

const int FDB_SIZE_CLIFF = 6;

const int FDB_SIZE_CURRENT = 2;

const int FDB_SIZE_HARDWARE_VERSION = 4;

const int FDB_SIZE_FIRMWARE_VERSION = 4;

const int FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A = 14;

const int FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B = 20;

const int FDB_SIZE_GENERAL_PURPOSE_OUTPUT = 16;

const int FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER = 12;

const int FDB_SIZE_CONTROLLER_INFO = 13;
